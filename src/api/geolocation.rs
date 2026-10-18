use crate::models::location::{IpApiResponse, Location};
use anyhow::{Context, Result};

const IP_API_URL: &str = "https://ipapi.co/json/";

pub async fn get_location_from_ip(client: &reqwest::Client) -> Result<Location> {
    let response: IpApiResponse = client
        .get(IP_API_URL)
        .send()
        .await
        .context("Failed to fetch IP geolocation")?
        .error_for_status()
        .context("IP geolocation service returned an error")?
        .json()
        .await
        .context("Failed to parse IP geolocation response")?;

    Ok(response.into())
}
