use kalshi_pmx::ws::{Channel, LiveOrderbook, StreamMessage, Subscription, UpdateAction};

#[test]
fn subscription_uses_singular_and_plural_market_keys() {
    let one = serde_json::to_value(Subscription::ticker().markets(["ABC-1"])).unwrap();
    assert_eq!(one["market_ticker"], "ABC-1");
    assert!(one.get("market_tickers").is_none());

    let many = serde_json::to_value(Subscription::ticker().markets(["ABC-1", "ABC-2"])).unwrap();
    assert_eq!(many["market_tickers"][0], "ABC-1");
    assert_eq!(many["market_tickers"][1], "ABC-2");
    assert!(many.get("market_ticker").is_none());
}

#[test]
fn subscription_builder_covers_optional_params() {
    let value = serde_json::to_value(
        Subscription::new([Channel::Ticker])
            .channels([Channel::Trade])
            .market_ids(["id-1", "id-2"])
            .index_ids(["BRTI"])
            .send_initial_snapshot(true)
            .use_yes_price(true)
            .skip_ticker_ack(true)
            .shard(2, 10)
            .param("custom", "value"),
    )
    .unwrap();

    assert_eq!(value["channels"], serde_json::json!(["ticker", "trade"]));
    assert_eq!(value["market_ids"], serde_json::json!(["id-1", "id-2"]));
    assert_eq!(value["index_ids"], serde_json::json!(["BRTI"]));
    assert_eq!(value["send_initial_snapshot"], true);
    assert_eq!(value["use_yes_price"], true);
    assert_eq!(value["skip_ticker_ack"], true);
    assert_eq!(value["shard_key"], 2);
    assert_eq!(value["shard_factor"], 10);
    assert_eq!(value["custom"], "value");
}

#[test]
fn channel_and_update_action_serialization() {
    assert_eq!(
        serde_json::to_value(Channel::from("unknown_channel")).unwrap(),
        "unknown_channel"
    );
    assert_eq!(
        serde_json::to_value(UpdateAction::AddMarkets).unwrap(),
        "add_markets"
    );
    assert_eq!(
        serde_json::to_value(UpdateAction::DeleteMarkets).unwrap(),
        "delete_markets"
    );
    assert_eq!(
        serde_json::to_value(UpdateAction::GetSnapshot).unwrap(),
        "get_snapshot"
    );
    assert_eq!(
        serde_json::to_value(UpdateAction::SubscribeIndices).unwrap(),
        "subscribe_indices"
    );
    assert_eq!(
        serde_json::to_value(UpdateAction::UnsubscribeIndices).unwrap(),
        "unsubscribe_indices"
    );
    assert_eq!(
        serde_json::to_value(UpdateAction::IndexList).unwrap(),
        "indexlist"
    );
    assert_eq!(
        serde_json::to_value(UpdateAction::Custom("custom".to_owned())).unwrap(),
        "custom"
    );
}

#[test]
fn parses_ticker_message() {
    let message = StreamMessage::parse_text(
        r#"{
          "type": "ticker",
          "sid": 11,
          "msg": {
            "market_ticker": "FED-23DEC-T3.00",
            "price_dollars": "0.480",
            "yes_bid_dollars": "0.450",
            "yes_ask_dollars": "0.530"
          }
        }"#,
    )
    .unwrap();

    match message {
        StreamMessage::Ticker(envelope) => {
            assert_eq!(envelope.sid, Some(11));
            assert_eq!(envelope.msg.market_ticker, "FED-23DEC-T3.00");
            assert_eq!(envelope.msg.yes_bid_dollars.as_deref(), Some("0.450"));
        }
        other => panic!("unexpected message: {other:?}"),
    }
}

