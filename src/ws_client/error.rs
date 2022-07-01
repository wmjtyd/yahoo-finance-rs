use thiserror::Error;

/// Error during connection to websocket server
#[derive(Error, Debug)]
pub enum CreateWebsocketConnectionError {
    #[error("Failed to serialize quotes into valid JSON")]
    SerializeQuoteFailed,

    #[error("Failed to connect to websocket endpoint")]
    ConnectionFailed,

    #[error("Failed to send subscription message")]
    SubscriptionFailed,
}

/// error during streaming data from websocket server
#[derive(Error, Debug)]
pub enum WebsocketStreamError {
    #[error("invalid message received")]
    InvalidMessage,

    #[error("failed to deserialize message")]
    DeserializeMessageFailed,

    #[error("failed to parse binary data to valid UTF-8 string")]
    BinaryDataParseToUtf8Failed,
}
