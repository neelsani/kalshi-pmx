//! Error and result types used by the SDK.

use http::header::InvalidHeaderValue;
use thiserror::Error;

/// SDK result type.
pub type Result<T> = std::result::Result<T, Error>;

/// Errors returned by REST, WebSocket, authentication, parsing, and helper APIs.
#[derive(Debug, Error)]
pub enum Error {
    /// RSA signing or credential parsing failed.
    #[error("authentication error: {0}")]
    Auth(String),

    /// An authenticated endpoint was used without configured credentials.
    #[error("missing API credentials")]
    MissingCredentials,

    /// The HTTP client failed before receiving an API response.
    #[error("HTTP request failed: {0}")]
    Http(#[from] reqwest::Error),

    /// Kalshi returned a non-2xx HTTP response.
    #[error("Kalshi API returned {status}: {body}")]
    Api {
        /// HTTP status code returned by Kalshi.
        status: reqwest::StatusCode,
        /// Response body returned by Kalshi, preserved as text.
        body: String,
    },

    /// JSON serialization or deserialization failed.
    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),

    /// URL parsing failed.
    #[error("URL parse error: {0}")]
    Url(#[from] url::ParseError),

    /// An authentication header could not be represented as an HTTP header value.
    #[error("invalid header value: {0}")]
    InvalidHeaderValue(#[from] InvalidHeaderValue),

    /// HTTP request construction failed.
    #[error("HTTP request build error: {0}")]
    HttpBuild(#[from] http::Error),

    /// WebSocket transport failed.
    #[error("WebSocket error: {0}")]
    WebSocket(#[from] tokio_tungstenite::tungstenite::Error),

    /// File or network I/O failed outside reqwest/tungstenite.
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// An operation exceeded its configured timeout.
    #[error("timeout")]
    Timeout,

    /// A WebSocket stream closed and could not be reconnected.
    #[error("stream closed")]
    StreamClosed,

    /// Client-side rate-limit settings were invalid.
    #[error("invalid rate limit config: {0}")]
    InvalidRateLimitConfig(String),

    /// Environment-based configuration was invalid.
    #[error("environment configuration error: {0}")]
    Env(String),

    /// An orderbook delta could not be safely applied to local state.
    #[error("orderbook out of sync: {0}")]
    OrderbookOutOfSync(String),

    /// A fixed-point decimal string was not in the supported format.
    #[error("invalid fixed point value `{0}`")]
    InvalidFixedPoint(String),

    /// A catch-all for errors produced by extension code.
    #[error("{0}")]
    Other(String),
}
