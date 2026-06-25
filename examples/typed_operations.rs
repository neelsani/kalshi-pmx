mod common;

use kalshi_pmx::Kalshi;
use kalshi_pmx::params::GetMarketsParams;

#[tokio::main]
async fn main() -> common::ExampleResult {
    let kalshi = Kalshi::demo().build()?;
    let typed = kalshi.typed();

    let status = typed.get_exchange_status().await?;
    println!("exchange active: {:?}", status.exchange_active);

    let markets = typed
        .get_markets(&GetMarketsParams::new().status("open").limit(3))
        .await?;
    println!("typed markets response: {markets:#?}");

    Ok(())
}
