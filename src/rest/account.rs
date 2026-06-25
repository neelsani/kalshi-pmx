use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{
    EmptyResponse, GetAccountApiLimitsResponse, GetAccountApiUsageLevelVolumeProgressResponse,
    GetAccountEndpointCostsResponse,
};

/// Client for account API limit, usage level, and endpoint cost endpoints.
#[derive(Clone, Debug)]
pub struct AccountClient {
    client: Kalshi,
}

impl AccountClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn limits(&self) -> Result<GetAccountApiLimitsResponse> {
        self.client.typed().get_account_api_limits().await
    }

    pub async fn upgrade_api_usage_level(&self) -> Result<EmptyResponse> {
        self.client.typed().upgrade_account_api_usage_level().await
    }

    pub async fn api_usage_level_volume_progress(
        &self,
    ) -> Result<GetAccountApiUsageLevelVolumeProgressResponse> {
        self.client
            .typed()
            .get_account_api_usage_level_volume_progress()
            .await
    }

    pub async fn endpoint_costs(&self) -> Result<GetAccountEndpointCostsResponse> {
        self.client.typed().get_account_endpoint_costs().await
    }
}
