mod builder;
pub mod error;
pub mod quote_history;
mod transport;

use chrono::Duration;
use error_stack::Report;

use self::error::CreateHttpClientError;
use self::transport::Transport;

pub use self::builder::Builder;

pub struct HttpClient {
    transport: Transport,
}

impl HttpClient {
    fn new(url: String, timeout: Duration) -> Result<HttpClient, Report<CreateHttpClientError>> {
        Ok(Self {
            transport: Transport::new(url, timeout)?,
        })
    }
}
