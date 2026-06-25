//! Authenticated Kalshi WebSocket streams.
//!
//! Use [`Kalshi::stream`](crate::Kalshi::stream) to connect, [`Subscription`] to build channel
//! requests, and [`LiveOrderbook`] to maintain aggregate local orderbook state from snapshot and
//! delta messages.

pub mod client;
pub mod message;
pub mod orderbook;
pub mod subscription;

pub use client::{KalshiWebSocket, ReconnectPolicy, StreamBuilder};
pub use message::{
    CommandEnvelope, Envelope, FillMessage, OrderbookDelta, OrderbookSnapshot, RawStreamMessage,
    StreamMessage, TickerMessage, TradeMessage, UserOrderMessage, WsErrorMessage,
};
pub use orderbook::LiveOrderbook;
pub use subscription::{Channel, Subscription, UpdateAction};
