use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{
    CreateMarketInMultivariateEventCollectionRequest,
    CreateMarketInMultivariateEventCollectionResponse,
    GetMultivariateEventCollectionLookupHistoryResponse, GetMultivariateEventCollectionResponse,
    GetMultivariateEventCollectionsResponse,
    LookupTickersForMarketInMultivariateEventCollectionRequest,
    LookupTickersForMarketInMultivariateEventCollectionResponse,
};
use crate::params::{
    GetMultivariateEventCollectionLookupHistoryParams, GetMultivariateEventCollectionsParams,
};

/// Client for multivariate event collection endpoints.
#[derive(Clone, Debug)]
pub struct MultivariateClient {
    client: Kalshi,
}

impl MultivariateClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn collections(
        &self,
        params: &GetMultivariateEventCollectionsParams,
    ) -> Result<GetMultivariateEventCollectionsResponse> {
        self.client
            .typed()
            .get_multivariate_event_collections(params)
            .await
    }

    pub async fn collection(
        &self,
        collection_ticker: impl AsRef<str>,
    ) -> Result<GetMultivariateEventCollectionResponse> {
        self.client
            .typed()
            .get_multivariate_event_collection(collection_ticker)
            .await
    }

    pub async fn create_market_in_collection(
        &self,
        collection_ticker: impl AsRef<str>,
        body: &CreateMarketInMultivariateEventCollectionRequest,
    ) -> Result<CreateMarketInMultivariateEventCollectionResponse> {
        self.client
            .typed()
            .create_market_in_multivariate_event_collection(collection_ticker, body)
            .await
    }

    pub async fn lookup_tickers(
        &self,
        collection_ticker: impl AsRef<str>,
        body: &LookupTickersForMarketInMultivariateEventCollectionRequest,
    ) -> Result<LookupTickersForMarketInMultivariateEventCollectionResponse> {
        self.client
            .typed()
            .lookup_tickers_for_market_in_multivariate_event_collection(collection_ticker, body)
            .await
    }

    pub async fn lookup_history(
        &self,
        collection_ticker: impl AsRef<str>,
        params: &GetMultivariateEventCollectionLookupHistoryParams,
    ) -> Result<GetMultivariateEventCollectionLookupHistoryResponse> {
        self.client
            .typed()
            .get_multivariate_event_collection_lookup_history(collection_ticker, params)
            .await
    }
}
