# kalshi-pmx

`kalshi-pmx` is a high-level async Rust SDK for Kalshi's Trade API v2 and authenticated WebSocket streams.

It is built for Tokio applications that want typed request builders, typed query params, typed response models, ergonomic REST namespaces, and first-class streaming support without hand-writing raw HTTP calls.

## Features

- Tokio async REST client with production, demo, and custom environments.
- RSA API-key signing for REST and WebSocket authentication.
- High-level REST namespaces for exchange, markets, events, orders, portfolio, communications, API keys, account, live data, milestones, search, FCM, historical data, multivariate markets, structured targets, incentive programs, and order groups.
- Generated OpenAPI structs, enums, query params, request builders, and typed operation methods.
- WebSocket subscriptions for public and private channels, including ticker, trades, orderbook deltas, fills, user orders, market positions, lifecycle, communications, order groups, and CF Benchmarks channels.
- `LiveOrderbook` helper that applies snapshots and deltas into local best-bid state.
- Conservative optional rate limiter and retry handling for `429` responses.
- Offline unit and route tests plus ignored live demo integration tests.

## Install

```toml
[dependencies]
kalshi-pmx = "0.1"
tokio = { version = "1", features = ["macros", "rt-multi-thread"] }
```

The crate uses Rust 1.85 or newer.

## Authentication

Public market-data REST calls can be made without credentials:

```rust,no_run
use kalshi_pmx::params::GetMarketsParams;
use kalshi_pmx::{Kalshi, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let kalshi = Kalshi::demo().build()?;

    let markets = kalshi
        .markets()
        .list(&GetMarketsParams::new().status("open").limit(10))
        .await?;

    println!("{:#?}", markets.markets);
    Ok(())
}
```

Authenticated REST and WebSocket calls need an API key id and RSA private key:

```rust,no_run
use kalshi_pmx::{Kalshi, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let kalshi = Kalshi::builder()
        .from_env()?
        .build()?;

    let balance = kalshi.portfolio().balance().await?;
    println!("{balance:#?}");

    Ok(())
}
```

For examples and live tests, create a `.env` file:

```bash
KALSHI_ENV=demo
KALSHI_API_KEY_ID=your-demo-api-key-id
KALSHI_PRIV_KEY="-----BEGIN PRIVATE KEY-----\n...\n-----END PRIVATE KEY-----"
KALSHI_TICKER=replace-with-active-market-ticker
```

`KALSHI_PRIV_KEY` may be PEM text or a path to a PEM file. `KALSHI_ENV` accepts `demo`/`sandbox`, `production`/`prod`, or a custom REST base URL paired with `KALSHI_WS_URL`.

You can also configure credentials manually:

```rust,no_run
use kalshi_pmx::{Kalshi, Result};

fn client() -> Result<kalshi_pmx::KalshiBuilder> {
    Kalshi::demo().with_key_file("your-api-key-id", "kalshi-private-key.pem")
}
```

## High-Level REST

Normal SDK usage should go through namespace clients and typed params:

```rust,no_run
use kalshi_pmx::params::{GetMarketOrderbookParams, GetMarketsParams, GetTradesParams};
use kalshi_pmx::{Kalshi, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let kalshi = Kalshi::demo().build()?;

    let markets = kalshi
        .markets()
        .list(&GetMarketsParams::new().status("open").limit(1))
        .await?;

    if let Some(market) = markets.markets.first() {
        let book = kalshi
            .markets()
            .orderbook(&market.ticker, &GetMarketOrderbookParams::new().depth(5))
            .await?;

        let trades = kalshi
            .markets()
            .trades(&GetTradesParams::new().ticker(&market.ticker).limit(5))
            .await?;

        println!("{book:#?}");
        println!("{:#?}", trades.trades);
    }

    Ok(())
}
```

Order requests use generated builders and enums. Building a request is side-effect free; calling
`orders().create(...)` is the step that actually submits an order.

```rust,no_run
use kalshi_pmx::generated::{BookSide, CreateOrderV2Request, SelfTradePreventionType};
use kalshi_pmx::models::TimeInForce;
use kalshi_pmx::Result;

#[tokio::main]
async fn main() -> Result<()> {
    let order = CreateOrderV2Request::new(
        "KXEXAMPLE-26JUN24-B100",
        BookSide::Bid,
        "1.0000",
        "0.0500",
        TimeInForce::GoodTillCanceled,
        SelfTradePreventionType::TakerAtCross,
    )
    .client_order_id("client-order-id")
    .post_only(true);

    println!("{}", serde_json::to_string_pretty(&order)?);
    Ok(())
}
```

