//! Core HTTP client and builder for the SDK.

use std::env;
use std::fs;
use std::path::Path;
use std::sync::Arc;
use std::time::Duration;

use reqwest::Method;
use serde::Serialize;
use serde::de::DeserializeOwned;
use serde_json::Value;
use tokio::time::sleep;
use url::Url;

use crate::auth::{
    ACCESS_KEY_HEADER, ACCESS_SIGNATURE_HEADER, ACCESS_TIMESTAMP_HEADER, ApiKeySigner,
};
use crate::config::{Environment, KalshiConfig};
use crate::error::{Error, Result};
use crate::rate_limit::{RateLimitConfig, RateLimiter};
use crate::rest;
use crate::ws::StreamBuilder;

const ENVIRONMENT_VARS: &[&str] = &["KALSHI_ENV", "KALSHI_API_ENV"];
const API_KEY_ID_VARS: &[&str] = &[
    "KALSHI_API_KEY_ID",
    "KALSHI_KEY_ID",
    "KALSHI_ACCESS_KEY",
    "KALSHI_ACCESS_KEY_ID",
];
const PRIVATE_KEY_VARS: &[&str] = &[
    "KALSHI_PRIV_KEY",
    "KALSHI_PRIVATE_KEY",
    "KALSHI_PRIVATE_KEY_PEM",
    "KALSHI_PRIVATE_KEY_FILE",
];
const WS_URL_VARS: &[&str] = &["KALSHI_WS_URL"];

/// Async Kalshi client.
///
/// Use [`Kalshi::demo`] or [`Kalshi::production`] to build a client, then access typed REST
/// namespaces such as [`Kalshi::markets`], [`Kalshi::orders`], and [`Kalshi::portfolio`], or
/// connect to WebSocket streams with [`Kalshi::stream`].
#[derive(Clone, Debug)]
pub struct Kalshi {
    pub(crate) inner: Arc<Inner>,
}

#[derive(Debug)]
pub(crate) struct Inner {
    pub config: KalshiConfig,
    pub http: reqwest::Client,
    pub auth: Option<ApiKeySigner>,
    pub rate_limiter: Option<RateLimiter>,
    pub rest_sign_prefix: String,
}

/// Builder for [`Kalshi`].
///
/// Credentials are optional for public REST reads, but authenticated REST endpoints and all
/// WebSocket connections require an API key id and RSA private key.
#[derive(Debug, Clone)]
pub struct KalshiBuilder {
    config: KalshiConfig,
    key_id: Option<String>,
    private_key_pem: Option<String>,
    rate_limit: Option<RateLimitConfig>,
}

impl Kalshi {
    /// Starts a client builder with the default demo environment.
    pub fn builder() -> KalshiBuilder {
        KalshiBuilder::new()
    }

    /// Starts a client builder for Kalshi's demo environment.
    pub fn demo() -> KalshiBuilder {
        KalshiBuilder::new().environment(Environment::Demo)
    }

    /// Starts a client builder for Kalshi's production environment.
    pub fn production() -> KalshiBuilder {
        KalshiBuilder::new().environment(Environment::Production)
    }

    /// Exchange metadata, status, schedule, announcements, and API usage endpoints.
    pub fn exchange(&self) -> rest::exchange::ExchangeClient {
        rest::exchange::ExchangeClient::new(self.clone())
    }

    /// Market, series, orderbook, trades, and candlestick endpoints.
    pub fn markets(&self) -> rest::markets::MarketsClient {
        rest::markets::MarketsClient::new(self.clone())
    }

    /// Event, multivariate event, metadata, forecast history, and fee-change endpoints.
    pub fn events(&self) -> rest::events::EventsClient {
        rest::events::EventsClient::new(self.clone())
    }

    /// Order listing, creation, cancellation, amendment, decrease, and queue-position endpoints.
    pub fn orders(&self) -> rest::orders::OrdersClient {
        rest::orders::OrdersClient::new(self.clone())
    }

