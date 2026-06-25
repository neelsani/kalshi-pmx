use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{
    GetEventFeeChangesResponse, GetEventForecastPercentilesHistoryResponse,
    GetEventMetadataResponse, GetEventResponse, GetEventsResponse, GetMultivariateEventsResponse,
};
use crate::params::{
    GetEventFeeChangesParams, GetEventForecastPercentilesHistoryParams, GetEventParams,
    GetEventsParams, GetMultivariateEventsParams,
};

/// Client for event metadata, listings, fee changes, and forecast history endpoints.
#[derive(Clone, Debug)]
pub struct EventsClient {
    client: Kalshi,
}

impl EventsClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: &GetEventsParams) -> Result<GetEventsResponse> {
        self.client.typed().get_events(params).await
    }

    pub async fn multivariate(
        &self,
        params: &GetMultivariateEventsParams,
    ) -> Result<GetMultivariateEventsResponse> {
        self.client.typed().get_multivariate_events(params).await
    }

    pub async fn fee_changes(
        &self,
        params: &GetEventFeeChangesParams,
    ) -> Result<GetEventFeeChangesResponse> {
        self.client.typed().get_event_fee_changes(params).await
    }

    pub async fn get(&self, event_ticker: impl AsRef<str>) -> Result<GetEventResponse> {
        self.get_with_params(event_ticker, &GetEventParams::new())
            .await
    }

    pub async fn get_with_params(
        &self,
        event_ticker: impl AsRef<str>,
        params: &GetEventParams,
    ) -> Result<GetEventResponse> {
        self.client.typed().get_event(event_ticker, params).await
    }

    pub async fn metadata(
        &self,
        event_ticker: impl AsRef<str>,
    ) -> Result<GetEventMetadataResponse> {
        self.client.typed().get_event_metadata(event_ticker).await
    }

    pub async fn forecast_percentile_history(
        &self,
        series_ticker: impl AsRef<str>,
        ticker: impl AsRef<str>,
        params: &GetEventForecastPercentilesHistoryParams,
    ) -> Result<GetEventForecastPercentilesHistoryResponse> {
        self.client
            .typed()
            .get_event_forecast_percentiles_history(series_ticker, ticker, params)
            .await
    }
}
