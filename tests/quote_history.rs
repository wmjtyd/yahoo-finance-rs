use chrono::{DateTime, Utc};
use wiremock::matchers::{method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};
use yahoo_finance_rs::{HttpClientBuilder, QuoteHistoryPeriod};

#[tokio::test]
async fn happy_path_ok() {
    let mock_server = MockServer::start().await;
    let response_body = r#"
{
	"chart": {
		"result": [
			{
				"meta": {
					"currency": "USD",
					"symbol": "AAPL",
					"exchangeName": "NMS",
					"instrumentType": "EQUITY",
					"firstTradeDate": 345479400,
					"regularMarketTime": 1656100804,
					"gmtoffset": -14400,
					"timezone": "EDT",
					"exchangeTimezoneName": "America/New_York",
					"regularMarketPrice": 141.66,
					"chartPreviousClose": 177.57,
					"priceHint": 2,
					"currentTradingPeriod": {
						"pre": {
							"timezone": "EDT",
							"start": 1656316800,
							"end": 1656336600,
							"gmtoffset": -14400
						},
						"regular": {
							"timezone": "EDT",
							"start": 1656336600,
							"end": 1656360000,
							"gmtoffset": -14400
						},
						"post": {
							"timezone": "EDT",
							"start": 1656360000,
							"end": 1656374400,
							"gmtoffset": -14400
						}
					},
					"dataGranularity": "1d",
					"range": "",
					"validRanges": [
						"1d",
						"5d",
						"1mo",
						"3mo",
						"6mo",
						"1y",
						"2y",
						"5y",
						"10y",
						"ytd",
						"max"
					]
				},
				"timestamp": [
					1641220200,
					1641306600,
					1641393000,
					1641479400,
					1641565800,
					1641825000,
					1641911400,
					1641997800,
					1642084200,
					1642170600,
					1642516200,
					1642602600,
					1642689000,
					1642775400,
					1643034600,
					1643121000,
					1643207400,
					1643293800,
					1643380200,
					1643639400
				],
				"indicators": {
					"quote": [
						{
							"low": [
								177.7100067138672,
								179.1199951171875,
								174.63999938964844,
								171.63999938964844,
								171.02999877929688,
								168.1699981689453,
								170.82000732421875,
								174.82000732421875,
								171.7899932861328,
								171.08999633789062,
								169.41000366210938,
								165.94000244140625,
								164.17999267578125,
								162.3000030517578,
								154.6999969482422,
								157.02000427246094,
								157.82000732421875,
								158.27999877929688,
								162.8000030517578,
								169.50999450683594
							],
							"high": [
								182.8800048828125,
								182.94000244140625,
								180.1699981689453,
								175.3000030517578,
								174.13999938964844,
								172.5,
								175.17999267578125,
								177.17999267578125,
								176.6199951171875,
								173.77999877929688,
								172.5399932861328,
								171.0800018310547,
								169.67999267578125,
								166.3300018310547,
								162.3000030517578,
								162.75999450683594,
								164.38999938964844,
								163.83999633789062,
								170.35000610351562,
								175.0
							],
							"volume": [
								104487900,
								99310400,
								94537600,
								96904000,
								86709100,
								106765600,
								76138300,
								74805200,
								84505800,
								80440800,
								90956700,
								94815000,
								91420500,
								122848900,
								162294600,
								115798400,
								108275300,
								121954600,
								179935700,
								115541600
							],
							"open": [
								177.8300018310547,
								182.6300048828125,
								179.61000061035156,
								172.6999969482422,
								172.88999938964844,
								169.0800018310547,
								172.32000732421875,
								176.1199951171875,
								175.77999877929688,
								171.33999633789062,
								171.50999450683594,
								170.0,
								166.97999572753906,
								164.4199981689453,
								160.02000427246094,
								158.97999572753906,
								163.5,
								162.4499969482422,
								165.7100067138672,
								170.16000366210938
							],
							"close": [
								182.00999450683594,
								179.6999969482422,
								174.9199981689453,
								172.0,
								172.1699981689453,
								172.19000244140625,
								175.0800018310547,
								175.52999877929688,
								172.19000244140625,
								173.07000732421875,
								169.8000030517578,
								166.22999572753906,
								164.50999450683594,
								162.41000366210938,
								161.6199951171875,
								159.77999877929688,
								159.69000244140625,
								159.22000122070312,
								170.3300018310547,
								174.77999877929688
							]
						}
					],
					"adjclose": [
						{
							"adjclose": [
								181.51170349121094,
								179.20803833007812,
								174.4411163330078,
								171.5291290283203,
								171.6986541748047,
								171.71859741210938,
								174.60069274902344,
								175.04945373535156,
								171.71859741210938,
								172.59620666503906,
								169.33514404296875,
								165.77491760253906,
								164.0596160888672,
								161.9653778076172,
								161.17752075195312,
								159.3425750732422,
								159.25282287597656,
								158.78411865234375,
								169.8636932373047,
								174.301513671875
							]
						}
					]
				}
			}
		],
		"error": null
	}
}
    "#;

    Mock::given(method("GET"))
        .and(path("/v8/finance/chart/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_body_raw(response_body, "application/json"))
        .mount(&mock_server)
        .await;

    let client = HttpClientBuilder::new()
        .url(mock_server.uri().clone())
        .build()
        .unwrap();
    let start = DateTime::parse_from_rfc3339("2022-01-01T00:00:00.00Z")
        .unwrap()
        .with_timezone(&Utc);
    let end = DateTime::parse_from_rfc3339("2022-02-01T00:00:00.00Z")
        .unwrap()
        .with_timezone(&Utc);

    let quote_history = client
        .get_quote_history("AAPL", start, end, "1d")
        .await
        .unwrap();

    assert_eq!(quote_history.indicators.len(), 20);
    assert_eq!(
        quote_history.indicators[19].timestamp.date(),
        DateTime::parse_from_rfc3339("2022-01-31T00:00:00.00Z")
            .unwrap()
            .with_timezone(&Utc)
            .date(),
    );
    assert_eq!(quote_history.indicators[19].open, 170.16000366210938);
    assert_eq!(
        quote_history.indicators[19].adj_close.unwrap(),
        174.301513671875
    );
}

#[tokio::test]
async fn timeout_fails() {
    let mock_server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v8/finance/chart/AAPL"))
        .respond_with(ResponseTemplate::new(200).set_delay(std::time::Duration::from_secs(5)))
        .mount(&mock_server)
        .await;

    let client = HttpClientBuilder::new()
        .url(mock_server.uri().clone())
        .timeout(1)
        .build()
        .unwrap();
    let start = DateTime::parse_from_rfc3339("2022-01-01T00:00:00.00Z")
        .unwrap()
        .with_timezone(&Utc);
    let end = DateTime::parse_from_rfc3339("2022-02-01T00:00:00.00Z")
        .unwrap()
        .with_timezone(&Utc);
    let result = client.get_quote_history("AAPL", start, end, "1d").await;

    assert!(result
        .err()
        .unwrap()
        .downcast_ref::<reqwest::Error>()
        .unwrap()
        .to_string()
        .contains("operation timed out"));
}

#[tokio::test]
async fn invalid_quote_fails() {
    let mock_server = MockServer::start().await;
    let response_body = r#"
{
	"chart": {
		"result": null,
		"error": {
			"code": "Not Found",
			"description": "No data found, symbol may be delisted"
		}
	}
}
    "#;

    Mock::given(method("GET"))
        .and(path("/v8/finance/chart/AABBCC"))
        .respond_with(ResponseTemplate::new(404).set_body_raw(response_body, "application/json"))
        .mount(&mock_server)
        .await;

    let client = HttpClientBuilder::new()
        .url(mock_server.uri().clone())
        .build()
        .unwrap();
    let start = DateTime::parse_from_rfc3339("2022-01-01T00:00:00.00Z")
        .unwrap()
        .with_timezone(&Utc);
    let end = DateTime::parse_from_rfc3339("2022-02-01T00:00:00.00Z")
        .unwrap()
        .with_timezone(&Utc);

    let result = client.get_quote_history("AABBCC", start, end, "1d").await;

    assert!(result
        .err()
        .unwrap()
        .downcast_ref::<reqwest::Error>()
        .unwrap()
        .to_string()
        .contains("404 Not Found"));
}
