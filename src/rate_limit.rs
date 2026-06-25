//! Optional client-side REST rate limiting.

use std::sync::Arc;
use std::time::{Duration, Instant};

use reqwest::Method;
use tokio::sync::Mutex;
use tokio::time::sleep;

use crate::error::{Error, Result};

/// Token-bucket limits applied before REST requests.
///
/// Reads are `GET` requests. Writes are every other method. The limiter is intentionally
/// conservative and local to the client process; Kalshi's server-side limits remain authoritative.
#[derive(Debug, Clone)]
pub struct RateLimitConfig {
    /// Maximum number of read tokens that can accumulate.
    pub read_capacity: f64,
    /// Read tokens added per second.
    pub read_refill_per_second: f64,
    /// Maximum number of write tokens that can accumulate.
    pub write_capacity: f64,
    /// Write tokens added per second.
    pub write_refill_per_second: f64,
    /// Token cost charged to each REST request.
    pub default_cost: f64,
}

impl RateLimitConfig {
    /// Convenience value for builder APIs that accept `Option<RateLimitConfig>`.
    pub fn disabled() -> Option<Self> {
        None
    }

    /// Conservative default limits suitable for light applications.
    pub fn conservative() -> Self {
        Self {
            read_capacity: 100.0,
            read_refill_per_second: 100.0,
            write_capacity: 50.0,
            write_refill_per_second: 50.0,
            default_cost: 1.0,
        }
    }
}

/// Shared read/write token-bucket rate limiter.
#[derive(Debug, Clone)]
pub struct RateLimiter {
    read: Arc<Mutex<TokenBucket>>,
    write: Arc<Mutex<TokenBucket>>,
    default_cost: f64,
}

impl RateLimiter {
    /// Creates a validated rate limiter.
    pub fn new(config: RateLimitConfig) -> Result<Self> {
        config.validate()?;
        Ok(Self {
            read: Arc::new(Mutex::new(TokenBucket::new(
                config.read_capacity,
                config.read_refill_per_second,
            ))),
            write: Arc::new(Mutex::new(TokenBucket::new(
                config.write_capacity,
                config.write_refill_per_second,
            ))),
            default_cost: config.default_cost,
        })
    }

    /// Waits until the bucket for `method` has enough tokens.
    pub async fn acquire_for(&self, method: &Method) {
        let bucket = if method == Method::GET {
            &self.read
        } else {
            &self.write
        };
        bucket.lock().await.acquire(self.default_cost).await;
    }
}

impl RateLimitConfig {
    fn validate(&self) -> Result<()> {
        validate_positive_finite("read_capacity", self.read_capacity)?;
        validate_positive_finite("read_refill_per_second", self.read_refill_per_second)?;
        validate_positive_finite("write_capacity", self.write_capacity)?;
        validate_positive_finite("write_refill_per_second", self.write_refill_per_second)?;
        validate_positive_finite("default_cost", self.default_cost)?;

        if self.default_cost > self.read_capacity {
            return Err(Error::InvalidRateLimitConfig(
                "default_cost must be less than or equal to read_capacity".to_owned(),
            ));
        }
        if self.default_cost > self.write_capacity {
            return Err(Error::InvalidRateLimitConfig(
                "default_cost must be less than or equal to write_capacity".to_owned(),
            ));
        }

        Ok(())
    }
}

fn validate_positive_finite(name: &str, value: f64) -> Result<()> {
    if value.is_finite() && value > 0.0 {
        Ok(())
    } else {
        Err(Error::InvalidRateLimitConfig(format!(
            "{name} must be finite and positive"
        )))
    }
}

#[derive(Debug)]
struct TokenBucket {
    capacity: f64,
    tokens: f64,
    refill_per_second: f64,
    last_refill: Instant,
}

impl TokenBucket {
    fn new(capacity: f64, refill_per_second: f64) -> Self {
        Self {
            capacity,
            tokens: capacity,
            refill_per_second,
            last_refill: Instant::now(),
        }
    }

    async fn acquire(&mut self, cost: f64) {
        loop {
            self.refill();

            if self.tokens >= cost {
                self.tokens -= cost;
                return;
            }

            let deficit = cost - self.tokens;
            let wait_seconds = deficit / self.refill_per_second.max(0.000_001);
            sleep(Duration::from_secs_f64(wait_seconds)).await;
        }
    }

    fn refill(&mut self) {
        let now = Instant::now();
        let elapsed = now.duration_since(self.last_refill).as_secs_f64();
        self.last_refill = now;
        self.tokens = (self.tokens + elapsed * self.refill_per_second).min(self.capacity);
    }
}
