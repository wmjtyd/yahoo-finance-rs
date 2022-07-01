use super::{
    error::{CreateWebsocketConnectionError, WebsocketStreamError},
    quote_data::QuoteData,
    WsClient,
};
use async_stream::try_stream;
use error_stack::{IntoReport, Report, ResultExt};
use futures::{SinkExt, StreamExt};
use futures_util::Stream;
use std::str::FromStr;
// use prost::Message as _;
use serde::Serialize;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

#[derive(Debug, Clone, Serialize)]
struct Subs<'a> {
    subscribe: Vec<&'a str>,
}

impl WsClient {
    pub async fn stream(
        &self,
    ) -> Result<
        impl Stream<Item = Result<QuoteData, Report<WebsocketStreamError>>>,
        Report<CreateWebsocketConnectionError>,
    > {
        let url = url::Url::parse(&self.url).unwrap();
        let (ws_stream, _) = connect_async(url).await.expect("Failed to connect");

        let message = serde_json::to_string(&Subs {
            subscribe: self.quotes.iter().map(|q| q.as_str()).collect(),
        })
        .report()
        .change_context_lazy(|| CreateWebsocketConnectionError::SerializeQuoteFailed)?;

        let (mut sink, mut source) = ws_stream.split();

        sink.send(Message::Text(message)).await.unwrap();

        Ok(try_stream! {
            while let Some(message) = source.next().await {
                let message = message.report().change_context_lazy(|| {
                    WebsocketStreamError::InvalidMessage
                })?;

                match message {
                            //         match msg.unwrap() {
                    Message::Ping(_) => {
                        sink.send(Message::Pong("pong".as_bytes().to_vec()));
                    }
                    Message::Close(_) => {
                        break;
                    }
                    Message::Text(data) => {
                        let quote_data = QuoteData::from_str(&data)?;
                        yield quote_data;
                    }
                    Message::Binary(data) => {
                        let data = String::from_utf8(data).report().change_context_lazy(|| {
                            WebsocketStreamError::BinaryDataParseToUtf8Failed
                        })?;
                        let quote_data = QuoteData::from_str(&data)?;
                        yield quote_data;
                    }
                    _ => {}
                }
            }
        })
    }
}
