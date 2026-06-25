//! Runtime configuration for REST and WebSocket clients.

use std::time::Duration;

/// Kalshi API environment.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Environment {
    /// Kalshi demo environment.
    Demo,
    /// Kalshi production environment.
    Production,
    /// A custom REST and WebSocket endpoint pair.
    ///
    /// Use this for tests, proxies, or any future Kalshi-compatible endpoint that keeps the same
    /// Trade API v2 path shape.
    Custom {
        /// REST base URL, usually ending in `/trade-api/v2`.
        rest_base_url: String,
        /// WebSocket URL, usually ending in `/trade-api/ws/v2`.
        ws_url: String,
    },
}

impl Environment {
    /// Returns the REST base URL for this environment.
    pub fn rest_base_url(&self) -> &str {
        match self {
            Self::Demo => "https://external-api.demo.kalshi.co/trade-api/v2",
            Self::Production => "https://external-api.kalshi.com/trade-api/v2",
            Self::Custom { rest_base_url, .. } => rest_base_url,
        }
    }

    /// Returns the authenticated WebSocket URL for this environment.
    pub fn ws_url(&self) -> &str {
        match self {
            Self::Demo => "wss://external-api-ws.demo.kalshi.co/trade-api/ws/v2",
            Self::Production => "wss://external-api-ws.kalshi.com/trade-api/ws/v2",
            Self::Custom { ws_url, .. } => ws_url,
        }
    }
}

/// Runtime configuration for REST and WebSocket clients.
#[derive(Debug, Clone)]
pub struct KalshiConfig {
    /// API environment.
    pub environment: Environment,
    /// HTTP user-agent header.
    pub user_agent: String,
    /// HTTP request timeout and WebSocket connect timeout.
    pub timeout: Duration,
    /// Number of REST retries after `429 Too Many Requests`.
    pub max_retries: usize,
    /// Path used when signing WebSocket authentication headers.
    pub ws_sign_path: String,
}

impl Default for KalshiConfig {
    fn default() -> Self {
        Self {
            environment: Environment::Demo,
            user_agent: format!("{}/{}", env!("CARGO_PKG_NAME"), env!("CARGO_PKG_VERSION")),
            timeout: Duration::from_secs(30),
            max_retries: 2,
            ws_sign_path: "/trade-api/ws/v2".to_owned(),
        }
    }
}

impl KalshiConfig {
    /// Returns default configuration for the demo environment.
    pub fn demo() -> Self {
        Self {
            environment: Environment::Demo,
            ..Self::default()
        }
    }

    /// Returns default configuration for the production environment.
    pub fn production() -> Self {
        Self {
            environment: Environment::Production,
            ..Self::default()
        }
    }

    /// Returns the normalized REST base URL without a trailing slash.
    pub fn rest_base_url(&self) -> String {
        self.environment
            .rest_base_url()
            .trim_end_matches('/')
            .to_owned()
    }

    /// Returns the configured WebSocket URL.
    pub fn ws_url(&self) -> String {
        self.environment.ws_url().to_owned()
    }
}