    /// Order group lifecycle and limit endpoints.
    pub fn order_groups(&self) -> rest::order_groups::OrderGroupsClient {
        rest::order_groups::OrderGroupsClient::new(self.clone())
    }

    /// Portfolio balances, positions, fills, settlements, deposits, and withdrawals.
    pub fn portfolio(&self) -> rest::portfolio::PortfolioClient {
        rest::portfolio::PortfolioClient::new(self.clone())
    }

    /// RFQ, quote, block-trade proposal, and communications endpoints.
    pub fn communications(&self) -> rest::communications::CommunicationsClient {
        rest::communications::CommunicationsClient::new(self.clone())
    }

    /// API-key management endpoints.
    pub fn api_keys(&self) -> rest::api_keys::ApiKeysClient {
        rest::api_keys::ApiKeysClient::new(self.clone())
    }

    /// Account API limits, endpoint cost, and usage-level endpoints.
    pub fn account(&self) -> rest::account::AccountClient {
        rest::account::AccountClient::new(self.clone())
    }

    /// Search and discovery helper endpoints.
    pub fn search(&self) -> rest::search::SearchClient {
        rest::search::SearchClient::new(self.clone())
    }

    /// Live data endpoints keyed by type and milestone.
    pub fn live_data(&self) -> rest::live_data::LiveDataClient {
        rest::live_data::LiveDataClient::new(self.clone())
    }

    /// Structured target lookup endpoints.
    pub fn structured_targets(&self) -> rest::structured_targets::StructuredTargetsClient {
        rest::structured_targets::StructuredTargetsClient::new(self.clone())
    }

    /// Milestone listing and lookup endpoints.
    pub fn milestones(&self) -> rest::milestones::MilestonesClient {
        rest::milestones::MilestonesClient::new(self.clone())
    }

    /// Multivariate event collection endpoints.
    pub fn multivariate(&self) -> rest::multivariate::MultivariateClient {
        rest::multivariate::MultivariateClient::new(self.clone())
    }

    /// Incentive program endpoints.
    pub fn incentive_programs(&self) -> rest::incentive_programs::IncentiveProgramsClient {
        rest::incentive_programs::IncentiveProgramsClient::new(self.clone())
    }

    /// FCM order and position endpoints.
    pub fn fcm(&self) -> rest::fcm::FcmClient {
        rest::fcm::FcmClient::new(self.clone())
    }

    /// Historical market, trade, order, fill, and candlestick endpoints.
    pub fn historical(&self) -> rest::historical::HistoricalClient {
        rest::historical::HistoricalClient::new(self.clone())
    }

    /// Builds an authenticated WebSocket stream connector.
    pub fn stream(&self) -> StreamBuilder {
        StreamBuilder::new(self.clone())
    }

    /// Direct access to generated OpenAPI operation methods.
    ///
    /// Prefer the high-level namespace clients for normal use. This layer is useful when you
    /// want exact operation names from the checked-in OpenAPI spec.
    pub fn typed(&self) -> crate::typed::TypedClient {
        crate::typed::TypedClient::new(self.clone())
    }

    #[doc(hidden)]
    pub async fn raw_get<Q>(&self, path: &str, query: &Q) -> Result<Value>
    where
        Q: Serialize + ?Sized,
    {
        self.get_value(path, Some(query)).await
    }

    #[doc(hidden)]
    pub async fn raw_get_no_query(&self, path: &str) -> Result<Value> {
        self.get_value(path, None::<&()>).await
    }

    #[doc(hidden)]
    pub async fn raw_post<B>(&self, path: &str, body: &B) -> Result<Value>
    where
        B: Serialize + ?Sized,
    {
        self.post_value(path, body).await
    }

    #[doc(hidden)]
    pub async fn raw_put<B>(&self, path: &str, body: &B) -> Result<Value>
    where
        B: Serialize + ?Sized,
    {
        self.put_value(path, body).await
    }

    #[doc(hidden)]
    pub async fn raw_delete<Q>(&self, path: &str, query: &Q) -> Result<Value>
    where
        Q: Serialize + ?Sized,
    {
        self.delete_value(path, Some(query)).await
    }

