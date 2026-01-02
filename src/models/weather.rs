use serde::Deserialize;

/// Complete weather data from Open-Meteo API
#[derive(Debug, Clone)]
pub struct WeatherData {
    pub current: CurrentWeather,
    pub hourly: Vec<HourlyForecast>,
    pub daily: Vec<DailyForecast>,
}

#[derive(Debug, Clone)]
pub struct CurrentWeather {
    pub temperature: f64,
    pub apparent_temperature: f64,
    pub humidity: i32,
    pub weather_code: i32,
    pub wind_speed: f64,
    pub wind_direction: i32,
    pub wind_gusts: f64,
    pub cloud_cover: i32,
    pub pressure: f64,
    pub precipitation: f64,
    pub uv_index: f64,
    pub is_day: bool,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct HourlyForecast {
    pub time: String,
    pub temperature: f64,
    pub apparent_temperature: f64,
    pub precipitation_probability: i32,
    pub weather_code: i32,
    pub wind_speed: f64,
}

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct DailyForecast {
    pub date: String,
    pub weather_code: i32,
    pub temp_max: f64,
    pub temp_min: f64,
    pub apparent_temp_max: f64,
    pub apparent_temp_min: f64,
    pub sunrise: String,
    pub sunset: String,
    pub precipitation_sum: f64,
    pub precipitation_probability: i32,
    pub wind_speed_max: f64,
    pub uv_index_max: f64,
}

/// Raw API response from Open-Meteo
#[derive(Debug, Deserialize)]
pub struct OpenMeteoResponse {
    pub current: OpenMeteoCurrent,
    pub hourly: OpenMeteoHourly,
    pub daily: OpenMeteoDaily,
}

#[derive(Debug, Deserialize)]
pub struct OpenMeteoCurrent {
    pub temperature_2m: f64,
    pub relative_humidity_2m: i32,
    pub apparent_temperature: f64,
    pub precipitation: f64,
    pub weather_code: i32,
    pub wind_speed_10m: f64,
    pub wind_direction_10m: i32,
    pub wind_gusts_10m: f64,
    pub cloud_cover: i32,
    pub pressure_msl: f64,
    pub uv_index: f64,
    pub is_day: i32,
}

#[derive(Debug, Deserialize)]
pub struct OpenMeteoHourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub apparent_temperature: Vec<f64>,
    pub precipitation_probability: Vec<i32>,
    pub weather_code: Vec<i32>,
    pub wind_speed_10m: Vec<f64>,
}

#[derive(Debug, Deserialize)]
pub struct OpenMeteoDaily {
    pub time: Vec<String>,
    pub weather_code: Vec<i32>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
    pub apparent_temperature_max: Vec<f64>,
    pub apparent_temperature_min: Vec<f64>,
    pub sunrise: Vec<String>,
    pub sunset: Vec<String>,
    pub precipitation_sum: Vec<f64>,
    pub precipitation_probability_max: Vec<i32>,
    pub wind_speed_10m_max: Vec<f64>,
    pub uv_index_max: Vec<f64>,
}

impl From<OpenMeteoResponse> for WeatherData {
    fn from(resp: OpenMeteoResponse) -> Self {
        let current = CurrentWeather {
            temperature: resp.current.temperature_2m,
            apparent_temperature: resp.current.apparent_temperature,
            humidity: resp.current.relative_humidity_2m,
            weather_code: resp.current.weather_code,
            wind_speed: resp.current.wind_speed_10m,
            wind_direction: resp.current.wind_direction_10m,
            wind_gusts: resp.current.wind_gusts_10m,
            cloud_cover: resp.current.cloud_cover,
            pressure: resp.current.pressure_msl,
            precipitation: resp.current.precipitation,
            uv_index: resp.current.uv_index,
            is_day: resp.current.is_day == 1,
        };

        let hourly: Vec<HourlyForecast> = resp
            .hourly
            .time
            .iter()
            .enumerate()
            .map(|(i, time)| HourlyForecast {
                time: time.clone(),
                temperature: resp.hourly.temperature_2m[i],
                apparent_temperature: resp.hourly.apparent_temperature[i],
                precipitation_probability: resp.hourly.precipitation_probability[i],
                weather_code: resp.hourly.weather_code[i],
                wind_speed: resp.hourly.wind_speed_10m[i],
            })
            .collect();

        let daily: Vec<DailyForecast> = resp
            .daily
            .time
            .iter()
            .enumerate()
            .map(|(i, date)| DailyForecast {
                date: date.clone(),
                weather_code: resp.daily.weather_code[i],
                temp_max: resp.daily.temperature_2m_max[i],
                temp_min: resp.daily.temperature_2m_min[i],
                apparent_temp_max: resp.daily.apparent_temperature_max[i],
                apparent_temp_min: resp.daily.apparent_temperature_min[i],
                sunrise: resp.daily.sunrise[i].clone(),
                sunset: resp.daily.sunset[i].clone(),
                precipitation_sum: resp.daily.precipitation_sum[i],
                precipitation_probability: resp.daily.precipitation_probability_max[i],
                wind_speed_max: resp.daily.wind_speed_10m_max[i],
                uv_index_max: resp.daily.uv_index_max[i],
            })
            .collect();

        Self {
            current,
            hourly,
            daily,
        }
    }
}
