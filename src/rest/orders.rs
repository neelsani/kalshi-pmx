use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{
    AmendOrderV2Request, AmendOrderV2Response, BatchCancelOrdersV2Request,
    BatchCancelOrdersV2Response, BatchCreateOrdersV2Request, BatchCreateOrdersV2Response,
    CancelOrderV2Response, CreateOrderV2Request, CreateOrderV2Response, DecreaseOrderV2Request,
    DecreaseOrderV2Response, GetOrderQueuePositionResponse, GetOrderQueuePositionsResponse,
    GetOrderResponse, GetOrdersResponse,
};
use crate::params::{
    AmendOrderV2Params, CancelOrderV2Params, DecreaseOrderV2Params, GetOrderQueuePositionsParams,
    GetOrdersParams,
};

/// Client for order listing, creation, cancellation, amendment, and queue-position endpoints.
#[derive(Clone, Debug)]
pub struct OrdersClient {
    client: Kalshi,
}

impl OrdersClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn list(&self, params: &GetOrdersParams) -> Result<GetOrdersResponse> {
        self.client.typed().get_orders(params).await
    }

    pub async fn get(&self, order_id: impl AsRef<str>) -> Result<GetOrderResponse> {
        self.client.typed().get_order(order_id).await
    }

    pub async fn queue_positions(
        &self,
        params: &GetOrderQueuePositionsParams,
    ) -> Result<GetOrderQueuePositionsResponse> {
        self.client.typed().get_order_queue_positions(params).await
    }

    pub async fn queue_position(
        &self,
        order_id: impl AsRef<str>,
    ) -> Result<GetOrderQueuePositionResponse> {
        self.client.typed().get_order_queue_position(order_id).await
    }

    pub async fn create(&self, order: &CreateOrderV2Request) -> Result<CreateOrderV2Response> {
        self.client.typed().create_order_v2(order).await
    }

    pub async fn batch_create(
        &self,
        body: &BatchCreateOrdersV2Request,
    ) -> Result<BatchCreateOrdersV2Response> {
        self.client.typed().batch_create_orders_v2(body).await
    }

    pub async fn batch_cancel(
        &self,
        body: &BatchCancelOrdersV2Request,
    ) -> Result<BatchCancelOrdersV2Response> {
        self.client.typed().batch_cancel_orders_v2(body).await
    }

    pub async fn cancel(&self, order_id: impl AsRef<str>) -> Result<CancelOrderV2Response> {
        self.cancel_with_params(order_id, &CancelOrderV2Params::new())
            .await
    }

    pub async fn cancel_with_params(
        &self,
        order_id: impl AsRef<str>,
        params: &CancelOrderV2Params,
    ) -> Result<CancelOrderV2Response> {
        self.client.typed().cancel_order_v2(order_id, params).await
    }

    pub async fn amend(
        &self,
        order_id: impl AsRef<str>,
        body: &AmendOrderV2Request,
    ) -> Result<AmendOrderV2Response> {
        self.amend_with_params(order_id, &AmendOrderV2Params::new(), body)
            .await
    }

    pub async fn amend_with_params(
        &self,
        order_id: impl AsRef<str>,
        params: &AmendOrderV2Params,
        body: &AmendOrderV2Request,
    ) -> Result<AmendOrderV2Response> {
        self.client
            .typed()
            .amend_order_v2(order_id, params, body)
            .await
    }

    pub async fn decrease(
        &self,
        order_id: impl AsRef<str>,
        body: &DecreaseOrderV2Request,
    ) -> Result<DecreaseOrderV2Response> {
        self.decrease_with_params(order_id, &DecreaseOrderV2Params::new(), body)
            .await
    }

    pub async fn decrease_with_params(
        &self,
        order_id: impl AsRef<str>,
        params: &DecreaseOrderV2Params,
        body: &DecreaseOrderV2Request,
    ) -> Result<DecreaseOrderV2Response> {
        self.client
            .typed()
            .decrease_order_v2(order_id, params, body)
            .await
    }
}
