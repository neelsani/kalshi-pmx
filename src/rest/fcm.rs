use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{GetOrdersResponse, GetPositionsResponse};
use crate::params::{GetFCMOrdersParams, GetFCMPositionsParams};

/// Client for FCM order and position endpoints.
#[derive(Clone, Debug)]
pub struct FcmClient {
    client: Kalshi,
}

impl FcmClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn orders(&self, params: &GetFCMOrdersParams) -> Result<GetOrdersResponse> {
        self.client.typed().get_fcm_orders(params).await
    }

    pub async fn positions(&self, params: &GetFCMPositionsParams) -> Result<GetPositionsResponse> {
        self.client.typed().get_fcm_positions(params).await
    }
}
