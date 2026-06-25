use kalshi_pmx::generated::{
    BatchCreateOrdersV2Request, BookSide, CreateOrderGroupRequest, CreateOrderV2Request,
    CreateRFQRequest, SelfTradePreventionType, UpdateOrderGroupLimitRequest,
};
use serde_json::json;

#[test]
fn generated_component_enums_serialize_and_deserialize() {
    assert_eq!(serde_json::to_value(BookSide::Bid).unwrap(), json!("bid"));
    assert_eq!(
        serde_json::from_value::<BookSide>(json!("ask")).unwrap(),
        BookSide::Ask
    );
    assert_eq!(
        serde_json::to_value(SelfTradePreventionType::TakerAtCross).unwrap(),
        json!("taker_at_cross")
    );
    assert_eq!(
        serde_json::from_value::<SelfTradePreventionType>(json!("maker")).unwrap(),
        SelfTradePreventionType::Maker
    );
}

#[test]
fn generated_order_request_builder_sets_required_and_optional_fields() {
    let order = CreateOrderV2Request::new(
        "TICKER-1",
        BookSide::Bid,
        "1.00",
        "0.5000",
        "good_till_canceled",
        SelfTradePreventionType::TakerAtCross,
    )
    .client_order_id("client-1")
    .post_only(true)
    .subaccount(7);

    let value = serde_json::to_value(&order).unwrap();
    assert_eq!(value["ticker"], "TICKER-1");
    assert_eq!(value["side"], "bid");
    assert_eq!(value["count"], "1.00");
    assert_eq!(value["price"], "0.5000");
    assert_eq!(value["client_order_id"], "client-1");
    assert_eq!(value["post_only"], true);
    assert_eq!(value["subaccount"], 7);
    assert!(value.get("reduce_only").is_none());

    let batch = BatchCreateOrdersV2Request::new(vec![order]);
    let batch = serde_json::to_value(batch).unwrap();
    assert_eq!(batch["orders"].as_array().unwrap().len(), 1);
}

#[test]
fn generated_misc_request_builders_cover_empty_and_fixed_point_bodies() {
    let group = CreateOrderGroupRequest::new()
        .subaccount(4)
        .contracts_limit_fp("3.50");
    let group = serde_json::to_value(group).unwrap();
    assert_eq!(group["subaccount"], 4);
    assert_eq!(group["contracts_limit_fp"], "3.50");

    let limit = UpdateOrderGroupLimitRequest::new().contracts_limit(10);
    let limit = serde_json::to_value(limit).unwrap();
    assert_eq!(limit["contracts_limit"], 10);

    let rfq = CreateRFQRequest::new("TICKER-1", false)
        .contracts_fp("2.00")
        .target_cost_dollars("1.25")
        .replace_existing(true);
    let rfq = serde_json::to_value(rfq).unwrap();
    assert_eq!(rfq["market_ticker"], "TICKER-1");
    assert_eq!(rfq["contracts_fp"], "2.00");
    assert_eq!(rfq["target_cost_dollars"], "1.25");
    assert_eq!(rfq["rest_remainder"], false);
    assert_eq!(rfq["replace_existing"], true);
}
