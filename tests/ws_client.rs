use std::sync::{Arc, Mutex};

use futures_util::{SinkExt, StreamExt};
use kalshi_pmx::ws::{ReconnectPolicy, StreamMessage, Subscription, UpdateAction};
use kalshi_pmx::{Environment, Error, Kalshi};
use rsa::RsaPrivateKey;
use rsa::pkcs1::{EncodeRsaPrivateKey, LineEnding};
use rsa::rand_core::OsRng;
use serde_json::{Value, json};
use tokio::net::TcpListener;
use tokio::time::{Duration, timeout};
use tokio_tungstenite::tungstenite::Message;
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};
use tokio_tungstenite::{accept_async, accept_hdr_async};

fn test_private_key_pem() -> String {
    RsaPrivateKey::new(&mut OsRng, 1024)
        .unwrap()
        .to_pkcs1_pem(LineEnding::LF)
        .unwrap()
        .to_string()
}

fn client_for_ws_url(ws_url: String) -> Kalshi {
    Kalshi::builder()
        .environment(Environment::Custom {
            rest_base_url: "http://127.0.0.1".to_owned(),
            ws_url,
        })
        .with_key_pem("test-key", test_private_key_pem())
        .build()
        .unwrap()
}

#[tokio::test]
async fn websocket_methods_send_expected_commands() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let ws_url = format!("ws://{}/trade-api/ws/v2", listener.local_addr().unwrap());

    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = accept_async(stream).await.unwrap();
        let mut messages = Vec::new();

        for _ in 0..10 {
            let message = timeout(Duration::from_secs(2), socket.next())
                .await
                .unwrap()
                .unwrap()
                .unwrap();
            match message {
                Message::Text(text) => messages.push(serde_json::from_str::<Value>(&text).unwrap()),
                other => panic!("unexpected websocket message: {other:?}"),
            }
        }

        messages
    });

    let client = client_for_ws_url(ws_url);
    let mut socket = client.stream().connect().await.unwrap();

    assert_eq!(
        socket
            .subscribe(Subscription::ticker().markets(["TICKER-1"]))
            .await
            .unwrap(),
        1
    );
    assert_eq!(socket.unsubscribe([42]).await.unwrap(), 2);
    assert_eq!(socket.list_subscriptions().await.unwrap(), 3);
    assert_eq!(socket.add_markets(7, ["TICKER-2"]).await.unwrap(), 4);
    assert_eq!(socket.delete_markets(7, ["TICKER-3"]).await.unwrap(), 5);
    assert_eq!(
        socket
            .get_orderbook_snapshot(7, ["TICKER-4"])
            .await
            .unwrap(),
        6
    );
    assert_eq!(socket.subscribe_indices(8, ["BRTI"]).await.unwrap(), 7);
    assert_eq!(socket.unsubscribe_indices(8, ["BRTI"]).await.unwrap(), 8);
    assert_eq!(socket.list_indices(8).await.unwrap(), 9);
    assert_eq!(
        socket
            .update_subscription(
                9,
                UpdateAction::Custom("refresh".to_owned()),
                "non-object params",
            )
            .await
            .unwrap(),
        10
    );

    let messages = server.await.unwrap();
    assert_eq!(
        messages[0],
        json!({
            "id": 1,
            "cmd": "subscribe",
            "params": {
                "channels": ["ticker"],
                "market_ticker": "TICKER-1"
            }
        })
    );
    assert_eq!(
        messages[1],
        json!({ "id": 2, "cmd": "unsubscribe", "params": { "sids": [42] } })
    );
    assert_eq!(
        messages[2],
        json!({ "id": 3, "cmd": "list_subscriptions", "params": {} })
    );
    assert_eq!(
        messages[3],
        json!({
            "id": 4,
            "cmd": "update_subscription",
            "params": {
                "sid": 7,
                "action": "add_markets",
                "market_ticker": "TICKER-2"
            }
        })
    );
    assert_eq!(
        messages[4],
        json!({
            "id": 5,
            "cmd": "update_subscription",
            "params": {
                "sid": 7,
                "action": "delete_markets",
                "market_ticker": "TICKER-3"
            }
        })
    );
    assert_eq!(
        messages[5],
        json!({
            "id": 6,
            "cmd": "update_subscription",
            "params": {
                "sid": 7,
                "action": "get_snapshot",
                "market_ticker": "TICKER-4"
            }
        })
    );
    assert_eq!(
        messages[6],
        json!({
            "id": 7,
            "cmd": "update_subscription",
            "params": {
                "sid": 8,
                "action": "subscribe_indices",
                "index_ids": ["BRTI"]
            }
        })
    );
    assert_eq!(
        messages[7],
        json!({
            "id": 8,
            "cmd": "update_subscription",
            "params": {
                "sid": 8,
                "action": "unsubscribe_indices",
                "index_ids": ["BRTI"]
            }
        })
    );
    assert_eq!(
        messages[8],
        json!({
            "id": 9,
            "cmd": "update_subscription",
            "params": {
                "sid": 8,
                "action": "indexlist"
            }
        })
    );
    assert_eq!(
        messages[9],
        json!({
            "id": 10,
            "cmd": "update_subscription",
            "params": {
                "sid": 9,
                "action": "refresh"
            }
        })
    );
}

