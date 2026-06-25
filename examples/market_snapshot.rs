mod common;

use kalshi_pmx::Kalshi;
use kalshi_pmx::params::{GetMarketOrderbookParams, GetMarketsParams, GetTradesParams};

#[tokio::main]
async fn main() -> common::ExampleResult {
    let kalshi = Kalshi::demo().build()?;

    let markets = kalshi
        .markets()
        .list(&GetMarketsParams::new().status("open").limit(1))
        .await?;
    let Some(market) = markets.markets.first() else {
        println!("no open demo markets returned");
        return Ok(());
    };

    println!("market: {} | {:?}", market.ticker, market.title);

    let orderbook = kalshi
        .markets()
        .orderbook(&market.ticker, &GetMarketOrderbookParams::new().depth(5))
        .await?;
    println!("orderbook: {:#?}", orderbook.orderbook_fp);

    let trades = kalshi
        .markets()
        .trades(&GetTradesParams::new().ticker(&market.ticker).limit(5))
        .await?;
    println!("recent trades: {:#?}", trades.trades);

    Ok(())
}