#[test]
fn parses_command_response_messages() {
    let subscribed = StreamMessage::parse_text(
        r#"{"id":1,"type":"subscribed","msg":{"channel":"ticker","sid":7}}"#,
    )
    .unwrap();
    assert!(
        matches!(subscribed, StreamMessage::Subscribed(envelope) if envelope.id == Some(1) && envelope.msg.sid == 7)
    );

    let ok = StreamMessage::parse_text(r#"{"id":2,"type":"ok","sid":7,"seq":8,"msg":{}}"#).unwrap();
    assert!(
        matches!(ok, StreamMessage::Ok(envelope) if envelope.sid == Some(7) && envelope.seq == Some(8))
    );

    let unsubscribed =
        StreamMessage::parse_text(r#"{"id":3,"type":"unsubscribed","sid":7,"seq":9,"msg":{}}"#)
            .unwrap();
    assert!(matches!(
        unsubscribed,
        StreamMessage::Unsubscribed(envelope) if envelope.id == Some(3)
    ));

    let list = StreamMessage::parse_text(
        r#"{"id":4,"type":"list_subscriptions","msg":{"subscriptions":[]}}"#,
    )
    .unwrap();
    assert!(matches!(
        list,
        StreamMessage::ListSubscriptions(envelope) if envelope.id == Some(4)
    ));

    let error = StreamMessage::parse_text(
        r#"{"id":5,"type":"error","msg":{"code":8,"msg":"Unknown channel name"}}"#,
    )
    .unwrap();
    assert!(matches!(
        error,
        StreamMessage::Error(envelope) if envelope.msg.code == 8
    ));
}

#[test]
fn parses_market_private_and_raw_messages() {
    let trade = StreamMessage::parse_text(
        r#"{"type":"trade","sid":1,"msg":{"trade_id":"t1","market_ticker":"M","count_fp":"1.00"}}"#,
    )
    .unwrap();
    assert!(matches!(trade, StreamMessage::Trade(envelope) if envelope.msg.trade_id == "t1"));

    let fill = StreamMessage::parse_text(
        r#"{"type":"fill","sid":1,"msg":{"trade_id":"t1","order_id":"o1","market_ticker":"M"}}"#,
    )
    .unwrap();
    assert!(matches!(fill, StreamMessage::Fill(envelope) if envelope.msg.order_id == "o1"));

    let user_order = StreamMessage::parse_text(
        r#"{"type":"user_order","sid":1,"msg":{"order_id":"o1","ticker":"M","status":"resting"}}"#,
    )
    .unwrap();
    assert!(matches!(user_order, StreamMessage::UserOrder(envelope) if envelope.msg.ticker == "M"));

    for (kind, expected) in [
        ("market_position", "market_position"),
        ("market_lifecycle_v2", "market_lifecycle_v2"),
        ("event_lifecycle", "event_lifecycle"),
        ("event_fee_update", "event_fee_update"),
        (
            "multivariate_market_lifecycle",
            "multivariate_market_lifecycle",
        ),
        ("multivariate_lookup", "multivariate_lookup"),
        ("rfq_created", "communications"),
        ("rfq_deleted", "communications"),
        ("quote_created", "communications"),
        ("quote_accepted", "communications"),
        ("quote_executed", "communications"),
        ("order_group_updates", "order_group_updates"),
        ("cfbenchmarks_value", "cfbenchmarks_value"),
        (
            "cfbenchmarks_value_indexlist",
            "cfbenchmarks_value_indexlist",
        ),
    ] {
        let message = StreamMessage::parse_text(&format!(
            r#"{{"type":"{kind}","sid":1,"msg":{{"field":"value"}}}}"#
        ))
        .unwrap();
        match expected {
            "market_position" => assert!(matches!(message, StreamMessage::MarketPosition(_))),
            "market_lifecycle_v2" => {
                assert!(matches!(message, StreamMessage::MarketLifecycleV2(_)))
            }
            "event_lifecycle" => assert!(matches!(message, StreamMessage::EventLifecycle(_))),
            "event_fee_update" => assert!(matches!(message, StreamMessage::EventFeeUpdate(_))),
            "multivariate_market_lifecycle" => {
                assert!(matches!(
                    message,
                    StreamMessage::MultivariateMarketLifecycle(_)
                ))
            }
            "multivariate_lookup" => {
                assert!(matches!(message, StreamMessage::MultivariateLookup(_)))
            }
            "communications" => assert!(matches!(message, StreamMessage::Communications(_))),
            "order_group_updates" => {
                assert!(matches!(message, StreamMessage::OrderGroupUpdates(_)))
            }
            "cfbenchmarks_value" => assert!(matches!(message, StreamMessage::CfBenchmarksValue(_))),
            "cfbenchmarks_value_indexlist" => {
                assert!(matches!(message, StreamMessage::CfBenchmarksIndexList(_)))
            }
            _ => unreachable!(),
        }
    }

    let raw = StreamMessage::parse_text(r#"{"type":"new_future_message","msg":{"x":1}}"#).unwrap();
    assert!(matches!(raw, StreamMessage::Raw(raw) if raw.kind == "new_future_message"));
}

#[test]
fn live_orderbook_applies_snapshot_and_delta() {
    let snapshot = StreamMessage::parse_text(
        r#"{
          "type": "orderbook_snapshot",
          "sid": 2,
          "seq": 2,
          "msg": {
            "market_ticker": "FED-23DEC-T3.00",
            "yes_dollars_fp": [["0.0800", "300.00"]],
            "no_dollars_fp": [["0.5400", "20.00"]]
          }
        }"#,
    )
    .unwrap();
    let delta = StreamMessage::parse_text(
        r#"{
          "type": "orderbook_delta",
          "sid": 2,
          "seq": 3,
          "msg": {
            "market_ticker": "FED-23DEC-T3.00",
            "price_dollars": "0.0800",
            "delta_fp": "-100.00",
            "side": "yes"
          }
        }"#,
    )
    .unwrap();

    let mut book = LiveOrderbook::new();
    assert!(book.apply_message(&snapshot).unwrap());
    assert_eq!(
        book.best_yes_bid().unwrap(),
        ("0.0800".parse().unwrap(), "300.00".parse().unwrap())
    );

    assert!(book.apply_message(&delta).unwrap());
    assert_eq!(
        book.best_yes_bid().unwrap(),
        ("0.0800".parse().unwrap(), "200.00".parse().unwrap())
    );
    assert_eq!(book.last_seq, Some(3));
}

#[test]
fn live_orderbook_removes_empty_levels_and_ignores_other_messages() {
    let snapshot = StreamMessage::parse_text(
        r#"{
          "type": "orderbook_snapshot",
          "sid": 2,
          "seq": 1,
          "msg": {
            "market_ticker": "M",
            "yes_dollars_fp": [["0.5000", "10.00"]],
            "no_dollars_fp": [["0.4000", "5.00"]]
          }
        }"#,
    )
    .unwrap();
    let remove_yes = StreamMessage::parse_text(
        r#"{
          "type": "orderbook_delta",
          "sid": 2,
          "seq": 2,
          "msg": {
            "market_ticker": "M",
            "price_dollars": "0.5000",
            "delta_fp": "-10.00",
            "side": "yes"
          }
        }"#,
    )
    .unwrap();
    let remove_no = StreamMessage::parse_text(
        r#"{
          "type": "orderbook_delta",
          "sid": 2,
          "seq": 3,
          "msg": {
            "market_ticker": "M",
            "price_dollars": "0.4000",
            "delta_fp": "-10.00",
            "side": "no"
          }
        }"#,
    )
    .unwrap();
    let ticker =
        StreamMessage::parse_text(r#"{"type":"ticker","sid":1,"msg":{"market_ticker":"M"}}"#)
            .unwrap();

    let mut book = LiveOrderbook::new();
    assert!(book.apply_message(&snapshot).unwrap());
    assert!(book.apply_message(&remove_yes).unwrap());
    assert!(book.yes().is_empty());
    assert!(book.apply_message(&remove_no).unwrap());
    assert!(book.no().is_empty());
    assert!(!book.apply_message(&ticker).unwrap());
}

