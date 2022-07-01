pub(crate) mod abi;
pub(crate) mod builder;
mod error;
pub mod quote_data;
mod stream;

pub struct WsClient {
    url: String,
    quotes: Vec<String>,
}
