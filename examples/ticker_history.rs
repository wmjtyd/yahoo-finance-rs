use std::time::{Duration, UNIX_EPOCH};

use chrono::{DateTime, Utc};
use yahoo_finance_rs::HttpClientBuilder;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    let client = HttpClientBuilder::new().build().unwrap();
    let start = DateTime::parse_from_rfc3339("2022-01-01T00:00:00.00Z")
        .unwrap()
        .with_timezone(&Utc);
    let end = DateTime::parse_from_rfc3339("2022-02-01T00:00:00.00Z")
        .unwrap()
        .with_timezone(&Utc);
    let quote = client
        .get_quote_history("AAPL", start, end, "1d")
        .await
        .unwrap();

    println!("{:?}", quote);
}
