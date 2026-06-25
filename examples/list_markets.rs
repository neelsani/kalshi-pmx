mod common;

use kalshi_pmx::Kalshi;
use kalshi_pmx::params::GetMarketsParams;

#[tokio::main]
async fn main() -> common::ExampleResult {
    let kalshi = Kalshi::demo().build()?;
    let markets = kalshi
        .markets()
        .list(&GetMarketsParams::new().status("open").limit(10))
        .await?;

    for market in markets.markets {
        println!(
            "{} | {:?} | yes bid {} ask {}",
            market.ticker, market.title, market.yes_bid_dollars, market.yes_ask_dollars
        );
    }

    Ok(())
}
