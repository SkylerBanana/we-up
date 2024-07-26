use reqwest::{Client, Error};
use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct ForecastPeriod {
    pub startTime: String,
    pub temperature: Option<i64>,
    pub shortForecast: String,
}

#[derive(Debug, Deserialize)]
pub struct ApiResponse {
    pub properties: Properties,
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    pub periods: Vec<ForecastPeriod>,
}

pub async fn api_test() -> Result<Vec<ForecastPeriod>, reqwest::Error> {
    let url = "https://api.weather.gov/gridpoints/TOP/32,81/forecast/hourly";
    let client = Client::new();

    let response = client
        .get(url)
        .header("User-Agent", "rust api test")
        .send()
        .await?;

    if response.status().is_success() {
        let api_response: ApiResponse = response.json().await?;
        Ok(api_response.properties.periods)
    } else {
        Err(response.error_for_status().unwrap_err())
    }
}
