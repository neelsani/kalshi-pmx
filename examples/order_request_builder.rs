use kalshi_pmx::generated::{BookSide, CreateOrderV2Request, SelfTradePreventionType};
use kalshi_pmx::models::TimeInForce;

fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let order = CreateOrderV2Request::new(
        "KXEXAMPLE-26JUN24-B100",
        BookSide::Bid,
        "1.0000",
        "0.0500",
        TimeInForce::GoodTillCanceled,
        SelfTradePreventionType::TakerAtCross,
    )
    .client_order_id("example-client-order-id")
    .post_only(true)
    .cancel_order_on_pause(true);

    println!("{}", serde_json::to_string_pretty(&order)?);
    Ok(())
}
