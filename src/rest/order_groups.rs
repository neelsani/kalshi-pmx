use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{
    CreateOrderGroupRequest, CreateOrderGroupResponse, EmptyResponse, GetOrderGroupResponse,
    GetOrderGroupsResponse, UpdateOrderGroupLimitRequest,
};
use crate::params::{
    DeleteOrderGroupParams, GetOrderGroupParams, GetOrderGroupsParams, ResetOrderGroupParams,
    TriggerOrderGroupParams, UpdateOrderGroupLimitParams,
};

/// Client for order group lifecycle, trigger, reset, and limit endpoints.
#[derive(Clone, Debug)]
pub struct OrderGroupsClient {
    client: Kalshi,
}

impl OrderGroupsClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: &GetOrderGroupsParams) -> Result<GetOrderGroupsResponse> {
        self.client.typed().get_order_groups(params).await
    }

    pub async fn create(&self, body: &CreateOrderGroupRequest) -> Result<CreateOrderGroupResponse> {
        self.client.typed().create_order_group(body).await
    }

    pub async fn get(&self, order_group_id: impl AsRef<str>) -> Result<GetOrderGroupResponse> {
        self.get_with_params(order_group_id, &GetOrderGroupParams::new())
            .await
    }

    pub async fn get_with_params(
        &self,
        order_group_id: impl AsRef<str>,
        params: &GetOrderGroupParams,
    ) -> Result<GetOrderGroupResponse> {
        self.client
            .typed()
            .get_order_group(order_group_id, params)
            .await
    }

    pub async fn delete(&self, order_group_id: impl AsRef<str>) -> Result<EmptyResponse> {
        self.delete_with_params(order_group_id, &DeleteOrderGroupParams::new())
            .await
    }

    pub async fn delete_with_params(
        &self,
        order_group_id: impl AsRef<str>,
        params: &DeleteOrderGroupParams,
    ) -> Result<EmptyResponse> {
        self.client
            .typed()
            .delete_order_group(order_group_id, params)
            .await
    }

    pub async fn reset(&self, order_group_id: impl AsRef<str>) -> Result<EmptyResponse> {
        self.reset_with_params(order_group_id, &ResetOrderGroupParams::new())
            .await
    }

    pub async fn reset_with_params(
        &self,
        order_group_id: impl AsRef<str>,
        params: &ResetOrderGroupParams,
    ) -> Result<EmptyResponse> {
        let body = EmptyResponse::new();
        self.client
            .typed()
            .reset_order_group(order_group_id, params, &body)
            .await
    }

    pub async fn trigger(&self, order_group_id: impl AsRef<str>) -> Result<EmptyResponse> {
        self.trigger_with_params(order_group_id, &TriggerOrderGroupParams::new())
            .await
    }

    pub async fn trigger_with_params(
        &self,
        order_group_id: impl AsRef<str>,
        params: &TriggerOrderGroupParams,
    ) -> Result<EmptyResponse> {
        let body = EmptyResponse::new();
        self.client
            .typed()
            .trigger_order_group(order_group_id, params, &body)
            .await
    }

    pub async fn update_limit(
        &self,
        order_group_id: impl AsRef<str>,
        body: &UpdateOrderGroupLimitRequest,
    ) -> Result<EmptyResponse> {
        self.update_limit_with_params(order_group_id, &UpdateOrderGroupLimitParams::new(), body)
            .await
    }

    pub async fn update_limit_with_params(
        &self,
        order_group_id: impl AsRef<str>,
        params: &UpdateOrderGroupLimitParams,
        body: &UpdateOrderGroupLimitRequest,
    ) -> Result<EmptyResponse> {
        self.client
            .typed()
            .update_order_group_limit(order_group_id, params, body)
            .await
    }
}
