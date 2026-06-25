use crate::client::Kalshi;
use crate::error::Result;
use crate::generated::{GetFiltersBySportsResponse, GetTagsForSeriesCategoriesResponse};

/// Client for search and discovery endpoints.
#[derive(Clone, Debug)]
pub struct SearchClient {
    client: Kalshi,
}

impl SearchClient {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self { client }
    }

    pub async fn tags_by_categories(&self) -> Result<GetTagsForSeriesCategoriesResponse> {
        self.client.typed().get_tags_for_series_categories().await
    }

    pub async fn filters_by_sport(&self) -> Result<GetFiltersBySportsResponse> {
        self.client.typed().get_filters_for_sports().await
    }
}
