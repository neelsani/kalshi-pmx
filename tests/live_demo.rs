use std::time::{Duration, Instant};

use kalshi_pmx::Error;
use kalshi_pmx::Kalshi;
use kalshi_pmx::params::*;
use kalshi_pmx::rate_limit::RateLimitConfig;
use kalshi_pmx::ws::{Channel, LiveOrderbook, StreamMessage, Subscription};
use reqwest::StatusCode;
use tokio::time::timeout;

type TestResult<T = ()> = Result<T, Box<dyn std::error::Error + Send + Sync>>;

const LIVE_TIMEOUT: Duration = Duration::from_secs(20);

fn live_client() -> TestResult<Kalshi> {
    Ok(Kalshi::builder()
        .from_env()?
        .with_rate_limit(RateLimitConfig::conservative())
        .build()?)
}

async fn first_open_market_ticker(kalshi: &Kalshi) -> TestResult<String> {
    let open = kalshi
        .markets()
        .list(&GetMarketsParams::new().status("open").limit(20))
        .await?;

    if let Some(ticker) = open.markets.first().map(|market| market.ticker.clone()) {
        return Ok(ticker);
    }

    let any = kalshi
        .markets()
        .list(&GetMarketsParams::new().limit(20))
        .await?;
    any.markets
        .first()
        .map(|market| market.ticker.clone())
        .ok_or_else(|| "demo API returned no markets".into())
}

async fn recv_until<F>(
    ws: &mut kalshi_pmx::ws::KalshiWebSocket,
    mut f: F,
) -> TestResult<StreamMessage>
where
    F: FnMut(&StreamMessage) -> bool,
{
    let deadline = Instant::now() + LIVE_TIMEOUT;
    while Instant::now() < deadline {
        let remaining = deadline.saturating_duration_since(Instant::now());
        let message = timeout(remaining, ws.recv()).await??;

        if let StreamMessage::Error(error) = &message {
            return Err(format!("websocket error response: {:?}", error.msg).into());
        }

        if f(&message) {
            return Ok(message);
        }
    }

    Err("timed out waiting for websocket message".into())
}

async fn wait_for_subscribed(
    ws: &mut kalshi_pmx::ws::KalshiWebSocket,
    command_id: u64,
) -> TestResult<u64> {
    let message = recv_until(ws, |message| {
        matches!(
            message,
            StreamMessage::Subscribed(envelope) if envelope.id == Some(command_id)
        )
    })
    .await?;

    match message {
        StreamMessage::Subscribed(envelope) => Ok(envelope.msg.sid),
        _ => unreachable!("recv_until predicate only accepts subscribed messages"),
    }
}

fn assert_ok_or_permission_denied<T>(result: Result<T, Error>, _label: &str) -> TestResult {
    match result {
        Ok(_) => Ok(()),
        Err(Error::Api { status, body })
            if status == StatusCode::FORBIDDEN && body.contains("permission_denied") =>
        {
            Ok(())
        }
        Err(err) => Err(Box::new(err)),
    }
}

#[tokio::test]
#[ignore = "requires Kalshi credentials in .env"]
async fn live_demo_authenticated_rest_smoke() -> TestResult {
    let kalshi = live_client()?;

    let status = kalshi.exchange().status().await?;
    let _ = (status.exchange_active, status.trading_active);

    let user_data_timestamp = kalshi.exchange().user_data_timestamp().await?;
    assert!(!user_data_timestamp.as_of_time.is_empty());

    let balance = kalshi.portfolio().balance().await?;
    let _ = (balance.balance, balance.portfolio_value);

    let limits = kalshi.account().limits().await?;
    assert!(!limits.usage_tier.is_empty());

    let endpoint_costs = kalshi.account().endpoint_costs().await?;
    let _ = endpoint_costs.default_cost;

    Ok(())
}

