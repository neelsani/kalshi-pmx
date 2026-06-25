use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::GetIncentiveProgramsResponse;
use crate::params::GetIncentiveProgramsParams;

/// Client for incentive program endpoints.
#[derive(Clone, Debug)]
pub struct IncentiveProgramsClient {
    client: Kalshi,
}

impl IncentiveProgramsClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        params: &GetIncentiveProgramsParams,
    ) -> Result<GetIncentiveProgramsResponse> {
        self.client.typed().get_incentive_programs(params).await
    }
}
