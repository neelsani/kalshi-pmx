//! WebSocket connection management and reconnect behavior.

use std::collections::BTreeMap;
use std::time::Duration;

use futures_util::{SinkExt, StreamExt};
use http::header::HeaderValue;
use serde::Serialize;
use serde_json::Value;
use tokio::net::TcpStream;
use tokio::time::{sleep, timeout};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::client::IntoClientRequest;
use tokio_tungstenite::{MaybeTlsStream, WebSocketStream, connect_async};

use crate::auth::{ACCESS_KEY_HEADER, ACCESS_SIGNATURE_HEADER, ACCESS_TIMESTAMP_HEADER};
use crate::client::Kalshi;
use crate::error::{Error, Result};
use crate::ws::message::StreamMessage;
use crate::ws::subscription::{Subscription, UpdateAction};

type Socket = WebSocketStream<MaybeTlsStream<TcpStream>>;

/// Builder for authenticated Kalshi WebSocket streams.
#[derive(Debug, Clone)]
pub struct StreamBuilder {
    client: Kalshi,
    reconnect_policy: ReconnectPolicy,
}

impl StreamBuilder {
    pub(crate) fn new(client: Kalshi) -> Self {
        Self {
            client,
            reconnect_policy: ReconnectPolicy::default(),
        }
    }

    /// Sets reconnect behavior for the stream.
    pub fn reconnect_policy(mut self, policy: ReconnectPolicy) -> Self {
        self.reconnect_policy = policy;
        self
    }

    /// Opens an authenticated WebSocket connection.
    pub async fn connect(self) -> Result<KalshiWebSocket> {
        KalshiWebSocket::connect(self.client, self.reconnect_policy).await
    }
}

/// Reconnect behavior for a WebSocket stream.
#[derive(Debug, Clone)]
pub struct ReconnectPolicy {
    /// Whether the client should reconnect after read errors or close frames.
    pub enabled: bool,
    /// Maximum reconnect attempts per disconnect. `None` retries indefinitely.
    pub max_attempts: Option<usize>,
    /// Delay before the first reconnect attempt.
    pub initial_delay: Duration,
    /// Maximum exponential-backoff delay between reconnect attempts.
    pub max_delay: Duration,
}

impl Default for ReconnectPolicy {
    fn default() -> Self {
        Self {
            enabled: true,
            max_attempts: None,
            initial_delay: Duration::from_millis(250),
            max_delay: Duration::from_secs(10),
        }
    }
}

/// Authenticated Kalshi WebSocket connection.
///
/// The connection records confirmed subscriptions from `subscribed` messages. On reconnect it
/// resubscribes confirmed subscriptions and any subscribe commands that were sent but not yet
/// acknowledged.
#[derive(Debug)]
pub struct KalshiWebSocket {
    client: Kalshi,
    socket: Socket,
    next_id: u64,
    pending_subscriptions: BTreeMap<u64, Subscription>,
    subscriptions: BTreeMap<u64, Subscription>,
    reconnect_policy: ReconnectPolicy,
}

impl KalshiWebSocket {
    /// Opens an authenticated WebSocket connection with the provided reconnect policy.
    pub async fn connect(client: Kalshi, reconnect_policy: ReconnectPolicy) -> Result<Self> {
        let socket = open_socket(&client).await?;
        Ok(Self {
            client,
            socket,
            next_id: 1,
            pending_subscriptions: BTreeMap::new(),
            subscriptions: BTreeMap::new(),
            reconnect_policy,
        })
    }

    /// Sends a subscribe command and returns the client command id.
    ///
    /// The server subscription id is available from the subsequent `subscribed` stream message.
    pub async fn subscribe(&mut self, subscription: Subscription) -> Result<u64> {
        let id = self
            .send_command("subscribe", subscription.to_params_value())
            .await?;
        self.pending_subscriptions.insert(id, subscription);
        Ok(id)
    }

    /// Sends an unsubscribe command for server subscription ids.
    pub async fn unsubscribe(&mut self, sids: impl IntoIterator<Item = u64>) -> Result<u64> {
        let sids = sids.into_iter().collect::<Vec<_>>();
        let params = serde_json::json!({ "sids": &sids });
        let id = self.send_command("unsubscribe", params).await?;
        for sid in sids {
            self.subscriptions.remove(&sid);
        }
        Ok(id)
    }