#[tokio::test]
#[ignore = "requires Kalshi credentials in .env"]
async fn live_demo_market_data_reads() -> TestResult {
    let kalshi = live_client()?;
    let ticker = first_open_market_ticker(&kalshi).await?;

    let market = kalshi.markets().get(&ticker).await?;
    assert_eq!(market.market.ticker, ticker);

    let orderbook = kalshi
        .markets()
        .orderbook(&ticker, &GetMarketOrderbookParams::new().depth(5))
        .await?;
    let _ = orderbook.orderbook_fp;

    let trades = kalshi
        .markets()
        .trades(&GetTradesParams::new().ticker(ticker).limit(5))
        .await?;
    assert!(trades.trades.len() <= 5);

    let events = kalshi
        .events()
        .list(&GetEventsParams::new().status("open").limit(5))
        .await?;
    assert!(events.events.len() <= 5);

    Ok(())
}

#[tokio::test]
#[ignore = "requires Kalshi credentials in .env"]
async fn live_demo_typed_methods_smoke() -> TestResult {
    let kalshi = live_client()?;

    let status = kalshi.typed().get_exchange_status().await?;
    let _ = (status.exchange_active, status.trading_active);

    let markets = kalshi
        .typed()
        .get_markets(&GetMarketsParams::new().status("open").limit(1))
        .await?;
    assert!(markets.markets.len() <= 1);

    Ok(())
}

#[tokio::test]
#[ignore = "requires Kalshi credentials in .env"]
async fn live_demo_portfolio_read_only_endpoints() -> TestResult {
    let kalshi = live_client()?;

    assert_ok_or_permission_denied(
        kalshi
            .portfolio()
            .positions(&GetPositionsParams::new().limit(5))
            .await,
        "positions",
    )?;
    assert_ok_or_permission_denied(
        kalshi
            .portfolio()
            .fills(&GetFillsParams::new().limit(5))
            .await,
        "fills",
    )?;
    assert_ok_or_permission_denied(
        kalshi.orders().list(&GetOrdersParams::new().limit(5)).await,
        "orders",
    )?;
    assert_ok_or_permission_denied(
        kalshi.portfolio().total_resting_order_value().await,
        "resting order value",
    )?;

    Ok(())
}

#[tokio::test]
#[ignore = "requires Kalshi credentials in .env"]
async fn live_demo_websocket_ticker_subscription_ack() -> TestResult {
    let kalshi = live_client()?;
    let ticker = first_open_market_ticker(&kalshi).await?;

    let mut ws = kalshi.stream().connect().await?;
    let command_id = ws
        .subscribe(
            Subscription::ticker()
                .markets([ticker.clone()])
                .send_initial_snapshot(true),
        )
        .await?;
    let _sid = wait_for_subscribed(&mut ws, command_id).await?;

    Ok(())
}

#[tokio::test]
#[ignore = "requires Kalshi credentials in .env"]
async fn live_demo_websocket_orderbook_snapshot_updates_book() -> TestResult {
    let kalshi = live_client()?;
    let ticker = first_open_market_ticker(&kalshi).await?;

    let mut ws = kalshi.stream().connect().await?;
    let command_id = ws
        .subscribe(
            Subscription::orderbook()
                .markets([ticker.clone()])
                .use_yes_price(true),
        )
        .await?;
    let _sid = wait_for_subscribed(&mut ws, command_id).await?;

    let snapshot = recv_until(&mut ws, |message| {
        matches!(
            message,
            StreamMessage::OrderbookSnapshot(envelope) if envelope.msg.market_ticker == ticker
        )
    })
    .await?;

    let mut book = LiveOrderbook::new();
    assert!(book.apply_message(&snapshot)?);
    assert_eq!(book.market_ticker.as_deref(), Some(ticker.as_str()));
    assert!(
        book.last_seq.is_some(),
        "orderbook snapshot should include seq"
    );

    Ok(())
}

#[tokio::test]
#[ignore = "requires Kalshi credentials in .env"]
async fn live_demo_websocket_private_read_only_subscriptions_ack() -> TestResult {
    let kalshi = live_client()?;
    let mut ws = kalshi.stream().connect().await?;

    for subscription in [
        Subscription::fills(),
        Subscription::market_positions(),
        Subscription::user_orders(),
        Subscription::new([Channel::OrderGroupUpdates]),
    ] {
        let command_id = ws.subscribe(subscription).await?;
        let sid = wait_for_subscribed(&mut ws, command_id).await?;
        assert!(sid > 0, "subscription id should be positive");
    }

    Ok(())
}