## Typed OpenAPI Layer

The higher-level namespace clients delegate to generated typed operation methods. You can also call the generated layer directly when you want exact operation names from the OpenAPI spec:

```rust,no_run
use kalshi_pmx::params::GetMarketsParams;
use kalshi_pmx::{Kalshi, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let kalshi = Kalshi::demo().build()?;

    let status = kalshi.typed().get_exchange_status().await?;
    let markets = kalshi
        .typed()
        .get_markets(&GetMarketsParams::new().status("open").limit(5))
        .await?;

    println!("{status:#?}");
    println!("{:#?}", markets.markets);

    Ok(())
}
```

## WebSockets

WebSocket streams are authenticated and use typed subscription builders:

```rust,no_run
use kalshi_pmx::ws::Subscription;
use kalshi_pmx::{Kalshi, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let kalshi = Kalshi::demo()
        .with_key_file("your-api-key-id", "kalshi-private-key.pem")?
        .build()?;

    let mut stream = kalshi.stream().connect().await?;
    stream
        .subscribe(Subscription::ticker().markets(["replace-with-active-market-ticker"]))
        .await?;

    while let Some(message) = stream.next().await {
        println!("{:#?}", message?);
    }

    Ok(())
}
```

For orderbook streams, `LiveOrderbook` keeps local state:

```rust,no_run
use kalshi_pmx::ws::{LiveOrderbook, Subscription};
use kalshi_pmx::{Kalshi, Result};

#[tokio::main]
async fn main() -> Result<()> {
    let kalshi = Kalshi::demo()
        .with_key_file("your-api-key-id", "kalshi-private-key.pem")?
        .build()?;

    let mut stream = kalshi.stream().connect().await?;
    stream
        .subscribe(
            Subscription::orderbook()
                .markets(["replace-with-active-market-ticker"])
                .use_yes_price(true),
        )
        .await?;

    let mut book = LiveOrderbook::new();
    while let Some(message) = stream.next().await {
        let message = message?;
        if book.apply_message(&message)? {
            println!("best yes bid: {:?}", book.best_yes_bid());
        }
    }

    Ok(())
}
```

### Market Data Granularity

Kalshi's public streams provide market data such as ticker updates, trades, and aggregate
orderbook snapshots/deltas. They are suitable for maintaining a local aggregate orderbook with
`LiveOrderbook`, but they are not a feed of every individual order submitted by every participant.
Private streams such as fills and user orders report your authenticated account activity.

## Examples

Run public REST examples without credentials:

```bash
cargo run --example list_markets
cargo run --example market_snapshot
cargo run --example typed_operations
cargo run --example order_request_builder
```

Run authenticated read or stream examples after creating `.env`:

```bash
cargo run --example read_portfolio
cargo run --example stream_ticker
cargo run --example live_orderbook
```

## Specs and Generation

The repository keeps the Kalshi OpenAPI and AsyncAPI references used by this SDK in `specs/`:

- `specs/openapi.yaml`
- `specs/asyncapi.yaml`

From a repository checkout, regenerate typed schemas, params, and operation methods after updating the OpenAPI spec:

```bash
cargo run --manifest-path xtask/Cargo.toml -- generate-openapi-types
```

The crates.io package ships the generated Rust source, not the upstream spec files, because the checked-in specs carry their own upstream licensing metadata.

## Tests

Offline tests cover auth signing and redaction, authenticated request headers, encoded path
segments, API error bodies, `429` retry behavior, generated builders and enums, model
serialization, route construction, typed query encoding, pagination helpers, WebSocket message
parsing, WebSocket client command flow, reconnect behavior, and spec coverage:

```bash
cargo test
```

Read-only live demo tests are ignored by default. They load `.env`, use demo mode unless configured otherwise, and exercise authenticated REST, public market data, portfolio reads, typed operations, and WebSocket subscriptions:

```bash
cargo test --test live_demo -- --ignored --test-threads=1
```

Release checks:

```bash
cargo fmt --check
cargo clippy --all-targets -- -D warnings
cargo test
cargo check --examples
cargo package
cargo publish --dry-run
```

## API Coverage

The SDK includes generated types and typed operations for the checked-in OpenAPI spec, plus high-level REST namespace clients for the documented API areas. WebSocket channels from the checked-in AsyncAPI spec are represented as typed subscriptions and parsed stream messages, with raw message preservation for forward compatibility.
