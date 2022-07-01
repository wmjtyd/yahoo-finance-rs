use futures::{pin_mut, StreamExt};

#[tokio::main]
async fn main() {
    let client = yahoo_finance_rs::WsClientBuilder::new()
        .add_quote("AAPL".to_owned())
        .build();
    let stream = client.stream().await.unwrap();
    pin_mut!(stream);

    while let Some(quote_data) = stream.next().await {
        let quote_data = quote_data.unwrap();
        println!(
            "At {}, {} is trading for ${} [{}]",
            quote_data.timestamp, quote_data.symbol, quote_data.price, quote_data.volume
        );
    }
}
