//! Shared model helpers used by the handwritten SDK surface.
//!
//! The complete OpenAPI-derived request and response types live in [`crate::generated`]. This
//! module contains small convenience enums, legacy-compatible request helpers, fixed-point parsing,
//! and the extra-field map used by forward-compatible WebSocket models.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::error::{Error, Result};

/// Untyped JSON value alias used where Kalshi exposes intentionally flexible payloads.
pub type Json = Value;
/// Additional unknown fields preserved while deserializing forward-compatible models.
pub type ExtraFields = BTreeMap<String, Value>;

/// Market lifecycle status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MarketStatus {
    Initialized,
    Open,
    Closed,
    Settled,
    Determined,
    Finalized,
}

/// YES/NO contract side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Side {
    Yes,
    No,
}

/// Orderbook side.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BookSide {
    Bid,
    Ask,
}

/// Buy or sell order action.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum OrderAction {
    Buy,
    Sell,
}

/// Order type.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderType {
    Limit,
    Market,
}

/// Order time-in-force.
///
/// This enum implements `Display` and `From<TimeInForce> for String` so it can be passed directly
/// to generated builders that accept string-like time-in-force values.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TimeInForce {
    FillOrKill,
    GoodTillCanceled,
    ImmediateOrCancel,
}

impl TimeInForce {
    /// Returns the snake_case API value.
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::FillOrKill => "fill_or_kill",
            Self::GoodTillCanceled => "good_till_canceled",
            Self::ImmediateOrCancel => "immediate_or_cancel",
        }
    }
}

impl fmt::Display for TimeInForce {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

impl From<TimeInForce> for String {
    fn from(value: TimeInForce) -> Self {
        value.as_str().to_owned()
    }
}

/// Self-trade prevention behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SelfTradePreventionType {
    TakerAtCross,
    Maker,
}

/// Order status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum OrderStatus {
    Resting,
    Canceled,
    Executed,
}

/// Convenience limit-order request shape.
///
/// Prefer [`crate::generated::CreateOrderV2Request`] for full coverage of the current OpenAPI
/// schema. This type remains useful for simple handwritten examples and compatibility.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CreateOrderRequest {
    pub ticker: String,
    pub side: BookSide,
    pub count: String,
    pub price: String,
    pub time_in_force: TimeInForce,
    pub self_trade_prevention_type: SelfTradePreventionType,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

impl CreateOrderRequest {
    /// Builds a limit order request with default `good_till_canceled` time-in-force.
    pub fn limit(
        ticker: impl Into<String>,
        side: BookSide,
        count: impl Into<String>,
        price: impl Into<String>,
    ) -> Self {
        Self {
            ticker: ticker.into(),
            side,
            count: count.into(),
            price: price.into(),
            time_in_force: TimeInForce::GoodTillCanceled,
            self_trade_prevention_type: SelfTradePreventionType::TakerAtCross,
            client_order_id: None,
            expiration_time: None,
            post_only: None,
            cancel_order_on_pause: None,
            reduce_only: None,
            subaccount: None,
            order_group_id: None,
            exchange_index: None,
            extra: ExtraFields::new(),
        }
    }

    /// Builds a bid limit order request.
    pub fn bid(
        ticker: impl Into<String>,
        count: impl Into<String>,
        price: impl Into<String>,
    ) -> Self {
        Self::limit(ticker, BookSide::Bid, count, price)
    }

    /// Builds an ask limit order request.
    pub fn ask(
        ticker: impl Into<String>,
        count: impl Into<String>,
        price: impl Into<String>,
    ) -> Self {
        Self::limit(ticker, BookSide::Ask, count, price)
    }

    /// Sets a client-provided order id.
    pub fn client_order_id(mut self, client_order_id: impl Into<String>) -> Self {
        self.client_order_id = Some(client_order_id.into());
        self
    }

    /// Sets order time-in-force.
    pub fn time_in_force(mut self, time_in_force: TimeInForce) -> Self {
        self.time_in_force = time_in_force;
        self
    }

    /// Sets self-trade prevention behavior.
    pub fn self_trade_prevention_type(
        mut self,
        self_trade_prevention_type: SelfTradePreventionType,
    ) -> Self {
        self.self_trade_prevention_type = self_trade_prevention_type;
        self
    }

    /// Sets whether the order should post only.
    pub fn post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    /// Sets whether the order should only reduce exposure.
    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }
}

/// Convenience amend-order request shape.
///
/// Prefer [`crate::generated::AmendOrderV2Request`] for full coverage of the current OpenAPI
/// schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct AmendOrderRequest {
    pub ticker: String,
    pub side: BookSide,
    pub price: String,
    pub count: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub updated_client_order_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
    #[serde(flatten)]
    pub extra: ExtraFields,
}

