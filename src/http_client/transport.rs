use chrono::Duration;
use error_stack::{report, IntoReport, Report, ResultExt};
use reqwest::Response;
use serde::{de::DeserializeOwned, Serialize};

use crate::{CreateHttpClientError, HttpRequestError};

/// Inner transport module for HTTP client
pub(super) struct Transport {
    base_url: String,
    client: reqwest::Client,
}

impl Transport {
    pub(super) fn new(
        url: String,
        timeout: Duration,
    ) -> Result<Self, Report<CreateHttpClientError>> {
        let client = reqwest::ClientBuilder::new()
            .timeout(
                timeout
                    .to_std()
                    .report()
                    .change_context(CreateHttpClientError)?,
            )
            .build()
            .report()
            .change_context(CreateHttpClientError)?;

        Ok(Self {
            base_url: url.to_owned(),
            client,
        })
    }

    pub(super) async fn get<O, S>(
        &self,
        path: &str,
        query: Option<&S>,
    ) -> Result<O, Report<HttpRequestError>>
    where
        O: DeserializeOwned,
        S: Serialize + ?Sized,
    {
        let url = format!("{}{}", self.base_url, path);
        let mut request = self.client.get(url);
        if let Some(query) = query {
            request = request.query(query);
        }
        let response = request
            .send()
            .await
            .report()
            .change_context(HttpRequestError::ConnectionFailed)?;

        self.handle_response(response).await
    }

    async fn handle_response<O>(&self, response: Response) -> Result<O, Report<HttpRequestError>>
    where
        O: DeserializeOwned,
    {
        match response.error_for_status() {
            Ok(response) => Ok(response
                .json::<O>()
                .await
                .report()
                .change_context(HttpRequestError::DeserializeFailed)?),
            Err(err) => Err(report!(err).change_context(HttpRequestError::FetchFailed)),
        }
    }
}
