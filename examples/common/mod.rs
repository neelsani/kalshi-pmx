#![allow(dead_code)]

use std::env;

use kalshi_pmx::Kalshi;
use kalshi_pmx::rate_limit::RateLimitConfig;

pub type ExampleResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

pub fn authenticated_client() -> ExampleResult<Kalshi> {
    Ok(Kalshi::builder()
        .from_env()?
        .with_rate_limit(RateLimitConfig::conservative())
        .build()?)
}

pub fn optional_env(names: &[&str]) -> Option<String> {
    let _ = dotenvy::dotenv();
    names.iter().find_map(|name| {
        env::var(name)
            .ok()
            .map(|value| value.trim().to_owned())
            .filter(|value| !value.is_empty())
    })
}

pub fn required_env(names: &[&str]) -> ExampleResult<String> {
    optional_env(names).ok_or_else(|| format!("missing one of: {}", names.join(", ")).into())
}
