//! Generated OpenAPI query parameter structs.
//! Regenerate with `cargo run --manifest-path xtask/Cargo.toml -- generate-openapi-types`.

use crate::generated::*;

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSeriesFeeChangesParams {
    #[serde(rename = "series_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "show_historical")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub show_historical: Option<bool>,
}

impl GetSeriesFeeChangesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn series_ticker(mut self, series_ticker: impl Into<String>) -> Self {
        self.series_ticker = Some(series_ticker.into());
        self
    }

    pub fn show_historical(mut self, show_historical: bool) -> Self {
        self.show_historical = Some(show_historical);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketCandlesticksParams {
    #[serde(rename = "start_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<i64>,
    #[serde(rename = "end_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<i64>,
    #[serde(rename = "period_interval")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_interval: Option<i64>,
    #[serde(rename = "include_latest_before_start")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_latest_before_start: Option<bool>,
}

impl GetMarketCandlesticksParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_ts(mut self, start_ts: i64) -> Self {
        self.start_ts = Some(start_ts);
        self
    }

    pub fn end_ts(mut self, end_ts: i64) -> Self {
        self.end_ts = Some(end_ts);
        self
    }

    pub fn period_interval(mut self, period_interval: i64) -> Self {
        self.period_interval = Some(period_interval);
        self
    }

    pub fn include_latest_before_start(mut self, include_latest_before_start: bool) -> Self {
        self.include_latest_before_start = Some(include_latest_before_start);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetTradesParams {
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "min_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(rename = "max_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(rename = "is_block_trade")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_block_trade: Option<bool>,
}

impl GetTradesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn min_ts(mut self, min_ts: i64) -> Self {
        self.min_ts = Some(min_ts);
        self
    }

    pub fn max_ts(mut self, max_ts: i64) -> Self {
        self.max_ts = Some(max_ts);
        self
    }

    pub fn is_block_trade(mut self, is_block_trade: bool) -> Self {
        self.is_block_trade = Some(is_block_trade);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketOrderbookParams {
    #[serde(rename = "depth")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub depth: Option<i64>,
}

impl GetMarketOrderbookParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn depth(mut self, depth: i64) -> Self {
        self.depth = Some(depth);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketOrderbooksParams {
    #[serde(rename = "tickers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tickers: Option<Vec<String>>,
}

impl GetMarketOrderbooksParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn tickers(mut self, tickers: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.tickers = Some(tickers.into_iter().map(Into::into).collect());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSeriesParams {
    #[serde(rename = "include_volume")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_volume: Option<bool>,
}

impl GetSeriesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn include_volume(mut self, include_volume: bool) -> Self {
        self.include_volume = Some(include_volume);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSeriesListParams {
    #[serde(rename = "category")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<String>,
    #[serde(rename = "include_product_metadata")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_product_metadata: Option<bool>,
    #[serde(rename = "include_volume")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_volume: Option<bool>,
    #[serde(rename = "min_updated_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<i64>,
}

impl GetSeriesListParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn tags(mut self, tags: impl Into<String>) -> Self {
        self.tags = Some(tags.into());
        self
    }

    pub fn include_product_metadata(mut self, include_product_metadata: bool) -> Self {
        self.include_product_metadata = Some(include_product_metadata);
        self
    }

    pub fn include_volume(mut self, include_volume: bool) -> Self {
        self.include_volume = Some(include_volume);
        self
    }

    pub fn min_updated_ts(mut self, min_updated_ts: i64) -> Self {
        self.min_updated_ts = Some(min_updated_ts);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketsParams {
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "series_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "min_created_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_created_ts: Option<i64>,
    #[serde(rename = "max_created_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_created_ts: Option<i64>,
    #[serde(rename = "min_updated_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<i64>,
    #[serde(rename = "max_close_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_close_ts: Option<i64>,
    #[serde(rename = "min_close_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_close_ts: Option<i64>,
    #[serde(rename = "min_settled_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_settled_ts: Option<i64>,
    #[serde(rename = "max_settled_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_settled_ts: Option<i64>,
    #[serde(rename = "status")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "tickers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    #[serde(rename = "mve_filter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mve_filter: Option<String>,
}

impl GetMarketsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn event_ticker(mut self, event_ticker: impl Into<String>) -> Self {
        self.event_ticker = Some(event_ticker.into());
        self
    }

    pub fn series_ticker(mut self, series_ticker: impl Into<String>) -> Self {
        self.series_ticker = Some(series_ticker.into());
        self
    }

    pub fn min_created_ts(mut self, min_created_ts: i64) -> Self {
        self.min_created_ts = Some(min_created_ts);
        self
    }

    pub fn max_created_ts(mut self, max_created_ts: i64) -> Self {
        self.max_created_ts = Some(max_created_ts);
        self
    }

    pub fn min_updated_ts(mut self, min_updated_ts: i64) -> Self {
        self.min_updated_ts = Some(min_updated_ts);
        self
    }

    pub fn max_close_ts(mut self, max_close_ts: i64) -> Self {
        self.max_close_ts = Some(max_close_ts);
        self
    }

    pub fn min_close_ts(mut self, min_close_ts: i64) -> Self {
        self.min_close_ts = Some(min_close_ts);
        self
    }

    pub fn min_settled_ts(mut self, min_settled_ts: i64) -> Self {
        self.min_settled_ts = Some(min_settled_ts);
        self
    }

    pub fn max_settled_ts(mut self, max_settled_ts: i64) -> Self {
        self.max_settled_ts = Some(max_settled_ts);
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn tickers(mut self, tickers: impl Into<String>) -> Self {
        self.tickers = Some(tickers.into());
        self
    }

    pub fn mve_filter(mut self, mve_filter: impl Into<String>) -> Self {
        self.mve_filter = Some(mve_filter.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchGetMarketCandlesticksParams {
    #[serde(rename = "market_tickers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<String>,
    #[serde(rename = "start_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<i64>,
    #[serde(rename = "end_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<i64>,
    #[serde(rename = "period_interval")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_interval: Option<i64>,
    #[serde(rename = "include_latest_before_start")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_latest_before_start: Option<bool>,
}

impl BatchGetMarketCandlesticksParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn market_tickers(mut self, market_tickers: impl Into<String>) -> Self {
        self.market_tickers = Some(market_tickers.into());
        self
    }

    pub fn start_ts(mut self, start_ts: i64) -> Self {
        self.start_ts = Some(start_ts);
        self
    }

    pub fn end_ts(mut self, end_ts: i64) -> Self {
        self.end_ts = Some(end_ts);
        self
    }

    pub fn period_interval(mut self, period_interval: i64) -> Self {
        self.period_interval = Some(period_interval);
        self
    }

    pub fn include_latest_before_start(mut self, include_latest_before_start: bool) -> Self {
        self.include_latest_before_start = Some(include_latest_before_start);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketCandlesticksByEventParams {
    #[serde(rename = "start_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<i64>,
    #[serde(rename = "end_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<i64>,
    #[serde(rename = "period_interval")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_interval: Option<i64>,
}

impl GetMarketCandlesticksByEventParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_ts(mut self, start_ts: i64) -> Self {
        self.start_ts = Some(start_ts);
        self
    }

    pub fn end_ts(mut self, end_ts: i64) -> Self {
        self.end_ts = Some(end_ts);
        self
    }

    pub fn period_interval(mut self, period_interval: i64) -> Self {
        self.period_interval = Some(period_interval);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetEventsParams {
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "with_nested_markets")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_nested_markets: Option<bool>,
    #[serde(rename = "with_milestones")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_milestones: Option<bool>,
    #[serde(rename = "status")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "series_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "tickers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    #[serde(rename = "min_close_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_close_ts: Option<i64>,
    #[serde(rename = "min_updated_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<i64>,
}

impl GetEventsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn with_nested_markets(mut self, with_nested_markets: bool) -> Self {
        self.with_nested_markets = Some(with_nested_markets);
        self
    }

    pub fn with_milestones(mut self, with_milestones: bool) -> Self {
        self.with_milestones = Some(with_milestones);
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn series_ticker(mut self, series_ticker: impl Into<String>) -> Self {
        self.series_ticker = Some(series_ticker.into());
        self
    }

    pub fn tickers(mut self, tickers: impl Into<String>) -> Self {
        self.tickers = Some(tickers.into());
        self
    }

    pub fn min_close_ts(mut self, min_close_ts: i64) -> Self {
        self.min_close_ts = Some(min_close_ts);
        self
    }

    pub fn min_updated_ts(mut self, min_updated_ts: i64) -> Self {
        self.min_updated_ts = Some(min_updated_ts);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMultivariateEventsParams {
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "series_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "collection_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub collection_ticker: Option<String>,
    #[serde(rename = "with_nested_markets")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_nested_markets: Option<bool>,
}

impl GetMultivariateEventsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn series_ticker(mut self, series_ticker: impl Into<String>) -> Self {
        self.series_ticker = Some(series_ticker.into());
        self
    }

    pub fn collection_ticker(mut self, collection_ticker: impl Into<String>) -> Self {
        self.collection_ticker = Some(collection_ticker.into());
        self
    }

    pub fn with_nested_markets(mut self, with_nested_markets: bool) -> Self {
        self.with_nested_markets = Some(with_nested_markets);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetEventFeeChangesParams {
    #[serde(rename = "event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetEventFeeChangesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn event_ticker(mut self, event_ticker: impl Into<String>) -> Self {
        self.event_ticker = Some(event_ticker.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetEventParams {
    #[serde(rename = "with_nested_markets")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_nested_markets: Option<bool>,
}

impl GetEventParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_nested_markets(mut self, with_nested_markets: bool) -> Self {
        self.with_nested_markets = Some(with_nested_markets);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetEventForecastPercentilesHistoryParams {
    #[serde(rename = "percentiles")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub percentiles: Option<Vec<i64>>,
    #[serde(rename = "start_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<i64>,
    #[serde(rename = "end_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<i64>,
    #[serde(rename = "period_interval")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_interval: Option<i64>,
}

impl GetEventForecastPercentilesHistoryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn percentiles(mut self, percentiles: Vec<i64>) -> Self {
        self.percentiles = Some(percentiles);
        self
    }

    pub fn start_ts(mut self, start_ts: i64) -> Self {
        self.start_ts = Some(start_ts);
        self
    }

    pub fn end_ts(mut self, end_ts: i64) -> Self {
        self.end_ts = Some(end_ts);
        self
    }

    pub fn period_interval(mut self, period_interval: i64) -> Self {
        self.period_interval = Some(period_interval);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrdersParams {
    #[serde(rename = "ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "min_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(rename = "max_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(rename = "status")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

impl GetOrdersParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn event_ticker(mut self, event_ticker: impl Into<String>) -> Self {
        self.event_ticker = Some(event_ticker.into());
        self
    }

    pub fn min_ts(mut self, min_ts: i64) -> Self {
        self.min_ts = Some(min_ts);
        self
    }

    pub fn max_ts(mut self, max_ts: i64) -> Self {
        self.max_ts = Some(max_ts);
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrderQueuePositionsParams {
    #[serde(rename = "market_tickers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_tickers: Option<String>,
    #[serde(rename = "event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

impl GetOrderQueuePositionsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn market_tickers(mut self, market_tickers: impl Into<String>) -> Self {
        self.market_tickers = Some(market_tickers.into());
        self
    }

    pub fn event_ticker(mut self, event_ticker: impl Into<String>) -> Self {
        self.event_ticker = Some(event_ticker.into());
        self
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelOrderV2Params {
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(rename = "market_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
}

impl CancelOrderV2Params {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    pub fn exchange_index(mut self, exchange_index: ExchangeIndex) -> Self {
        self.exchange_index = Some(exchange_index);
        self
    }

    pub fn market_ticker(mut self, market_ticker: impl Into<String>) -> Self {
        self.market_ticker = Some(market_ticker.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AmendOrderV2Params {
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

impl AmendOrderV2Params {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreaseOrderV2Params {
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

impl DecreaseOrderV2Params {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrderGroupsParams {
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

impl GetOrderGroupsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrderGroupParams {
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

impl GetOrderGroupParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DeleteOrderGroupParams {
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
}

impl DeleteOrderGroupParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    pub fn exchange_index(mut self, exchange_index: ExchangeIndex) -> Self {
        self.exchange_index = Some(exchange_index);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ResetOrderGroupParams {
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
}

impl ResetOrderGroupParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    pub fn exchange_index(mut self, exchange_index: ExchangeIndex) -> Self {
        self.exchange_index = Some(exchange_index);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TriggerOrderGroupParams {
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
}

impl TriggerOrderGroupParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    pub fn exchange_index(mut self, exchange_index: ExchangeIndex) -> Self {
        self.exchange_index = Some(exchange_index);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOrderGroupLimitParams {
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
}

impl UpdateOrderGroupLimitParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn exchange_index(mut self, exchange_index: ExchangeIndex) -> Self {
        self.exchange_index = Some(exchange_index);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetBalanceParams {
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

impl GetBalanceParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSubaccountTransfersParams {
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetSubaccountTransfersParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetPositionsParams {
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "count_filter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count_filter: Option<String>,
    #[serde(rename = "ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

impl GetPositionsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn count_filter(mut self, count_filter: impl Into<String>) -> Self {
        self.count_filter = Some(count_filter.into());
        self
    }

    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn event_ticker(mut self, event_ticker: impl Into<String>) -> Self {
        self.event_ticker = Some(event_ticker.into());
        self
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSettlementsParams {
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "min_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(rename = "max_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

impl GetSettlementsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn event_ticker(mut self, event_ticker: impl Into<String>) -> Self {
        self.event_ticker = Some(event_ticker.into());
        self
    }

    pub fn min_ts(mut self, min_ts: i64) -> Self {
        self.min_ts = Some(min_ts);
        self
    }

    pub fn max_ts(mut self, max_ts: i64) -> Self {
        self.max_ts = Some(max_ts);
        self
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetDepositsParams {
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetDepositsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetWithdrawalsParams {
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetWithdrawalsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetFillsParams {
    #[serde(rename = "ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_id: Option<String>,
    #[serde(rename = "min_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(rename = "max_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
}

impl GetFillsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn order_id(mut self, order_id: impl Into<String>) -> Self {
        self.order_id = Some(order_id.into());
        self
    }

    pub fn min_ts(mut self, min_ts: i64) -> Self {
        self.min_ts = Some(min_ts);
        self
    }

    pub fn max_ts(mut self, max_ts: i64) -> Self {
        self.max_ts = Some(max_ts);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetBlockTradeProposalsParams {
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "market_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "status")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl GetBlockTradeProposalsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn market_ticker(mut self, market_ticker: impl Into<String>) -> Self {
        self.market_ticker = Some(market_ticker.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetRFQsParams {
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "market_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "status")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "creator_user_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    #[serde(rename = "user_filter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_filter: Option<UserFilter>,
}

impl GetRFQsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn event_ticker(mut self, event_ticker: impl Into<String>) -> Self {
        self.event_ticker = Some(event_ticker.into());
        self
    }

    pub fn market_ticker(mut self, market_ticker: impl Into<String>) -> Self {
        self.market_ticker = Some(market_ticker.into());
        self
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn creator_user_id(mut self, creator_user_id: impl Into<String>) -> Self {
        self.creator_user_id = Some(creator_user_id.into());
        self
    }

    pub fn user_filter(mut self, user_filter: UserFilter) -> Self {
        self.user_filter = Some(user_filter);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetQuotesParams {
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "min_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(rename = "max_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "status")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "quote_creator_user_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub quote_creator_user_id: Option<String>,
    #[serde(rename = "user_filter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub user_filter: Option<UserFilter>,
    #[serde(rename = "rfq_user_filter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_user_filter: Option<UserFilter>,
    #[serde(rename = "rfq_creator_user_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_user_id: Option<String>,
    #[serde(rename = "rfq_creator_subtrader_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_subtrader_id: Option<String>,
    #[serde(rename = "rfq_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_id: Option<String>,
}

impl GetQuotesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn min_ts(mut self, min_ts: i64) -> Self {
        self.min_ts = Some(min_ts);
        self
    }

    pub fn max_ts(mut self, max_ts: i64) -> Self {
        self.max_ts = Some(max_ts);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn quote_creator_user_id(mut self, quote_creator_user_id: impl Into<String>) -> Self {
        self.quote_creator_user_id = Some(quote_creator_user_id.into());
        self
    }

    pub fn user_filter(mut self, user_filter: UserFilter) -> Self {
        self.user_filter = Some(user_filter);
        self
    }

    pub fn rfq_user_filter(mut self, rfq_user_filter: UserFilter) -> Self {
        self.rfq_user_filter = Some(rfq_user_filter);
        self
    }

    pub fn rfq_creator_user_id(mut self, rfq_creator_user_id: impl Into<String>) -> Self {
        self.rfq_creator_user_id = Some(rfq_creator_user_id.into());
        self
    }

    pub fn rfq_creator_subtrader_id(mut self, rfq_creator_subtrader_id: impl Into<String>) -> Self {
        self.rfq_creator_subtrader_id = Some(rfq_creator_subtrader_id.into());
        self
    }

    pub fn rfq_id(mut self, rfq_id: impl Into<String>) -> Self {
        self.rfq_id = Some(rfq_id.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetLiveDataByMilestoneParams {
    #[serde(rename = "include_player_stats")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_player_stats: Option<bool>,
}

impl GetLiveDataByMilestoneParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn include_player_stats(mut self, include_player_stats: bool) -> Self {
        self.include_player_stats = Some(include_player_stats);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetLiveDataParams {
    #[serde(rename = "include_player_stats")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_player_stats: Option<bool>,
}

impl GetLiveDataParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn include_player_stats(mut self, include_player_stats: bool) -> Self {
        self.include_player_stats = Some(include_player_stats);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetLiveDatasParams {
    #[serde(rename = "milestone_ids")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestone_ids: Option<Vec<String>>,
    #[serde(rename = "include_player_stats")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub include_player_stats: Option<bool>,
}

impl GetLiveDatasParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn milestone_ids(
        mut self,
        milestone_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.milestone_ids = Some(milestone_ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn include_player_stats(mut self, include_player_stats: bool) -> Self {
        self.include_player_stats = Some(include_player_stats);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetStructuredTargetsParams {
    #[serde(rename = "ids")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "competition")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub competition: Option<String>,
    #[serde(rename = "page_size")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub page_size: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetStructuredTargetsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ids(mut self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn competition(mut self, competition: impl Into<String>) -> Self {
        self.competition = Some(competition.into());
        self
    }

    pub fn page_size(mut self, page_size: i64) -> Self {
        self.page_size = Some(page_size);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMilestonesParams {
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "minimum_start_date")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub minimum_start_date: Option<String>,
    #[serde(rename = "category")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "competition")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub competition: Option<String>,
    #[serde(rename = "source_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "related_event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub related_event_ticker: Option<String>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "min_updated_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_updated_ts: Option<i64>,
}

impl GetMilestonesParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn minimum_start_date(mut self, minimum_start_date: impl Into<String>) -> Self {
        self.minimum_start_date = Some(minimum_start_date.into());
        self
    }

    pub fn category(mut self, category: impl Into<String>) -> Self {
        self.category = Some(category.into());
        self
    }

    pub fn competition(mut self, competition: impl Into<String>) -> Self {
        self.competition = Some(competition.into());
        self
    }

    pub fn source_id(mut self, source_id: impl Into<String>) -> Self {
        self.source_id = Some(source_id.into());
        self
    }

    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn related_event_ticker(mut self, related_event_ticker: impl Into<String>) -> Self {
        self.related_event_ticker = Some(related_event_ticker.into());
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn min_updated_ts(mut self, min_updated_ts: i64) -> Self {
        self.min_updated_ts = Some(min_updated_ts);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMultivariateEventCollectionsParams {
    #[serde(rename = "status")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "associated_event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub associated_event_ticker: Option<String>,
    #[serde(rename = "series_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetMultivariateEventCollectionsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn associated_event_ticker(mut self, associated_event_ticker: impl Into<String>) -> Self {
        self.associated_event_ticker = Some(associated_event_ticker.into());
        self
    }

    pub fn series_ticker(mut self, series_ticker: impl Into<String>) -> Self {
        self.series_ticker = Some(series_ticker.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMultivariateEventCollectionLookupHistoryParams {
    #[serde(rename = "lookback_seconds")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub lookback_seconds: Option<i64>,
}

impl GetMultivariateEventCollectionLookupHistoryParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn lookback_seconds(mut self, lookback_seconds: i64) -> Self {
        self.lookback_seconds = Some(lookback_seconds);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetIncentiveProgramsParams {
    #[serde(rename = "status")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "incentive_description")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub incentive_description: Option<String>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetIncentiveProgramsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn type_(mut self, type_: impl Into<String>) -> Self {
        self.type_ = Some(type_.into());
        self
    }

    pub fn incentive_description(mut self, incentive_description: impl Into<String>) -> Self {
        self.incentive_description = Some(incentive_description.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetFCMOrdersParams {
    #[serde(rename = "subtrader_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "min_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(rename = "max_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(rename = "status")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
}

impl GetFCMOrdersParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subtrader_id(mut self, subtrader_id: impl Into<String>) -> Self {
        self.subtrader_id = Some(subtrader_id.into());
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn event_ticker(mut self, event_ticker: impl Into<String>) -> Self {
        self.event_ticker = Some(event_ticker.into());
        self
    }

    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn min_ts(mut self, min_ts: i64) -> Self {
        self.min_ts = Some(min_ts);
        self
    }

    pub fn max_ts(mut self, max_ts: i64) -> Self {
        self.max_ts = Some(max_ts);
        self
    }

    pub fn status(mut self, status: impl Into<String>) -> Self {
        self.status = Some(status.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetFCMPositionsParams {
    #[serde(rename = "subtrader_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(rename = "ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "count_filter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count_filter: Option<String>,
    #[serde(rename = "settlement_status")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_status: Option<String>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetFCMPositionsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn subtrader_id(mut self, subtrader_id: impl Into<String>) -> Self {
        self.subtrader_id = Some(subtrader_id.into());
        self
    }

    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn event_ticker(mut self, event_ticker: impl Into<String>) -> Self {
        self.event_ticker = Some(event_ticker.into());
        self
    }

    pub fn count_filter(mut self, count_filter: impl Into<String>) -> Self {
        self.count_filter = Some(count_filter.into());
        self
    }

    pub fn settlement_status(mut self, settlement_status: impl Into<String>) -> Self {
        self.settlement_status = Some(settlement_status.into());
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketCandlesticksHistoricalParams {
    #[serde(rename = "start_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub start_ts: Option<i64>,
    #[serde(rename = "end_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_ts: Option<i64>,
    #[serde(rename = "period_interval")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub period_interval: Option<i64>,
}

impl GetMarketCandlesticksHistoricalParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn start_ts(mut self, start_ts: i64) -> Self {
        self.start_ts = Some(start_ts);
        self
    }

    pub fn end_ts(mut self, end_ts: i64) -> Self {
        self.end_ts = Some(end_ts);
        self
    }

    pub fn period_interval(mut self, period_interval: i64) -> Self {
        self.period_interval = Some(period_interval);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetFillsHistoricalParams {
    #[serde(rename = "ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "max_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetFillsHistoricalParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn max_ts(mut self, max_ts: i64) -> Self {
        self.max_ts = Some(max_ts);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetHistoricalOrdersParams {
    #[serde(rename = "ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "max_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
}

impl GetHistoricalOrdersParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn max_ts(mut self, max_ts: i64) -> Self {
        self.max_ts = Some(max_ts);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetTradesHistoricalParams {
    #[serde(rename = "ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ticker: Option<String>,
    #[serde(rename = "min_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_ts: Option<i64>,
    #[serde(rename = "max_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_ts: Option<i64>,
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "is_block_trade")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_block_trade: Option<bool>,
}

impl GetTradesHistoricalParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn ticker(mut self, ticker: impl Into<String>) -> Self {
        self.ticker = Some(ticker.into());
        self
    }

    pub fn min_ts(mut self, min_ts: i64) -> Self {
        self.min_ts = Some(min_ts);
        self
    }

    pub fn max_ts(mut self, max_ts: i64) -> Self {
        self.max_ts = Some(max_ts);
        self
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn is_block_trade(mut self, is_block_trade: bool) -> Self {
        self.is_block_trade = Some(is_block_trade);
        self
    }
}

#[derive(Debug, Clone, Default, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetHistoricalMarketsParams {
    #[serde(rename = "limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub limit: Option<i64>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "tickers")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tickers: Option<String>,
    #[serde(rename = "event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "series_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub series_ticker: Option<String>,
    #[serde(rename = "mve_filter")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mve_filter: Option<String>,
}

impl GetHistoricalMarketsParams {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    pub fn cursor(mut self, cursor: impl Into<String>) -> Self {
        self.cursor = Some(cursor.into());
        self
    }

    pub fn tickers(mut self, tickers: impl Into<String>) -> Self {
        self.tickers = Some(tickers.into());
        self
    }

    pub fn event_ticker(mut self, event_ticker: impl Into<String>) -> Self {
        self.event_ticker = Some(event_ticker.into());
        self
    }

    pub fn series_ticker(mut self, series_ticker: impl Into<String>) -> Self {
        self.series_ticker = Some(series_ticker.into());
        self
    }

    pub fn mve_filter(mut self, mve_filter: impl Into<String>) -> Self {
        self.mve_filter = Some(mve_filter.into());
        self
    }
}