impl AmendOrderRequest {
    /// Builds an amend request for price and count.
    pub fn new(
        ticker: impl Into<String>,
        side: BookSide,
        count: impl Into<String>,
        price: impl Into<String>,
    ) -> Self {
        Self {
            ticker: ticker.into(),
            side,
            price: price.into(),
            count: count.into(),
            client_order_id: None,
            updated_client_order_id: None,
            exchange_index: None,
            extra: ExtraFields::new(),
        }
    }
}

/// Convenience decrease-order request shape.
///
/// Prefer [`crate::generated::DecreaseOrderV2Request`] for full coverage of the current OpenAPI
/// schema.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct DecreaseOrderRequest {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_by: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reduce_to: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<i32>,
}

impl DecreaseOrderRequest {
    /// Decreases an order by an aggregate quantity.
    pub fn reduce_by(reduce_by: impl Into<String>) -> Self {
        Self {
            reduce_by: Some(reduce_by.into()),
            reduce_to: None,
            exchange_index: None,
        }
    }

    /// Decreases an order to an aggregate quantity.
    pub fn reduce_to(reduce_to: impl Into<String>) -> Self {
        Self {
            reduce_by: None,
            reduce_to: Some(reduce_to.into()),
            exchange_index: None,
        }
    }
}

/// Fixed-point decimal with four fractional digits.
///
/// Kalshi commonly exposes prices and quantities as string decimals such as `"0.0500"`. This type
/// parses those values into an orderable integer representation without floating-point rounding.
#[derive(Clone, Copy, Default, Eq, Hash, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct FixedPoint(i64);

impl FixedPoint {
    /// Number of integer units per one display unit.
    pub const SCALE: i64 = 10_000;

    /// Returns the raw scaled integer value.
    pub fn raw(self) -> i64 {
        self.0
    }

    /// Returns zero.
    pub fn zero() -> Self {
        Self(0)
    }

    /// Adds two fixed-point values, returning `None` on integer overflow.
    pub fn checked_add(self, other: Self) -> Option<Self> {
        self.0.checked_add(other.0).map(Self)
    }

    /// Returns true when the value is less than or equal to zero.
    pub fn is_zero_or_negative(self) -> bool {
        self.0 <= 0
    }
}

impl FromStr for FixedPoint {
    type Err = Error;

    fn from_str(input: &str) -> Result<Self> {
        parse_fixed_point(input)
    }
}

impl fmt::Debug for FixedPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Display::fmt(self, f)
    }
}

impl fmt::Display for FixedPoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let sign = if self.0 < 0 { "-" } else { "" };
        let value = self.0.abs();
        let whole = value / Self::SCALE;
        let frac = value % Self::SCALE;
        write!(f, "{sign}{whole}.{frac:04}")
    }
}

impl Ord for FixedPoint {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl PartialOrd for FixedPoint {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn parse_fixed_point(input: &str) -> Result<FixedPoint> {
    let input = input.trim();
    if input.is_empty() {
        return Err(Error::InvalidFixedPoint(input.to_owned()));
    }

    let negative = input.starts_with('-');
    let unsigned = input.strip_prefix('-').unwrap_or(input);
    let mut parts = unsigned.split('.');
    let whole = parts
        .next()
        .ok_or_else(|| Error::InvalidFixedPoint(input.to_owned()))?;
    let frac = parts.next().unwrap_or("");

    if parts.next().is_some() || frac.len() > 4 {
        return Err(Error::InvalidFixedPoint(input.to_owned()));
    }

    let whole: i64 = whole
        .parse()
        .map_err(|_| Error::InvalidFixedPoint(input.to_owned()))?;

    let mut frac_string = frac.to_owned();
    while frac_string.len() < 4 {
        frac_string.push('0');
    }

    let frac: i64 = if frac_string.is_empty() {
        0
    } else {
        frac_string
            .parse()
            .map_err(|_| Error::InvalidFixedPoint(input.to_owned()))?
    };

    let value = whole
        .checked_mul(FixedPoint::SCALE)
        .and_then(|whole| whole.checked_add(frac))
        .ok_or_else(|| Error::InvalidFixedPoint(input.to_owned()))?;

    Ok(FixedPoint(if negative { -value } else { value }))
}

#[cfg(test)]
mod tests {
    use super::FixedPoint;

    #[test]
    fn parses_fixed_point_strings() {
        assert_eq!(
            "12.3400".parse::<FixedPoint>().unwrap().to_string(),
            "12.3400"
        );
        assert_eq!("0.1".parse::<FixedPoint>().unwrap().to_string(), "0.1000");
        assert_eq!("-54.00".parse::<FixedPoint>().unwrap().raw(), -540000);
    }
}
