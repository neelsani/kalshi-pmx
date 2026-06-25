use kalshi_pmx::generated::ExchangeStatus;
use kalshi_pmx::{Environment, Error, Kalshi};
use rsa::RsaPrivateKey;
use rsa::pkcs1::{EncodeRsaPrivateKey, LineEnding};
use rsa::rand_core::OsRng;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[derive(Debug)]
struct CapturedRequest {
    method: String,
    target: String,
    headers: String,
    body: String,
}

fn test_private_key_pem() -> String {
    RsaPrivateKey::new(&mut OsRng, 1024)
        .unwrap()
        .to_pkcs1_pem(LineEnding::LF)
        .unwrap()
        .to_string()
}

fn custom_client(addr: std::net::SocketAddr) -> kalshi_pmx::KalshiBuilder {
    Kalshi::builder().environment(Environment::Custom {
        rest_base_url: format!("http://{addr}"),
        ws_url: "ws://127.0.0.1/ws".to_owned(),
    })
}

async fn read_request(socket: &mut TcpStream) -> CapturedRequest {
    let mut bytes = Vec::new();
    let header_end = loop {
        let mut buf = [0_u8; 1024];
        let n = socket.read(&mut buf).await.unwrap();
        assert!(n > 0, "client closed before sending headers");
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
        assert!(n > 0, "client closed before sending request body");
        bytes.extend_from_slice(&buf[..n]);
    }

    let first_line = headers.lines().next().unwrap();
    let mut parts = first_line.split_whitespace();
    let method = parts.next().unwrap().to_owned();
    let target = parts.next().unwrap().to_owned();
    let body = String::from_utf8_lossy(&bytes[header_end..header_end + content_length]).to_string();

    CapturedRequest {
        method,
        target,
        headers,
        body,
    }
}

async fn write_json_response(
    socket: &mut TcpStream,
    status: &str,
    body: impl AsRef<[u8]>,
    close: bool,
) {
    let body = body.as_ref();
    let connection = if close { "close" } else { "keep-alive" };
    let headers = format!(
        "HTTP/1.1 {status}\r\ncontent-type: application/json\r\ncontent-length: {}\r\nconnection: {connection}\r\n\r\n",
        body.len()
    );
    socket.write_all(headers.as_bytes()).await.unwrap();
    socket.write_all(body).await.unwrap();
}

#[tokio::test]
async fn authenticated_requests_include_headers_user_agent_and_encoded_paths() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = custom_client(listener.local_addr().unwrap())
        .user_agent("kalshi-pmx-test")
        .with_key_pem("test-key-id", test_private_key_pem())
        .build()
        .unwrap();

    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let request = read_request(&mut socket).await;
        write_json_response(&mut socket, "204 No Content", [], true).await;
        request
    });

    let response = client.api_keys().delete("key with/slash").await.unwrap();
    assert!(response.is_empty());

    let request = server.await.unwrap();
    assert_eq!(request.method, "DELETE");
    assert_eq!(request.target, "/api_keys/key%20with%2Fslash");
    assert!(request.body.is_empty());

    let headers = request.headers.to_ascii_lowercase();
    assert!(headers.contains("user-agent: kalshi-pmx-test"));
    assert!(headers.contains("kalshi-access-key: test-key-id"));
    assert!(headers.contains("kalshi-access-signature: "));
    assert!(headers.contains("kalshi-access-timestamp: "));
}

#[tokio::test]
async fn api_errors_preserve_status_and_body() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = custom_client(listener.local_addr().unwrap())
        .max_retries(0)
        .build()
        .unwrap();

    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
        let request = read_request(&mut socket).await;
        write_json_response(
            &mut socket,
            "403 Forbidden",
            br#"{"error":"permission_denied","message":"read only"}"#,
            true,
        )
        .await;
        request
    });

    let err = client
        .exchange()
        .status()
        .await
        .expect_err("403 should fail");
    match err {
        Error::Api { status, body } => {
            assert_eq!(status, reqwest::StatusCode::FORBIDDEN);
            assert!(body.contains("permission_denied"));
            assert!(body.contains("read only"));
        }
        other => panic!("expected API error, got {other:?}"),
    }

    let request = server.await.unwrap();
    assert_eq!(request.method, "GET");
    assert_eq!(request.target, "/exchange/status");
}

#[tokio::test]
async fn rate_limit_responses_are_retried_then_decoded() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = custom_client(listener.local_addr().unwrap())
        .max_retries(1)
        .build()
        .unwrap();

    let server = tokio::spawn(async move {
        let (mut first, _) = listener.accept().await.unwrap();
        let first_request = read_request(&mut first).await;
        write_json_response(
            &mut first,
            "429 Too Many Requests",
            br#"{"error":"slow"}"#,
            true,
        )
        .await;

        let (mut second, _) = listener.accept().await.unwrap();
        let second_request = read_request(&mut second).await;
        write_json_response(
            &mut second,
            "200 OK",
            br#"{"exchange_active":true,"trading_active":false}"#,
            true,
        )
        .await;

        (first_request, second_request)
    });

    let status: ExchangeStatus = client.exchange().status().await.unwrap();
    assert!(status.exchange_active);
    assert!(!status.trading_active);

    let (first, second) = server.await.unwrap();
    assert_eq!(first.target, "/exchange/status");
    assert_eq!(second.target, "/exchange/status");
}
