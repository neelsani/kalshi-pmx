//! Local aggregate orderbook maintenance from WebSocket messages.

use std::collections::BTreeMap;

use crate::error::{Error, Result};
use crate::models::{FixedPoint, Side};
use crate::ws::message::{Envelope, OrderbookDelta, OrderbookSnapshot, StreamMessage};

/// Local orderbook state maintained from Kalshi orderbook snapshots and deltas.
///
/// The book rejects deltas that arrive before a snapshot, target a different market, contain an
/// unknown side, or skip sequence numbers.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct LiveOrderbook {
    /// Market ticker for the current snapshot.
    pub market_ticker: Option<String>,
    /// Market id for the current snapshot when supplied by the stream.
    pub market_id: Option<String>,
    /// Last applied WebSocket sequence number.
    pub last_seq: Option<u64>,
    yes: BTreeMap<FixedPoint, FixedPoint>,
    no: BTreeMap<FixedPoint, FixedPoint>,
}

impl LiveOrderbook {
    /// Creates an empty orderbook.
    pub fn new() -> Self {
        Self::default()
    }

    /// Returns aggregate YES bid levels keyed by fixed-point price.
    pub fn yes(&self) -> &BTreeMap<FixedPoint, FixedPoint> {
        &self.yes
    }

    /// Returns aggregate NO bid levels keyed by fixed-point price.
    pub fn no(&self) -> &BTreeMap<FixedPoint, FixedPoint> {
        &self.no
    }

    /// Returns the highest YES bid price and aggregate quantity.
    pub fn best_yes_bid(&self) -> Option<(FixedPoint, FixedPoint)> {
        self.yes
            .iter()
            .next_back()
            .map(|(price, qty)| (*price, *qty))
    }

    /// Returns the highest NO bid price and aggregate quantity.
    pub fn best_no_bid(&self) -> Option<(FixedPoint, FixedPoint)> {
        self.no
            .iter()
            .next_back()
            .map(|(price, qty)| (*price, *qty))
    }

    /// Applies an orderbook snapshot or delta stream message.
    ///
    /// Returns `Ok(true)` when the message changed the book, `Ok(false)` for unrelated messages,
    /// and an error when a delta cannot be safely applied.
    pub fn apply_message(&mut self, message: &StreamMessage) -> Result<bool> {
        match message {
            StreamMessage::OrderbookSnapshot(envelope) => {
                self.apply_snapshot(envelope)?;
                Ok(true)
            }
            StreamMessage::OrderbookDelta(envelope) => {
                self.apply_delta(envelope)?;
                Ok(true)
            }
            _ => Ok(false),
        }
    }

    /// Replaces local state from a full orderbook snapshot.
    pub fn apply_snapshot(&mut self, envelope: &Envelope<OrderbookSnapshot>) -> Result<()> {
        self.market_ticker = Some(envelope.msg.market_ticker.clone());
        self.market_id = envelope.msg.market_id.clone();
        self.last_seq = envelope.seq;
        self.yes = parse_levels(&envelope.msg.yes_dollars_fp)?;
        self.no = parse_levels(&envelope.msg.no_dollars_fp)?;
        Ok(())
    }

    /// Applies one aggregate orderbook delta to local state.
    pub fn apply_delta(&mut self, envelope: &Envelope<OrderbookDelta>) -> Result<()> {
        let Some(market_ticker) = &self.market_ticker else {
            return Err(Error::OrderbookOutOfSync(
                "received delta before snapshot".to_owned(),
            ));
        };
        if market_ticker != &envelope.msg.market_ticker {
            return Err(Error::OrderbookOutOfSync(format!(
                "received delta for {}, but current book is {}",
                envelope.msg.market_ticker, market_ticker
            )));
        }

        if let (Some(current_id), Some(delta_id)) = (&self.market_id, &envelope.msg.market_id)
            && current_id != delta_id
        {
            return Err(Error::OrderbookOutOfSync(format!(
                "received delta for market id {delta_id}, but current book is {current_id}"
            )));
        }

        if let (Some(last_seq), Some(seq)) = (self.last_seq, envelope.seq)
            && seq != last_seq + 1
        {
            return Err(Error::OrderbookOutOfSync(format!(
                "expected sequence {}, got {seq}",
                last_seq + 1
            )));
        }

        let price = envelope.msg.price_dollars.parse::<FixedPoint>()?;
        let delta = envelope.msg.delta_fp.parse::<FixedPoint>()?;
        let side = match envelope.msg.side.as_str() {
            "yes" => Side::Yes,
            "no" => Side::No,
            side => {
                return Err(Error::OrderbookOutOfSync(format!(
                    "unknown orderbook side `{side}`"
                )));
            }
        };
        self.last_seq = envelope.seq.or(self.last_seq);
        let levels = match side {
            Side::Yes => &mut self.yes,
            Side::No => &mut self.no,
        };

        let next_qty = levels
            .get(&price)
            .copied()
            .unwrap_or_else(FixedPoint::zero)
            .checked_add(delta)
            .unwrap_or_else(FixedPoint::zero);

        if next_qty.is_zero_or_negative() {
            levels.remove(&price);
        } else {
            levels.insert(price, next_qty);
        }

        Ok(())
    }
}

fn parse_levels(levels: &[(String, String)]) -> Result<BTreeMap<FixedPoint, FixedPoint>> {
    let mut map = BTreeMap::new();
    for (price, quantity) in levels {
        map.insert(price.parse()?, quantity.parse()?);
    }
    Ok(map)
}
