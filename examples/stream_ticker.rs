mod common;

use kalshi_pmx::ws::Subscription;

#[tokio::main]
async fn main() -> common::ExampleResult {
    let kalshi = common::authenticated_client()?;
    let ticker = common::required_env(&["KALSHI_TICKER"])?;

    let mut stream = kalshi.stream().connect().await?;
    stream
        .subscribe(Subscription::ticker().markets([ticker]))
        .await?;

    while let Some(message) = stream.next().await {
        println!("{:#?}", message?);
    }

    Ok(())
}
