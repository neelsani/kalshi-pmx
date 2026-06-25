use kalshi_pmx::models::{
    AmendOrderRequest, BookSide, CreateOrderRequest, DecreaseOrderRequest, FixedPoint,
    SelfTradePreventionType, TimeInForce,
};

#[test]
fn create_order_request_matches_v2_shape() {
    let order = CreateOrderRequest::bid("HIGHNY-24JAN01-T60", "10.00", "0.5600")
        .client_order_id("client-1")
        .time_in_force(TimeInForce::ImmediateOrCancel);

    let value = serde_json::to_value(order).unwrap();
    assert_eq!(value["ticker"], "HIGHNY-24JAN01-T60");
    assert_eq!(value["side"], "bid");
    assert_eq!(value["count"], "10.00");
    assert_eq!(value["price"], "0.5600");
    assert_eq!(value["time_in_force"], "immediate_or_cancel");
    assert_eq!(value["self_trade_prevention_type"], "taker_at_cross");
    assert_eq!(value["client_order_id"], "client-1");

    let ask = CreateOrderRequest::limit("HIGHNY-24JAN01-T60", BookSide::Ask, "5.00", "0.5800");
    let ask = serde_json::to_value(ask).unwrap();
    assert_eq!(ask["side"], "ask");
}

#[test]
fn amend_and_decrease_requests_match_v2_shape() {
    let mut amend = AmendOrderRequest::new("HIGHNY-24JAN01-T60", BookSide::Ask, "8.00", "0.5700");
    amend.client_order_id = Some("old".to_owned());
    amend.updated_client_order_id = Some("new".to_owned());
    amend.exchange_index = Some(-1);

    let value = serde_json::to_value(amend).unwrap();
    assert_eq!(value["ticker"], "HIGHNY-24JAN01-T60");
    assert_eq!(value["side"], "ask");
    assert_eq!(value["price"], "0.5700");
    assert_eq!(value["count"], "8.00");
    assert_eq!(value["client_order_id"], "old");
    assert_eq!(value["updated_client_order_id"], "new");
    assert_eq!(value["exchange_index"], -1);

    let by = serde_json::to_value(DecreaseOrderRequest::reduce_by("2.00")).unwrap();
    assert_eq!(by["reduce_by"], "2.00");
    assert!(by.get("reduce_to").is_none());

    let to = serde_json::to_value(DecreaseOrderRequest::reduce_to("5.00")).unwrap();
    assert_eq!(to["reduce_to"], "5.00");
    assert!(to.get("reduce_by").is_none());
}

#[test]
fn order_request_builder_options_serialize() {
    let order = CreateOrderRequest::ask("TICKER", "3.00", "0.6200")
        .client_order_id("client-2")
        .time_in_force(TimeInForce::FillOrKill)
        .self_trade_prevention_type(SelfTradePreventionType::Maker)
        .post_only(true)
        .reduce_only(false);

    let value = serde_json::to_value(order).unwrap();
    assert_eq!(value["side"], "ask");
    assert_eq!(value["time_in_force"], "fill_or_kill");
    assert_eq!(value["self_trade_prevention_type"], "maker");
    assert_eq!(value["post_only"], true);
    assert_eq!(value["reduce_only"], false);
}

#[test]
fn fixed_point_rejects_invalid_values() {
    for invalid in ["", ".", "1.23456", "abc", "1.2.3"] {
        assert!(
            invalid.parse::<FixedPoint>().is_err(),
            "{invalid} should be invalid"
        );
    }
}

#[test]
fn fixed_point_formats_orders_and_adds_without_float_rounding() {
    let mut values = [
        "1.0000".parse::<FixedPoint>().unwrap(),
        "-0.0001".parse::<FixedPoint>().unwrap(),
        "0.1000".parse::<FixedPoint>().unwrap(),
    ];
    values.sort();

    assert_eq!(
        values.map(|value| value.to_string()),
        ["-0.0001", "0.1000", "1.0000"]
    );

    let sum = "0.1000"
        .parse::<FixedPoint>()
        .unwrap()
        .checked_add("0.0200".parse().unwrap())
        .unwrap();
    assert_eq!(sum.to_string(), "0.1200");
    assert!(
        "0.0000"
            .parse::<FixedPoint>()
            .unwrap()
            .is_zero_or_negative()
    );
    assert!(
        "-0.0001"
            .parse::<FixedPoint>()
            .unwrap()
            .is_zero_or_negative()
    );
}

#[test]
fn time_in_force_display_and_string_conversion_match_api_values() {
    assert_eq!(TimeInForce::GoodTillCanceled.as_str(), "good_till_canceled");
    assert_eq!(
        TimeInForce::ImmediateOrCancel.to_string(),
        "immediate_or_cancel"
    );

    let as_string: String = TimeInForce::FillOrKill.into();
    assert_eq!(as_string, "fill_or_kill");
}
