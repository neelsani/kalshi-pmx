//! Generated typed Trade API methods.
//! Regenerate with `cargo run --manifest-path xtask/Cargo.toml -- generate-openapi-types`.

use crate::generated::*;

#[derive(Clone, Debug)]
pub struct TypedClient {
    client: crate::Kalshi,
}

impl TypedClient {
    pub(crate) fn new(client: crate::Kalshi) -> Self {
        Self { client }
    }

    pub async fn get_exchange_status(&self) -> crate::Result<ExchangeStatus> {
        let path = "/exchange/status";
        self.client
            .request_json::<ExchangeStatus, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_exchange_announcements(
        &self,
    ) -> crate::Result<GetExchangeAnnouncementsResponse> {
        let path = "/exchange/announcements";
        self.client
            .request_json::<GetExchangeAnnouncementsResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_series_fee_changes(
        &self,
        params: &crate::params::GetSeriesFeeChangesParams,
    ) -> crate::Result<GetSeriesFeeChangesResponse> {
        let path = "/series/fee_changes";
        self.client
            .request_json::<GetSeriesFeeChangesResponse, crate::params::GetSeriesFeeChangesParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_exchange_schedule(&self) -> crate::Result<GetExchangeScheduleResponse> {
        let path = "/exchange/schedule";
        self.client
            .request_json::<GetExchangeScheduleResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_user_data_timestamp(&self) -> crate::Result<GetUserDataTimestampResponse> {
        let path = "/exchange/user_data_timestamp";
        self.client
            .request_json::<GetUserDataTimestampResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_market_candlesticks(
        &self,
        series_ticker: impl AsRef<str>,
        ticker: impl AsRef<str>,
        params: &crate::params::GetMarketCandlesticksParams,
    ) -> crate::Result<GetMarketCandlesticksResponse> {
        let path = format!(
            "/series/{}/markets/{}/candlesticks",
            crate::rest::enc(series_ticker),
            crate::rest::enc(ticker)
        );
        self.client
            .request_json::<GetMarketCandlesticksResponse, crate::params::GetMarketCandlesticksParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_trades(
        &self,
        params: &crate::params::GetTradesParams,
    ) -> crate::Result<GetTradesResponse> {
        let path = "/markets/trades";
        self.client
            .request_json::<GetTradesResponse, crate::params::GetTradesParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_market_orderbook(
        &self,
        ticker: impl AsRef<str>,
        params: &crate::params::GetMarketOrderbookParams,
    ) -> crate::Result<GetMarketOrderbookResponse> {
        let path = format!("/markets/{}/orderbook", crate::rest::enc(ticker));
        self.client
            .request_json::<GetMarketOrderbookResponse, crate::params::GetMarketOrderbookParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_market_orderbooks(
        &self,
        params: &crate::params::GetMarketOrderbooksParams,
    ) -> crate::Result<GetMarketOrderbooksResponse> {
        let path = "/markets/orderbooks";
        self.client
            .request_json::<GetMarketOrderbooksResponse, crate::params::GetMarketOrderbooksParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_series(
        &self,
        series_ticker: impl AsRef<str>,
        params: &crate::params::GetSeriesParams,
    ) -> crate::Result<GetSeriesResponse> {
        let path = format!("/series/{}", crate::rest::enc(series_ticker));
        self.client
            .request_json::<GetSeriesResponse, crate::params::GetSeriesParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_series_list(
        &self,
        params: &crate::params::GetSeriesListParams,
    ) -> crate::Result<GetSeriesListResponse> {
        let path = "/series";
        self.client
            .request_json::<GetSeriesListResponse, crate::params::GetSeriesListParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_markets(
        &self,
        params: &crate::params::GetMarketsParams,
    ) -> crate::Result<GetMarketsResponse> {
        let path = "/markets";
        self.client
            .request_json::<GetMarketsResponse, crate::params::GetMarketsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_market(&self, ticker: impl AsRef<str>) -> crate::Result<GetMarketResponse> {
        let path = format!("/markets/{}", crate::rest::enc(ticker));
        self.client
            .request_json::<GetMarketResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn batch_get_market_candlesticks(
        &self,
        params: &crate::params::BatchGetMarketCandlesticksParams,
    ) -> crate::Result<BatchGetMarketCandlesticksResponse> {
        let path = "/markets/candlesticks";
        self.client
            .request_json::<BatchGetMarketCandlesticksResponse, crate::params::BatchGetMarketCandlesticksParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_market_candlesticks_by_event(
        &self,
        series_ticker: impl AsRef<str>,
        ticker: impl AsRef<str>,
        params: &crate::params::GetMarketCandlesticksByEventParams,
    ) -> crate::Result<GetEventCandlesticksResponse> {
        let path = format!(
            "/series/{}/events/{}/candlesticks",
            crate::rest::enc(series_ticker),
            crate::rest::enc(ticker)
        );
        self.client
            .request_json::<GetEventCandlesticksResponse, crate::params::GetMarketCandlesticksByEventParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_events(
        &self,
        params: &crate::params::GetEventsParams,
    ) -> crate::Result<GetEventsResponse> {
        let path = "/events";
        self.client
            .request_json::<GetEventsResponse, crate::params::GetEventsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_multivariate_events(
        &self,
        params: &crate::params::GetMultivariateEventsParams,
    ) -> crate::Result<GetMultivariateEventsResponse> {
        let path = "/events/multivariate";
        self.client
            .request_json::<GetMultivariateEventsResponse, crate::params::GetMultivariateEventsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_event_fee_changes(
        &self,
        params: &crate::params::GetEventFeeChangesParams,
    ) -> crate::Result<GetEventFeeChangesResponse> {
        let path = "/events/fee_changes";
        self.client
            .request_json::<GetEventFeeChangesResponse, crate::params::GetEventFeeChangesParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_event(
        &self,
        event_ticker: impl AsRef<str>,
        params: &crate::params::GetEventParams,
    ) -> crate::Result<GetEventResponse> {
        let path = format!("/events/{}", crate::rest::enc(event_ticker));
        self.client
            .request_json::<GetEventResponse, crate::params::GetEventParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_event_metadata(
        &self,
        event_ticker: impl AsRef<str>,
    ) -> crate::Result<GetEventMetadataResponse> {
        let path = format!("/events/{}/metadata", crate::rest::enc(event_ticker));
        self.client
            .request_json::<GetEventMetadataResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_event_forecast_percentiles_history(
        &self,
        series_ticker: impl AsRef<str>,
        ticker: impl AsRef<str>,
        params: &crate::params::GetEventForecastPercentilesHistoryParams,
    ) -> crate::Result<GetEventForecastPercentilesHistoryResponse> {
        let path = format!(
            "/series/{}/events/{}/forecast_percentile_history",
            crate::rest::enc(series_ticker),
            crate::rest::enc(ticker)
        );
        self.client
            .request_json::<GetEventForecastPercentilesHistoryResponse, crate::params::GetEventForecastPercentilesHistoryParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_orders(
        &self,
        params: &crate::params::GetOrdersParams,
    ) -> crate::Result<GetOrdersResponse> {
        let path = "/portfolio/orders";
        self.client
            .request_json::<GetOrdersResponse, crate::params::GetOrdersParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_order(&self, order_id: impl AsRef<str>) -> crate::Result<GetOrderResponse> {
        let path = format!("/portfolio/orders/{}", crate::rest::enc(order_id));
        self.client
            .request_json::<GetOrderResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_order_queue_positions(
        &self,
        params: &crate::params::GetOrderQueuePositionsParams,
    ) -> crate::Result<GetOrderQueuePositionsResponse> {
        let path = "/portfolio/orders/queue_positions";
        self.client
            .request_json::<GetOrderQueuePositionsResponse, crate::params::GetOrderQueuePositionsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_order_queue_position(
        &self,
        order_id: impl AsRef<str>,
    ) -> crate::Result<GetOrderQueuePositionResponse> {
        let path = format!(
            "/portfolio/orders/{}/queue_position",
            crate::rest::enc(order_id)
        );
        self.client
            .request_json::<GetOrderQueuePositionResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn create_order_v2(
        &self,
        body: &CreateOrderV2Request,
    ) -> crate::Result<CreateOrderV2Response> {
        let path = "/portfolio/events/orders";
        self.client
            .request_json::<CreateOrderV2Response, (), CreateOrderV2Request>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn batch_create_orders_v2(
        &self,
        body: &BatchCreateOrdersV2Request,
    ) -> crate::Result<BatchCreateOrdersV2Response> {
        let path = "/portfolio/events/orders/batched";
        self.client
            .request_json::<BatchCreateOrdersV2Response, (), BatchCreateOrdersV2Request>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn batch_cancel_orders_v2(
        &self,
        body: &BatchCancelOrdersV2Request,
    ) -> crate::Result<BatchCancelOrdersV2Response> {
        let path = "/portfolio/events/orders/batched";
        self.client
            .request_json::<BatchCancelOrdersV2Response, (), BatchCancelOrdersV2Request>(
                reqwest::Method::DELETE,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn cancel_order_v2(
        &self,
        order_id: impl AsRef<str>,
        params: &crate::params::CancelOrderV2Params,
    ) -> crate::Result<CancelOrderV2Response> {
        let path = format!("/portfolio/events/orders/{}", crate::rest::enc(order_id));
        self.client
            .request_json::<CancelOrderV2Response, crate::params::CancelOrderV2Params, ()>(
                reqwest::Method::DELETE,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn amend_order_v2(
        &self,
        order_id: impl AsRef<str>,
        params: &crate::params::AmendOrderV2Params,
        body: &AmendOrderV2Request,
    ) -> crate::Result<AmendOrderV2Response> {
        let path = format!(
            "/portfolio/events/orders/{}/amend",
            crate::rest::enc(order_id)
        );
        self.client
            .request_json::<AmendOrderV2Response, crate::params::AmendOrderV2Params, AmendOrderV2Request>(
                reqwest::Method::POST,
                path.as_ref(),
                Some(params),
                Some(body),
            )
            .await
    }

    pub async fn decrease_order_v2(
        &self,
        order_id: impl AsRef<str>,
        params: &crate::params::DecreaseOrderV2Params,
        body: &DecreaseOrderV2Request,
    ) -> crate::Result<DecreaseOrderV2Response> {
        let path = format!(
            "/portfolio/events/orders/{}/decrease",
            crate::rest::enc(order_id)
        );
        self.client
            .request_json::<DecreaseOrderV2Response, crate::params::DecreaseOrderV2Params, DecreaseOrderV2Request>(
                reqwest::Method::POST,
                path.as_ref(),
                Some(params),
                Some(body),
            )
            .await
    }

    pub async fn get_order_groups(
        &self,
        params: &crate::params::GetOrderGroupsParams,
    ) -> crate::Result<GetOrderGroupsResponse> {
        let path = "/portfolio/order_groups";
        self.client
            .request_json::<GetOrderGroupsResponse, crate::params::GetOrderGroupsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn create_order_group(
        &self,
        body: &CreateOrderGroupRequest,
    ) -> crate::Result<CreateOrderGroupResponse> {
        let path = "/portfolio/order_groups/create";
        self.client
            .request_json::<CreateOrderGroupResponse, (), CreateOrderGroupRequest>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn get_order_group(
        &self,
        order_group_id: impl AsRef<str>,
        params: &crate::params::GetOrderGroupParams,
    ) -> crate::Result<GetOrderGroupResponse> {
        let path = format!(
            "/portfolio/order_groups/{}",
            crate::rest::enc(order_group_id)
        );
        self.client
            .request_json::<GetOrderGroupResponse, crate::params::GetOrderGroupParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn delete_order_group(
        &self,
        order_group_id: impl AsRef<str>,
        params: &crate::params::DeleteOrderGroupParams,
    ) -> crate::Result<EmptyResponse> {
        let path = format!(
            "/portfolio/order_groups/{}",
            crate::rest::enc(order_group_id)
        );
        self.client
            .request_json::<EmptyResponse, crate::params::DeleteOrderGroupParams, ()>(
                reqwest::Method::DELETE,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn reset_order_group(
        &self,
        order_group_id: impl AsRef<str>,
        params: &crate::params::ResetOrderGroupParams,
        body: &EmptyResponse,
    ) -> crate::Result<EmptyResponse> {
        let path = format!(
            "/portfolio/order_groups/{}/reset",
            crate::rest::enc(order_group_id)
        );
        self.client
            .request_json::<EmptyResponse, crate::params::ResetOrderGroupParams, EmptyResponse>(
                reqwest::Method::PUT,
                path.as_ref(),
                Some(params),
                Some(body),
            )
            .await
    }

    pub async fn trigger_order_group(
        &self,
        order_group_id: impl AsRef<str>,
        params: &crate::params::TriggerOrderGroupParams,
        body: &EmptyResponse,
    ) -> crate::Result<EmptyResponse> {
        let path = format!(
            "/portfolio/order_groups/{}/trigger",
            crate::rest::enc(order_group_id)
        );
        self.client
            .request_json::<EmptyResponse, crate::params::TriggerOrderGroupParams, EmptyResponse>(
                reqwest::Method::PUT,
                path.as_ref(),
                Some(params),
                Some(body),
            )
            .await
    }

    pub async fn update_order_group_limit(
        &self,
        order_group_id: impl AsRef<str>,
        params: &crate::params::UpdateOrderGroupLimitParams,
        body: &UpdateOrderGroupLimitRequest,
    ) -> crate::Result<EmptyResponse> {
        let path = format!(
            "/portfolio/order_groups/{}/limit",
            crate::rest::enc(order_group_id)
        );
        self.client
            .request_json::<EmptyResponse, crate::params::UpdateOrderGroupLimitParams, UpdateOrderGroupLimitRequest>(
                reqwest::Method::PUT,
                path.as_ref(),
                Some(params),
                Some(body),
            )
            .await
    }

    pub async fn get_balance(
        &self,
        params: &crate::params::GetBalanceParams,
    ) -> crate::Result<GetBalanceResponse> {
        let path = "/portfolio/balance";
        self.client
            .request_json::<GetBalanceResponse, crate::params::GetBalanceParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn create_subaccount(&self) -> crate::Result<CreateSubaccountResponse> {
        let path = "/portfolio/subaccounts";
        self.client
            .request_json::<CreateSubaccountResponse, (), ()>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn apply_subaccount_transfer(
        &self,
        body: &ApplySubaccountTransferRequest,
    ) -> crate::Result<ApplySubaccountTransferResponse> {
        let path = "/portfolio/subaccounts/transfer";
        self.client
            .request_json::<ApplySubaccountTransferResponse, (), ApplySubaccountTransferRequest>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn get_subaccount_balances(&self) -> crate::Result<GetSubaccountBalancesResponse> {
        let path = "/portfolio/subaccounts/balances";
        self.client
            .request_json::<GetSubaccountBalancesResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_subaccount_transfers(
        &self,
        params: &crate::params::GetSubaccountTransfersParams,
    ) -> crate::Result<GetSubaccountTransfersResponse> {
        let path = "/portfolio/subaccounts/transfers";
        self.client
            .request_json::<GetSubaccountTransfersResponse, crate::params::GetSubaccountTransfersParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_subaccount_netting(&self) -> crate::Result<GetSubaccountNettingResponse> {
        let path = "/portfolio/subaccounts/netting";
        self.client
            .request_json::<GetSubaccountNettingResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn update_subaccount_netting(
        &self,
        body: &UpdateSubaccountNettingRequest,
    ) -> crate::Result<EmptyResponse> {
        let path = "/portfolio/subaccounts/netting";
        self.client
            .request_json::<EmptyResponse, (), UpdateSubaccountNettingRequest>(
                reqwest::Method::PUT,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn get_positions(
        &self,
        params: &crate::params::GetPositionsParams,
    ) -> crate::Result<GetPositionsResponse> {
        let path = "/portfolio/positions";
        self.client
            .request_json::<GetPositionsResponse, crate::params::GetPositionsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_settlements(
        &self,
        params: &crate::params::GetSettlementsParams,
    ) -> crate::Result<GetSettlementsResponse> {
        let path = "/portfolio/settlements";
        self.client
            .request_json::<GetSettlementsResponse, crate::params::GetSettlementsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_deposits(
        &self,
        params: &crate::params::GetDepositsParams,
    ) -> crate::Result<GetDepositsResponse> {
        let path = "/portfolio/deposits";
        self.client
            .request_json::<GetDepositsResponse, crate::params::GetDepositsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_withdrawals(
        &self,
        params: &crate::params::GetWithdrawalsParams,
    ) -> crate::Result<GetWithdrawalsResponse> {
        let path = "/portfolio/withdrawals";
        self.client
            .request_json::<GetWithdrawalsResponse, crate::params::GetWithdrawalsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_portfolio_resting_order_total_value(
        &self,
    ) -> crate::Result<GetPortfolioRestingOrderTotalValueResponse> {
        let path = "/portfolio/summary/total_resting_order_value";
        self.client
            .request_json::<GetPortfolioRestingOrderTotalValueResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_fills(
        &self,
        params: &crate::params::GetFillsParams,
    ) -> crate::Result<GetFillsResponse> {
        let path = "/portfolio/fills";
        self.client
            .request_json::<GetFillsResponse, crate::params::GetFillsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_communications_id(&self) -> crate::Result<GetCommunicationsIDResponse> {
        let path = "/communications/id";
        self.client
            .request_json::<GetCommunicationsIDResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_block_trade_proposals(
        &self,
        params: &crate::params::GetBlockTradeProposalsParams,
    ) -> crate::Result<GetBlockTradeProposalsResponse> {
        let path = "/communications/block-trade-proposals";
        self.client
            .request_json::<GetBlockTradeProposalsResponse, crate::params::GetBlockTradeProposalsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn propose_block_trade(
        &self,
        body: &ProposeBlockTradeRequest,
    ) -> crate::Result<ProposeBlockTradeResponse> {
        let path = "/communications/block-trade-proposals";
        self.client
            .request_json::<ProposeBlockTradeResponse, (), ProposeBlockTradeRequest>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn accept_block_trade_proposal(
        &self,
        block_trade_proposal_id: impl AsRef<str>,
        body: &AcceptBlockTradeProposalRequest,
    ) -> crate::Result<EmptyResponse> {
        let path = format!(
            "/communications/block-trade-proposals/{}/accept",
            crate::rest::enc(block_trade_proposal_id)
        );
        self.client
            .request_json::<EmptyResponse, (), AcceptBlockTradeProposalRequest>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn get_rf_qs(
        &self,
        params: &crate::params::GetRFQsParams,
    ) -> crate::Result<GetRFQsResponse> {
        let path = "/communications/rfqs";
        self.client
            .request_json::<GetRFQsResponse, crate::params::GetRFQsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn create_rfq(&self, body: &CreateRFQRequest) -> crate::Result<CreateRFQResponse> {
        let path = "/communications/rfqs";
        self.client
            .request_json::<CreateRFQResponse, (), CreateRFQRequest>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn get_rfq(&self, rfq_id: impl AsRef<str>) -> crate::Result<GetRFQResponse> {
        let path = format!("/communications/rfqs/{}", crate::rest::enc(rfq_id));
        self.client
            .request_json::<GetRFQResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn delete_rfq(&self, rfq_id: impl AsRef<str>) -> crate::Result<EmptyResponse> {
        let path = format!("/communications/rfqs/{}", crate::rest::enc(rfq_id));
        self.client
            .request_json::<EmptyResponse, (), ()>(
                reqwest::Method::DELETE,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_quotes(
        &self,
        params: &crate::params::GetQuotesParams,
    ) -> crate::Result<GetQuotesResponse> {
        let path = "/communications/quotes";
        self.client
            .request_json::<GetQuotesResponse, crate::params::GetQuotesParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn create_quote(
        &self,
        body: &CreateQuoteRequest,
    ) -> crate::Result<CreateQuoteResponse> {
        let path = "/communications/quotes";
        self.client
            .request_json::<CreateQuoteResponse, (), CreateQuoteRequest>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn get_quote(&self, quote_id: impl AsRef<str>) -> crate::Result<GetQuoteResponse> {
        let path = format!("/communications/quotes/{}", crate::rest::enc(quote_id));
        self.client
            .request_json::<GetQuoteResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn delete_quote(&self, quote_id: impl AsRef<str>) -> crate::Result<EmptyResponse> {
        let path = format!("/communications/quotes/{}", crate::rest::enc(quote_id));
        self.client
            .request_json::<EmptyResponse, (), ()>(
                reqwest::Method::DELETE,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn accept_quote(
        &self,
        quote_id: impl AsRef<str>,
        body: &AcceptQuoteRequest,
    ) -> crate::Result<EmptyResponse> {
        let path = format!(
            "/communications/quotes/{}/accept",
            crate::rest::enc(quote_id)
        );
        self.client
            .request_json::<EmptyResponse, (), AcceptQuoteRequest>(
                reqwest::Method::PUT,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn confirm_quote(
        &self,
        quote_id: impl AsRef<str>,
        body: &EmptyResponse,
    ) -> crate::Result<EmptyResponse> {
        let path = format!(
            "/communications/quotes/{}/confirm",
            crate::rest::enc(quote_id)
        );
        self.client
            .request_json::<EmptyResponse, (), EmptyResponse>(
                reqwest::Method::PUT,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn get_api_keys(&self) -> crate::Result<GetApiKeysResponse> {
        let path = "/api_keys";
        self.client
            .request_json::<GetApiKeysResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn create_api_key(
        &self,
        body: &CreateApiKeyRequest,
    ) -> crate::Result<CreateApiKeyResponse> {
        let path = "/api_keys";
        self.client
            .request_json::<CreateApiKeyResponse, (), CreateApiKeyRequest>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn generate_api_key(
        &self,
        body: &GenerateApiKeyRequest,
    ) -> crate::Result<GenerateApiKeyResponse> {
        let path = "/api_keys/generate";
        self.client
            .request_json::<GenerateApiKeyResponse, (), GenerateApiKeyRequest>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn delete_api_key(&self, api_key: impl AsRef<str>) -> crate::Result<EmptyResponse> {
        let path = format!("/api_keys/{}", crate::rest::enc(api_key));
        self.client
            .request_json::<EmptyResponse, (), ()>(
                reqwest::Method::DELETE,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_account_api_limits(&self) -> crate::Result<GetAccountApiLimitsResponse> {
        let path = "/account/limits";
        self.client
            .request_json::<GetAccountApiLimitsResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn upgrade_account_api_usage_level(&self) -> crate::Result<EmptyResponse> {
        let path = "/account/api_usage_level/upgrade";
        self.client
            .request_json::<EmptyResponse, (), ()>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_account_api_usage_level_volume_progress(
        &self,
    ) -> crate::Result<GetAccountApiUsageLevelVolumeProgressResponse> {
        let path = "/account/api_usage_level/volume_progress";
        self.client
            .request_json::<GetAccountApiUsageLevelVolumeProgressResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_account_endpoint_costs(
        &self,
    ) -> crate::Result<GetAccountEndpointCostsResponse> {
        let path = "/account/endpoint_costs";
        self.client
            .request_json::<GetAccountEndpointCostsResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_tags_for_series_categories(
        &self,
    ) -> crate::Result<GetTagsForSeriesCategoriesResponse> {
        let path = "/search/tags_by_categories";
        self.client
            .request_json::<GetTagsForSeriesCategoriesResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_filters_for_sports(&self) -> crate::Result<GetFiltersBySportsResponse> {
        let path = "/search/filters_by_sport";
        self.client
            .request_json::<GetFiltersBySportsResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_live_data_by_milestone(
        &self,
        milestone_id: impl AsRef<str>,
        params: &crate::params::GetLiveDataByMilestoneParams,
    ) -> crate::Result<GetLiveDataResponse> {
        let path = format!("/live_data/milestone/{}", crate::rest::enc(milestone_id));
        self.client
            .request_json::<GetLiveDataResponse, crate::params::GetLiveDataByMilestoneParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_live_data(
        &self,
        type_: impl AsRef<str>,
        milestone_id: impl AsRef<str>,
        params: &crate::params::GetLiveDataParams,
    ) -> crate::Result<GetLiveDataResponse> {
        let path = format!(
            "/live_data/{}/milestone/{}",
            crate::rest::enc(type_),
            crate::rest::enc(milestone_id)
        );
        self.client
            .request_json::<GetLiveDataResponse, crate::params::GetLiveDataParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_live_datas(
        &self,
        params: &crate::params::GetLiveDatasParams,
    ) -> crate::Result<GetLiveDatasResponse> {
        let path = "/live_data/batch";
        self.client
            .request_json::<GetLiveDatasResponse, crate::params::GetLiveDatasParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_game_stats(
        &self,
        milestone_id: impl AsRef<str>,
    ) -> crate::Result<GetGameStatsResponse> {
        let path = format!(
            "/live_data/milestone/{}/game_stats",
            crate::rest::enc(milestone_id)
        );
        self.client
            .request_json::<GetGameStatsResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_structured_targets(
        &self,
        params: &crate::params::GetStructuredTargetsParams,
    ) -> crate::Result<GetStructuredTargetsResponse> {
        let path = "/structured_targets";
        self.client
            .request_json::<GetStructuredTargetsResponse, crate::params::GetStructuredTargetsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_structured_target(
        &self,
        structured_target_id: impl AsRef<str>,
    ) -> crate::Result<GetStructuredTargetResponse> {
        let path = format!(
            "/structured_targets/{}",
            crate::rest::enc(structured_target_id)
        );
        self.client
            .request_json::<GetStructuredTargetResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_milestone(
        &self,
        milestone_id: impl AsRef<str>,
    ) -> crate::Result<GetMilestoneResponse> {
        let path = format!("/milestones/{}", crate::rest::enc(milestone_id));
        self.client
            .request_json::<GetMilestoneResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_milestones(
        &self,
        params: &crate::params::GetMilestonesParams,
    ) -> crate::Result<GetMilestonesResponse> {
        let path = "/milestones";
        self.client
            .request_json::<GetMilestonesResponse, crate::params::GetMilestonesParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_multivariate_event_collection(
        &self,
        collection_ticker: impl AsRef<str>,
    ) -> crate::Result<GetMultivariateEventCollectionResponse> {
        let path = format!(
            "/multivariate_event_collections/{}",
            crate::rest::enc(collection_ticker)
        );
        self.client
            .request_json::<GetMultivariateEventCollectionResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn create_market_in_multivariate_event_collection(
        &self,
        collection_ticker: impl AsRef<str>,
        body: &CreateMarketInMultivariateEventCollectionRequest,
    ) -> crate::Result<CreateMarketInMultivariateEventCollectionResponse> {
        let path = format!(
            "/multivariate_event_collections/{}",
            crate::rest::enc(collection_ticker)
        );
        self.client
            .request_json::<CreateMarketInMultivariateEventCollectionResponse, (), CreateMarketInMultivariateEventCollectionRequest>(
                reqwest::Method::POST,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn get_multivariate_event_collections(
        &self,
        params: &crate::params::GetMultivariateEventCollectionsParams,
    ) -> crate::Result<GetMultivariateEventCollectionsResponse> {
        let path = "/multivariate_event_collections";
        self.client
            .request_json::<GetMultivariateEventCollectionsResponse, crate::params::GetMultivariateEventCollectionsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_multivariate_event_collection_lookup_history(
        &self,
        collection_ticker: impl AsRef<str>,
        params: &crate::params::GetMultivariateEventCollectionLookupHistoryParams,
    ) -> crate::Result<GetMultivariateEventCollectionLookupHistoryResponse> {
        let path = format!(
            "/multivariate_event_collections/{}/lookup",
            crate::rest::enc(collection_ticker)
        );
        self.client
            .request_json::<GetMultivariateEventCollectionLookupHistoryResponse, crate::params::GetMultivariateEventCollectionLookupHistoryParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn lookup_tickers_for_market_in_multivariate_event_collection(
        &self,
        collection_ticker: impl AsRef<str>,
        body: &LookupTickersForMarketInMultivariateEventCollectionRequest,
    ) -> crate::Result<LookupTickersForMarketInMultivariateEventCollectionResponse> {
        let path = format!(
            "/multivariate_event_collections/{}/lookup",
            crate::rest::enc(collection_ticker)
        );
        self.client
            .request_json::<LookupTickersForMarketInMultivariateEventCollectionResponse, (), LookupTickersForMarketInMultivariateEventCollectionRequest>(
                reqwest::Method::PUT,
                path.as_ref(),
                None::<&()>,
                Some(body),
            )
            .await
    }

    pub async fn get_incentive_programs(
        &self,
        params: &crate::params::GetIncentiveProgramsParams,
    ) -> crate::Result<GetIncentiveProgramsResponse> {
        let path = "/incentive_programs";
        self.client
            .request_json::<GetIncentiveProgramsResponse, crate::params::GetIncentiveProgramsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_fcm_orders(
        &self,
        params: &crate::params::GetFCMOrdersParams,
    ) -> crate::Result<GetOrdersResponse> {
        let path = "/fcm/orders";
        self.client
            .request_json::<GetOrdersResponse, crate::params::GetFCMOrdersParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_fcm_positions(
        &self,
        params: &crate::params::GetFCMPositionsParams,
    ) -> crate::Result<GetPositionsResponse> {
        let path = "/fcm/positions";
        self.client
            .request_json::<GetPositionsResponse, crate::params::GetFCMPositionsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_historical_cutoff(&self) -> crate::Result<GetHistoricalCutoffResponse> {
        let path = "/historical/cutoff";
        self.client
            .request_json::<GetHistoricalCutoffResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }

    pub async fn get_market_candlesticks_historical(
        &self,
        ticker: impl AsRef<str>,
        params: &crate::params::GetMarketCandlesticksHistoricalParams,
    ) -> crate::Result<GetMarketCandlesticksHistoricalResponse> {
        let path = format!(
            "/historical/markets/{}/candlesticks",
            crate::rest::enc(ticker)
        );
        self.client
            .request_json::<GetMarketCandlesticksHistoricalResponse, crate::params::GetMarketCandlesticksHistoricalParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_fills_historical(
        &self,
        params: &crate::params::GetFillsHistoricalParams,
    ) -> crate::Result<GetFillsResponse> {
        let path = "/historical/fills";
        self.client
            .request_json::<GetFillsResponse, crate::params::GetFillsHistoricalParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_historical_orders(
        &self,
        params: &crate::params::GetHistoricalOrdersParams,
    ) -> crate::Result<GetOrdersResponse> {
        let path = "/historical/orders";
        self.client
            .request_json::<GetOrdersResponse, crate::params::GetHistoricalOrdersParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_trades_historical(
        &self,
        params: &crate::params::GetTradesHistoricalParams,
    ) -> crate::Result<GetTradesResponse> {
        let path = "/historical/trades";
        self.client
            .request_json::<GetTradesResponse, crate::params::GetTradesHistoricalParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_historical_markets(
        &self,
        params: &crate::params::GetHistoricalMarketsParams,
    ) -> crate::Result<GetMarketsResponse> {
        let path = "/historical/markets";
        self.client
            .request_json::<GetMarketsResponse, crate::params::GetHistoricalMarketsParams, ()>(
                reqwest::Method::GET,
                path.as_ref(),
                Some(params),
                None::<&()>,
            )
            .await
    }

    pub async fn get_historical_market(
        &self,
        ticker: impl AsRef<str>,
    ) -> crate::Result<GetMarketResponse> {
        let path = format!("/historical/markets/{}", crate::rest::enc(ticker));
        self.client
            .request_json::<GetMarketResponse, (), ()>(
                reqwest::Method::GET,
                path.as_ref(),
                None::<&()>,
                None::<&()>,
            )
            .await
    }
}