    #[doc(hidden)]
    pub async fn raw_delete_no_query(&self, path: &str) -> Result<Value> {
        self.delete_value(path, None::<&()>).await
    }

    #[doc(hidden)]
    pub async fn raw_delete_with_body<B>(&self, path: &str, body: &B) -> Result<Value>
    where
        B: Serialize + ?Sized,
    {
        self.delete_value_with_body(path, body).await
    }

    pub(crate) fn ws_url(&self) -> String {
        self.inner.config.ws_url()
    }

    pub(crate) fn ws_auth_headers(&self) -> Result<crate::auth::AuthHeaders> {
        let auth = self.inner.auth.as_ref().ok_or(Error::MissingCredentials)?;
        auth.sign_headers("GET", &self.inner.config.ws_sign_path)
    }

    pub(crate) async fn get_value<Q>(&self, path: &str, query: Option<&Q>) -> Result<Value>
    where
        Q: Serialize + ?Sized,
    {
        self.request_value::<Q, EmptyBody>(Method::GET, path, query, None)
            .await
    }

    pub(crate) async fn delete_value<Q>(&self, path: &str, query: Option<&Q>) -> Result<Value>
    where
        Q: Serialize + ?Sized,
    {
        self.request_value::<Q, EmptyBody>(Method::DELETE, path, query, None)
            .await
    }

    pub(crate) async fn post_value<B>(&self, path: &str, body: &B) -> Result<Value>
    where
        B: Serialize + ?Sized,
    {
        self.request_value::<EmptyQuery, B>(Method::POST, path, None, Some(body))
            .await
    }

    pub(crate) async fn put_value<B>(&self, path: &str, body: &B) -> Result<Value>
    where
        B: Serialize + ?Sized,
    {
        self.request_value::<EmptyQuery, B>(Method::PUT, path, None, Some(body))
            .await
    }

    pub(crate) async fn delete_value_with_body<B>(&self, path: &str, body: &B) -> Result<Value>
    where
        B: Serialize + ?Sized,
    {
        self.request_value::<EmptyQuery, B>(Method::DELETE, path, None, Some(body))
            .await
    }

