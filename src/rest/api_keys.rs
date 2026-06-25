use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{
    CreateApiKeyRequest, CreateApiKeyResponse, EmptyResponse, GenerateApiKeyRequest,
    GenerateApiKeyResponse, GetApiKeysResponse,
};

/// Client for API-key management endpoints.
#[derive(Clone, Debug)]
pub struct ApiKeysClient {
    client: Kalshi,
}

impl ApiKeysClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn list(&self) -> Result<GetApiKeysResponse> {
        self.client.typed().get_api_keys().await
    }

    pub async fn create(&self, body: &CreateApiKeyRequest) -> Result<CreateApiKeyResponse> {
        self.client.typed().create_api_key(body).await
    }

    pub async fn generate(&self, body: &GenerateApiKeyRequest) -> Result<GenerateApiKeyResponse> {
        self.client.typed().generate_api_key(body).await
    }

    pub async fn delete(&self, api_key: impl AsRef<str>) -> Result<EmptyResponse> {
        self.client.typed().delete_api_key(api_key).await
    }
}
