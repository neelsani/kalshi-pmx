use std::future::Future;

use kalshi_pmx::generated::CreateOrderV2Request;
use kalshi_pmx::params::{GetMarketOrderbooksParams, GetMarketsParams};
use kalshi_pmx::{Environment, Kalshi};
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug)]
struct CapturedRequest {
    method: String,
    target: String,
    body: String,
}

async fn typed_capture<F, Fut, T>(response: Value, call: F) -> (CapturedRequest, T)
where
    F: FnOnce(Kalshi) -> Fut,
    Fut: Future<Output = kalshi_pmx::Result<T>>,
{
    typed_capture_http("200 OK", Some(response), call).await
}

async fn typed_capture_http<F, Fut, T>(
    status: &str,
    response: Option<Value>,
    call: F,
) -> (CapturedRequest, T)
where
    F: FnOnce(Kalshi) -> Fut,
    Fut: Future<Output = kalshi_pmx::Result<T>>,
{
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let client = Kalshi::builder()
        .environment(Environment::Custom {
            rest_base_url: format!("http://{addr}"),
            ws_url: "ws://127.0.0.1/ws".to_owned(),
        })
        .build()
        .unwrap();
    let body = response.map(|response| serde_json::to_vec(&response).unwrap());
    let status = status.to_owned();

    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let mut bytes = Vec::new();
        let header_end = loop {
            let mut buf = [0_u8; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            bytes.extend_from_slice(&buf[..n]);
            if let Some(pos) = bytes.windows(4).position(|window| window == b"\r\n\r\n") {
                break pos + 4;
            }
        };
        let headers = String::from_utf8_lossy(&bytes[..header_end]).to_string();
        let content_length = headers
            .lines()
            .find_map(|line| {
                let (name, value) = line.split_once(':')?;
                name.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().unwrap())
            })
            .unwrap_or(0);
        while bytes.len() < header_end + content_length {
            let mut buf = [0_u8; 1024];
            let n = socket.read(&mut buf).await.unwrap();
            bytes.extend_from_slice(&buf[..n]);
        }

        let first_line = headers.lines().next().unwrap();
        let mut parts = first_line.split_whitespace();
        let method = parts.next().unwrap().to_owned();
        let target = parts.next().unwrap().to_owned();
        let request_body =
            String::from_utf8_lossy(&bytes[header_end..header_end + content_length]).to_string();

        let body_len = body.as_ref().map_or(0, Vec::len);
        let response = format!(
            "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {body_len}\r\nconnection: close\r\n\r\n",
        );
        socket.write_all(response.as_bytes()).await.unwrap();
        if let Some(body) = body {
            socket.write_all(&body).await.unwrap();
        }

        CapturedRequest {
            method,
            target,
            body: request_body,
        }
    });

    let decoded = call(client).await.unwrap();
    (server.await.unwrap(), decoded)
}

#[tokio::test]
async fn typed_exchange_status_decodes_response() {
    let (request, status) = typed_capture(
        json!({
            "exchange_active": true,
            "trading_active": false,
            "future_extra_field": "kept"
        }),
        |client| async move { client.typed().get_exchange_status().await },
    )
    .await;

    assert_eq!(request.method, "GET");
    assert_eq!(request.target, "/exchange/status");
    assert!(status.exchange_active);
    assert!(!status.trading_active);
    assert_eq!(status.extra["future_extra_field"], "kept");
}

#[tokio::test]
async fn typed_get_markets_keeps_query_and_decodes_empty_list() {
    let (request, markets) = typed_capture(
        json!({ "markets": [], "cursor": "" }),
        |client| async move {
            client
                .typed()
                .get_markets(&GetMarketsParams::new().status("open").limit(1))
                .await
        },
    )
    .await;

    assert_eq!(request.method, "GET");
    assert!(request.target.starts_with("/markets?"));
    assert!(request.target.contains("status=open"));
    assert!(request.target.contains("limit=1"));
    assert!(markets.markets.is_empty());
    assert_eq!(markets.cursor, "");
}

#[tokio::test]
async fn typed_query_encoder_expands_array_params_as_repeated_keys() {
    let (request, orderbooks) = typed_capture(json!({ "orderbooks": [] }), |client| async move {
        client
            .typed()
            .get_market_orderbooks(
                &GetMarketOrderbooksParams::new().tickers(["TICKER-1", "TICKER-2"]),
            )
            .await
    })
    .await;

    assert_eq!(request.method, "GET");
    assert!(request.target.starts_with("/markets/orderbooks?"));
    assert!(request.target.contains("tickers=TICKER-1"));
    assert!(request.target.contains("tickers=TICKER-2"));
    assert!(orderbooks.orderbooks.is_empty());
}

#[tokio::test]
async fn typed_create_order_uses_generated_request_and_response() {
    let order = CreateOrderV2Request::new(
        "TICKER-1",
        kalshi_pmx::generated::BookSide::Bid,
        "1.00",
        "0.5000",
        "good_till_canceled",
        kalshi_pmx::generated::SelfTradePreventionType::TakerAtCross,
    )
    .client_order_id("client-1")
    .post_only(true)
    .subaccount(0)
    .exchange_index(0);

    let (request, response) = typed_capture(
        json!({
            "order_id": "order-1",
            "client_order_id": "client-1",
            "fill_count": "0.00",
            "remaining_count": "1.00",
            "ts_ms": 123
        }),
        |client| {
            let order = order.clone();
            async move { client.typed().create_order_v2(&order).await }
        },
    )
    .await;

    assert_eq!(request.method, "POST");
    assert_eq!(request.target, "/portfolio/events/orders");
    assert!(request.body.contains("\"ticker\":\"TICKER-1\""));
    assert_eq!(response.order_id, "order-1");
    assert_eq!(response.remaining_count, "1.00");
}

#[tokio::test]
async fn typed_empty_response_accepts_success_with_no_body() {
    let (request, response) = typed_capture_http("204 No Content", None, |client| async move {
        client.typed().delete_api_key("key-1").await
    })
    .await;

    assert_eq!(request.method, "DELETE");
    assert_eq!(request.target, "/api_keys/key-1");
    assert!(response.is_empty());
}
