use crate::models::weather::{OpenMeteoResponse, WeatherData};
use anyhow::{Context, Result};

const WEATHER_API_URL: &str = "https://api.open-meteo.com/v1/forecast";

/// Fetches weather data from Open-Meteo API.
/// Always requests metric units (Celsius, km/h, mm) so conversions can be done
/// client-side for live unit switching without re-fetching.
pub async fn fetch_weather(
    client: &reqwest::Client,
    latitude: f64,
    longitude: f64,
) -> Result<WeatherData> {
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
        "precipitation",
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

    let response: OpenMeteoResponse = client
        .get(WEATHER_API_URL)
        .query(&[
            ("latitude", latitude.to_string()),
            ("longitude", longitude.to_string()),
            ("current", current_params),
            ("hourly", hourly_params),
            ("daily", daily_params),
            ("temperature_unit", "celsius".into()),
            ("wind_speed_unit", "kmh".into()),
            ("precipitation_unit", "mm".into()),
            ("timezone", "auto".into()),
            ("forecast_days", "5".into()),
        ])
        .send()
        .await
        .context("Failed to fetch weather data")?
        .error_for_status()
        .context("Weather service returned an error")?
        .json()
        .await
        .context("Failed to parse weather response")?;

    response.try_into()
}