    /// Requests the server's current subscription list.
    pub async fn list_subscriptions(&mut self) -> Result<u64> {
        self.send_command("list_subscriptions", serde_json::json!({}))
            .await
    }

    /// Sends a subscription update command with a typed action and serializable params.
    pub async fn update_subscription(
        &mut self,
        sid: u64,
        action: UpdateAction,
        params: impl Serialize,
    ) -> Result<u64> {
        let mut value = serde_json::to_value(params)?;
        if !value.is_object() {
            value = serde_json::json!({});
        }

        let object = value.as_object_mut().expect("object checked above");
        object.insert("sid".to_owned(), Value::Number(sid.into()));
        object.insert(
            "action".to_owned(),
            Value::String(action.as_str().to_owned()),
        );

        self.send_command("update_subscription", value).await
    }

    /// Adds market tickers to an existing market subscription.
    pub async fn add_markets(
        &mut self,
        sid: u64,
        market_tickers: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<u64> {
        let market_tickers = market_tickers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let id = self
            .update_subscription(
                sid,
                UpdateAction::AddMarkets,
                markets_param(market_tickers.clone()),
            )
            .await?;
        if let Some(subscription) = self.subscriptions.get_mut(&sid) {
            subscription.add_markets(market_tickers);
        }
        Ok(id)
    }

    /// Removes market tickers from an existing market subscription.
    pub async fn delete_markets(
        &mut self,
        sid: u64,
        market_tickers: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<u64> {
        let market_tickers = market_tickers
            .into_iter()
            .map(Into::into)
            .collect::<Vec<_>>();
        let id = self
            .update_subscription(
                sid,
                UpdateAction::DeleteMarkets,
                markets_param(market_tickers.clone()),
            )
            .await?;
        if let Some(subscription) = self.subscriptions.get_mut(&sid) {
            subscription.delete_markets(&market_tickers);
        }
        Ok(id)
    }

    /// Requests a fresh orderbook snapshot for market tickers on an existing subscription.
    pub async fn get_orderbook_snapshot(
        &mut self,
        sid: u64,
        market_tickers: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<u64> {
        self.update_subscription(
            sid,
            UpdateAction::GetSnapshot,
            markets_param(market_tickers),
        )
        .await
    }

    /// Subscribes an existing CF Benchmarks subscription to index ids.
    pub async fn subscribe_indices(
        &mut self,
        sid: u64,
        index_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<u64> {
        let index_ids = index_ids.into_iter().map(Into::into).collect::<Vec<_>>();
        let id = self
            .update_subscription(
                sid,
                UpdateAction::SubscribeIndices,
                serde_json::json!({ "index_ids": &index_ids }),
            )
            .await?;
        if let Some(subscription) = self.subscriptions.get_mut(&sid) {
            subscription.add_index_ids(index_ids);
        }
        Ok(id)
    }

    /// Unsubscribes index ids from an existing CF Benchmarks subscription.
    pub async fn unsubscribe_indices(
        &mut self,
        sid: u64,
        index_ids: impl IntoIterator<Item = impl Into<String>>,
    ) -> Result<u64> {
        let index_ids = index_ids.into_iter().map(Into::into).collect::<Vec<_>>();
        let id = self
            .update_subscription(
                sid,
                UpdateAction::UnsubscribeIndices,
                serde_json::json!({ "index_ids": &index_ids }),
            )
            .await?;
        if let Some(subscription) = self.subscriptions.get_mut(&sid) {
            subscription.delete_index_ids(&index_ids);
        }
        Ok(id)
    }

    /// Requests the index list for an existing CF Benchmarks subscription.
    pub async fn list_indices(&mut self, sid: u64) -> Result<u64> {
        self.update_subscription(sid, UpdateAction::IndexList, serde_json::json!({}))
            .await
    }

    /// Receives the next stream message.
    ///
    /// This convenience method mirrors async-stream style APIs. Transport closure is reported as
    /// `Some(Err(Error::StreamClosed))`; use [`KalshiWebSocket::recv`] if you prefer a plain
    /// `Result`.
    pub async fn next(&mut self) -> Option<Result<StreamMessage>> {
        Some(self.recv().await)
    }

    /// Receives and parses the next stream message, reconnecting according to policy if needed.
    pub async fn recv(&mut self) -> Result<StreamMessage> {
        loop {
            match self.socket.next().await {
                Some(Ok(Message::Text(text))) => {
                    let message = StreamMessage::parse_text(&text)?;
                    self.record_message(&message);
                    return Ok(message);
                }
                Some(Ok(Message::Binary(bytes))) => {
                    let text = String::from_utf8_lossy(&bytes);
                    let message = StreamMessage::parse_text(&text)?;
                    self.record_message(&message);
                    return Ok(message);
                }
                Some(Ok(Message::Ping(payload))) => {
                    self.socket.send(Message::Pong(payload)).await?;
                }
                Some(Ok(Message::Pong(_))) => {}
                Some(Ok(Message::Close(_))) => {
                    if self.reconnect_policy.enabled {
                        self.reconnect().await?;
                        continue;
                    }
                    return Err(Error::StreamClosed);
                }
                Some(Ok(Message::Frame(_))) => {}
                Some(Err(err)) => {
                    tracing::warn!(error = ?err, "Kalshi websocket read failed");
                    if self.reconnect_policy.enabled {
                        self.reconnect().await?;
                        continue;
                    }
                    return Err(err.into());
                }
                None => {
                    if self.reconnect_policy.enabled {
                        self.reconnect().await?;
                        continue;
                    }
                    return Err(Error::StreamClosed);
                }
            }
        }
    }

    async fn send_command(&mut self, cmd: &str, params: Value) -> Result<u64> {
        let id = self.next_id;
        self.next_id += 1;
        let payload = serde_json::json!({
            "id": id,
            "cmd": cmd,
            "params": params,
        });
        self.socket
            .send(Message::Text(serde_json::to_string(&payload)?.into()))
            .await?;
        Ok(id)
    }

    async fn reconnect(&mut self) -> Result<()> {
        let mut attempt = 0usize;
        let mut delay = self.reconnect_policy.initial_delay;

        loop {
            attempt += 1;
            if let Some(max_attempts) = self.reconnect_policy.max_attempts
                && attempt > max_attempts
            {
                return Err(Error::StreamClosed);
            }

            sleep(delay).await;
            match open_socket(&self.client).await {
                Ok(socket) => {
                    self.socket = socket;
                    let subscriptions = self
                        .subscriptions
                        .values()
                        .chain(self.pending_subscriptions.values())
                        .cloned()
                        .collect::<Vec<_>>();
                    self.subscriptions.clear();
                    self.pending_subscriptions.clear();
                    for subscription in subscriptions {
                        self.subscribe(subscription).await?;
                    }
                    return Ok(());
                }
                Err(err) => {
                    tracing::warn!(attempt, error = ?err, "Kalshi websocket reconnect failed");
                    delay = (delay * 2).min(self.reconnect_policy.max_delay);
                }
            }
        }
    }

    fn record_message(&mut self, message: &StreamMessage) {
        match message {
            StreamMessage::Subscribed(envelope) => {
                if let Some(id) = envelope.id
                    && let Some(subscription) = self.pending_subscriptions.remove(&id)
                {
                    self.subscriptions.insert(envelope.msg.sid, subscription);
                }
            }
            StreamMessage::Unsubscribed(envelope) => {
                if let Some(sid) = envelope.sid {
                    self.subscriptions.remove(&sid);
                }
            }
            _ => {}
        }
    }
}

async fn open_socket(client: &Kalshi) -> Result<Socket> {
    let auth = client.ws_auth_headers()?;
    let mut request = client.ws_url().into_client_request()?;
    let headers = request.headers_mut();
    headers.insert(ACCESS_KEY_HEADER, HeaderValue::from_str(&auth.key_id)?);
    headers.insert(
        ACCESS_SIGNATURE_HEADER,
        HeaderValue::from_str(&auth.signature)?,
    );
    headers.insert(
        ACCESS_TIMESTAMP_HEADER,
        HeaderValue::from_str(&auth.timestamp)?,
    );

    let (socket, _) = timeout(client.inner.config.timeout, connect_async(request))
        .await
        .map_err(|_| Error::Timeout)??;
    Ok(socket)
}

fn markets_param(markets: impl IntoIterator<Item = impl Into<String>>) -> Value {
    let markets = markets.into_iter().map(Into::into).collect::<Vec<String>>();
    match markets.as_slice() {
        [] => serde_json::json!({}),
        [one] => serde_json::json!({ "market_ticker": one }),
        many => serde_json::json!({ "market_tickers": many }),
    }
}
