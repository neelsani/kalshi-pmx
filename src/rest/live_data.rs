use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{GetGameStatsResponse, GetLiveDataResponse, GetLiveDatasResponse};
use crate::params::{GetLiveDataByMilestoneParams, GetLiveDataParams, GetLiveDatasParams};

/// Client for live data and game stats endpoints.
#[derive(Clone, Debug)]
pub struct LiveDataClient {
    client: Kalshi,
}

impl LiveDataClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn by_milestone(&self, milestone_id: impl AsRef<str>) -> Result<GetLiveDataResponse> {
        self.by_milestone_with_params(milestone_id, &GetLiveDataByMilestoneParams::new())
            .await
    }

    pub async fn by_milestone_with_params(
        &self,
        milestone_id: impl AsRef<str>,
        params: &GetLiveDataByMilestoneParams,
    ) -> Result<GetLiveDataResponse> {
        self.client
            .typed()
            .get_live_data_by_milestone(milestone_id, params)
            .await
    }

    pub async fn by_type_and_milestone(
        &self,
        kind: impl AsRef<str>,
        milestone_id: impl AsRef<str>,
    ) -> Result<GetLiveDataResponse> {
        self.by_type_and_milestone_with_params(kind, milestone_id, &GetLiveDataParams::new())
            .await
    }

    pub async fn by_type_and_milestone_with_params(
        &self,
        kind: impl AsRef<str>,
        milestone_id: impl AsRef<str>,
        params: &GetLiveDataParams,
    ) -> Result<GetLiveDataResponse> {
        self.client
            .typed()
            .get_live_data(kind, milestone_id, params)
            .await
    }

    pub async fn batch(&self, params: &GetLiveDatasParams) -> Result<GetLiveDatasResponse> {
        self.client.typed().get_live_datas(params).await
    }

    pub async fn game_stats(&self, milestone_id: impl AsRef<str>) -> Result<GetGameStatsResponse> {
        self.client.typed().get_game_stats(milestone_id).await
    }
}
