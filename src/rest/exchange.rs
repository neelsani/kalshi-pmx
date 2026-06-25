use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{
    ExchangeStatus, GetExchangeAnnouncementsResponse, GetExchangeScheduleResponse,
    GetSeriesFeeChangesResponse, GetUserDataTimestampResponse,
};
use crate::params::GetSeriesFeeChangesParams;

/// Client for exchange status, schedule, announcements, and user-data timestamp endpoints.
#[derive(Clone, Debug)]
pub struct ExchangeClient {
    client: Kalshi,
}

impl ExchangeClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn status(&self) -> Result<ExchangeStatus> {
        self.client.typed().get_exchange_status().await
    }

    pub async fn announcements(&self) -> Result<GetExchangeAnnouncementsResponse> {
        self.client.typed().get_exchange_announcements().await
    }

    pub async fn schedule(&self) -> Result<GetExchangeScheduleResponse> {
        self.client.typed().get_exchange_schedule().await
    }

    pub async fn user_data_timestamp(&self) -> Result<GetUserDataTimestampResponse> {
        self.client.typed().get_user_data_timestamp().await
    }

    pub async fn series_fee_changes(
        &self,
        params: &GetSeriesFeeChangesParams,
    ) -> Result<GetSeriesFeeChangesResponse> {
        self.client.typed().get_series_fee_changes(params).await
    }
}
