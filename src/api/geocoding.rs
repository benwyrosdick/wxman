use crate::models::location::{GeocodingResponse, Location};
use anyhow::{anyhow, Context, Result};

const GEOCODING_API_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";

pub async fn lookup_zipcode(zipcode: &str) -> Result<Location> {
    let client = reqwest::Client::new();

    let url = format!(
        "{}?name={}&count=1&language=en&format=json",
        GEOCODING_API_URL, zipcode
    );

    let response: GeocodingResponse = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch geocoding data")?
        .json()
        .await
        .context("Failed to parse geocoding response")?;

    response
        .results
        .and_then(|mut results| results.pop())
        .map(|r| r.into())
        .ok_or_else(|| anyhow!("No location found for zipcode: {}", zipcode))
}
