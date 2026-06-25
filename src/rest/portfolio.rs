use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{
    ApplySubaccountTransferRequest, ApplySubaccountTransferResponse, CreateSubaccountResponse,
    EmptyResponse, GetBalanceResponse, GetDepositsResponse, GetFillsResponse,
    GetPortfolioRestingOrderTotalValueResponse, GetPositionsResponse, GetSettlementsResponse,
    GetSubaccountBalancesResponse, GetSubaccountNettingResponse, GetSubaccountTransfersResponse,
    GetWithdrawalsResponse, UpdateSubaccountNettingRequest,
};
use crate::params::{
    GetBalanceParams, GetDepositsParams, GetFillsParams, GetPositionsParams, GetSettlementsParams,
    GetSubaccountTransfersParams, GetWithdrawalsParams,
};

/// Client for balances, positions, fills, settlements, transfers, deposits, and withdrawals.
#[derive(Clone, Debug)]
pub struct PortfolioClient {
    client: Kalshi,
}

impl PortfolioClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn balance(&self) -> Result<GetBalanceResponse> {
        self.balance_with_params(&GetBalanceParams::new()).await
    }

    pub async fn balance_with_params(
        &self,
        params: &GetBalanceParams,
    ) -> Result<GetBalanceResponse> {
        self.client.typed().get_balance(params).await
    }

    pub async fn create_subaccount(&self) -> Result<CreateSubaccountResponse> {
        self.client.typed().create_subaccount().await
    }

    pub async fn transfer_subaccount(
        &self,
        body: &ApplySubaccountTransferRequest,
    ) -> Result<ApplySubaccountTransferResponse> {
        self.client.typed().apply_subaccount_transfer(body).await
    }

    pub async fn subaccount_balances(&self) -> Result<GetSubaccountBalancesResponse> {
        self.client.typed().get_subaccount_balances().await
    }

    pub async fn subaccount_transfers(
        &self,
        params: &GetSubaccountTransfersParams,
    ) -> Result<GetSubaccountTransfersResponse> {
        self.client.typed().get_subaccount_transfers(params).await
    }

    pub async fn update_subaccount_netting(
        &self,
        body: &UpdateSubaccountNettingRequest,
    ) -> Result<EmptyResponse> {
        self.client.typed().update_subaccount_netting(body).await
    }

    pub async fn subaccount_netting(&self) -> Result<GetSubaccountNettingResponse> {
        self.client.typed().get_subaccount_netting().await
    }

    pub async fn positions(&self, params: &GetPositionsParams) -> Result<GetPositionsResponse> {
        self.client.typed().get_positions(params).await
    }

    pub async fn settlements(
        &self,
        params: &GetSettlementsParams,
    ) -> Result<GetSettlementsResponse> {
        self.client.typed().get_settlements(params).await
    }

    pub async fn deposits(&self, params: &GetDepositsParams) -> Result<GetDepositsResponse> {
        self.client.typed().get_deposits(params).await
    }

    pub async fn withdrawals(
        &self,
        params: &GetWithdrawalsParams,
    ) -> Result<GetWithdrawalsResponse> {
        self.client.typed().get_withdrawals(params).await
    }

    pub async fn total_resting_order_value(
        &self,
    ) -> Result<GetPortfolioRestingOrderTotalValueResponse> {
        self.client
            .typed()
            .get_portfolio_resting_order_total_value()
            .await
    }

    pub async fn fills(&self, params: &GetFillsParams) -> Result<GetFillsResponse> {
        self.client.typed().get_fills(params).await
    }
}