#[test]
fn live_orderbook_rejects_unsafe_deltas() {
    let snapshot = StreamMessage::parse_text(
        r#"{
          "type": "orderbook_snapshot",
          "sid": 2,
          "seq": 10,
          "msg": {
            "market_ticker": "M",
            "market_id": "market-id-1",
            "yes_dollars_fp": [["0.5000", "10.00"]],
            "no_dollars_fp": []
          }
        }"#,
    )
    .unwrap();
    let before_snapshot = StreamMessage::parse_text(
        r#"{
          "type": "orderbook_delta",
          "sid": 2,
          "seq": 11,
          "msg": {
            "market_ticker": "M",
            "price_dollars": "0.5000",
            "delta_fp": "1.00",
            "side": "yes"
          }
        }"#,
    )
    .unwrap();

    let mut book = LiveOrderbook::new();
    assert!(book.apply_message(&before_snapshot).is_err());
    assert!(book.apply_message(&snapshot).unwrap());

    for message in [
        r#"{
          "type": "orderbook_delta",
          "sid": 2,
          "seq": 12,
          "msg": {
            "market_ticker": "M",
            "price_dollars": "0.5000",
            "delta_fp": "1.00",
            "side": "yes"
          }
        }"#,
        r#"{
          "type": "orderbook_delta",
          "sid": 2,
          "seq": 11,
          "msg": {
            "market_ticker": "OTHER",
            "price_dollars": "0.5000",
            "delta_fp": "1.00",
            "side": "yes"
          }
        }"#,
        r#"{
          "type": "orderbook_delta",
          "sid": 2,
          "seq": 11,
          "msg": {
            "market_ticker": "M",
            "market_id": "other-market-id",
            "price_dollars": "0.5000",
            "delta_fp": "1.00",
            "side": "yes"
          }
        }"#,
        r#"{
          "type": "orderbook_delta",
          "sid": 2,
          "seq": 11,
          "msg": {
            "market_ticker": "M",
            "price_dollars": "0.5000",
            "delta_fp": "1.00",
            "side": "maybe"
          }
        }"#,
    ] {
        let delta = StreamMessage::parse_text(message).unwrap();
        assert!(book.apply_message(&delta).is_err());
    }

    assert_eq!(book.last_seq, Some(10));
    assert_eq!(
        book.best_yes_bid().unwrap(),
        ("0.5000".parse().unwrap(), "10.00".parse().unwrap())
    );
}