#[tokio::test]
#[allow(clippy::result_large_err)]
async fn websocket_connect_sends_auth_headers() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let ws_url = format!("ws://{}/trade-api/ws/v2", listener.local_addr().unwrap());
    let captured_headers = Arc::new(Mutex::new(None));
    let server_headers = Arc::clone(&captured_headers);

    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let _socket = accept_hdr_async(stream, move |request: &Request, response: Response| {
            let headers = request.headers();
            let key = headers
                .get("KALSHI-ACCESS-KEY")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            let signature = headers
                .get("KALSHI-ACCESS-SIGNATURE")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            let timestamp = headers
                .get("KALSHI-ACCESS-TIMESTAMP")
                .unwrap()
                .to_str()
                .unwrap()
                .to_owned();
            *server_headers.lock().unwrap() = Some((key, signature, timestamp));
            Ok(response)
        })
        .await
        .unwrap();
    });

    let client = client_for_ws_url(ws_url);
    let _socket = client.stream().connect().await.unwrap();
    server.await.unwrap();

    let (key, signature, timestamp) = captured_headers
        .lock()
        .unwrap()
        .clone()
        .expect("websocket handshake headers should be captured");
    assert_eq!(key, "test-key");
    assert!(!signature.is_empty());
    assert!(timestamp.parse::<u128>().is_ok());
}

#[tokio::test]
async fn websocket_next_reports_stream_closed_when_reconnect_is_disabled() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let ws_url = format!("ws://{}/trade-api/ws/v2", listener.local_addr().unwrap());

    let server = tokio::spawn(async move {
        let (stream, _) = listener.accept().await.unwrap();
        let mut socket = accept_async(stream).await.unwrap();
        socket.close(None).await.unwrap();
    });

    let client = client_for_ws_url(ws_url);
    let mut socket = client
        .stream()
        .reconnect_policy(ReconnectPolicy {
            enabled: false,
            ..ReconnectPolicy::default()
        })
        .connect()
        .await
        .unwrap();

    let err = timeout(Duration::from_secs(5), socket.next())
        .await
        .unwrap()
        .unwrap()
        .expect_err("closed stream should return an error");
    assert!(matches!(err, Error::StreamClosed));

    server.await.unwrap();
}

