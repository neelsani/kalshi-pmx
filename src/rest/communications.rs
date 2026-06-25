use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{
    AcceptBlockTradeProposalRequest, AcceptQuoteRequest, CreateQuoteRequest, CreateQuoteResponse,
    CreateRFQRequest, CreateRFQResponse, EmptyResponse, GetBlockTradeProposalsResponse,
    GetCommunicationsIDResponse, GetQuoteResponse, GetQuotesResponse, GetRFQResponse,
    GetRFQsResponse, ProposeBlockTradeRequest, ProposeBlockTradeResponse,
};
use crate::params::{GetBlockTradeProposalsParams, GetQuotesParams, GetRFQsParams};

/// Client for RFQs, quotes, block trades, and communications endpoints.
#[derive(Clone, Debug)]
pub struct CommunicationsClient {
    client: Kalshi,
}

impl CommunicationsClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn id(&self) -> Result<GetCommunicationsIDResponse> {
        self.client.typed().get_communications_id().await
    }

    pub async fn block_trade_proposals(
        &self,
        params: &GetBlockTradeProposalsParams,
    ) -> Result<GetBlockTradeProposalsResponse> {
        self.client.typed().get_block_trade_proposals(params).await
    }

    pub async fn propose_block_trade(
        &self,
        body: &ProposeBlockTradeRequest,
    ) -> Result<ProposeBlockTradeResponse> {
        self.client.typed().propose_block_trade(body).await
    }

    pub async fn accept_block_trade_proposal(
        &self,
        block_trade_proposal_id: impl AsRef<str>,
    ) -> Result<EmptyResponse> {
        let body = AcceptBlockTradeProposalRequest {
            subtrader_id: None,
            subaccount: None,
            extra: Default::default(),
        };
        self.accept_block_trade_proposal_with_options(block_trade_proposal_id, &body)
            .await
    }

    pub async fn accept_block_trade_proposal_with_options(
        &self,
        block_trade_proposal_id: impl AsRef<str>,
        body: &AcceptBlockTradeProposalRequest,
    ) -> Result<EmptyResponse> {
        self.client
            .typed()
            .accept_block_trade_proposal(block_trade_proposal_id, body)
            .await
    }

    pub async fn rfqs(&self, params: &GetRFQsParams) -> Result<GetRFQsResponse> {
        self.client.typed().get_rf_qs(params).await
    }

    pub async fn create_rfq(&self, body: &CreateRFQRequest) -> Result<CreateRFQResponse> {
        self.client.typed().create_rfq(body).await
    }

    pub async fn rfq(&self, rfq_id: impl AsRef<str>) -> Result<GetRFQResponse> {
        self.client.typed().get_rfq(rfq_id).await
    }

    pub async fn delete_rfq(&self, rfq_id: impl AsRef<str>) -> Result<EmptyResponse> {
        self.client.typed().delete_rfq(rfq_id).await
    }

    pub async fn quotes(&self, params: &GetQuotesParams) -> Result<GetQuotesResponse> {
        self.client.typed().get_quotes(params).await
    }

    pub async fn create_quote(&self, body: &CreateQuoteRequest) -> Result<CreateQuoteResponse> {
        self.client.typed().create_quote(body).await
    }

    pub async fn quote(&self, quote_id: impl AsRef<str>) -> Result<GetQuoteResponse> {
        self.client.typed().get_quote(quote_id).await
    }

    pub async fn delete_quote(&self, quote_id: impl AsRef<str>) -> Result<EmptyResponse> {
        self.client.typed().delete_quote(quote_id).await
    }

    pub async fn accept_quote(
        &self,
        quote_id: impl AsRef<str>,
        body: &AcceptQuoteRequest,
    ) -> Result<EmptyResponse> {
        self.client.typed().accept_quote(quote_id, body).await
    }

    pub async fn confirm_quote(&self, quote_id: impl AsRef<str>) -> Result<EmptyResponse> {
        let body = EmptyResponse::new();
        self.client.typed().confirm_quote(quote_id, &body).await
    }
}
