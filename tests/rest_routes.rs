use std::future::Future;
use std::time::Duration;

use kalshi_pmx::generated::*;
use kalshi_pmx::params::*;
use kalshi_pmx::{Environment, Kalshi};
use serde_json::{Value, json};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[derive(Debug)]
struct CapturedRequest {
    method: String,
    target: String,
    path: String,
    body: String,
}

async fn capture_request<F, Fut, T>(call: F) -> CapturedRequest
where
    F: FnOnce(Kalshi) -> Fut,
    Fut: Future<Output = kalshi_pmx::Result<T>>,
{
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr = listener.local_addr().unwrap();
    let base_url = format!("http://{addr}");
    let client = Kalshi::builder()
        .environment(Environment::Custom {
            rest_base_url: base_url,
            ws_url: "ws://127.0.0.1/ws".to_owned(),
        })
        .build()
        .unwrap();

    let server = tokio::spawn(async move {
        let (mut socket, _) = listener.accept().await.unwrap();
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
        let path = target.split('?').next().unwrap().to_owned();
        let body =
            String::from_utf8_lossy(&bytes[header_end..header_end + content_length]).to_string();

        let response = b"HTTP/1.1 200 OK\r\ncontent-type: application/json\r\ncontent-length: 2\r\nconnection: close\r\n\r\n{}";
        socket.write_all(response).await.unwrap();

        CapturedRequest {
            method,
            target,
            path,
            body,
        }
    });

    let call_error = call(client).await.err();

    tokio::time::timeout(Duration::from_secs(5), server)
        .await
        .unwrap_or_else(|_| {
            panic!("server timed out waiting for request; call error: {call_error:?}")
        })
        .unwrap()
}

async fn assert_route<F, Fut, T>(method: &str, path: &str, call: F)
where
    F: FnOnce(Kalshi) -> Fut,
    Fut: Future<Output = kalshi_pmx::Result<T>>,
{
    let captured = capture_request(call).await;
    assert_eq!(captured.method, method);
    assert_eq!(captured.path, path);
}

async fn assert_route_with_query<F, Fut, T>(method: &str, path: &str, query: &str, call: F)
where
    F: FnOnce(Kalshi) -> Fut,
    Fut: Future<Output = kalshi_pmx::Result<T>>,
{
    let captured = capture_request(call).await;
    assert_eq!(captured.method, method);
    assert_eq!(captured.path, path);
    assert!(
        captured.target.contains(query),
        "target `{}` did not contain query fragment `{}`",
        captured.target,
        query
    );
}

async fn assert_route_with_body<F, Fut, T>(method: &str, path: &str, body_fragment: &str, call: F)
where
    F: FnOnce(Kalshi) -> Fut,
    Fut: Future<Output = kalshi_pmx::Result<T>>,
{
    let captured = capture_request(call).await;
    assert_eq!(captured.method, method);
    assert_eq!(captured.path, path);
    assert!(
        captured.body.contains(body_fragment),
        "body `{}` did not contain fragment `{}`",
        captured.body,
        body_fragment
    );
}

fn order() -> CreateOrderV2Request {
    CreateOrderV2Request {
        ticker: "TICKER-1".to_owned(),
        client_order_id: Some("client-1".to_owned()),
        side: BookSide::Bid,
        count: "1.00".to_owned(),
        price: "0.5000".to_owned(),
        expiration_time: None,
        time_in_force: "good_till_canceled".to_owned(),
        post_only: None,
        self_trade_prevention_type: SelfTradePreventionType::TakerAtCross,
        cancel_order_on_pause: None,
        reduce_only: None,
        subaccount: None,
        order_group_id: None,
        exchange_index: None,
        extra: Default::default(),
    }
}

fn amend_order() -> AmendOrderV2Request {
    AmendOrderV2Request {
        ticker: "TICKER-1".to_owned(),
        side: BookSide::Bid,
        price: "0.5100".to_owned(),
        count: "1.00".to_owned(),
        client_order_id: None,
        updated_client_order_id: Some("client-2".to_owned()),
        exchange_index: None,
        extra: Default::default(),
    }
}