    #[doc(hidden)]
    pub async fn request_json<T, Q, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<T>
    where
        T: DeserializeOwned,
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        let value = self.request_value(method, path, query, body).await?;
        Ok(serde_json::from_value(value)?)
    }

    async fn request_value<Q, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<Value>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        if let Some(rate_limiter) = &self.inner.rate_limiter {
            rate_limiter.acquire_for(&method).await;
        }

        let mut attempt = 0usize;
        loop {
            let result = self.send_once(method.clone(), path, query, body).await;

            match result {
                Err(Error::Api { status, body })
                    if status == reqwest::StatusCode::TOO_MANY_REQUESTS
                        && attempt < self.inner.config.max_retries =>
                {
                    attempt += 1;
                    let delay = Duration::from_millis(250 * 2_u64.pow(attempt as u32 - 1));
                    tracing::debug!(attempt, ?delay, "Kalshi rate limited request; backing off");
                    sleep(delay).await;
                    let _ = body;
                }
                other => return other,
            }
        }
    }

    async fn send_once<Q, B>(
        &self,
        method: Method,
        path: &str,
        query: Option<&Q>,
        body: Option<&B>,
    ) -> Result<Value>
    where
        Q: Serialize + ?Sized,
        B: Serialize + ?Sized,
    {
        let mut url = Url::parse(&format!("{}{}", self.inner.config.rest_base_url(), path))?;

        if let Some(query) = query {
            Self::append_query(&mut url, query)?;
        }

        let mut request = self.inner.http.request(method.clone(), url);

        if let Some(body) = body {
            request = request.json(body);
        }

        if let Some(auth) = &self.inner.auth {
            let sign_path = self.sign_path(path);
            let headers = auth.sign_headers(method.as_str(), &sign_path)?;
            request = request
                .header(ACCESS_KEY_HEADER, headers.key_id)
                .header(ACCESS_SIGNATURE_HEADER, headers.signature)
                .header(ACCESS_TIMESTAMP_HEADER, headers.timestamp);
        }

        let response = request.send().await?;
        let status = response.status();
        let bytes = response.bytes().await?;

        if !status.is_success() {
            return Err(Error::Api {
                status,
                body: String::from_utf8_lossy(&bytes).into_owned(),
            });
        }

        // Several mutation endpoints return 2xx with an empty body. Treat those as `{}` so the
        // generated `EmptyResponse` type can deserialize through the same path as JSON responses.
        if bytes.is_empty() {
            Ok(Value::Object(Default::default()))
        } else {
            Ok(serde_json::from_slice(&bytes)?)
        }
    }

    fn sign_path(&self, endpoint_path: &str) -> String {
        format!("{}{}", self.inner.rest_sign_prefix, endpoint_path)
    }

    fn append_query<Q>(url: &mut Url, query: &Q) -> Result<()>
    where
        Q: Serialize + ?Sized,
    {
        let value = serde_json::to_value(query)?;
        let mut flattened = Vec::new();

        match value {
            Value::Object(fields) => {
                for (key, value) in fields {
                    Self::collect_query_pairs(&key, value, &mut flattened);
                }
            }
            Value::Null => {}
            value => Self::collect_query_pairs("value", value, &mut flattened),
        }

        let mut pairs = url.query_pairs_mut();
        for (key, value) in flattened {
            pairs.append_pair(&key, &value);
        }

        Ok(())
    }

    fn collect_query_pairs(key: &str, value: Value, output: &mut Vec<(String, String)>) {
        match value {
            Value::Null => {}
            Value::Array(values) => {
                for value in values {
                    Self::collect_query_pairs(key, value, output);
                }
            }
            Value::Bool(value) => output.push((key.to_owned(), value.to_string())),
            Value::Number(value) => output.push((key.to_owned(), value.to_string())),
            Value::String(value) => output.push((key.to_owned(), value)),
            value @ Value::Object(_) => output.push((key.to_owned(), value.to_string())),
        }
    }
}

impl KalshiBuilder {
    /// Creates a builder with demo defaults and no credentials.
    pub fn new() -> Self {
        Self {
            config: KalshiConfig::default(),
            key_id: None,
            private_key_pem: None,
            rate_limit: None,
        }
    }

    /// Replaces the entire runtime configuration.
    pub fn config(mut self, config: KalshiConfig) -> Self {
        self.config = config;
        self
    }

    /// Selects the Kalshi environment to use.
    pub fn environment(mut self, environment: Environment) -> Self {
        self.config.environment = environment;
        self
    }

