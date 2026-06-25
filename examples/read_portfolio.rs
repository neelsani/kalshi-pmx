mod common;

use kalshi_pmx::params::{GetFillsParams, GetPositionsParams};

#[tokio::main]
async fn main() -> common::ExampleResult {
    let kalshi = common::authenticated_client()?;

    let balance = kalshi.portfolio().balance().await?;
    println!("balance: {balance:#?}");

    let positions = kalshi
        .portfolio()
        .positions(&GetPositionsParams::new().limit(10))
        .await?;
    println!("positions: {:#?}", positions.market_positions);

    let fills = kalshi
        .portfolio()
        .fills(&GetFillsParams::new().limit(10))
        .await?;
    println!("fills: {:#?}", fills.fills);

    Ok(())
}
