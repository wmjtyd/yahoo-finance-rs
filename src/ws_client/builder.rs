use super::WsClient as Client;

pub struct Builder {
    /// websocket endpoint
    pub(crate) url: String,
    /// subscribed qutoes
    pub(crate) quotes: Vec<String>,
}

impl Builder {
    /// create a new builder
    pub fn new() -> Self {
        const DEFAULT_URL: &str = "wss://streamer.finance.yahoo.com";

        Self {
            url: DEFAULT_URL.to_owned(),
            quotes: Vec::new(),
        }
    }

    pub fn url(mut self, url: String) -> Self {
        self.url = url;
        self
    }

    pub fn quotes(mut self, quotes: Vec<String>) -> Self {
        self.quotes = quotes;
        self
    }

    /// add a quote to subscribe
    pub fn add_quote(mut self, quote: String) -> Self {
        self.quotes.push(quote);
        self
    }

    /// build a new client
    pub fn build(self) -> Client {
        Client {
            url: self.url,
            quotes: self.quotes,
        }
    }
}
