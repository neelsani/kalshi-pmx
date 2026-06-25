use std::time::Duration;

use kalshi_pmx::pagination;
use kalshi_pmx::rate_limit::{RateLimitConfig, RateLimiter};
use kalshi_pmx::{Environment, Kalshi, KalshiConfig};
use reqwest::Method;
use serde_json::json;

#[test]
fn pagination_helpers_read_and_insert_cursor() {
    assert_eq!(
        pagination::cursor(&json!({ "cursor": "next" })),
        Some("next")
    );
    assert_eq!(pagination::cursor(&json!({ "cursor": "" })), None);
    assert_eq!(pagination::cursor(&json!({})), None);

    assert_eq!(
        pagination::with_cursor(json!({ "limit": 10 }), "next"),
        json!({ "limit": 10, "cursor": "next" })
    );
    assert_eq!(
        pagination::with_cursor(json!(null), "next"),
        json!({ "cursor": "next" })
    );
    assert_eq!(pagination::cursor(&json!({ "cursor": 123 })), None);
}

#[test]
fn config_defaults_and_custom_urls_are_stable() {
    let demo = KalshiConfig::demo();
    assert!(demo.rest_base_url().contains("demo"));
    assert!(demo.ws_url().contains("demo"));

    let production = KalshiConfig::production();
    assert_eq!(
        production.rest_base_url(),
        "https://external-api.kalshi.com/trade-api/v2"
    );
    assert_eq!(
        production.ws_url(),
        "wss://external-api-ws.kalshi.com/trade-api/ws/v2"
    );

    let custom = KalshiConfig {
        environment: Environment::Custom {
            rest_base_url: "http://localhost:1234/base/".to_owned(),
            ws_url: "ws://localhost:1234/ws".to_owned(),
        },
        ..KalshiConfig::default()
    };
    assert_eq!(custom.rest_base_url(), "http://localhost:1234/base");
    assert_eq!(custom.ws_url(), "ws://localhost:1234/ws");
}

#[tokio::test]
async fn builder_rejects_partial_credentials() {
    let public_client = Kalshi::demo().build().expect("public client should build");
    let err = public_client
        .stream()
        .connect()
        .await
        .expect_err("websocket auth should require credentials");
    assert!(matches!(err, kalshi_pmx::Error::MissingCredentials));

    let err = Kalshi::demo()
        .with_key_pem("key", "")
        .build()
        .expect_err("empty private key should fail");
    assert!(err.to_string().contains("failed to parse RSA private key"));
}

#[tokio::test]
async fn conservative_rate_limiter_allows_read_and_write() {
    let limiter = RateLimiter::new(RateLimitConfig {
        read_capacity: 1.0,
        read_refill_per_second: 100.0,
        write_capacity: 1.0,
        write_refill_per_second: 100.0,
        default_cost: 1.0,
    })
    .unwrap();

    tokio::time::timeout(Duration::from_secs(1), limiter.acquire_for(&Method::GET))
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(1), limiter.acquire_for(&Method::POST))
        .await
        .unwrap();
}

#[test]
fn rate_limiter_rejects_invalid_configs() {
    assert!(RateLimitConfig::disabled().is_none());

    let err = RateLimiter::new(RateLimitConfig {
        read_capacity: 1.0,
        read_refill_per_second: 100.0,
        write_capacity: 1.0,
        write_refill_per_second: 100.0,
        default_cost: 2.0,
    })
    .expect_err("cost larger than capacity should be rejected");

    assert!(err.to_string().contains("default_cost"));

    let err = Kalshi::demo()
        .with_rate_limit(RateLimitConfig {
            read_capacity: f64::NAN,
            read_refill_per_second: 100.0,
            write_capacity: 1.0,
            write_refill_per_second: 100.0,
            default_cost: 1.0,
        })
        .build()
        .expect_err("builder should reject invalid rate-limit config");

    assert!(err.to_string().contains("read_capacity"));
}
