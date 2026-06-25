use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{GetStructuredTargetResponse, GetStructuredTargetsResponse};
use crate::params::GetStructuredTargetsParams;

/// Client for structured target listing and lookup endpoints.
#[derive(Clone, Debug)]
pub struct StructuredTargetsClient {
    client: Kalshi,
}

impl StructuredTargetsClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn list(
        &self,
        params: &GetStructuredTargetsParams,
    ) -> Result<GetStructuredTargetsResponse> {
        self.client.typed().get_structured_targets(params).await
    }

    pub async fn get(
        &self,
        structured_target_id: impl AsRef<str>,
    ) -> Result<GetStructuredTargetResponse> {
        self.client
            .typed()
            .get_structured_target(structured_target_id)
            .await
    }
}