fn decrease_order() -> DecreaseOrderV2Request {
    DecreaseOrderV2Request {
        reduce_by: Some("1.00".to_owned()),
        reduce_to: None,
        exchange_index: None,
        extra: Default::default(),
    }
}

fn ticker_pair() -> TickerPair {
    TickerPair {
        market_ticker: "TICKER-1".to_owned(),
        event_ticker: "EVENT".to_owned(),
        side: "yes".to_owned(),
        extra: Default::default(),
    }
}

#[tokio::test]
async fn exchange_and_market_routes() {
    assert_route("GET", "/exchange/status", |c| async move {
        c.exchange().status().await
    })
    .await;
    assert_route("GET", "/exchange/announcements", |c| async move {
        c.exchange().announcements().await
    })
    .await;
    assert_route("GET", "/exchange/schedule", |c| async move {
        c.exchange().schedule().await
    })
    .await;
    assert_route("GET", "/exchange/user_data_timestamp", |c| async move {
        c.exchange().user_data_timestamp().await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/series/fee_changes",
        "show_historical=true",
        |c| async move {
            c.exchange()
                .series_fee_changes(&GetSeriesFeeChangesParams::new().show_historical(true))
                .await
        },
    )
    .await;

    assert_route_with_query("GET", "/markets", "limit=1", |c| async move {
        c.markets().list(&GetMarketsParams::new().limit(1)).await
    })
    .await;
    assert_route("GET", "/markets/TICKER-1", |c| async move {
        c.markets().get("TICKER-1").await
    })
    .await;
    assert_route_with_query("GET", "/markets/trades", "limit=1", |c| async move {
        c.markets().trades(&GetTradesParams::new().limit(1)).await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/markets/TICKER-1/orderbook",
        "depth=5",
        |c| async move {
            c.markets()
                .orderbook("TICKER-1", &GetMarketOrderbookParams::new().depth(5))
                .await
        },
    )
    .await;
    assert_route_with_query(
        "GET",
        "/markets/orderbooks",
        "tickers=TICKER-1",
        |c| async move {
            c.markets()
                .orderbooks(&GetMarketOrderbooksParams::new().tickers(["TICKER-1"]))
                .await
        },
    )
    .await;
    assert_route("GET", "/series/SERIES", |c| async move {
        c.markets().series("SERIES").await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/series/SERIES",
        "include_volume=true",
        |c| async move {
            c.markets()
                .series_with_params("SERIES", &GetSeriesParams::new().include_volume(true))
                .await
        },
    )
    .await;
    assert_route_with_query("GET", "/series", "category=sports", |c| async move {
        c.markets()
            .series_list(&GetSeriesListParams::new().category("sports"))
            .await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/series/SERIES/markets/TICKER-1/candlesticks",
        "period_interval=1",
        |c| async move {
            c.markets()
                .candlesticks(
                    "SERIES",
                    "TICKER-1",
                    &GetMarketCandlesticksParams::new()
                        .start_ts(1)
                        .end_ts(2)
                        .period_interval(1),
                )
                .await
        },
    )
    .await;
    assert_route_with_query(
        "GET",
        "/series/SERIES/events/EVENT/candlesticks",
        "period_interval=1",
        |c| async move {
            c.markets()
                .candlesticks_by_event(
                    "SERIES",
                    "EVENT",
                    &GetMarketCandlesticksByEventParams::new()
                        .start_ts(1)
                        .end_ts(2)
                        .period_interval(1),
                )
                .await
        },
    )
    .await;
    assert_route_with_query(
        "GET",
        "/markets/candlesticks",
        "market_tickers=TICKER-1",
        |c| async move {
            c.markets()
                .batch_candlesticks(
                    &BatchGetMarketCandlesticksParams::new().market_tickers("TICKER-1"),
                )
                .await
        },
    )
    .await;
}

#[tokio::test]
async fn event_and_order_routes() {
    assert_route_with_query("GET", "/events", "limit=1", |c| async move {
        c.events().list(&GetEventsParams::new().limit(1)).await
    })
    .await;
    assert_route_with_query("GET", "/events/multivariate", "limit=1", |c| async move {
        c.events()
            .multivariate(&GetMultivariateEventsParams::new().limit(1))
            .await
    })
    .await;
    assert_route_with_query("GET", "/events/fee_changes", "limit=1", |c| async move {
        c.events()
            .fee_changes(&GetEventFeeChangesParams::new().limit(1))
            .await
    })
    .await;
    assert_route("GET", "/events/EVENT", |c| async move {
        c.events().get("EVENT").await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/events/EVENT",
        "with_nested_markets=true",
        |c| async move {
            c.events()
                .get_with_params("EVENT", &GetEventParams::new().with_nested_markets(true))
                .await
        },
    )
    .await;
    assert_route("GET", "/events/EVENT/metadata", |c| async move {
        c.events().metadata("EVENT").await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/series/SERIES/events/EVENT/forecast_percentile_history",
        "period_interval=1",
        |c| async move {
            c.events()
                .forecast_percentile_history(
                    "SERIES",
                    "EVENT",
                    &GetEventForecastPercentilesHistoryParams::new()
                        .percentiles(vec![1])
                        .start_ts(1)
                        .end_ts(2)
                        .period_interval(1),
                )
                .await
        },
    )
    .await;

    assert_route_with_query("GET", "/portfolio/orders", "limit=1", |c| async move {
        c.orders().list(&GetOrdersParams::new().limit(1)).await
    })
    .await;
    assert_route("GET", "/portfolio/orders/order-1", |c| async move {
        c.orders().get("order-1").await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/portfolio/orders/queue_positions",
        "event_ticker=EVENT",
        |c| async move {
            c.orders()
                .queue_positions(&GetOrderQueuePositionsParams::new().event_ticker("EVENT"))
                .await
        },
    )
    .await;
    assert_route(
        "GET",
        "/portfolio/orders/order-1/queue_position",
        |c| async move { c.orders().queue_position("order-1").await },
    )
    .await;

    let order = order();
    assert_route_with_body(
        "POST",
        "/portfolio/events/orders",
        "\"ticker\":\"TICKER-1\"",
        |c| {
            let order = order.clone();
            async move { c.orders().create(&order).await }
        },
    )
    .await;
    let batch = BatchCreateOrdersV2Request {
        orders: vec![order.clone()],
        extra: Default::default(),
    };
    assert_route_with_body(
        "POST",
        "/portfolio/events/orders/batched",
        "\"orders\"",
        |c| {
            let batch = batch.clone();
            async move { c.orders().batch_create(&batch).await }
        },
    )
    .await;
    let batch_cancel = BatchCancelOrdersV2Request {
        orders: vec![json!({ "order_id": "order-1" })],
        extra: Default::default(),
    };
    assert_route_with_body(
        "DELETE",
        "/portfolio/events/orders/batched",
        "\"orders\"",
        |c| {
            let batch_cancel = batch_cancel.clone();
            async move { c.orders().batch_cancel(&batch_cancel).await }
        },
    )
    .await;
    assert_route(
        "DELETE",
        "/portfolio/events/orders/order-1",
        |c| async move { c.orders().cancel("order-1").await },
    )
    .await;

    let amend = amend_order();
    assert_route_with_body(
        "POST",
        "/portfolio/events/orders/order-1/amend",
        "\"price\":\"0.5100\"",
        |c| {
            let amend = amend.clone();
            async move { c.orders().amend("order-1", &amend).await }
        },
    )
    .await;

    let decrease = decrease_order();
    assert_route_with_body(
        "POST",
        "/portfolio/events/orders/order-1/decrease",
        "\"reduce_by\":\"1.00\"",
        |c| {
            let decrease = decrease.clone();
            async move { c.orders().decrease("order-1", &decrease).await }
        },
    )
    .await;
}

#[tokio::test]
async fn portfolio_and_order_group_routes() {
    assert_route_with_query(
        "GET",
        "/portfolio/order_groups",
        "subaccount=0",
        |c| async move {
            c.order_groups()
                .list(&GetOrderGroupsParams::new().subaccount(0))
                .await
        },
    )
    .await;
    let order_group = CreateOrderGroupRequest {
        subaccount: Some(0),
        contracts_limit: Some(1),
        contracts_limit_fp: None,
        exchange_index: None,
        extra: Default::default(),
    };
    assert_route_with_body(
        "POST",
        "/portfolio/order_groups/create",
        "\"contracts_limit\"",
        |c| {
            let order_group = order_group.clone();
            async move { c.order_groups().create(&order_group).await }
        },
    )
    .await;
    assert_route("GET", "/portfolio/order_groups/group-1", |c| async move {
        c.order_groups().get("group-1").await
    })
    .await;
    assert_route(
        "DELETE",
        "/portfolio/order_groups/group-1",
        |c| async move { c.order_groups().delete("group-1").await },
    )
    .await;
    assert_route_with_body(
        "PUT",
        "/portfolio/order_groups/group-1/reset",
        "{}",
        |c| async move { c.order_groups().reset("group-1").await },
    )
    .await;
    assert_route_with_body(
        "PUT",
        "/portfolio/order_groups/group-1/trigger",
        "{}",
        |c| async move { c.order_groups().trigger("group-1").await },
    )
    .await;
    assert_route_with_body(
        "PUT",
        "/portfolio/order_groups/group-1/limit",
        "\"contracts_limit\"",
        |c| async move {
            let body = UpdateOrderGroupLimitRequest {
                contracts_limit: Some(1),
                contracts_limit_fp: None,
                extra: Default::default(),
            };
            c.order_groups().update_limit("group-1", &body).await
        },
    )
    .await;

    assert_route("GET", "/portfolio/balance", |c| async move {
        c.portfolio().balance().await
    })
    .await;
    assert_route("POST", "/portfolio/subaccounts", |c| async move {
        c.portfolio().create_subaccount().await
    })
    .await;
    let transfer = ApplySubaccountTransferRequest {
        client_transfer_id: "transfer-1".to_owned(),
        from_subaccount: 0,
        to_subaccount: 1,
        amount_cents: 100,
        extra: Default::default(),
    };
    assert_route_with_body(
        "POST",
        "/portfolio/subaccounts/transfer",
        "\"amount_cents\"",
        |c| {
            let transfer = transfer.clone();
            async move { c.portfolio().transfer_subaccount(&transfer).await }
        },
    )
    .await;
    assert_route("GET", "/portfolio/subaccounts/balances", |c| async move {
        c.portfolio().subaccount_balances().await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/portfolio/subaccounts/transfers",
        "limit=1",
        |c| async move {
            c.portfolio()
                .subaccount_transfers(&GetSubaccountTransfersParams::new().limit(1))
                .await
        },
    )
    .await;
    let netting = UpdateSubaccountNettingRequest {
        subaccount_number: 0,
        enabled: true,
        extra: Default::default(),
    };
    assert_route_with_body(
        "PUT",
        "/portfolio/subaccounts/netting",
        "\"enabled\"",
        |c| {
            let netting = netting.clone();
            async move { c.portfolio().update_subaccount_netting(&netting).await }
        },
    )
    .await;
    assert_route("GET", "/portfolio/subaccounts/netting", |c| async move {
        c.portfolio().subaccount_netting().await
    })
    .await;
    assert_route_with_query("GET", "/portfolio/positions", "limit=1", |c| async move {
        c.portfolio()
            .positions(&GetPositionsParams::new().limit(1))
            .await
    })
    .await;
    assert_route_with_query("GET", "/portfolio/settlements", "limit=1", |c| async move {
        c.portfolio()
            .settlements(&GetSettlementsParams::new().limit(1))
            .await
    })
    .await;
    assert_route_with_query("GET", "/portfolio/deposits", "limit=1", |c| async move {
        c.portfolio()
            .deposits(&GetDepositsParams::new().limit(1))
            .await
    })
    .await;
    assert_route_with_query("GET", "/portfolio/withdrawals", "limit=1", |c| async move {
        c.portfolio()
            .withdrawals(&GetWithdrawalsParams::new().limit(1))
            .await
    })
    .await;
    assert_route(
        "GET",
        "/portfolio/summary/total_resting_order_value",
        |c| async move { c.portfolio().total_resting_order_value().await },
    )
    .await;
    assert_route_with_query("GET", "/portfolio/fills", "limit=1", |c| async move {
        c.portfolio().fills(&GetFillsParams::new().limit(1)).await
    })
    .await;
}

#[tokio::test]
async fn communications_account_and_admin_routes() {
    assert_route("GET", "/communications/id", |c| async move {
        c.communications().id().await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/communications/block-trade-proposals",
        "limit=1",
        |c| async move {
            c.communications()
                .block_trade_proposals(&GetBlockTradeProposalsParams::new().limit(1))
                .await
        },
    )
    .await;
    let block_trade = ProposeBlockTradeRequest {
        buyer_user_id: "buyer".to_owned(),
        buyer_subtrader_id: None,
        buyer_subaccount: None,
        seller_user_id: "seller".to_owned(),
        seller_subtrader_id: None,
        seller_subaccount: None,
        market_ticker: "TICKER-1".to_owned(),
        price_centi_cents: 5000,
        centicount: 100,
        maker_side: "yes".to_owned(),
        expiration_ts: "2026-01-01T00:00:00Z".to_owned(),
        extra: Default::default(),
    };
    assert_route_with_body(
        "POST",
        "/communications/block-trade-proposals",
        "\"buyer_user_id\"",
        |c| {
            let block_trade = block_trade.clone();
            async move { c.communications().propose_block_trade(&block_trade).await }
        },
    )
    .await;
    assert_route_with_body(
        "POST",
        "/communications/block-trade-proposals/proposal-1/accept",
        "{}",
        |c| async move {
            c.communications()
                .accept_block_trade_proposal("proposal-1")
                .await
        },
    )
    .await;
    assert_route_with_query("GET", "/communications/rfqs", "limit=1", |c| async move {
        c.communications()
            .rfqs(&GetRFQsParams::new().limit(1))
            .await
    })
    .await;
    let rfq = CreateRFQRequest {
        market_ticker: "TICKER-1".to_owned(),
        contracts: Some(1),
        contracts_fp: None,
        target_cost_centi_cents: None,
        target_cost_dollars: None,
        rest_remainder: false,
        replace_existing: None,
        subtrader_id: None,
        subaccount: None,
        extra: Default::default(),
    };
    assert_route_with_body("POST", "/communications/rfqs", "\"market_ticker\"", |c| {
        let rfq = rfq.clone();
        async move { c.communications().create_rfq(&rfq).await }
    })
    .await;
    assert_route("GET", "/communications/rfqs/rfq-1", |c| async move {
        c.communications().rfq("rfq-1").await
    })
    .await;
    assert_route("DELETE", "/communications/rfqs/rfq-1", |c| async move {
        c.communications().delete_rfq("rfq-1").await
    })
    .await;
    assert_route_with_query("GET", "/communications/quotes", "limit=1", |c| async move {
        c.communications()
            .quotes(&GetQuotesParams::new().limit(1))
            .await
    })
    .await;
    let quote = CreateQuoteRequest {
        rfq_id: "rfq-1".to_owned(),
        yes_bid: "0.5000".to_owned(),
        no_bid: "0.5000".to_owned(),
        rest_remainder: false,
        post_only: None,
        subaccount: None,
        extra: Default::default(),
    };
    assert_route_with_body("POST", "/communications/quotes", "\"rfq_id\"", |c| {
        let quote = quote.clone();
        async move { c.communications().create_quote(&quote).await }
    })
    .await;
    assert_route("GET", "/communications/quotes/quote-1", |c| async move {
        c.communications().quote("quote-1").await
    })
    .await;
    assert_route("DELETE", "/communications/quotes/quote-1", |c| async move {
        c.communications().delete_quote("quote-1").await
    })
    .await;
    assert_route_with_body(
        "PUT",
        "/communications/quotes/quote-1/accept",
        "\"accepted_side\"",
        |c| async move {
            let body = AcceptQuoteRequest {
                accepted_side: "yes".to_owned(),
                extra: Default::default(),
            };
            c.communications().accept_quote("quote-1", &body).await
        },
    )
    .await;
    assert_route_with_body(
        "PUT",
        "/communications/quotes/quote-1/confirm",
        "{}",
        |c| async move { c.communications().confirm_quote("quote-1").await },
    )
    .await;

    assert_route(
        "GET",
        "/api_keys",
        |c| async move { c.api_keys().list().await },
    )
    .await;
    assert_route_with_body("POST", "/api_keys", "\"name\"", |c| async move {
        let body = CreateApiKeyRequest {
            name: "key".to_owned(),
            public_key: "public".to_owned(),
            scopes: None,
            extra: Default::default(),
        };
        c.api_keys().create(&body).await
    })
    .await;
    assert_route_with_body("POST", "/api_keys/generate", "\"name\"", |c| async move {
        let body = GenerateApiKeyRequest {
            name: "key".to_owned(),
            scopes: None,
            extra: Default::default(),
        };
        c.api_keys().generate(&body).await
    })
    .await;
    assert_route("DELETE", "/api_keys/key-1", |c| async move {
        c.api_keys().delete("key-1").await
    })
    .await;
    assert_route("GET", "/account/limits", |c| async move {
        c.account().limits().await
    })
    .await;
    assert_route("POST", "/account/api_usage_level/upgrade", |c| async move {
        c.account().upgrade_api_usage_level().await
    })
    .await;
    assert_route(
        "GET",
        "/account/api_usage_level/volume_progress",
        |c| async move { c.account().api_usage_level_volume_progress().await },
    )
    .await;
    assert_route("GET", "/account/endpoint_costs", |c| async move {
        c.account().endpoint_costs().await
    })
    .await;
}

#[tokio::test]
async fn search_live_data_and_misc_routes() {
    assert_route("GET", "/search/tags_by_categories", |c| async move {
        c.search().tags_by_categories().await
    })
    .await;
    assert_route("GET", "/search/filters_by_sport", |c| async move {
        c.search().filters_by_sport().await
    })
    .await;
    assert_route("GET", "/live_data/milestone/mile-1", |c| async move {
        c.live_data().by_milestone("mile-1").await
    })
    .await;
    assert_route("GET", "/live_data/game/milestone/mile-1", |c| async move {
        c.live_data().by_type_and_milestone("game", "mile-1").await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/live_data/batch",
        "milestone_ids=mile-1",
        |c| async move {
            c.live_data()
                .batch(&GetLiveDatasParams::new().milestone_ids(["mile-1"]))
                .await
        },
    )
    .await;
    assert_route(
        "GET",
        "/live_data/milestone/mile-1/game_stats",
        |c| async move { c.live_data().game_stats("mile-1").await },
    )
    .await;
    assert_route_with_query(
        "GET",
        "/structured_targets",
        "page_size=1",
        |c| async move {
            c.structured_targets()
                .list(&GetStructuredTargetsParams::new().page_size(1))
                .await
        },
    )
    .await;
    assert_route("GET", "/structured_targets/target-1", |c| async move {
        c.structured_targets().get("target-1").await
    })
    .await;
    assert_route_with_query("GET", "/milestones", "limit=1", |c| async move {
        c.milestones()
            .list(&GetMilestonesParams::new().limit(1))
            .await
    })
    .await;
    assert_route("GET", "/milestones/mile-1", |c| async move {
        c.milestones().get("mile-1").await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/multivariate_event_collections",
        "limit=1",
        |c| async move {
            c.multivariate()
                .collections(&GetMultivariateEventCollectionsParams::new().limit(1))
                .await
        },
    )
    .await;
    assert_route(
        "GET",
        "/multivariate_event_collections/COLL",
        |c| async move { c.multivariate().collection("COLL").await },
    )
    .await;
    assert_route_with_body(
        "POST",
        "/multivariate_event_collections/COLL",
        "\"selected_markets\"",
        |c| async move {
            let body = CreateMarketInMultivariateEventCollectionRequest {
                selected_markets: vec![ticker_pair()],
                with_market_payload: None,
                extra: Default::default(),
            };
            c.multivariate()
                .create_market_in_collection("COLL", &body)
                .await
        },
    )
    .await;
    assert_route_with_body(
        "PUT",
        "/multivariate_event_collections/COLL/lookup",
        "\"selected_markets\"",
        |c| async move {
            let body = LookupTickersForMarketInMultivariateEventCollectionRequest {
                selected_markets: vec![ticker_pair()],
                extra: Default::default(),
            };
            c.multivariate().lookup_tickers("COLL", &body).await
        },
    )
    .await;
    assert_route_with_query(
        "GET",
        "/multivariate_event_collections/COLL/lookup",
        "lookback_seconds=1",
        |c| async move {
            c.multivariate()
                .lookup_history(
                    "COLL",
                    &GetMultivariateEventCollectionLookupHistoryParams::new().lookback_seconds(1),
                )
                .await
        },
    )
    .await;
    assert_route_with_query("GET", "/incentive_programs", "limit=1", |c| async move {
        c.incentive_programs()
            .list(&GetIncentiveProgramsParams::new().limit(1))
            .await
    })
    .await;
    assert_route_with_query("GET", "/fcm/orders", "limit=1", |c| async move {
        c.fcm().orders(&GetFCMOrdersParams::new().limit(1)).await
    })
    .await;
    assert_route_with_query("GET", "/fcm/positions", "limit=1", |c| async move {
        c.fcm()
            .positions(&GetFCMPositionsParams::new().limit(1))
            .await
    })
    .await;
}

#[tokio::test]
async fn historical_and_raw_routes() {
    assert_route("GET", "/historical/cutoff", |c| async move {
        c.historical().cutoff().await
    })
    .await;
    assert_route_with_query(
        "GET",
        "/historical/markets/TICKER-1/candlesticks",
        "start_ts=1",
        |c| async move {
            c.historical()
                .market_candlesticks(
                    "TICKER-1",
                    &GetMarketCandlesticksHistoricalParams::new().start_ts(1),
                )
                .await
        },
    )
    .await;
    assert_route_with_query("GET", "/historical/fills", "limit=1", |c| async move {
        c.historical()
            .fills(&GetFillsHistoricalParams::new().limit(1))
            .await
    })
    .await;
    assert_route_with_query("GET", "/historical/orders", "limit=1", |c| async move {
        c.historical()
            .orders(&GetHistoricalOrdersParams::new().limit(1))
            .await
    })
    .await;
    assert_route_with_query("GET", "/historical/trades", "limit=1", |c| async move {
        c.historical()
            .trades(&GetTradesHistoricalParams::new().limit(1))
            .await
    })
    .await;
    assert_route_with_query("GET", "/historical/markets", "limit=1", |c| async move {
        c.historical()
            .markets(&GetHistoricalMarketsParams::new().limit(1))
            .await
    })
    .await;
    assert_route("GET", "/historical/markets/TICKER-1", |c| async move {
        c.historical().market("TICKER-1").await
    })
    .await;

    assert_route_with_query("GET", "/raw", "q=1", |c| async move {
        c.raw_get("/raw", &json!({ "q": 1 })).await
    })
    .await;
    assert_route("GET", "/raw", |c| async move {
        c.raw_get_no_query("/raw").await
    })
    .await;
    assert_route_with_body("POST", "/raw", "\"body\"", |c| async move {
        c.raw_post("/raw", &json!({ "body": true })).await
    })
    .await;
    assert_route_with_body("PUT", "/raw", "\"body\"", |c| async move {
        c.raw_put("/raw", &json!({ "body": true })).await
    })
    .await;
    assert_route_with_query("DELETE", "/raw", "q=1", |c| async move {
        c.raw_delete("/raw", &json!({ "q": 1 })).await
    })
    .await;
    assert_route("DELETE", "/raw", |c| async move {
        c.raw_delete_no_query("/raw").await
    })
    .await;
    assert_route_with_body("DELETE", "/raw", "\"body\"", |c| async move {
        c.raw_delete_with_body("/raw", &json!({ "body": true }))
            .await
    })
    .await;
    assert_route_with_query("GET", "/typed", "q=1", |c| async move {
        c.request_json::<Value, _, Value>(
            reqwest::Method::GET,
            "/typed",
            Some(&json!({ "q": 1 })),
            None::<&Value>,
        )
        .await
    })
    .await;
}
