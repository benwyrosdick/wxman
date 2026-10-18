use crate::models::location::{GeocodingResponse, Location};
use anyhow::{anyhow, Context, Result};

const GEOCODING_API_URL: &str = "https://geocoding-api.open-meteo.com/v1/search";

pub async fn lookup_zipcode(client: &reqwest::Client, zipcode: &str) -> Result<Location> {
    let response: GeocodingResponse = client
        .get(GEOCODING_API_URL)
        .query(&[
            ("name", zipcode),
            ("count", "1"),
            ("language", "en"),
            ("format", "json"),
        ])
        .send()
        .await
        .context("Failed to fetch geocoding data")?
        .error_for_status()
        .context("Geocoding service returned an error")?
        .json()
        .await
        .context("Failed to parse geocoding response")?;

    response
        .results
        .and_then(|mut results| results.pop())
        .map(|r| r.into())
        .ok_or_else(|| anyhow!("No location found for: {}", zipcode))
}
