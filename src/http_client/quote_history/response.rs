//! raw models from query1.finance.yahoo.com/v8/finance/chart

use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub(super) struct QuoteHistoryResponse {
    pub chart: QuoteHistoryChart,
}

#[derive(Deserialize, Debug)]
pub(super) struct QuoteHistoryChart {
    pub result: Vec<QuoteBlock>,
    pub error: Option<String>,
}

#[derive(Deserialize, Debug)]
pub(super) struct QuoteBlock {
    pub meta: QuoteMetaData,
    pub timestamp: Vec<i64>,
    // pub events: Option<EventsBlock>,
    pub indicators: QuoteIndicator,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub(super) struct QuoteMetaData {
    pub currency: String,
    pub symbol: String,
    pub exchange_name: String,
    pub instrument_type: String,
    pub first_trade_date: i32,
    pub regular_market_time: u32,
    pub gmtoffset: i32,
    pub timezone: String,
    pub exchange_timezone_name: String,
    pub regular_market_price: f64,
    pub chart_previous_close: f64,
    #[serde(default)]
    pub previous_close: Option<f64>,
    #[serde(default)]
    pub scale: Option<i32>,
    pub price_hint: i32,
    pub current_trading_period: TradingPeriod,
    #[serde(default)]
    pub trading_periods: Option<Vec<Vec<PeriodInfo>>>,
    pub data_granularity: String,
    pub range: String,
    pub valid_ranges: Vec<String>,
}

#[derive(Deserialize, Debug)]
pub(super) struct TradingPeriod {
    pub pre: PeriodInfo,
    pub regular: PeriodInfo,
    pub post: PeriodInfo,
}

#[derive(Deserialize, Debug)]
pub(super) struct PeriodInfo {
    pub timezone: String,
    pub start: u32,
    pub end: u32,
    pub gmtoffset: i32,
}

#[derive(Deserialize, Debug)]
pub(super) struct QuoteIndicator {
    pub quote: Vec<QuoteList>,
    #[serde(default)]
    pub adjclose: Option<Vec<AdjClose>>,
}

#[derive(Deserialize, Debug)]
pub(super) struct QuoteList {
    pub volume: Vec<Option<u64>>,
    pub high: Vec<Option<f64>>,
    pub close: Vec<Option<f64>>,
    pub low: Vec<Option<f64>>,
    pub open: Vec<Option<f64>>,
}

#[derive(Deserialize, Debug)]
pub(super) struct AdjClose {
    pub adjclose: Vec<Option<f64>>,
}
