//! Typed WebSocket subscription builders.

use serde::ser::SerializeMap;
use serde::{Deserialize, Serialize, Serializer};
use serde_json::{Map, Value};

/// Kalshi WebSocket channel name.
#[derive(Debug, Clone, PartialEq, Eq, Hash, Deserialize)]
pub enum Channel {
    /// Aggregate orderbook snapshots and deltas.
    OrderbookDelta,
    /// Best bid/ask, last price, and related ticker updates.
    Ticker,
    /// Public trade prints.
    Trade,
    /// Private fills for the authenticated account.
    Fill,
    /// Private market position updates.
    MarketPositions,
    /// Market lifecycle updates.
    MarketLifecycleV2,
    /// Multivariate market lifecycle updates.
    MultivariateMarketLifecycle,
    /// Multivariate lookup updates.
    Multivariate,
    /// RFQ, quote, and block-trade communication updates.
    Communications,
    /// Order group updates.
    OrderGroupUpdates,
    /// Private user order updates.
    UserOrders,
    /// CF Benchmarks value updates.
    CfBenchmarksValue,
    /// Forward-compatible channel name.
    Custom(String),
}

impl Channel {
    /// Returns the channel name expected by Kalshi's WebSocket API.
    pub fn as_str(&self) -> &str {
        match self {
            Self::OrderbookDelta => "orderbook_delta",
            Self::Ticker => "ticker",
            Self::Trade => "trade",
            Self::Fill => "fill",
            Self::MarketPositions => "market_positions",
            Self::MarketLifecycleV2 => "market_lifecycle_v2",
            Self::MultivariateMarketLifecycle => "multivariate_market_lifecycle",
            Self::Multivariate => "multivariate",
            Self::Communications => "communications",
            Self::OrderGroupUpdates => "order_group_updates",
            Self::UserOrders => "user_orders",
            Self::CfBenchmarksValue => "cfbenchmarks_value",
            Self::Custom(value) => value,
        }
    }
}

impl Serialize for Channel {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

impl From<&str> for Channel {
    fn from(value: &str) -> Self {
        match value {
            "orderbook_delta" => Self::OrderbookDelta,
            "ticker" => Self::Ticker,
            "trade" => Self::Trade,
            "fill" => Self::Fill,
            "market_positions" => Self::MarketPositions,
            "market_lifecycle_v2" => Self::MarketLifecycleV2,
            "multivariate_market_lifecycle" => Self::MultivariateMarketLifecycle,
            "multivariate" => Self::Multivariate,
            "communications" => Self::Communications,
            "order_group_updates" => Self::OrderGroupUpdates,
            "user_orders" => Self::UserOrders,
            "cfbenchmarks_value" => Self::CfBenchmarksValue,
            other => Self::Custom(other.to_owned()),
        }
    }
}

/// WebSocket subscription update action.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum UpdateAction {
    /// Add market tickers to a subscription.
    AddMarkets,
    /// Remove market tickers from a subscription.
    DeleteMarkets,
    /// Request a fresh orderbook snapshot.
    GetSnapshot,
    /// Subscribe to CF Benchmarks index ids.
    SubscribeIndices,
    /// Unsubscribe from CF Benchmarks index ids.
    UnsubscribeIndices,
    /// List available CF Benchmarks indices.
    IndexList,
    /// Forward-compatible update action.
    Custom(String),
}

impl UpdateAction {
    /// Returns the action name expected by Kalshi's WebSocket API.
    pub fn as_str(&self) -> &str {
        match self {
            Self::AddMarkets => "add_markets",
            Self::DeleteMarkets => "delete_markets",
            Self::GetSnapshot => "get_snapshot",
            Self::SubscribeIndices => "subscribe_indices",
            Self::UnsubscribeIndices => "unsubscribe_indices",
            Self::IndexList => "indexlist",
            Self::Custom(value) => value,
        }
    }
}

impl Serialize for UpdateAction {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.as_str())
    }
}

/// Builder for WebSocket subscription parameters.
///
/// The builder serializes single values with Kalshi's singular parameter names
/// (`market_ticker`, `market_id`) and multiple values with plural names
/// (`market_tickers`, `market_ids`).
#[derive(Debug, Clone, PartialEq)]
pub struct Subscription {
    channels: Vec<Channel>,
    market_tickers: Vec<String>,
    market_ids: Vec<String>,
    index_ids: Vec<String>,
    params: Map<String, Value>,
}

impl Subscription {
    /// Creates a subscription for one or more channels.
    pub fn new(channels: impl IntoIterator<Item = Channel>) -> Self {
        Self {
            channels: channels.into_iter().collect(),
            market_tickers: Vec::new(),
            market_ids: Vec::new(),
            index_ids: Vec::new(),
            params: Map::new(),
        }
    }

    /// Creates a ticker subscription.
    pub fn ticker() -> Self {
        Self::new([Channel::Ticker])
    }

    /// Creates a public trades subscription.
    pub fn trades() -> Self {
        Self::new([Channel::Trade])
    }

    /// Creates an aggregate orderbook subscription.
    pub fn orderbook() -> Self {
        Self::new([Channel::OrderbookDelta])
    }

    /// Creates a private fills subscription.
    pub fn fills() -> Self {
        Self::new([Channel::Fill])
    }

