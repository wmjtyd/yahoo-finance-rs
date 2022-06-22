//! # yahoo! finance API

// http client
mod http_client;
mod utils;
pub use http_client::{
    error::*,
    quote_history::{QuoteHistory, QuoteHistoryPeriod},
    Builder as HttpClientBuilder,
};
// pub use http_client::HttpClient as Http;

// stream client
