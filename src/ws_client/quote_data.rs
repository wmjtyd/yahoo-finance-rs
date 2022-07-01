use crate::ws_client::abi::PricingData;
use error_stack::{IntoReport, Report, ResultExt};
use prost::Message as _;
use std::str::FromStr;

use super::error::WebsocketStreamError;

pub struct QuoteData {
    /// The symbol for the quote
    pub symbol: String,

    /// The timestamp of the quote in millisecond accuracy
    pub timestamp: i64,

    /// The price of the quote
    pub price: f64,

    /// The volume (daily or transactional) of the symbol
    pub volume: u64,
}

impl FromStr for QuoteData {
    type Err = Report<WebsocketStreamError>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = PricingData::decode(&mut s.as_bytes())
            .report()
            .change_context_lazy(|| WebsocketStreamError::DeserializeMessageFailed)?;

        Ok(QuoteData {
            symbol: data.id.to_string(),
            timestamp: data.time as i64,
            price: data.price as f64,
            volume: data.day_volume as u64,
        })
    }
}
