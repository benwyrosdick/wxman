use anyhow::{Context, Result};
use crate::models::weather::{OpenMeteoResponse, WeatherData};

const WEATHER_API_URL: &str = "https://api.open-meteo.com/v1/forecast";

/// Fetches weather data from Open-Meteo API.
/// Always requests metric units (Celsius, km/h, mm) so conversions can be done
/// client-side for live unit switching without re-fetching.
pub async fn fetch_weather(
    latitude: f64,
    longitude: f64,
) -> Result<WeatherData> {
    let client = reqwest::Client::new();

    let current_params = [
        "temperature_2m",
        "relative_humidity_2m",
        "apparent_temperature",
        "precipitation",
        "weather_code",
        "wind_speed_10m",
        "wind_direction_10m",
        "wind_gusts_10m",
        "cloud_cover",
        "pressure_msl",
        "uv_index",
        "is_day",
    ]
    .join(",");

    let hourly_params = [
        "temperature_2m",
        "apparent_temperature",
        "precipitation_probability",
        "weather_code",
        "wind_speed_10m",
    ]
    .join(",");

    let daily_params = [
        "weather_code",
        "temperature_2m_max",
        "temperature_2m_min",
        "apparent_temperature_max",
        "apparent_temperature_min",
        "sunrise",
        "sunset",
        "precipitation_sum",
        "precipitation_probability_max",
        "wind_speed_10m_max",
        "uv_index_max",
    ]
    .join(",");

    // Always request metric units: Celsius, km/h, mm
    // Conversion to user's preferred units is done at display time
    let url = format!(
        "{}?latitude={}&longitude={}&current={}&hourly={}&daily={}&temperature_unit=celsius&wind_speed_unit=kmh&precipitation_unit=mm&timezone=auto&forecast_days=5",
        WEATHER_API_URL,
        latitude,
        longitude,
        current_params,
        hourly_params,
        daily_params,
    );

    let response: OpenMeteoResponse = client
        .get(&url)
        .send()
        .await
        .context("Failed to fetch weather data")?
        .json()
        .await
        .context("Failed to parse weather response")?;

    Ok(response.into())
}
