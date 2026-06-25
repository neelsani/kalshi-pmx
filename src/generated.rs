//! Generated OpenAPI component schemas.
//! Regenerate with `cargo run --manifest-path xtask/Cargo.toml -- generate-openapi-types`.
#![allow(clippy::large_enum_variant)]
#![allow(clippy::struct_field_names)]
#![allow(clippy::module_name_repetitions)]
#![allow(clippy::new_without_default)]
#![allow(clippy::redundant_field_names)]
#![allow(clippy::too_many_arguments)]

pub type FixedPointDollars = String;

pub type FixedPointCount = String;

pub type ExchangeIndex = i64;

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum FeeType {
    #[serde(rename = "quadratic")]
    Quadratic,
    #[serde(rename = "quadratic_with_maker_fees")]
    QuadraticWithMakerFees,
    #[serde(rename = "flat")]
    Flat,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketCandlesticksHistoricalResponse {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "candlesticks")]
    pub candlesticks: Vec<MarketCandlestickHistorical>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarketCandlestickHistorical {
    #[serde(rename = "end_period_ts")]
    pub end_period_ts: i64,
    #[serde(rename = "yes_bid")]
    pub yes_bid: BidAskDistributionHistorical,
    #[serde(rename = "yes_ask")]
    pub yes_ask: BidAskDistributionHistorical,
    #[serde(rename = "price")]
    pub price: PriceDistributionHistorical,
    #[serde(rename = "volume")]
    pub volume: FixedPointCount,
    #[serde(rename = "open_interest")]
    pub open_interest: FixedPointCount,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BidAskDistributionHistorical {
    #[serde(rename = "open")]
    pub open: FixedPointDollars,
    #[serde(rename = "low")]
    pub low: FixedPointDollars,
    #[serde(rename = "high")]
    pub high: FixedPointDollars,
    #[serde(rename = "close")]
    pub close: FixedPointDollars,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PriceDistributionHistorical {
    #[serde(rename = "open")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open: Option<FixedPointDollars>,
    #[serde(rename = "low")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low: Option<FixedPointDollars>,
    #[serde(rename = "high")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high: Option<FixedPointDollars>,
    #[serde(rename = "close")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close: Option<FixedPointDollars>,
    #[serde(rename = "mean")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean: Option<FixedPointDollars>,
    #[serde(rename = "previous")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous: Option<FixedPointDollars>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ErrorResponse {
    #[serde(rename = "code")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub code: Option<String>,
    #[serde(rename = "message")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(rename = "details")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<String>,
    #[serde(rename = "service")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub service: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum SelfTradePreventionType {
    #[serde(rename = "taker_at_cross")]
    TakerAtCross,
    #[serde(rename = "maker")]
    Maker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum BookSide {
    #[serde(rename = "bid")]
    Bid,
    #[serde(rename = "ask")]
    Ask,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum OrderStatus {
    #[serde(rename = "resting")]
    Resting,
    #[serde(rename = "canceled")]
    Canceled,
    #[serde(rename = "executed")]
    Executed,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ExchangeInstance {
    #[serde(rename = "event_contract")]
    EventContract,
    #[serde(rename = "margined")]
    Margined,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum UserFilter {
    #[serde(rename = "self")]
    TypeSelf,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum ApiKeyScope {
    #[serde(rename = "read")]
    Read,
    #[serde(rename = "write")]
    Write,
    #[serde(rename = "read::block_trade_accept")]
    ReadBlockTradeAccept,
    #[serde(rename = "read::portfolio_balance")]
    ReadPortfolioBalance,
    #[serde(rename = "write::transfer")]
    WriteTransfer,
    #[serde(rename = "write::block_trade_accept")]
    WriteBlockTradeAccept,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiKey {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "scopes")]
    pub scopes: Vec<ApiKeyScope>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetApiKeysResponse {
    #[serde(rename = "api_keys")]
    pub api_keys: Vec<ApiKey>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateApiKeyRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "public_key")]
    pub public_key: String,
    #[serde(rename = "scopes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ApiKeyScope>>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl CreateApiKeyRequest {
    pub fn new(name: impl Into<String>, public_key: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            public_key: public_key.into(),
            scopes: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn scopes(mut self, scopes: Vec<ApiKeyScope>) -> Self {
        self.scopes = Some(scopes);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateApiKeyResponse {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenerateApiKeyRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "scopes")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub scopes: Option<Vec<ApiKeyScope>>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl GenerateApiKeyRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            scopes: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn scopes(mut self, scopes: Vec<ApiKeyScope>) -> Self {
        self.scopes = Some(scopes);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GenerateApiKeyResponse {
    #[serde(rename = "api_key_id")]
    pub api_key_id: String,
    #[serde(rename = "private_key")]
    pub private_key: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetTagsForSeriesCategoriesResponse {
    #[serde(rename = "tags_by_categories")]
    pub tags_by_categories: std::collections::BTreeMap<String, Vec<String>>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ScopeList {
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SportFilterDetails {
    #[serde(rename = "scopes")]
    pub scopes: Vec<String>,
    #[serde(rename = "competitions")]
    pub competitions: std::collections::BTreeMap<String, ScopeList>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetFiltersBySportsResponse {
    #[serde(rename = "filters_by_sports")]
    pub filters_by_sports: std::collections::BTreeMap<String, SportFilterDetails>,
    #[serde(rename = "sport_ordering")]
    pub sport_ordering: Vec<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BucketLimit {
    #[serde(rename = "refill_rate")]
    pub refill_rate: i64,
    #[serde(rename = "bucket_capacity")]
    pub bucket_capacity: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetAccountApiLimitsResponse {
    #[serde(rename = "usage_tier")]
    pub usage_tier: String,
    #[serde(rename = "read")]
    pub read: BucketLimit,
    #[serde(rename = "write")]
    pub write: BucketLimit,
    #[serde(rename = "grants")]
    pub grants: Vec<ApiUsageLevelGrant>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApiUsageLevelGrant {
    #[serde(rename = "exchange_instance")]
    pub exchange_instance: ExchangeInstance,
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "expires_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expires_ts: Option<i64>,
    #[serde(rename = "source")]
    pub source: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetAccountApiUsageLevelVolumeProgressResponse {
    #[serde(rename = "volume_progress")]
    pub volume_progress: Vec<AccountApiUsageLevelVolumeProgress>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountApiUsageLevelVolumeProgress {
    #[serde(rename = "computed_ts")]
    pub computed_ts: i64,
    #[serde(rename = "trailing_30d_volume_fp")]
    pub trailing_30d_volume_fp: FixedPointCount,
    #[serde(rename = "goals")]
    pub goals: Vec<AccountApiUsageLevelVolumeGoal>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AccountApiUsageLevelVolumeGoal {
    #[serde(rename = "level")]
    pub level: String,
    #[serde(rename = "earn_volume_goal_fp")]
    pub earn_volume_goal_fp: FixedPointCount,
    #[serde(rename = "keep_volume_goal_fp")]
    pub keep_volume_goal_fp: FixedPointCount,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EndpointTokenCost {
    #[serde(rename = "method")]
    pub method: String,
    #[serde(rename = "path")]
    pub path: String,
    #[serde(rename = "cost")]
    pub cost: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetAccountEndpointCostsResponse {
    #[serde(rename = "default_cost")]
    pub default_cost: i64,
    #[serde(rename = "endpoint_costs")]
    pub endpoint_costs: Vec<EndpointTokenCost>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ExchangeStatus {
    #[serde(rename = "exchange_active")]
    pub exchange_active: bool,
    #[serde(rename = "trading_active")]
    pub trading_active: bool,
    #[serde(rename = "exchange_estimated_resume_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_estimated_resume_time: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetExchangeAnnouncementsResponse {
    #[serde(rename = "announcements")]
    pub announcements: Vec<Announcement>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Announcement {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(rename = "delivery_time")]
    pub delivery_time: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetExchangeScheduleResponse {
    #[serde(rename = "schedule")]
    pub schedule: Schedule,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Schedule {
    #[serde(rename = "standard_hours")]
    pub standard_hours: Vec<WeeklySchedule>,
    #[serde(rename = "maintenance_windows")]
    pub maintenance_windows: Vec<MaintenanceWindow>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct WeeklySchedule {
    #[serde(rename = "start_time")]
    pub start_time: String,
    #[serde(rename = "end_time")]
    pub end_time: String,
    #[serde(rename = "monday")]
    pub monday: Vec<DailySchedule>,
    #[serde(rename = "tuesday")]
    pub tuesday: Vec<DailySchedule>,
    #[serde(rename = "wednesday")]
    pub wednesday: Vec<DailySchedule>,
    #[serde(rename = "thursday")]
    pub thursday: Vec<DailySchedule>,
    #[serde(rename = "friday")]
    pub friday: Vec<DailySchedule>,
    #[serde(rename = "saturday")]
    pub saturday: Vec<DailySchedule>,
    #[serde(rename = "sunday")]
    pub sunday: Vec<DailySchedule>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DailySchedule {
    #[serde(rename = "open_time")]
    pub open_time: String,
    #[serde(rename = "close_time")]
    pub close_time: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MaintenanceWindow {
    #[serde(rename = "start_datetime")]
    pub start_datetime: String,
    #[serde(rename = "end_datetime")]
    pub end_datetime: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetHistoricalCutoffResponse {
    #[serde(rename = "market_settled_ts")]
    pub market_settled_ts: String,
    #[serde(rename = "trades_created_ts")]
    pub trades_created_ts: String,
    #[serde(rename = "orders_updated_ts")]
    pub orders_updated_ts: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetUserDataTimestampResponse {
    #[serde(rename = "as_of_time")]
    pub as_of_time: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketCandlesticksResponse {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "candlesticks")]
    pub candlesticks: Vec<MarketCandlestick>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetEventCandlesticksResponse {
    #[serde(rename = "market_tickers")]
    pub market_tickers: Vec<String>,
    #[serde(rename = "market_candlesticks")]
    pub market_candlesticks: Vec<Vec<MarketCandlestick>>,
    #[serde(rename = "adjusted_end_ts")]
    pub adjusted_end_ts: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchGetMarketCandlesticksResponse {
    #[serde(rename = "markets")]
    pub markets: Vec<MarketCandlesticksResponse>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarketCandlesticksResponse {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "candlesticks")]
    pub candlesticks: Vec<MarketCandlestick>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarketCandlestick {
    #[serde(rename = "end_period_ts")]
    pub end_period_ts: i64,
    #[serde(rename = "yes_bid")]
    pub yes_bid: BidAskDistribution,
    #[serde(rename = "yes_ask")]
    pub yes_ask: BidAskDistribution,
    #[serde(rename = "price")]
    pub price: PriceDistribution,
    #[serde(rename = "volume_fp")]
    pub volume_fp: FixedPointCount,
    #[serde(rename = "open_interest_fp")]
    pub open_interest_fp: FixedPointCount,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BidAskDistribution {
    #[serde(rename = "open_dollars")]
    pub open_dollars: FixedPointDollars,
    #[serde(rename = "low_dollars")]
    pub low_dollars: FixedPointDollars,
    #[serde(rename = "high_dollars")]
    pub high_dollars: FixedPointDollars,
    #[serde(rename = "close_dollars")]
    pub close_dollars: FixedPointDollars,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PriceDistribution {
    #[serde(rename = "open_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub open_dollars: Option<FixedPointDollars>,
    #[serde(rename = "low_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub low_dollars: Option<FixedPointDollars>,
    #[serde(rename = "high_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub high_dollars: Option<FixedPointDollars>,
    #[serde(rename = "close_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub close_dollars: Option<FixedPointDollars>,
    #[serde(rename = "mean_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mean_dollars: Option<FixedPointDollars>,
    #[serde(rename = "previous_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub previous_dollars: Option<FixedPointDollars>,
    #[serde(rename = "min_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub min_dollars: Option<FixedPointDollars>,
    #[serde(rename = "max_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub max_dollars: Option<FixedPointDollars>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LiveData {
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "details")]
    pub details: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(rename = "milestone_id")]
    pub milestone_id: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetLiveDataResponse {
    #[serde(rename = "live_data")]
    pub live_data: LiveData,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetLiveDatasResponse {
    #[serde(rename = "live_datas")]
    pub live_datas: Vec<LiveData>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetGameStatsResponse {
    #[serde(rename = "pbp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub pbp: Option<PlayByPlay>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PlayByPlay {
    #[serde(rename = "periods")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub periods: Option<Vec<serde_json::Value>>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IndexedBalance {
    #[serde(rename = "exchange_index")]
    pub exchange_index: ExchangeIndex,
    #[serde(rename = "balance")]
    pub balance: FixedPointDollars,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetBalanceResponse {
    #[serde(rename = "balance")]
    pub balance: i64,
    #[serde(rename = "balance_dollars")]
    pub balance_dollars: FixedPointDollars,
    #[serde(rename = "portfolio_value")]
    pub portfolio_value: i64,
    #[serde(rename = "updated_ts")]
    pub updated_ts: i64,
    #[serde(rename = "balance_breakdown")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub balance_breakdown: Option<Vec<IndexedBalance>>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateSubaccountResponse {
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ApplySubaccountTransferRequest {
    #[serde(rename = "client_transfer_id")]
    pub client_transfer_id: String,
    #[serde(rename = "from_subaccount")]
    pub from_subaccount: i64,
    #[serde(rename = "to_subaccount")]
    pub to_subaccount: i64,
    #[serde(rename = "amount_cents")]
    pub amount_cents: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl ApplySubaccountTransferRequest {
    pub fn new(
        client_transfer_id: impl Into<String>,
        from_subaccount: i64,
        to_subaccount: i64,
        amount_cents: i64,
    ) -> Self {
        Self {
            client_transfer_id: client_transfer_id.into(),
            from_subaccount: from_subaccount,
            to_subaccount: to_subaccount,
            amount_cents: amount_cents,
            extra: std::collections::BTreeMap::new(),
        }
    }
}

pub type ApplySubaccountTransferResponse = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSubaccountBalancesResponse {
    #[serde(rename = "subaccount_balances")]
    pub subaccount_balances: Vec<SubaccountBalance>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubaccountBalance {
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: i64,
    #[serde(rename = "balance")]
    pub balance: FixedPointDollars,
    #[serde(rename = "updated_ts")]
    pub updated_ts: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSubaccountTransfersResponse {
    #[serde(rename = "transfers")]
    pub transfers: Vec<SubaccountTransfer>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubaccountTransfer {
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    #[serde(rename = "from_subaccount")]
    pub from_subaccount: i64,
    #[serde(rename = "to_subaccount")]
    pub to_subaccount: i64,
    #[serde(rename = "amount_cents")]
    pub amount_cents: i64,
    #[serde(rename = "created_ts")]
    pub created_ts: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateSubaccountNettingRequest {
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: i64,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl UpdateSubaccountNettingRequest {
    pub fn new(subaccount_number: i64, enabled: bool) -> Self {
        Self {
            subaccount_number: subaccount_number,
            enabled: enabled,
            extra: std::collections::BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSubaccountNettingResponse {
    #[serde(rename = "netting_configs")]
    pub netting_configs: Vec<SubaccountNettingConfig>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SubaccountNettingConfig {
    #[serde(rename = "subaccount_number")]
    pub subaccount_number: i64,
    #[serde(rename = "enabled")]
    pub enabled: bool,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSettlementsResponse {
    #[serde(rename = "settlements")]
    pub settlements: Vec<Settlement>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Settlement {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "market_result")]
    pub market_result: String,
    #[serde(rename = "yes_count_fp")]
    pub yes_count_fp: FixedPointCount,
    #[serde(rename = "yes_total_cost_dollars")]
    pub yes_total_cost_dollars: FixedPointDollars,
    #[serde(rename = "no_count_fp")]
    pub no_count_fp: FixedPointCount,
    #[serde(rename = "no_total_cost_dollars")]
    pub no_total_cost_dollars: FixedPointDollars,
    #[serde(rename = "revenue")]
    pub revenue: i64,
    #[serde(rename = "settled_time")]
    pub settled_time: String,
    #[serde(rename = "fee_cost")]
    pub fee_cost: FixedPointDollars,
    #[serde(rename = "value")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub value: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetPortfolioRestingOrderTotalValueResponse {
    #[serde(rename = "total_resting_order_value")]
    pub total_resting_order_value: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetDepositsResponse {
    #[serde(rename = "deposits")]
    pub deposits: Vec<Deposit>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Deposit {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "amount_cents")]
    pub amount_cents: i64,
    #[serde(rename = "fee_cents")]
    pub fee_cents: i64,
    #[serde(rename = "created_ts")]
    pub created_ts: i64,
    #[serde(rename = "finalized_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalized_ts: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetWithdrawalsResponse {
    #[serde(rename = "withdrawals")]
    pub withdrawals: Vec<Withdrawal>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Withdrawal {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "amount_cents")]
    pub amount_cents: i64,
    #[serde(rename = "fee_cents")]
    pub fee_cents: i64,
    #[serde(rename = "created_ts")]
    pub created_ts: i64,
    #[serde(rename = "finalized_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub finalized_ts: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Order {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "user_id")]
    pub user_id: String,
    #[serde(rename = "client_order_id")]
    pub client_order_id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "outcome_side")]
    pub outcome_side: String,
    #[serde(rename = "book_side")]
    pub book_side: BookSide,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "status")]
    pub status: OrderStatus,
    #[serde(rename = "yes_price_dollars")]
    pub yes_price_dollars: FixedPointDollars,
    #[serde(rename = "no_price_dollars")]
    pub no_price_dollars: FixedPointDollars,
    #[serde(rename = "fill_count_fp")]
    pub fill_count_fp: FixedPointCount,
    #[serde(rename = "remaining_count_fp")]
    pub remaining_count_fp: FixedPointCount,
    #[serde(rename = "initial_count_fp")]
    pub initial_count_fp: FixedPointCount,
    #[serde(rename = "taker_fill_cost_dollars")]
    pub taker_fill_cost_dollars: FixedPointDollars,
    #[serde(rename = "maker_fill_cost_dollars")]
    pub maker_fill_cost_dollars: FixedPointDollars,
    #[serde(rename = "taker_fees_dollars")]
    pub taker_fees_dollars: FixedPointDollars,
    #[serde(rename = "maker_fees_dollars")]
    pub maker_fees_dollars: FixedPointDollars,
    #[serde(rename = "expiration_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "created_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "last_update_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_update_time: Option<String>,
    #[serde(rename = "self_trade_prevention_type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_type: Option<SelfTradePreventionType>,
    #[serde(rename = "order_group_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(rename = "cancel_order_on_pause")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(rename = "subaccount_number")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount_number: Option<i64>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Milestone {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "type")]
    pub type_: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "related_event_tickers")]
    pub related_event_tickers: Vec<String>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "notification_message")]
    pub notification_message: String,
    #[serde(rename = "source_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "source_ids")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<std::collections::BTreeMap<String, String>>,
    #[serde(rename = "details")]
    pub details: std::collections::BTreeMap<String, serde_json::Value>,
    #[serde(rename = "primary_event_tickers")]
    pub primary_event_tickers: Vec<String>,
    #[serde(rename = "last_updated_ts")]
    pub last_updated_ts: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMilestoneResponse {
    #[serde(rename = "milestone")]
    pub milestone: Milestone,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMilestonesResponse {
    #[serde(rename = "milestones")]
    pub milestones: Vec<Milestone>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrdersResponse {
    #[serde(rename = "orders")]
    pub orders: Vec<Order>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrderQueuePositionResponse {
    #[serde(rename = "queue_position_fp")]
    pub queue_position_fp: FixedPointCount,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderQueuePosition {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "queue_position_fp")]
    pub queue_position_fp: FixedPointCount,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrderQueuePositionsResponse {
    #[serde(rename = "queue_positions")]
    pub queue_positions: Vec<OrderQueuePosition>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarketPosition {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "total_traded_dollars")]
    pub total_traded_dollars: FixedPointDollars,
    #[serde(rename = "position_fp")]
    pub position_fp: FixedPointCount,
    #[serde(rename = "market_exposure_dollars")]
    pub market_exposure_dollars: FixedPointDollars,
    #[serde(rename = "realized_pnl_dollars")]
    pub realized_pnl_dollars: FixedPointDollars,
    #[serde(rename = "resting_orders_count")]
    pub resting_orders_count: i64,
    #[serde(rename = "fees_paid_dollars")]
    pub fees_paid_dollars: FixedPointDollars,
    #[serde(rename = "last_updated_ts")]
    pub last_updated_ts: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EventPosition {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "total_cost_dollars")]
    pub total_cost_dollars: FixedPointDollars,
    #[serde(rename = "total_cost_shares_fp")]
    pub total_cost_shares_fp: FixedPointCount,
    #[serde(rename = "event_exposure_dollars")]
    pub event_exposure_dollars: FixedPointDollars,
    #[serde(rename = "realized_pnl_dollars")]
    pub realized_pnl_dollars: FixedPointDollars,
    #[serde(rename = "fees_paid_dollars")]
    pub fees_paid_dollars: FixedPointDollars,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetPositionsResponse {
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(rename = "market_positions")]
    pub market_positions: Vec<MarketPosition>,
    #[serde(rename = "event_positions")]
    pub event_positions: Vec<EventPosition>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Trade {
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "count_fp")]
    pub count_fp: FixedPointCount,
    #[serde(rename = "yes_price_dollars")]
    pub yes_price_dollars: FixedPointDollars,
    #[serde(rename = "no_price_dollars")]
    pub no_price_dollars: FixedPointDollars,
    #[serde(rename = "taker_side")]
    pub taker_side: String,
    #[serde(rename = "taker_outcome_side")]
    pub taker_outcome_side: String,
    #[serde(rename = "taker_book_side")]
    pub taker_book_side: BookSide,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "is_block_trade")]
    pub is_block_trade: bool,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetIncentiveProgramsResponse {
    #[serde(rename = "incentive_programs")]
    pub incentive_programs: Vec<IncentiveProgram>,
    #[serde(rename = "next_cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IncentiveProgram {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "market_id")]
    pub market_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "incentive_type")]
    pub incentive_type: String,
    #[serde(rename = "incentive_description")]
    pub incentive_description: String,
    #[serde(rename = "start_date")]
    pub start_date: String,
    #[serde(rename = "end_date")]
    pub end_date: String,
    #[serde(rename = "period_reward")]
    pub period_reward: i64,
    #[serde(rename = "paid_out")]
    pub paid_out: bool,
    #[serde(rename = "discount_factor_bps")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub discount_factor_bps: Option<i64>,
    #[serde(rename = "target_size_fp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_size_fp: Option<FixedPointCount>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetTradesResponse {
    #[serde(rename = "trades")]
    pub trades: Vec<Trade>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Fill {
    #[serde(rename = "fill_id")]
    pub fill_id: String,
    #[serde(rename = "trade_id")]
    pub trade_id: String,
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "outcome_side")]
    pub outcome_side: String,
    #[serde(rename = "book_side")]
    pub book_side: BookSide,
    #[serde(rename = "count_fp")]
    pub count_fp: FixedPointCount,
    #[serde(rename = "yes_price_dollars")]
    pub yes_price_dollars: FixedPointDollars,
    #[serde(rename = "no_price_dollars")]
    pub no_price_dollars: FixedPointDollars,
    #[serde(rename = "is_taker")]
    pub is_taker: bool,
    #[serde(rename = "created_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub created_time: Option<String>,
    #[serde(rename = "fee_cost")]
    pub fee_cost: FixedPointDollars,
    #[serde(rename = "subaccount_number")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount_number: Option<i64>,
    #[serde(rename = "ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ts: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetFillsResponse {
    #[serde(rename = "fills")]
    pub fills: Vec<Fill>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct StructuredTarget {
    #[serde(rename = "id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(rename = "name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub type_: Option<String>,
    #[serde(rename = "details")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub details: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "source_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_id: Option<String>,
    #[serde(rename = "source_ids")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_ids: Option<std::collections::BTreeMap<String, String>>,
    #[serde(rename = "last_updated_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_ts: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetStructuredTargetsResponse {
    #[serde(rename = "structured_targets")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structured_targets: Option<Vec<StructuredTarget>>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetStructuredTargetResponse {
    #[serde(rename = "structured_target")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub structured_target: Option<StructuredTarget>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

pub type EmptyResponse = std::collections::BTreeMap<String, serde_json::Value>;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IntraExchangeInstanceTransferRequest {
    #[serde(rename = "source")]
    pub source: ExchangeInstance,
    #[serde(rename = "destination")]
    pub destination: ExchangeInstance,
    #[serde(rename = "amount")]
    pub amount: i64,
    #[serde(rename = "source_exchange_shard")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source_exchange_shard: Option<i64>,
    #[serde(rename = "destination_exchange_shard")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub destination_exchange_shard: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl IntraExchangeInstanceTransferRequest {
    pub fn new(source: ExchangeInstance, destination: ExchangeInstance, amount: i64) -> Self {
        Self {
            source: source,
            destination: destination,
            amount: amount,
            source_exchange_shard: None,
            destination_exchange_shard: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn source_exchange_shard(mut self, source_exchange_shard: i64) -> Self {
        self.source_exchange_shard = Some(source_exchange_shard);
        self
    }

    pub fn destination_exchange_shard(mut self, destination_exchange_shard: i64) -> Self {
        self.destination_exchange_shard = Some(destination_exchange_shard);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct IntraExchangeInstanceTransferResponse {
    #[serde(rename = "transfer_id")]
    pub transfer_id: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderGroup {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "contracts_limit_fp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<FixedPointCount>,
    #[serde(rename = "is_auto_cancel_enabled")]
    pub is_auto_cancel_enabled: bool,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrderGroupsResponse {
    #[serde(rename = "order_groups")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_groups: Option<Vec<OrderGroup>>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrderGroupResponse {
    #[serde(rename = "is_auto_cancel_enabled")]
    pub is_auto_cancel_enabled: bool,
    #[serde(rename = "contracts_limit_fp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<FixedPointCount>,
    #[serde(rename = "orders")]
    pub orders: Vec<String>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderGroupRequest {
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(rename = "contracts_limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit: Option<i64>,
    #[serde(rename = "contracts_limit_fp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<FixedPointCount>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl CreateOrderGroupRequest {
    pub fn new() -> Self {
        Self {
            subaccount: None,
            contracts_limit: None,
            contracts_limit_fp: None,
            exchange_index: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    pub fn contracts_limit(mut self, contracts_limit: i64) -> Self {
        self.contracts_limit = Some(contracts_limit);
        self
    }

    pub fn contracts_limit_fp(mut self, contracts_limit_fp: impl Into<String>) -> Self {
        self.contracts_limit_fp = Some(contracts_limit_fp.into());
        self
    }

    pub fn exchange_index(mut self, exchange_index: ExchangeIndex) -> Self {
        self.exchange_index = Some(exchange_index);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct UpdateOrderGroupLimitRequest {
    #[serde(rename = "contracts_limit")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit: Option<i64>,
    #[serde(rename = "contracts_limit_fp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_limit_fp: Option<FixedPointCount>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl UpdateOrderGroupLimitRequest {
    pub fn new() -> Self {
        Self {
            contracts_limit: None,
            contracts_limit_fp: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn contracts_limit(mut self, contracts_limit: i64) -> Self {
        self.contracts_limit = Some(contracts_limit);
        self
    }

    pub fn contracts_limit_fp(mut self, contracts_limit_fp: impl Into<String>) -> Self {
        self.contracts_limit_fp = Some(contracts_limit_fp.into());
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderGroupResponse {
    #[serde(rename = "order_group_id")]
    pub order_group_id: String,
    #[serde(rename = "subaccount")]
    pub subaccount: i64,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetCommunicationsIDResponse {
    #[serde(rename = "communications_id")]
    pub communications_id: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BlockTradeProposal {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "proposer_user_id")]
    pub proposer_user_id: String,
    #[serde(rename = "buyer_user_id")]
    pub buyer_user_id: String,
    #[serde(rename = "buyer_subtrader_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_subtrader_id: Option<String>,
    #[serde(rename = "seller_user_id")]
    pub seller_user_id: String,
    #[serde(rename = "seller_subtrader_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_subtrader_id: Option<String>,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "price_centi_cents")]
    pub price_centi_cents: i64,
    #[serde(rename = "centicount")]
    pub centicount: i64,
    #[serde(rename = "maker_side")]
    pub maker_side: String,
    #[serde(rename = "expiration_ts")]
    pub expiration_ts: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(rename = "updated_ts")]
    pub updated_ts: String,
    #[serde(rename = "buyer_accepted")]
    pub buyer_accepted: bool,
    #[serde(rename = "seller_accepted")]
    pub seller_accepted: bool,
    #[serde(rename = "buyer_accepted_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_accepted_ts: Option<String>,
    #[serde(rename = "seller_accepted_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_accepted_ts: Option<String>,
    #[serde(rename = "executed_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_ts: Option<String>,
    #[serde(rename = "buyer_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_order_id: Option<String>,
    #[serde(rename = "seller_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_order_id: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetBlockTradeProposalsResponse {
    #[serde(rename = "block_trade_proposals")]
    pub block_trade_proposals: Vec<BlockTradeProposal>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProposeBlockTradeRequest {
    #[serde(rename = "buyer_user_id")]
    pub buyer_user_id: String,
    #[serde(rename = "buyer_subtrader_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_subtrader_id: Option<String>,
    #[serde(rename = "buyer_subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buyer_subaccount: Option<i64>,
    #[serde(rename = "seller_user_id")]
    pub seller_user_id: String,
    #[serde(rename = "seller_subtrader_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_subtrader_id: Option<String>,
    #[serde(rename = "seller_subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seller_subaccount: Option<i64>,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "price_centi_cents")]
    pub price_centi_cents: i64,
    #[serde(rename = "centicount")]
    pub centicount: i64,
    #[serde(rename = "maker_side")]
    pub maker_side: String,
    #[serde(rename = "expiration_ts")]
    pub expiration_ts: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl ProposeBlockTradeRequest {
    pub fn new(
        buyer_user_id: impl Into<String>,
        seller_user_id: impl Into<String>,
        market_ticker: impl Into<String>,
        price_centi_cents: i64,
        centicount: i64,
        maker_side: impl Into<String>,
        expiration_ts: impl Into<String>,
    ) -> Self {
        Self {
            buyer_user_id: buyer_user_id.into(),
            seller_user_id: seller_user_id.into(),
            market_ticker: market_ticker.into(),
            price_centi_cents: price_centi_cents,
            centicount: centicount,
            maker_side: maker_side.into(),
            expiration_ts: expiration_ts.into(),
            buyer_subtrader_id: None,
            buyer_subaccount: None,
            seller_subtrader_id: None,
            seller_subaccount: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn buyer_subtrader_id(mut self, buyer_subtrader_id: impl Into<String>) -> Self {
        self.buyer_subtrader_id = Some(buyer_subtrader_id.into());
        self
    }

    pub fn buyer_subaccount(mut self, buyer_subaccount: i64) -> Self {
        self.buyer_subaccount = Some(buyer_subaccount);
        self
    }

    pub fn seller_subtrader_id(mut self, seller_subtrader_id: impl Into<String>) -> Self {
        self.seller_subtrader_id = Some(seller_subtrader_id.into());
        self
    }

    pub fn seller_subaccount(mut self, seller_subaccount: i64) -> Self {
        self.seller_subaccount = Some(seller_subaccount);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ProposeBlockTradeResponse {
    #[serde(rename = "block_trade_proposal_id")]
    pub block_trade_proposal_id: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AcceptBlockTradeProposalRequest {
    #[serde(rename = "subtrader_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl AcceptBlockTradeProposalRequest {
    pub fn new() -> Self {
        Self {
            subtrader_id: None,
            subaccount: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn subtrader_id(mut self, subtrader_id: impl Into<String>) -> Self {
        self.subtrader_id = Some(subtrader_id.into());
        self
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct RFQ {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "creator_id")]
    pub creator_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "contracts_fp")]
    pub contracts_fp: FixedPointCount,
    #[serde(rename = "target_cost_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_dollars: Option<FixedPointDollars>,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(rename = "mve_collection_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mve_collection_ticker: Option<String>,
    #[serde(rename = "mve_selected_legs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mve_selected_legs: Option<Vec<MveSelectedLeg>>,
    #[serde(rename = "rest_remainder")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rest_remainder: Option<bool>,
    #[serde(rename = "cancellation_reason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[serde(rename = "creator_user_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    #[serde(rename = "creator_subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_subaccount: Option<i64>,
    #[serde(rename = "cancelled_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled_ts: Option<String>,
    #[serde(rename = "updated_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_ts: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetRFQsResponse {
    #[serde(rename = "rfqs")]
    pub rfqs: Vec<RFQ>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetRFQResponse {
    #[serde(rename = "rfq")]
    pub rfq: RFQ,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateRFQRequest {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "contracts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts: Option<i64>,
    #[serde(rename = "contracts_fp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub contracts_fp: Option<FixedPointCount>,
    #[serde(rename = "target_cost_centi_cents")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_centi_cents: Option<i64>,
    #[serde(rename = "target_cost_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub target_cost_dollars: Option<FixedPointDollars>,
    #[serde(rename = "rest_remainder")]
    pub rest_remainder: bool,
    #[serde(rename = "replace_existing")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub replace_existing: Option<bool>,
    #[serde(rename = "subtrader_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtrader_id: Option<String>,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl CreateRFQRequest {
    pub fn new(market_ticker: impl Into<String>, rest_remainder: bool) -> Self {
        Self {
            market_ticker: market_ticker.into(),
            rest_remainder: rest_remainder,
            contracts: None,
            contracts_fp: None,
            target_cost_centi_cents: None,
            target_cost_dollars: None,
            replace_existing: None,
            subtrader_id: None,
            subaccount: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn contracts(mut self, contracts: i64) -> Self {
        self.contracts = Some(contracts);
        self
    }

    pub fn contracts_fp(mut self, contracts_fp: impl Into<String>) -> Self {
        self.contracts_fp = Some(contracts_fp.into());
        self
    }

    pub fn target_cost_centi_cents(mut self, target_cost_centi_cents: i64) -> Self {
        self.target_cost_centi_cents = Some(target_cost_centi_cents);
        self
    }

    pub fn target_cost_dollars(mut self, target_cost_dollars: impl Into<String>) -> Self {
        self.target_cost_dollars = Some(target_cost_dollars.into());
        self
    }

    pub fn replace_existing(mut self, replace_existing: bool) -> Self {
        self.replace_existing = Some(replace_existing);
        self
    }

    pub fn subtrader_id(mut self, subtrader_id: impl Into<String>) -> Self {
        self.subtrader_id = Some(subtrader_id.into());
        self
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateRFQResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Quote {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "creator_id")]
    pub creator_id: String,
    #[serde(rename = "rfq_creator_id")]
    pub rfq_creator_id: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "contracts_fp")]
    pub contracts_fp: FixedPointCount,
    #[serde(rename = "yes_bid_dollars")]
    pub yes_bid_dollars: FixedPointDollars,
    #[serde(rename = "no_bid_dollars")]
    pub no_bid_dollars: FixedPointDollars,
    #[serde(rename = "created_ts")]
    pub created_ts: String,
    #[serde(rename = "updated_ts")]
    pub updated_ts: String,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "accepted_side")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_side: Option<String>,
    #[serde(rename = "accepted_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub accepted_ts: Option<String>,
    #[serde(rename = "confirmed_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub confirmed_ts: Option<String>,
    #[serde(rename = "executed_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub executed_ts: Option<String>,
    #[serde(rename = "cancelled_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancelled_ts: Option<String>,
    #[serde(rename = "rest_remainder")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rest_remainder: Option<bool>,
    #[serde(rename = "post_only")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "cancellation_reason")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancellation_reason: Option<String>,
    #[serde(rename = "creator_user_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_user_id: Option<String>,
    #[serde(rename = "rfq_creator_user_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_user_id: Option<String>,
    #[serde(rename = "rfq_target_cost_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_target_cost_dollars: Option<FixedPointDollars>,
    #[serde(rename = "rfq_creator_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_order_id: Option<String>,
    #[serde(rename = "creator_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_order_id: Option<String>,
    #[serde(rename = "creator_subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub creator_subaccount: Option<i64>,
    #[serde(rename = "rfq_creator_subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub rfq_creator_subaccount: Option<i64>,
    #[serde(rename = "yes_contracts_fp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_contracts_fp: Option<FixedPointCount>,
    #[serde(rename = "no_contracts_fp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_contracts_fp: Option<FixedPointCount>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetQuotesResponse {
    #[serde(rename = "quotes")]
    pub quotes: Vec<Quote>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetQuoteResponse {
    #[serde(rename = "quote")]
    pub quote: Quote,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateQuoteRequest {
    #[serde(rename = "rfq_id")]
    pub rfq_id: String,
    #[serde(rename = "yes_bid")]
    pub yes_bid: FixedPointDollars,
    #[serde(rename = "no_bid")]
    pub no_bid: FixedPointDollars,
    #[serde(rename = "rest_remainder")]
    pub rest_remainder: bool,
    #[serde(rename = "post_only")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl CreateQuoteRequest {
    pub fn new(
        rfq_id: impl Into<String>,
        yes_bid: impl Into<String>,
        no_bid: impl Into<String>,
        rest_remainder: bool,
    ) -> Self {
        Self {
            rfq_id: rfq_id.into(),
            yes_bid: yes_bid.into(),
            no_bid: no_bid.into(),
            rest_remainder: rest_remainder,
            post_only: None,
            subaccount: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateQuoteResponse {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AcceptQuoteRequest {
    #[serde(rename = "accepted_side")]
    pub accepted_side: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl AcceptQuoteRequest {
    pub fn new(accepted_side: impl Into<String>) -> Self {
        Self {
            accepted_side: accepted_side.into(),
            extra: std::collections::BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetOrderResponse {
    #[serde(rename = "order")]
    pub order: Order,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderRequest {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "client_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "count_fp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count_fp: Option<FixedPointCount>,
    #[serde(rename = "yes_price")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_price: Option<i64>,
    #[serde(rename = "no_price")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_price: Option<i64>,
    #[serde(rename = "yes_price_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_price_dollars: Option<FixedPointDollars>,
    #[serde(rename = "no_price_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_price_dollars: Option<FixedPointDollars>,
    #[serde(rename = "expiration_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_ts: Option<i64>,
    #[serde(rename = "time_in_force")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub time_in_force: Option<String>,
    #[serde(rename = "buy_max_cost")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub buy_max_cost: Option<i64>,
    #[serde(rename = "post_only")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "reduce_only")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "sell_position_floor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub sell_position_floor: Option<i64>,
    #[serde(rename = "self_trade_prevention_type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub self_trade_prevention_type: Option<SelfTradePreventionType>,
    #[serde(rename = "order_group_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(rename = "cancel_order_on_pause")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl CreateOrderRequest {
    pub fn new(
        ticker: impl Into<String>,
        side: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            ticker: ticker.into(),
            side: side.into(),
            action: action.into(),
            client_order_id: None,
            count: None,
            count_fp: None,
            yes_price: None,
            no_price: None,
            yes_price_dollars: None,
            no_price_dollars: None,
            expiration_ts: None,
            time_in_force: None,
            buy_max_cost: None,
            post_only: None,
            reduce_only: None,
            sell_position_floor: None,
            self_trade_prevention_type: None,
            order_group_id: None,
            cancel_order_on_pause: None,
            subaccount: None,
            exchange_index: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn client_order_id(mut self, client_order_id: impl Into<String>) -> Self {
        self.client_order_id = Some(client_order_id.into());
        self
    }

    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }

    pub fn count_fp(mut self, count_fp: impl Into<String>) -> Self {
        self.count_fp = Some(count_fp.into());
        self
    }

    pub fn yes_price(mut self, yes_price: i64) -> Self {
        self.yes_price = Some(yes_price);
        self
    }

    pub fn no_price(mut self, no_price: i64) -> Self {
        self.no_price = Some(no_price);
        self
    }

    pub fn yes_price_dollars(mut self, yes_price_dollars: impl Into<String>) -> Self {
        self.yes_price_dollars = Some(yes_price_dollars.into());
        self
    }

    pub fn no_price_dollars(mut self, no_price_dollars: impl Into<String>) -> Self {
        self.no_price_dollars = Some(no_price_dollars.into());
        self
    }

    pub fn expiration_ts(mut self, expiration_ts: i64) -> Self {
        self.expiration_ts = Some(expiration_ts);
        self
    }

    pub fn time_in_force(mut self, time_in_force: impl Into<String>) -> Self {
        self.time_in_force = Some(time_in_force.into());
        self
    }

    pub fn buy_max_cost(mut self, buy_max_cost: i64) -> Self {
        self.buy_max_cost = Some(buy_max_cost);
        self
    }

    pub fn post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    pub fn sell_position_floor(mut self, sell_position_floor: i64) -> Self {
        self.sell_position_floor = Some(sell_position_floor);
        self
    }

    pub fn self_trade_prevention_type(
        mut self,
        self_trade_prevention_type: SelfTradePreventionType,
    ) -> Self {
        self.self_trade_prevention_type = Some(self_trade_prevention_type);
        self
    }

    pub fn order_group_id(mut self, order_group_id: impl Into<String>) -> Self {
        self.order_group_id = Some(order_group_id.into());
        self
    }

    pub fn cancel_order_on_pause(mut self, cancel_order_on_pause: bool) -> Self {
        self.cancel_order_on_pause = Some(cancel_order_on_pause);
        self
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

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderResponse {
    #[serde(rename = "order")]
    pub order: Order,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchCreateOrdersRequest {
    #[serde(rename = "orders")]
    pub orders: Vec<CreateOrderRequest>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl BatchCreateOrdersRequest {
    pub fn new(orders: Vec<CreateOrderRequest>) -> Self {
        Self {
            orders: orders,
            extra: std::collections::BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchCreateOrdersResponse {
    #[serde(rename = "orders")]
    pub orders: Vec<BatchCreateOrdersIndividualResponse>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchCreateOrdersIndividualResponse {
    #[serde(rename = "client_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "order")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(rename = "error")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchCancelOrdersRequest {
    #[serde(rename = "ids")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub ids: Option<Vec<String>>,
    #[serde(rename = "orders")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub orders: Option<Vec<BatchCancelOrdersRequestOrder>>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl BatchCancelOrdersRequest {
    pub fn new() -> Self {
        Self {
            ids: None,
            orders: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn ids(mut self, ids: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.ids = Some(ids.into_iter().map(Into::into).collect());
        self
    }

    pub fn orders(mut self, orders: Vec<BatchCancelOrdersRequestOrder>) -> Self {
        self.orders = Some(orders);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchCancelOrdersRequestOrder {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(rename = "market_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchCancelOrdersResponse {
    #[serde(rename = "orders")]
    pub orders: Vec<BatchCancelOrdersIndividualResponse>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchCancelOrdersIndividualResponse {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "order")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order: Option<Order>,
    #[serde(rename = "reduced_by_fp")]
    pub reduced_by_fp: FixedPointCount,
    #[serde(rename = "error")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub error: Option<ErrorResponse>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AmendOrderRequest {
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(rename = "action")]
    pub action: String,
    #[serde(rename = "client_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "updated_client_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_client_order_id: Option<String>,
    #[serde(rename = "yes_price")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_price: Option<i64>,
    #[serde(rename = "no_price")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_price: Option<i64>,
    #[serde(rename = "yes_price_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_price_dollars: Option<FixedPointDollars>,
    #[serde(rename = "no_price_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub no_price_dollars: Option<FixedPointDollars>,
    #[serde(rename = "count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count: Option<i64>,
    #[serde(rename = "count_fp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub count_fp: Option<FixedPointCount>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl AmendOrderRequest {
    pub fn new(
        ticker: impl Into<String>,
        side: impl Into<String>,
        action: impl Into<String>,
    ) -> Self {
        Self {
            ticker: ticker.into(),
            side: side.into(),
            action: action.into(),
            subaccount: None,
            client_order_id: None,
            updated_client_order_id: None,
            yes_price: None,
            no_price: None,
            yes_price_dollars: None,
            no_price_dollars: None,
            count: None,
            count_fp: None,
            exchange_index: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    pub fn client_order_id(mut self, client_order_id: impl Into<String>) -> Self {
        self.client_order_id = Some(client_order_id.into());
        self
    }

    pub fn updated_client_order_id(mut self, updated_client_order_id: impl Into<String>) -> Self {
        self.updated_client_order_id = Some(updated_client_order_id.into());
        self
    }

    pub fn yes_price(mut self, yes_price: i64) -> Self {
        self.yes_price = Some(yes_price);
        self
    }

    pub fn no_price(mut self, no_price: i64) -> Self {
        self.no_price = Some(no_price);
        self
    }

    pub fn yes_price_dollars(mut self, yes_price_dollars: impl Into<String>) -> Self {
        self.yes_price_dollars = Some(yes_price_dollars.into());
        self
    }

    pub fn no_price_dollars(mut self, no_price_dollars: impl Into<String>) -> Self {
        self.no_price_dollars = Some(no_price_dollars.into());
        self
    }

    pub fn count(mut self, count: i64) -> Self {
        self.count = Some(count);
        self
    }

    pub fn count_fp(mut self, count_fp: impl Into<String>) -> Self {
        self.count_fp = Some(count_fp.into());
        self
    }

    pub fn exchange_index(mut self, exchange_index: ExchangeIndex) -> Self {
        self.exchange_index = Some(exchange_index);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AmendOrderResponse {
    #[serde(rename = "old_order")]
    pub old_order: Order,
    #[serde(rename = "order")]
    pub order: Order,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderV2Request {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "client_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "side")]
    pub side: BookSide,
    #[serde(rename = "count")]
    pub count: FixedPointCount,
    #[serde(rename = "price")]
    pub price: FixedPointDollars,
    #[serde(rename = "expiration_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<i64>,
    #[serde(rename = "time_in_force")]
    pub time_in_force: String,
    #[serde(rename = "post_only")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub post_only: Option<bool>,
    #[serde(rename = "self_trade_prevention_type")]
    pub self_trade_prevention_type: SelfTradePreventionType,
    #[serde(rename = "cancel_order_on_pause")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cancel_order_on_pause: Option<bool>,
    #[serde(rename = "reduce_only")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduce_only: Option<bool>,
    #[serde(rename = "subaccount")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subaccount: Option<i64>,
    #[serde(rename = "order_group_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub order_group_id: Option<String>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl CreateOrderV2Request {
    pub fn new(
        ticker: impl Into<String>,
        side: BookSide,
        count: impl Into<String>,
        price: impl Into<String>,
        time_in_force: impl Into<String>,
        self_trade_prevention_type: SelfTradePreventionType,
    ) -> Self {
        Self {
            ticker: ticker.into(),
            side: side,
            count: count.into(),
            price: price.into(),
            time_in_force: time_in_force.into(),
            self_trade_prevention_type: self_trade_prevention_type,
            client_order_id: None,
            expiration_time: None,
            post_only: None,
            cancel_order_on_pause: None,
            reduce_only: None,
            subaccount: None,
            order_group_id: None,
            exchange_index: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn client_order_id(mut self, client_order_id: impl Into<String>) -> Self {
        self.client_order_id = Some(client_order_id.into());
        self
    }

    pub fn expiration_time(mut self, expiration_time: i64) -> Self {
        self.expiration_time = Some(expiration_time);
        self
    }

    pub fn post_only(mut self, post_only: bool) -> Self {
        self.post_only = Some(post_only);
        self
    }

    pub fn cancel_order_on_pause(mut self, cancel_order_on_pause: bool) -> Self {
        self.cancel_order_on_pause = Some(cancel_order_on_pause);
        self
    }

    pub fn reduce_only(mut self, reduce_only: bool) -> Self {
        self.reduce_only = Some(reduce_only);
        self
    }

    pub fn subaccount(mut self, subaccount: i64) -> Self {
        self.subaccount = Some(subaccount);
        self
    }

    pub fn order_group_id(mut self, order_group_id: impl Into<String>) -> Self {
        self.order_group_id = Some(order_group_id.into());
        self
    }

    pub fn exchange_index(mut self, exchange_index: ExchangeIndex) -> Self {
        self.exchange_index = Some(exchange_index);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "fill_count")]
    pub fill_count: FixedPointCount,
    #[serde(rename = "remaining_count")]
    pub remaining_count: FixedPointCount,
    #[serde(rename = "average_fill_price")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<FixedPointDollars>,
    #[serde(rename = "average_fee_paid")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_fee_paid: Option<FixedPointDollars>,
    #[serde(rename = "ts_ms")]
    pub ts_ms: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CancelOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "reduced_by")]
    pub reduced_by: FixedPointCount,
    #[serde(rename = "ts_ms")]
    pub ts_ms: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreaseOrderV2Request {
    #[serde(rename = "reduce_by")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduce_by: Option<FixedPointCount>,
    #[serde(rename = "reduce_to")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reduce_to: Option<FixedPointCount>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl DecreaseOrderV2Request {
    pub fn new() -> Self {
        Self {
            reduce_by: None,
            reduce_to: None,
            exchange_index: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn reduce_by(mut self, reduce_by: impl Into<String>) -> Self {
        self.reduce_by = Some(reduce_by.into());
        self
    }

    pub fn reduce_to(mut self, reduce_to: impl Into<String>) -> Self {
        self.reduce_to = Some(reduce_to.into());
        self
    }

    pub fn exchange_index(mut self, exchange_index: ExchangeIndex) -> Self {
        self.exchange_index = Some(exchange_index);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct DecreaseOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "remaining_count")]
    pub remaining_count: FixedPointCount,
    #[serde(rename = "ts_ms")]
    pub ts_ms: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AmendOrderV2Request {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "side")]
    pub side: BookSide,
    #[serde(rename = "price")]
    pub price: FixedPointDollars,
    #[serde(rename = "count")]
    pub count: FixedPointCount,
    #[serde(rename = "client_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "updated_client_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub updated_client_order_id: Option<String>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl AmendOrderV2Request {
    pub fn new(
        ticker: impl Into<String>,
        side: BookSide,
        price: impl Into<String>,
        count: impl Into<String>,
    ) -> Self {
        Self {
            ticker: ticker.into(),
            side: side,
            price: price.into(),
            count: count.into(),
            client_order_id: None,
            updated_client_order_id: None,
            exchange_index: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn client_order_id(mut self, client_order_id: impl Into<String>) -> Self {
        self.client_order_id = Some(client_order_id.into());
        self
    }

    pub fn updated_client_order_id(mut self, updated_client_order_id: impl Into<String>) -> Self {
        self.updated_client_order_id = Some(updated_client_order_id.into());
        self
    }

    pub fn exchange_index(mut self, exchange_index: ExchangeIndex) -> Self {
        self.exchange_index = Some(exchange_index);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AmendOrderV2Response {
    #[serde(rename = "order_id")]
    pub order_id: String,
    #[serde(rename = "client_order_id")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub client_order_id: Option<String>,
    #[serde(rename = "remaining_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub remaining_count: Option<FixedPointCount>,
    #[serde(rename = "fill_count")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fill_count: Option<FixedPointCount>,
    #[serde(rename = "average_fill_price")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_fill_price: Option<FixedPointDollars>,
    #[serde(rename = "average_fee_paid")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub average_fee_paid: Option<FixedPointDollars>,
    #[serde(rename = "ts_ms")]
    pub ts_ms: i64,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchCreateOrdersV2Request {
    #[serde(rename = "orders")]
    pub orders: Vec<CreateOrderV2Request>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl BatchCreateOrdersV2Request {
    pub fn new(orders: Vec<CreateOrderV2Request>) -> Self {
        Self {
            orders: orders,
            extra: std::collections::BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchCreateOrdersV2Response {
    #[serde(rename = "orders")]
    pub orders: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchCancelOrdersV2Request {
    #[serde(rename = "orders")]
    pub orders: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl BatchCancelOrdersV2Request {
    pub fn new(orders: Vec<serde_json::Value>) -> Self {
        Self {
            orders: orders,
            extra: std::collections::BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct BatchCancelOrdersV2Response {
    #[serde(rename = "orders")]
    pub orders: Vec<serde_json::Value>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct AssociatedEvent {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "is_yes_only")]
    pub is_yes_only: bool,
    #[serde(rename = "size_max")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_max: Option<i64>,
    #[serde(rename = "size_min")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub size_min: Option<i64>,
    #[serde(rename = "active_quoters")]
    pub active_quoters: Vec<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MultivariateEventCollection {
    #[serde(rename = "collection_ticker")]
    pub collection_ticker: String,
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "open_date")]
    pub open_date: String,
    #[serde(rename = "close_date")]
    pub close_date: String,
    #[serde(rename = "associated_events")]
    pub associated_events: Vec<AssociatedEvent>,
    #[serde(rename = "associated_event_tickers")]
    pub associated_event_tickers: Vec<String>,
    #[serde(rename = "is_ordered")]
    pub is_ordered: bool,
    #[serde(rename = "is_single_market_per_event")]
    pub is_single_market_per_event: bool,
    #[serde(rename = "is_all_yes")]
    pub is_all_yes: bool,
    #[serde(rename = "size_min")]
    pub size_min: i64,
    #[serde(rename = "size_max")]
    pub size_max: i64,
    #[serde(rename = "functional_description")]
    pub functional_description: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMultivariateEventCollectionResponse {
    #[serde(rename = "multivariate_contract")]
    pub multivariate_contract: MultivariateEventCollection,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMultivariateEventCollectionsResponse {
    #[serde(rename = "multivariate_contracts")]
    pub multivariate_contracts: Vec<MultivariateEventCollection>,
    #[serde(rename = "cursor")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct TickerPair {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "side")]
    pub side: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LookupTickersForMarketInMultivariateEventCollectionRequest {
    #[serde(rename = "selected_markets")]
    pub selected_markets: Vec<TickerPair>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl LookupTickersForMarketInMultivariateEventCollectionRequest {
    pub fn new(selected_markets: Vec<TickerPair>) -> Self {
        Self {
            selected_markets: selected_markets,
            extra: std::collections::BTreeMap::new(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LookupTickersForMarketInMultivariateEventCollectionResponse {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct LookupPoint {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "selected_markets")]
    pub selected_markets: Vec<TickerPair>,
    #[serde(rename = "last_queried_ts")]
    pub last_queried_ts: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMultivariateEventCollectionLookupHistoryResponse {
    #[serde(rename = "lookup_points")]
    pub lookup_points: Vec<LookupPoint>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMarketInMultivariateEventCollectionRequest {
    #[serde(rename = "selected_markets")]
    pub selected_markets: Vec<TickerPair>,
    #[serde(rename = "with_market_payload")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub with_market_payload: Option<bool>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

impl CreateMarketInMultivariateEventCollectionRequest {
    pub fn new(selected_markets: Vec<TickerPair>) -> Self {
        Self {
            selected_markets: selected_markets,
            with_market_payload: None,
            extra: std::collections::BTreeMap::new(),
        }
    }

    pub fn with_market_payload(mut self, with_market_payload: bool) -> Self {
        self.with_market_payload = Some(with_market_payload);
        self
    }
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct CreateMarketInMultivariateEventCollectionResponse {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "market")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market: Option<Market>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

pub type PriceLevelDollarsCountFp = Vec<String>;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct OrderbookCountFp {
    #[serde(rename = "yes_dollars")]
    pub yes_dollars: Vec<PriceLevelDollarsCountFp>,
    #[serde(rename = "no_dollars")]
    pub no_dollars: Vec<PriceLevelDollarsCountFp>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketOrderbooksResponse {
    #[serde(rename = "orderbooks")]
    pub orderbooks: Vec<MarketOrderbookFp>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarketOrderbookFp {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "orderbook_fp")]
    pub orderbook_fp: OrderbookCountFp,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketOrderbookResponse {
    #[serde(rename = "orderbook_fp")]
    pub orderbook_fp: OrderbookCountFp,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetEventsResponse {
    #[serde(rename = "events")]
    pub events: Vec<EventData>,
    #[serde(rename = "milestones")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub milestones: Option<Vec<Milestone>>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMultivariateEventsResponse {
    #[serde(rename = "events")]
    pub events: Vec<EventData>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EventFeeChange {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "fee_type_override")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_type_override: Option<FeeType>,
    #[serde(rename = "fee_multiplier_override")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_multiplier_override: Option<f64>,
    #[serde(rename = "scheduled_ts")]
    pub scheduled_ts: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetEventFeeChangesResponse {
    #[serde(rename = "event_fee_changes")]
    pub event_fee_changes: Vec<EventFeeChange>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetEventResponse {
    #[serde(rename = "event")]
    pub event: EventData,
    #[serde(rename = "markets")]
    pub markets: Vec<Market>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MarketMetadata {
    #[serde(rename = "market_ticker")]
    pub market_ticker: String,
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "color_code")]
    pub color_code: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetEventMetadataResponse {
    #[serde(rename = "image_url")]
    pub image_url: String,
    #[serde(rename = "featured_image_url")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub featured_image_url: Option<String>,
    #[serde(rename = "market_details")]
    pub market_details: Vec<MarketMetadata>,
    #[serde(rename = "settlement_sources")]
    pub settlement_sources: Vec<SettlementSource>,
    #[serde(rename = "competition")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub competition: Option<String>,
    #[serde(rename = "competition_scope")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub competition_scope: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetEventForecastPercentilesHistoryResponse {
    #[serde(rename = "forecast_history")]
    pub forecast_history: Vec<ForecastPercentilesPoint>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct ForecastPercentilesPoint {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "end_period_ts")]
    pub end_period_ts: i64,
    #[serde(rename = "period_interval")]
    pub period_interval: i64,
    #[serde(rename = "percentile_points")]
    pub percentile_points: Vec<PercentilePoint>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PercentilePoint {
    #[serde(rename = "percentile")]
    pub percentile: i64,
    #[serde(rename = "raw_numerical_forecast")]
    pub raw_numerical_forecast: f64,
    #[serde(rename = "numerical_forecast")]
    pub numerical_forecast: f64,
    #[serde(rename = "formatted_forecast")]
    pub formatted_forecast: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct EventData {
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "sub_title")]
    pub sub_title: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "collateral_return_type")]
    pub collateral_return_type: String,
    #[serde(rename = "mutually_exclusive")]
    pub mutually_exclusive: bool,
    #[serde(rename = "category")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub category: Option<String>,
    #[serde(rename = "strike_date")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strike_date: Option<String>,
    #[serde(rename = "strike_period")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strike_period: Option<String>,
    #[serde(rename = "markets")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub markets: Option<Vec<Market>>,
    #[serde(rename = "available_on_brokers")]
    pub available_on_brokers: bool,
    #[serde(rename = "product_metadata")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "settlement_sources")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_sources: Option<Vec<SettlementSource>>,
    #[serde(rename = "last_updated_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_ts: Option<String>,
    #[serde(rename = "fee_type_override")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_type_override: Option<String>,
    #[serde(rename = "fee_multiplier_override")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_multiplier_override: Option<f64>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Series {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "frequency")]
    pub frequency: String,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "tags")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub tags: Option<Vec<String>>,
    #[serde(rename = "settlement_sources")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_sources: Option<Vec<SettlementSource>>,
    #[serde(rename = "contract_url")]
    pub contract_url: String,
    #[serde(rename = "contract_terms_url")]
    pub contract_terms_url: String,
    #[serde(rename = "product_metadata")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub product_metadata: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "fee_type")]
    pub fee_type: FeeType,
    #[serde(rename = "fee_multiplier")]
    pub fee_multiplier: f64,
    #[serde(rename = "additional_prohibitions")]
    pub additional_prohibitions: Vec<String>,
    #[serde(rename = "volume_fp")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub volume_fp: Option<FixedPointCount>,
    #[serde(rename = "last_updated_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_updated_ts: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SeriesFeeChange {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "series_ticker")]
    pub series_ticker: String,
    #[serde(rename = "fee_type")]
    pub fee_type: FeeType,
    #[serde(rename = "fee_multiplier")]
    pub fee_multiplier: f64,
    #[serde(rename = "scheduled_ts")]
    pub scheduled_ts: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSeriesResponse {
    #[serde(rename = "series")]
    pub series: Series,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSeriesListResponse {
    #[serde(rename = "series")]
    pub series: Vec<Series>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetSeriesFeeChangesResponse {
    #[serde(rename = "series_fee_change_arr")]
    pub series_fee_change_arr: Vec<SeriesFeeChange>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct SettlementSource {
    #[serde(rename = "name")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    #[serde(rename = "url")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketsResponse {
    #[serde(rename = "markets")]
    pub markets: Vec<Market>,
    #[serde(rename = "cursor")]
    pub cursor: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct GetMarketResponse {
    #[serde(rename = "market")]
    pub market: Market,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct MveSelectedLeg {
    #[serde(rename = "event_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub event_ticker: Option<String>,
    #[serde(rename = "market_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub market_ticker: Option<String>,
    #[serde(rename = "side")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub side: Option<String>,
    #[serde(rename = "yes_settlement_value_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub yes_settlement_value_dollars: Option<FixedPointDollars>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct PriceRange {
    #[serde(rename = "start")]
    pub start: String,
    #[serde(rename = "end")]
    pub end: String,
    #[serde(rename = "step")]
    pub step: String,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub struct Market {
    #[serde(rename = "ticker")]
    pub ticker: String,
    #[serde(rename = "event_ticker")]
    pub event_ticker: String,
    #[serde(rename = "market_type")]
    pub market_type: String,
    #[serde(rename = "title")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title: Option<String>,
    #[serde(rename = "subtitle")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subtitle: Option<String>,
    #[serde(rename = "yes_sub_title")]
    pub yes_sub_title: String,
    #[serde(rename = "no_sub_title")]
    pub no_sub_title: String,
    #[serde(rename = "created_time")]
    pub created_time: String,
    #[serde(rename = "updated_time")]
    pub updated_time: String,
    #[serde(rename = "open_time")]
    pub open_time: String,
    #[serde(rename = "close_time")]
    pub close_time: String,
    #[serde(rename = "expected_expiration_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expected_expiration_time: Option<String>,
    #[serde(rename = "expiration_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub expiration_time: Option<String>,
    #[serde(rename = "latest_expiration_time")]
    pub latest_expiration_time: String,
    #[serde(rename = "settlement_timer_seconds")]
    pub settlement_timer_seconds: i64,
    #[serde(rename = "status")]
    pub status: String,
    #[serde(rename = "response_price_units")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub response_price_units: Option<String>,
    #[serde(rename = "yes_bid_dollars")]
    pub yes_bid_dollars: FixedPointDollars,
    #[serde(rename = "yes_bid_size_fp")]
    pub yes_bid_size_fp: FixedPointCount,
    #[serde(rename = "yes_ask_dollars")]
    pub yes_ask_dollars: FixedPointDollars,
    #[serde(rename = "yes_ask_size_fp")]
    pub yes_ask_size_fp: FixedPointCount,
    #[serde(rename = "no_bid_dollars")]
    pub no_bid_dollars: FixedPointDollars,
    #[serde(rename = "no_ask_dollars")]
    pub no_ask_dollars: FixedPointDollars,
    #[serde(rename = "last_price_dollars")]
    pub last_price_dollars: FixedPointDollars,
    #[serde(rename = "volume_fp")]
    pub volume_fp: FixedPointCount,
    #[serde(rename = "volume_24h_fp")]
    pub volume_24h_fp: FixedPointCount,
    #[serde(rename = "result")]
    pub result: String,
    #[serde(rename = "can_close_early")]
    pub can_close_early: bool,
    #[serde(rename = "fractional_trading_enabled")]
    pub fractional_trading_enabled: bool,
    #[serde(rename = "open_interest_fp")]
    pub open_interest_fp: FixedPointCount,
    #[serde(rename = "notional_value_dollars")]
    pub notional_value_dollars: FixedPointDollars,
    #[serde(rename = "previous_yes_bid_dollars")]
    pub previous_yes_bid_dollars: FixedPointDollars,
    #[serde(rename = "previous_yes_ask_dollars")]
    pub previous_yes_ask_dollars: FixedPointDollars,
    #[serde(rename = "previous_price_dollars")]
    pub previous_price_dollars: FixedPointDollars,
    #[serde(rename = "liquidity_dollars")]
    pub liquidity_dollars: FixedPointDollars,
    #[serde(rename = "settlement_value_dollars")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_value_dollars: Option<FixedPointDollars>,
    #[serde(rename = "settlement_ts")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub settlement_ts: Option<String>,
    #[serde(rename = "expiration_value")]
    pub expiration_value: String,
    #[serde(rename = "occurrence_datetime")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub occurrence_datetime: Option<String>,
    #[serde(rename = "fee_waiver_expiration_time")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub fee_waiver_expiration_time: Option<String>,
    #[serde(rename = "early_close_condition")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub early_close_condition: Option<String>,
    #[serde(rename = "strike_type")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub strike_type: Option<String>,
    #[serde(rename = "floor_strike")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub floor_strike: Option<f64>,
    #[serde(rename = "cap_strike")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cap_strike: Option<f64>,
    #[serde(rename = "functional_strike")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub functional_strike: Option<String>,
    #[serde(rename = "custom_strike")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub custom_strike: Option<std::collections::BTreeMap<String, serde_json::Value>>,
    #[serde(rename = "rules_primary")]
    pub rules_primary: String,
    #[serde(rename = "rules_secondary")]
    pub rules_secondary: String,
    #[serde(rename = "mve_collection_ticker")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mve_collection_ticker: Option<String>,
    #[serde(rename = "mve_selected_legs")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub mve_selected_legs: Option<Vec<MveSelectedLeg>>,
    #[serde(rename = "primary_participant_key")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub primary_participant_key: Option<String>,
    #[serde(rename = "price_level_structure")]
    pub price_level_structure: String,
    #[serde(rename = "price_ranges")]
    pub price_ranges: Vec<PriceRange>,
    #[serde(rename = "is_provisional")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub is_provisional: Option<bool>,
    #[serde(rename = "exchange_index")]
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exchange_index: Option<ExchangeIndex>,
    #[serde(flatten)]
    pub extra: std::collections::BTreeMap<String, serde_json::Value>,
}