    /// Sets the HTTP user-agent header.
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.config.user_agent = user_agent.into();
        self
    }

    /// Sets the HTTP request timeout and WebSocket connect timeout.
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    /// Sets the number of retries for REST requests that receive `429 Too Many Requests`.
    pub fn max_retries(mut self, max_retries: usize) -> Self {
        self.config.max_retries = max_retries;
        self
    }

    /// Loads `.env` and applies supported Kalshi environment variables.
    ///
    /// Supported variables:
    ///
    /// - `KALSHI_ENV` or `KALSHI_API_ENV`: `demo`, `sandbox`, `production`, `prod`, or a custom
    ///   REST base URL. Custom REST URLs require `KALSHI_WS_URL`.
    /// - `KALSHI_API_KEY_ID`, `KALSHI_KEY_ID`, `KALSHI_ACCESS_KEY`, or
    ///   `KALSHI_ACCESS_KEY_ID`: API key id.
    /// - `KALSHI_PRIV_KEY`, `KALSHI_PRIVATE_KEY`, `KALSHI_PRIVATE_KEY_PEM`, or
    ///   `KALSHI_PRIVATE_KEY_FILE`: RSA private key PEM text or a path to a PEM file.
    ///
    /// Credentials are optional for public REST usage. If either the key id or private key is set,
    /// both must be set.
    pub fn from_env(self) -> Result<Self> {
        match dotenvy::dotenv() {
            Ok(_) => {}
            Err(err) if err.not_found() => {}
            Err(err) => {
                return Err(Error::Env(format!("failed to load .env: {err}")));
            }
        }

        self.apply_env_values(|name| env::var(name).ok())
    }

    /// Adds API credentials from an RSA private key in PEM text form.
    pub fn with_key_pem(
        mut self,
        key_id: impl Into<String>,
        private_key_pem: impl Into<String>,
    ) -> Self {
        self.key_id = Some(key_id.into());
        self.private_key_pem = Some(private_key_pem.into());
        self
    }

    /// Adds API credentials by reading an RSA private key from a PEM file.
    pub fn with_key_file(
        self,
        key_id: impl Into<String>,
        private_key_path: impl AsRef<Path>,
    ) -> Result<Self> {
        let pem = std::fs::read_to_string(private_key_path)?;
        Ok(self.with_key_pem(key_id, pem))
    }

    /// Enables the optional client-side REST rate limiter.
    pub fn with_rate_limit(mut self, config: RateLimitConfig) -> Self {
        self.rate_limit = Some(config);
        self
    }

    /// Builds the client and validates credentials, HTTP configuration, and rate-limit settings.
    pub fn build(self) -> Result<Kalshi> {
        let auth = match (self.key_id, self.private_key_pem) {
            (Some(key_id), Some(pem)) => Some(ApiKeySigner::from_pem(key_id, &pem)?),
            (None, None) => None,
            _ => {
                return Err(Error::Auth(
                    "key id and private key must be provided together".to_owned(),
                ));
            }
        };

        let http = reqwest::Client::builder()
            .timeout(self.config.timeout)
            .user_agent(self.config.user_agent.clone())
            .build()?;

        let rest_base = Url::parse(&self.config.rest_base_url())?;
        let rest_sign_prefix = rest_base.path().trim_end_matches('/').to_owned();

        Ok(Kalshi {
            inner: Arc::new(Inner {
                config: self.config,
                http,
                auth,
                rate_limiter: self.rate_limit.map(RateLimiter::new).transpose()?,
                rest_sign_prefix,
            }),
        })
    }

    fn apply_env_values(mut self, get: impl Fn(&str) -> Option<String>) -> Result<Self> {
        if let Some(environment) = first_env(&get, ENVIRONMENT_VARS) {
            let lower = environment.to_ascii_lowercase();
            self.config.environment = match lower.as_str() {
                "demo" | "sandbox" => Environment::Demo,
                "prod" | "production" => Environment::Production,
                _ if lower.starts_with("http://") || lower.starts_with("https://") => {
                    Environment::Custom {
                        rest_base_url: environment,
                        ws_url: first_env(&get, WS_URL_VARS).ok_or_else(|| {
                            Error::Env(
                                "custom KALSHI_ENV REST URLs require KALSHI_WS_URL".to_owned(),
                            )
                        })?,
                    }
                }
                other => {
                    return Err(Error::Env(format!(
                        "unsupported KALSHI_ENV `{other}`; use `demo`, `production`, or a custom REST URL with KALSHI_WS_URL"
                    )));
                }
            };
        }

        let key_id = first_env(&get, API_KEY_ID_VARS);
        let key_material = first_env(&get, PRIVATE_KEY_VARS);
        match (key_id, key_material) {
            (Some(key_id), Some(key_material)) => {
                self = self.with_key_pem(key_id, env_private_key_pem(&key_material)?);
            }
            (None, None) => {}
            _ => {
                return Err(Error::Env(
                    "Kalshi API key id and private key env vars must be provided together"
                        .to_owned(),
                ));
            }
        }

        Ok(self)
    }
}

impl Default for KalshiBuilder {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize)]
pub(crate) struct EmptyQuery {}

#[derive(Serialize)]
pub(crate) struct EmptyBody {}

fn first_env(get: &impl Fn(&str) -> Option<String>, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        get(name)
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
    })
}

