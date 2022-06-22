use thiserror::Error;

/// Error type for creating HttpClient
#[derive(Error, Debug)]
#[error("Could not create HTTP client")]
pub struct CreateHttpClientError;

/// error type for http requests
#[derive(Error, Debug)]
pub enum HttpRequestError {
    /// Failed to connect to API endpoint
    #[error("Failed to connect to API endpoint")]
    ConnectionFailed,

    /// Received an response with status code other than 200
    #[error("Received an response with status code other than 200")]
    FetchFailed,

    /// Failed to parse response body
    #[error("Failed to parse response body")]
    DeserializeFailed,

    /// Invalid dataset value
    #[error("Invalid dataset value")]
    InvalidDataSet,

    /// `error` field appears in response
    #[error("`error` field appears in response")]
    CustomError,
}
