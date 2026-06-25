use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{GetMilestoneResponse, GetMilestonesResponse};
use crate::params::GetMilestonesParams;

/// Client for milestone listing and lookup endpoints.
#[derive(Clone, Debug)]
pub struct MilestonesClient {
    client: Kalshi,
}

impl MilestonesClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: &GetMilestonesParams) -> Result<GetMilestonesResponse> {
        self.client.typed().get_milestones(params).await
    }

    pub async fn get(&self, milestone_id: impl AsRef<str>) -> Result<GetMilestoneResponse> {
        self.client.typed().get_milestone(milestone_id).await
    }
}