    /// Creates a private market positions subscription.
    pub fn market_positions() -> Self {
        Self::new([Channel::MarketPositions])
    }

    /// Creates a private user orders subscription.
    pub fn user_orders() -> Self {
        Self::new([Channel::UserOrders])
    }

    /// Creates a market lifecycle subscription.
    pub fn market_lifecycle() -> Self {
        Self::new([Channel::MarketLifecycleV2])
    }

    /// Creates a multivariate market lifecycle subscription.
    pub fn multivariate_market_lifecycle() -> Self {
        Self::new([Channel::MultivariateMarketLifecycle])
    }

    /// Creates a communications subscription.
    pub fn communications() -> Self {
        Self::new([Channel::Communications])
    }

    /// Creates an order group updates subscription.
    pub fn order_group_updates() -> Self {
        Self::new([Channel::OrderGroupUpdates])
    }

    /// Creates a CF Benchmarks value subscription.
    pub fn cfbenchmarks_value() -> Self {
        Self::new([Channel::CfBenchmarksValue])
    }

    /// Adds additional channels to the subscription.
    pub fn channels(mut self, channels: impl IntoIterator<Item = Channel>) -> Self {
        self.channels.extend(channels);
        self
    }

    /// Adds market tickers to the subscription.
    pub fn markets(mut self, market_tickers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.market_tickers
            .extend(market_tickers.into_iter().map(Into::into));
        self
    }

    /// Adds market ids to the subscription.
    pub fn market_ids(mut self, market_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.market_ids
            .extend(market_ids.into_iter().map(Into::into));
        self
    }

    /// Adds CF Benchmarks index ids to the subscription.
    pub fn index_ids(mut self, index_ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.index_ids.extend(index_ids.into_iter().map(Into::into));
        self
    }

    /// Sets Kalshi's `send_initial_snapshot` subscription option.
    pub fn send_initial_snapshot(mut self, value: bool) -> Self {
        self.params
            .insert("send_initial_snapshot".to_owned(), Value::Bool(value));
        self
    }

    /// Sets Kalshi's `use_yes_price` subscription option for orderbook pricing.
    pub fn use_yes_price(mut self, value: bool) -> Self {
        self.params
            .insert("use_yes_price".to_owned(), Value::Bool(value));
        self
    }

    /// Sets Kalshi's `skip_ticker_ack` subscription option.
    pub fn skip_ticker_ack(mut self, value: bool) -> Self {
        self.params
            .insert("skip_ticker_ack".to_owned(), Value::Bool(value));
        self
    }

    /// Sets Kalshi's sharding parameters.
    pub fn shard(mut self, shard_key: u64, shard_factor: u64) -> Self {
        self.params
            .insert("shard_key".to_owned(), Value::Number(shard_key.into()));
        self.params.insert(
            "shard_factor".to_owned(),
            Value::Number(shard_factor.into()),
        );
        self
    }

    /// Adds a custom subscription parameter for forward compatibility.
    pub fn param(mut self, key: impl Into<String>, value: impl Into<Value>) -> Self {
        self.params.insert(key.into(), value.into());
        self
    }

    /// Serializes the subscription into the `params` object used by Kalshi commands.
    pub fn to_params_value(&self) -> Value {
        let mut params = self.params.clone();
        params.insert("channels".to_owned(), serde_json::json!(self.channels));

        insert_singular_or_plural(
            &mut params,
            "market_ticker",
            "market_tickers",
            &self.market_tickers,
        );
        insert_singular_or_plural(&mut params, "market_id", "market_ids", &self.market_ids);

        if !self.index_ids.is_empty() {
            params.insert("index_ids".to_owned(), serde_json::json!(self.index_ids));
        }

        Value::Object(params)
    }

    pub(crate) fn add_markets(
        &mut self,
        market_tickers: impl IntoIterator<Item = impl Into<String>>,
    ) {
        self.market_tickers
            .extend(market_tickers.into_iter().map(Into::into));
        self.market_tickers.sort();
        self.market_tickers.dedup();
    }

    pub(crate) fn delete_markets(&mut self, market_tickers: &[String]) {
        self.market_tickers
            .retain(|ticker| !market_tickers.contains(ticker));
    }

    pub(crate) fn add_index_ids(&mut self, index_ids: impl IntoIterator<Item = impl Into<String>>) {
        self.index_ids.extend(index_ids.into_iter().map(Into::into));
        self.index_ids.sort();
        self.index_ids.dedup();
    }

    pub(crate) fn delete_index_ids(&mut self, index_ids: &[String]) {
        self.index_ids
            .retain(|index_id| !index_ids.contains(index_id));
    }
}

impl Serialize for Subscription {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let value = self.to_params_value();
        match value {
            Value::Object(map) => {
                let mut serializer = serializer.serialize_map(Some(map.len()))?;
                for (key, value) in map {
                    serializer.serialize_entry(&key, &value)?;
                }
                serializer.end()
            }
            _ => unreachable!("subscription params are always an object"),
        }
    }
}

fn insert_singular_or_plural(
    params: &mut Map<String, Value>,
    singular: &str,
    plural: &str,
    values: &[String],
) {
    match values {
        [] => {}
        [one] => {
            params.insert(singular.to_owned(), Value::String(one.clone()));
        }
        many => {
            params.insert(plural.to_owned(), serde_json::json!(many));
        }
    }
}
