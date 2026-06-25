mod common;

use kalshi_pmx::ws::{LiveOrderbook, StreamMessage, Subscription};

#[tokio::main]
async fn main() -> common::ExampleResult {
    let kalshi = common::authenticated_client()?;
    let ticker = common::required_env(&["KALSHI_TICKER"])?;

    let mut stream = kalshi.stream().connect().await?;
    stream
        .subscribe(
            Subscription::orderbook()
                .markets([ticker])
                .use_yes_price(true),
        )
        .await?;

    let mut book = LiveOrderbook::new();
    while let Some(message) = stream.next().await {
        let message = message?;
        if book.apply_message(&message)? {
            println!(
                "yes={:?} no={:?} seq={:?}",
                book.best_yes_bid(),
                book.best_no_bid(),
                book.last_seq
            );
        } else if let StreamMessage::Error(error) = message {
            eprintln!("stream error: {error:#?}");
        }
    }

    Ok(())
}