#[tokio::test]
async fn websocket_reconnects_resubscribes_and_returns_next_message() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let ws_url = format!("ws://{}/trade-api/ws/v2", listener.local_addr().unwrap());

    let server = tokio::spawn(async move {
        let (first_stream, _) = listener.accept().await.unwrap();
        let mut first = accept_async(first_stream).await.unwrap();
        let first_subscribe = first.next().await.unwrap().unwrap();
        first.close(None).await.unwrap();

        let (second_stream, _) = listener.accept().await.unwrap();
        let mut second = accept_async(second_stream).await.unwrap();
        let second_subscribe = second.next().await.unwrap().unwrap();
        second
            .send(Message::Text(
                r#"{"type":"ticker","sid":1,"msg":{"market_ticker":"TICKER-1"}}"#.into(),
            ))
            .await
            .unwrap();

        (first_subscribe, second_subscribe)
    });

    let client = client_for_ws_url(ws_url);
    let mut socket = client.stream().connect().await.unwrap();
    socket
        .subscribe(Subscription::ticker().markets(["TICKER-1"]))
        .await
        .unwrap();

    let message = timeout(Duration::from_secs(5), socket.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(message, StreamMessage::Ticker(envelope) if envelope.msg.market_ticker == "TICKER-1")
    );

    let (first_subscribe, second_subscribe) = server.await.unwrap();
    let first = serde_json::from_str::<Value>(&first_subscribe.into_text().unwrap()).unwrap();
    let second = serde_json::from_str::<Value>(&second_subscribe.into_text().unwrap()).unwrap();
    assert_eq!(first["id"], 1);
    assert_eq!(second["id"], 2);
    assert_eq!(first["cmd"], second["cmd"]);
    assert_eq!(first["params"], second["params"]);
}

#[tokio::test]
async fn websocket_reconnect_does_not_replay_unsubscribed_sid() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let ws_url = format!("ws://{}/trade-api/ws/v2", listener.local_addr().unwrap());

    let server = tokio::spawn(async move {
        let (first_stream, _) = listener.accept().await.unwrap();
        let mut first = accept_async(first_stream).await.unwrap();
        let subscribe = first.next().await.unwrap().unwrap();
        first
            .send(Message::Text(
                r#"{"id":1,"type":"subscribed","msg":{"channel":"ticker","sid":7}}"#.into(),
            ))
            .await
            .unwrap();
        let unsubscribe = first.next().await.unwrap().unwrap();
        first.close(None).await.unwrap();

        let (second_stream, _) = listener.accept().await.unwrap();
        let mut second = accept_async(second_stream).await.unwrap();
        let replay = timeout(Duration::from_millis(500), second.next()).await;
        assert!(replay.is_err(), "unsubscribed stream was replayed");
        second
            .send(Message::Text(
                r#"{"type":"ticker","sid":99,"msg":{"market_ticker":"OTHER"}}"#.into(),
            ))
            .await
            .unwrap();

        (subscribe, unsubscribe)
    });

    let client = client_for_ws_url(ws_url);
    let mut socket = client.stream().connect().await.unwrap();
    assert_eq!(
        socket
            .subscribe(Subscription::ticker().markets(["TICKER-1"]))
            .await
            .unwrap(),
        1
    );
    let ack = timeout(Duration::from_secs(5), socket.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(matches!(ack, StreamMessage::Subscribed(envelope) if envelope.msg.sid == 7));
    assert_eq!(socket.unsubscribe([7]).await.unwrap(), 2);

    let message = timeout(Duration::from_secs(5), socket.recv())
        .await
        .unwrap()
        .unwrap();
    assert!(
        matches!(message, StreamMessage::Ticker(envelope) if envelope.msg.market_ticker == "OTHER")
    );

    let (subscribe, unsubscribe) = server.await.unwrap();
    let subscribe = serde_json::from_str::<Value>(&subscribe.into_text().unwrap()).unwrap();
    let unsubscribe = serde_json::from_str::<Value>(&unsubscribe.into_text().unwrap()).unwrap();
    assert_eq!(subscribe["cmd"], "subscribe");
    assert_eq!(
        unsubscribe,
        json!({ "id": 2, "cmd": "unsubscribe", "params": { "sids": [7] } })
    );
}
