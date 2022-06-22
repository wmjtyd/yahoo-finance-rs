use chrono::Duration;
use error_stack::Report;

use super::{CreateHttpClientError, HttpClient};

pub struct Builder {
    url: String,
    timeout: Duration,
}

impl Builder {
    /// builder for http client
    pub fn new() -> Self {
        const DEFAULT_URL: &str = "https://query1.finance.yahoo.com";
        let default_timeout = Duration::seconds(30);

        Self {
            url: DEFAULT_URL.to_string(),
            timeout: default_timeout,
        }
    }

    /// yahoo endpoint url
    pub fn url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    /// timeout for requests in seconds
    pub fn timeout(mut self, seconds: usize) -> Self {
        self.timeout = Duration::seconds(seconds as i64);
        self
    }

    /// build http client
    pub fn build(self) -> Result<HttpClient, Report<CreateHttpClientError>> {
        let client = HttpClient::new(self.url, self.timeout)?;
        Ok(client)
    }
}
