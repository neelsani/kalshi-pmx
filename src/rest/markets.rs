use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{
    BatchGetMarketCandlesticksResponse, GetEventCandlesticksResponse,
    GetMarketCandlesticksResponse, GetMarketOrderbookResponse, GetMarketOrderbooksResponse,
    GetMarketResponse, GetMarketsResponse, GetSeriesListResponse, GetSeriesResponse,
    GetTradesResponse,
};
use crate::params::{
    BatchGetMarketCandlesticksParams, GetMarketCandlesticksByEventParams,
    GetMarketCandlesticksParams, GetMarketOrderbookParams, GetMarketOrderbooksParams,
    GetMarketsParams, GetSeriesListParams, GetSeriesParams, GetTradesParams,
};

/// Client for markets, series, orderbooks, trades, and candlesticks.
#[derive(Clone, Debug)]
pub struct MarketsClient {
    client: Kalshi,
}

impl MarketsClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: &GetMarketsParams) -> Result<GetMarketsResponse> {
        self.client.typed().get_markets(params).await
    }

    pub async fn get(&self, ticker: impl AsRef<str>) -> Result<GetMarketResponse> {
        self.client.typed().get_market(ticker).await
    }

    pub async fn trades(&self, params: &GetTradesParams) -> Result<GetTradesResponse> {
        self.client.typed().get_trades(params).await
    }

    pub async fn orderbook(
        &self,
        ticker: impl AsRef<str>,
        params: &GetMarketOrderbookParams,
    ) -> Result<GetMarketOrderbookResponse> {
        self.client
            .typed()
            .get_market_orderbook(ticker, params)
            .await
    }

    pub async fn orderbooks(
        &self,
        params: &GetMarketOrderbooksParams,
    ) -> Result<GetMarketOrderbooksResponse> {
        self.client.typed().get_market_orderbooks(params).await
    }

    pub async fn series(&self, series_ticker: impl AsRef<str>) -> Result<GetSeriesResponse> {
        self.series_with_params(series_ticker, &GetSeriesParams::new())
            .await
    }

    pub async fn series_with_params(
        &self,
        series_ticker: impl AsRef<str>,
        params: &GetSeriesParams,
    ) -> Result<GetSeriesResponse> {
        self.client.typed().get_series(series_ticker, params).await
    }

    pub async fn series_list(&self, params: &GetSeriesListParams) -> Result<GetSeriesListResponse> {
        self.client.typed().get_series_list(params).await
    }

    pub async fn candlesticks(
        &self,
        series_ticker: impl AsRef<str>,
        ticker: impl AsRef<str>,
        params: &GetMarketCandlesticksParams,
    ) -> Result<GetMarketCandlesticksResponse> {
        self.client
            .typed()
            .get_market_candlesticks(series_ticker, ticker, params)
            .await
    }

    pub async fn candlesticks_by_event(
        &self,
        series_ticker: impl AsRef<str>,
        ticker: impl AsRef<str>,
        params: &GetMarketCandlesticksByEventParams,
    ) -> Result<GetEventCandlesticksResponse> {
        self.client
            .typed()
            .get_market_candlesticks_by_event(series_ticker, ticker, params)
            .await
    }

    pub async fn batch_candlesticks(
        &self,
        params: &BatchGetMarketCandlesticksParams,
    ) -> Result<BatchGetMarketCandlesticksResponse> {
        self.client
            .typed()
            .batch_get_market_candlesticks(params)
            .await
    }
}
