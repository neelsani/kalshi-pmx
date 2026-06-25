use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{
    GetFillsResponse, GetHistoricalCutoffResponse, GetMarketCandlesticksHistoricalResponse,
    GetMarketResponse, GetMarketsResponse, GetOrdersResponse, GetTradesResponse,
};
use crate::params::{
    GetFillsHistoricalParams, GetHistoricalMarketsParams, GetHistoricalOrdersParams,
    GetMarketCandlesticksHistoricalParams, GetTradesHistoricalParams,
};

/// Client for historical markets, trades, orders, fills, and candlestick endpoints.
#[derive(Clone, Debug)]
pub struct HistoricalClient {
    client: Kalshi,
}

impl HistoricalClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn cutoff(&self) -> Result<GetHistoricalCutoffResponse> {
        self.client.typed().get_historical_cutoff().await
    }

    pub async fn market_candlesticks(
        &self,
        ticker: impl AsRef<str>,
        params: &GetMarketCandlesticksHistoricalParams,
    ) -> Result<GetMarketCandlesticksHistoricalResponse> {
        self.client
            .typed()
            .get_market_candlesticks_historical(ticker, params)
            .await
    }

    pub async fn fills(&self, params: &GetFillsHistoricalParams) -> Result<GetFillsResponse> {
        self.client.typed().get_fills_historical(params).await
    }

    pub async fn orders(&self, params: &GetHistoricalOrdersParams) -> Result<GetOrdersResponse> {
        self.client.typed().get_historical_orders(params).await
    }

    pub async fn trades(&self, params: &GetTradesHistoricalParams) -> Result<GetTradesResponse> {
        self.client.typed().get_trades_historical(params).await
    }

    pub async fn markets(&self, params: &GetHistoricalMarketsParams) -> Result<GetMarketsResponse> {
        self.client.typed().get_historical_markets(params).await
    }

    pub async fn market(&self, ticker: impl AsRef<str>) -> Result<GetMarketResponse> {
        self.client.typed().get_historical_market(ticker).await
    }
}
