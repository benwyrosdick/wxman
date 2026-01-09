use crate::models::location::{IpApiResponse, Location};
use anyhow::{Context, Result};

const IP_API_URL: &str = "https://ipapi.co/json/";

pub async fn get_location_from_ip() -> Result<Location> {
    let client = reqwest::Client::new();

    let response: IpApiResponse = client
        .get(IP_API_URL)
        .header("User-Agent", "wxman/0.1.0")
        .send()
        .await
        .context("Failed to fetch IP geolocation")?
        .json()
        .await
        .context("Failed to parse IP geolocation response")?;

    Ok(response.into())
}