fn env_private_key_pem(value: &str) -> Result<String> {
    let value = value.trim();
    if value.contains("BEGIN") {
        return Ok(value.replace("\\n", "\n"));
    }

    let path = Path::new(value);
    if path.exists() {
        return Ok(fs::read_to_string(path)?);
    }

    Err(Error::Env(
        "Kalshi private key env var must be PEM text or a path to a PEM file".to_owned(),
    ))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::time::{SystemTime, UNIX_EPOCH};

    use super::*;

    fn env(values: &[(&str, &str)]) -> impl Fn(&str) -> Option<String> {
        let values = values
            .iter()
            .map(|(key, value)| ((*key).to_owned(), (*value).to_owned()))
            .collect::<BTreeMap<_, _>>();
        move |name| values.get(name).cloned()
    }

    #[test]
    fn from_env_values_preserves_builder_when_env_is_empty() {
        let builder = KalshiBuilder::new()
            .environment(Environment::Production)
            .timeout(Duration::from_secs(7))
            .apply_env_values(env(&[]))
            .unwrap();

        assert!(matches!(
            builder.config.environment,
            Environment::Production
        ));
        assert_eq!(builder.config.timeout, Duration::from_secs(7));
        assert!(builder.key_id.is_none());
        assert!(builder.private_key_pem.is_none());
    }

    #[test]
    fn from_env_values_applies_environment_aliases_and_credentials() {
        let private_key =
            "-----BEGIN PRIVATE KEY-----\\nabc\\n-----END PRIVATE KEY-----".to_owned();
        let builder = KalshiBuilder::new()
            .apply_env_values(env(&[
                ("KALSHI_ENV", "sandbox"),
                ("KALSHI_API_KEY_ID", "key-id"),
                ("KALSHI_PRIV_KEY", &private_key),
            ]))
            .unwrap();

        assert!(matches!(builder.config.environment, Environment::Demo));
        assert_eq!(builder.key_id.as_deref(), Some("key-id"));
        assert_eq!(
            builder.private_key_pem.as_deref(),
            Some("-----BEGIN PRIVATE KEY-----\nabc\n-----END PRIVATE KEY-----")
        );
    }

    #[test]
    fn from_env_values_supports_custom_environment_urls() {
        let builder = KalshiBuilder::new()
            .apply_env_values(env(&[
                ("KALSHI_ENV", "https://example.test/trade-api/v2"),
                ("KALSHI_WS_URL", "wss://example.test/trade-api/ws/v2"),
            ]))
            .unwrap();

        match builder.config.environment {
            Environment::Custom {
                rest_base_url,
                ws_url,
            } => {
                assert_eq!(rest_base_url, "https://example.test/trade-api/v2");
                assert_eq!(ws_url, "wss://example.test/trade-api/ws/v2");
            }
            other => panic!("expected custom environment, got {other:?}"),
        }
    }

    #[test]
    fn from_env_values_rejects_bad_environment_and_partial_credentials() {
        let err = KalshiBuilder::new()
            .apply_env_values(env(&[("KALSHI_ENV", "staging")]))
            .unwrap_err();
        assert!(err.to_string().contains("unsupported KALSHI_ENV"));

        let err = KalshiBuilder::new()
            .apply_env_values(env(&[("KALSHI_API_KEY_ID", "key-id")]))
            .unwrap_err();
        assert!(err.to_string().contains("provided together"));

        let err = KalshiBuilder::new()
            .apply_env_values(env(&[("KALSHI_ENV", "https://example.test")]))
            .unwrap_err();
        assert!(err.to_string().contains("KALSHI_WS_URL"));
    }

    #[test]
    fn env_private_key_pem_reads_file_paths() {
        let unique = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_nanos();
        let path = std::env::temp_dir().join(format!("kalshi-pmx-test-key-{unique}.pem"));
        let pem = "-----BEGIN PRIVATE KEY-----\nabc\n-----END PRIVATE KEY-----\n";
        fs::write(&path, pem).unwrap();

        let loaded = env_private_key_pem(path.to_str().unwrap()).unwrap();
        fs::remove_file(&path).unwrap();

        assert_eq!(loaded, pem);
    }
}
