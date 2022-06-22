//! models for quote history

mod request;
mod response;

use self::request::QuoteHistoryRequest;
use self::response::QuoteHistoryResponse;

use super::{error::HttpRequestError, HttpClient};
use chrono::{TimeZone, Utc};
use error_stack::{ensure, report, Report};

#[derive(Debug, Clone)]
pub struct QuoteHistory {
    pub indicators: Vec<QuoteHistoryPeriod>,
}

#[derive(Debug, Clone)]
pub struct QuoteHistoryPeriod {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub volume: u64,
    pub high: f64,
    pub close: f64,
    pub low: f64,
    pub open: f64,
    pub adj_close: Option<f64>,
}

macro_rules! ensure_length {
    ($field:expr, $target:expr, $expr:literal) => {
        ensure!(
            $field.len() == $target,
            report!(HttpRequestError::InvalidDataSet).attach_printable(format!(
                "length of {} does not match with length of timestamp",
                $expr
            ))
        );
    };
}

impl TryFrom<QuoteHistoryResponse> for QuoteHistory {
    type Error = Report<HttpRequestError>;

    fn try_from(response: QuoteHistoryResponse) -> Result<Self, Self::Error> {
        let quote_block = response.chart.result.get(0).ok_or_else(|| {
            report!(HttpRequestError::InvalidDataSet).attach_printable("No quote block")
        })?;

        let quote = quote_block.indicators.quote.get(0).ok_or_else(|| {
            report!(HttpRequestError::InvalidDataSet).attach_printable("No quote data in block")
        })?;
        let period_count = quote_block.timestamp.len();
        ensure_length!(quote.volume, period_count, "quote.volume");
        ensure_length!(quote.high, period_count, "quote.high");
        ensure_length!(quote.close, period_count, "quote.close");
        ensure_length!(quote.low, period_count, "quote.low");
        ensure_length!(quote.open, period_count, "quote.open");

        let adj_close_list = if let Some(ref adj_close_wrapper) = quote_block.indicators.adjclose {
            let adj_close = adj_close_wrapper.get(0).ok_or_else(|| {
                report!(HttpRequestError::InvalidDataSet)
                    .attach_printable("The adj_close field appears without any data")
            })?;

            Some(&adj_close.adjclose)
        } else {
            None
        };

        let mut indicators: Vec<QuoteHistoryPeriod> = Vec::with_capacity(period_count);
        for i in 0..period_count {
            let timestamp = Utc.timestamp(quote_block.timestamp[i], 0);

            match (
                quote.volume[i],
                quote.high[i],
                quote.close[i],
                quote.low[i],
                quote.open[i],
            ) {
                (Some(volume), Some(high), Some(close), Some(low), Some(open)) => {
                    let adj_close = if let Some(adj_close_list) = adj_close_list {
                        adj_close_list.get(i).ok_or_else(|| {
                            report!(HttpRequestError::InvalidDataSet)
                                .attach_printable("The `adj_close` is `None` in index {i}")
                        })?
                    } else {
                        &None
                    }
                    .to_owned();

                    indicators.push(QuoteHistoryPeriod {
                        timestamp,
                        volume,
                        high,
                        close,
                        low,
                        open,
                        adj_close,
                    });
                }
                (None, None, None, None, None) => {
                    // ignore this period
                }
                _ => {
                    return Err(report!(HttpRequestError::InvalidDataSet).attach_printable(
                        "Some of the quote data is null while others are not in index {i}",
                    ))
                }
            }
        }
        Ok(QuoteHistory { indicators })
    }
}

impl HttpClient {
    pub async fn get_quote_history<T: TimeZone>(
        self,
        ticker: &str,
        start: chrono::DateTime<T>,
        end: chrono::DateTime<T>,
        interval: &str,
    ) -> Result<QuoteHistory, Report<HttpRequestError>> {
        let param = QuoteHistoryRequest {
            period1: start.timestamp(),
            period2: end.timestamp(),
            interval,
        };

        let quote_history = self
            .transport
            .get::<QuoteHistoryResponse, _>(&format!("/v8/finance/chart/{ticker}"), Some(&param))
            .await?;

        quote_history.try_into()
    }
}
