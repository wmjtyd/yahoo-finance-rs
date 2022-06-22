use serde::Serialize;

/// model for quote history request
#[derive(Debug, Serialize)]
pub(super) struct QuoteHistoryRequest<'a> {
    pub(super) period1: i64,
    pub(super) period2: i64,
    pub(super) interval: &'a str,
}
