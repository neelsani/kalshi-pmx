//! Parsed WebSocket message types.
//!
//! Known message kinds deserialize into typed variants. Unknown message kinds are preserved as
//! [`StreamMessage::Raw`] so applications can log or handle newly introduced Kalshi events before
//! the SDK adds first-class support.

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::models::ExtraFields;

/// Raw WebSocket frame shape before the message kind is dispatched.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct RawStreamMessage {
    /// Message kind from the frame's `type` field.
    #[serde(rename = "type")]
    pub kind: String,
    /// Client command id, when the frame is a command acknowledgement.
    #[serde(default)]
    pub id: Option<u64>,
    /// Server subscription id, when the frame is tied to a subscription.
    #[serde(default)]
    pub sid: Option<u64>,
    /// Stream sequence number, when supplied by Kalshi.
    #[serde(default)]
    pub seq: Option<u64>,
    /// Raw message payload.
    #[serde(default)]
    pub msg: Value,
}

/// Envelope for data messages tied to a subscription.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Envelope<T> {
    /// Server subscription id.
    pub sid: Option<u64>,
    /// Stream sequence number.
    pub seq: Option<u64>,
    /// Typed message payload.
    pub msg: T,
}

/// Envelope for command acknowledgement messages.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct CommandEnvelope<T> {
    /// Client command id.
    pub id: Option<u64>,
    /// Server subscription id, when the command created or affected one.
    pub sid: Option<u64>,
    /// Stream sequence number, when supplied by Kalshi.
    pub seq: Option<u64>,
    /// Typed acknowledgement payload.
    pub msg: T,
}

