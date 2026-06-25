#![doc = include_str!("../README.md")]

pub mod auth;
pub mod client;
pub mod config;
pub mod error;
pub mod generated;
pub mod models;
pub mod pagination;
pub mod params;
pub mod rate_limit;
pub mod rest;
pub mod typed;
pub mod ws;

pub use client::{Kalshi, KalshiBuilder};
pub use config::{Environment, KalshiConfig};
pub use error::{Error, Result};