/// Successful subscription acknowledgement payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct SubscribedMessage {
    /// Subscribed channel name.
    pub channel: String,
    /// Server subscription id.
    pub sid: u64,
    /// Additional fields preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Error payload returned by the WebSocket API.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct WsErrorMessage {
    /// Kalshi error code.
    pub code: i64,
    /// Human-readable error message.
    pub msg: String,
    /// Additional fields preserved for forward compatibility.
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Public ticker update payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TickerMessage {
    pub market_ticker: String,
    #[serde(default)]
    pub market_id: Option<String>,
    #[serde(default)]
    pub price_dollars: Option<String>,
    #[serde(default)]
    pub yes_bid_dollars: Option<String>,
    #[serde(default)]
    pub yes_ask_dollars: Option<String>,
    #[serde(default)]
    pub volume_fp: Option<String>,
    #[serde(default)]
    pub open_interest_fp: Option<String>,
    #[serde(default)]
    pub yes_bid_size_fp: Option<String>,
    #[serde(default)]
    pub yes_ask_size_fp: Option<String>,
    #[serde(default)]
    pub last_trade_size_fp: Option<String>,
    #[serde(default)]
    pub ts: Option<i64>,
    #[serde(default)]
    pub ts_ms: Option<i64>,
    #[serde(default)]
    pub time: Option<String>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Public trade print payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TradeMessage {
    pub trade_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub yes_price_dollars: Option<String>,
    #[serde(default)]
    pub no_price_dollars: Option<String>,
    #[serde(default)]
    pub count_fp: Option<String>,
    #[serde(default)]
    pub taker_side: Option<String>,
    #[serde(default)]
    pub ts: Option<i64>,
    #[serde(default)]
    pub ts_ms: Option<i64>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Private fill update payload for the authenticated account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct FillMessage {
    pub trade_id: String,
    pub order_id: String,
    pub market_ticker: String,
    #[serde(default)]
    pub is_taker: Option<bool>,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub yes_price_dollars: Option<String>,
    #[serde(default)]
    pub count_fp: Option<String>,
    #[serde(default)]
    pub action: Option<String>,
    #[serde(default)]
    pub post_position_fp: Option<String>,
    #[serde(default)]
    pub purchased_side: Option<String>,
    #[serde(default)]
    pub subaccount: Option<u64>,
    #[serde(default)]
    pub ts: Option<i64>,
    #[serde(default)]
    pub ts_ms: Option<i64>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Private user order update payload for the authenticated account.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct UserOrderMessage {
    pub order_id: String,
    #[serde(default)]
    pub user_id: Option<String>,
    #[serde(alias = "market_ticker")]
    pub ticker: String,
    pub status: String,
    #[serde(default)]
    pub side: Option<String>,
    #[serde(default)]
    pub is_yes: Option<bool>,
    #[serde(default)]
    pub yes_price_dollars: Option<String>,
    #[serde(default)]
    pub fill_count_fp: Option<String>,
    #[serde(default)]
    pub remaining_count_fp: Option<String>,
    #[serde(default)]
    pub initial_count_fp: Option<String>,
    #[serde(default)]
    pub client_order_id: Option<String>,
    #[serde(default)]
    pub order_group_id: Option<String>,
    #[serde(default)]
    pub created_time: Option<String>,
    #[serde(default)]
    pub created_ts_ms: Option<i64>,
    #[serde(default)]
    pub expiration_time: Option<String>,
    #[serde(default)]
    pub expiration_ts_ms: Option<i64>,
    #[serde(default)]
    pub subaccount_number: Option<u64>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Aggregate orderbook snapshot payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderbookSnapshot {
    pub market_ticker: String,
    #[serde(default)]
    pub market_id: Option<String>,
    #[serde(default)]
    pub yes_dollars_fp: Vec<(String, String)>,
    #[serde(default)]
    pub no_dollars_fp: Vec<(String, String)>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Aggregate orderbook delta payload.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OrderbookDelta {
    pub market_ticker: String,
    #[serde(default)]
    pub market_id: Option<String>,
    pub price_dollars: String,
    pub delta_fp: String,
    pub side: String,
    #[serde(default)]
    pub ts: Option<String>,
    #[serde(default)]
    pub ts_ms: Option<i64>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

/// Parsed Kalshi WebSocket message.
#[derive(Debug, Clone, PartialEq)]
pub enum StreamMessage {
    /// Subscription acknowledgement.
    Subscribed(CommandEnvelope<SubscribedMessage>),
    /// Unsubscribe acknowledgement.
    Unsubscribed(CommandEnvelope<Value>),
    /// Generic successful command acknowledgement.
    Ok(CommandEnvelope<Value>),
    /// Subscription list response.
    ListSubscriptions(CommandEnvelope<Value>),
    /// WebSocket API error.
    Error(CommandEnvelope<WsErrorMessage>),
    /// Full aggregate orderbook snapshot.
    OrderbookSnapshot(Envelope<OrderbookSnapshot>),
    /// Aggregate orderbook delta.
    OrderbookDelta(Envelope<OrderbookDelta>),
    /// Public ticker update.
    Ticker(Envelope<TickerMessage>),
    /// Public trade print.
    Trade(Envelope<TradeMessage>),
    /// Private fill update.
    Fill(Envelope<FillMessage>),
    /// Private market position update.
    MarketPosition(Envelope<Value>),
    /// Market lifecycle update.
    MarketLifecycleV2(Envelope<Value>),
    /// Event lifecycle update.
    EventLifecycle(Envelope<Value>),
    /// Event fee update.
    EventFeeUpdate(Envelope<Value>),
    /// Multivariate market lifecycle update.
    MultivariateMarketLifecycle(Envelope<Value>),
    /// Multivariate lookup update.
    MultivariateLookup(Envelope<Value>),
    /// RFQ, quote, or block-trade communication update.
    Communications(Envelope<Value>),
    /// Order group update.
    OrderGroupUpdates(Envelope<Value>),
    /// Private user order update.
    UserOrder(Envelope<UserOrderMessage>),
    /// CF Benchmarks value update.
    CfBenchmarksValue(Envelope<Value>),
    /// CF Benchmarks index-list update.
    CfBenchmarksIndexList(Envelope<Value>),
    /// Unknown message type preserved without loss.
    Raw(RawStreamMessage),
}

impl StreamMessage {
    /// Parses a text WebSocket frame into a typed stream message.
    pub fn parse_text(text: &str) -> serde_json::Result<Self> {
        let raw: RawStreamMessage = serde_json::from_str(text)?;
        Self::try_from_raw(raw)
    }

    /// Converts a raw message into a typed variant when the message kind is known.
    pub fn try_from_raw(raw: RawStreamMessage) -> serde_json::Result<Self> {
        Ok(match raw.kind.as_str() {
            "subscribed" => Self::Subscribed(raw.command_envelope()?),
            "unsubscribed" => Self::Unsubscribed(raw.command_envelope()?),
            "ok" => Self::Ok(raw.command_envelope()?),
            "list_subscriptions" => Self::ListSubscriptions(raw.command_envelope()?),
            "error" => Self::Error(raw.command_envelope()?),
            "orderbook_snapshot" => Self::OrderbookSnapshot(raw.envelope()?),
            "orderbook_delta" => Self::OrderbookDelta(raw.envelope()?),
            "ticker" => Self::Ticker(raw.envelope()?),
            "trade" => Self::Trade(raw.envelope()?),
            "fill" => Self::Fill(raw.envelope()?),
            "market_position" => Self::MarketPosition(raw.envelope()?),
            "market_lifecycle_v2" => Self::MarketLifecycleV2(raw.envelope()?),
            "event_lifecycle" => Self::EventLifecycle(raw.envelope()?),
            "event_fee_update" => Self::EventFeeUpdate(raw.envelope()?),
            "multivariate_market_lifecycle" => Self::MultivariateMarketLifecycle(raw.envelope()?),
            "multivariate_lookup" => Self::MultivariateLookup(raw.envelope()?),
            "rfq_created" | "rfq_deleted" | "quote_created" | "quote_accepted"
            | "quote_executed" => Self::Communications(raw.envelope()?),
            "order_group_updates" => Self::OrderGroupUpdates(raw.envelope()?),
            "user_order" => Self::UserOrder(raw.envelope()?),
            "cfbenchmarks_value" => Self::CfBenchmarksValue(raw.envelope()?),
            "cfbenchmarks_value_indexlist" => Self::CfBenchmarksIndexList(raw.envelope()?),
            _ => Self::Raw(raw),
        })
    }
}

impl RawStreamMessage {
    fn envelope<T>(self) -> serde_json::Result<Envelope<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        Ok(Envelope {
            sid: self.sid,
            seq: self.seq,
            msg: serde_json::from_value(self.msg)?,
        })
    }

    fn command_envelope<T>(self) -> serde_json::Result<CommandEnvelope<T>>
    where
        T: for<'de> Deserialize<'de>,
    {
        Ok(CommandEnvelope {
            id: self.id,
            sid: self.sid,
            seq: self.seq,
            msg: serde_json::from_value(self.msg)?,
        })
    }
}
