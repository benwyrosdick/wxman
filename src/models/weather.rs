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
#[derive(Debug, Clone, Deserialize)]
pub struct OpenMeteoResponse {
    pub current: OpenMeteoCurrent,
    pub hourly: OpenMeteoHourly,
    pub daily: OpenMeteoDaily,
}

#[derive(Debug, Clone, Deserialize)]
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

#[derive(Debug, Clone, Deserialize)]
pub struct OpenMeteoHourly {
    pub time: Vec<String>,
    pub temperature_2m: Vec<f64>,
    pub apparent_temperature: Vec<f64>,
    pub precipitation_probability: Vec<i32>,
    pub weather_code: Vec<i32>,
    pub wind_speed_10m: Vec<f64>,
}

#[derive(Debug, Clone, Deserialize)]
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

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_response() -> OpenMeteoResponse {
        OpenMeteoResponse {
            current: OpenMeteoCurrent {
                temperature_2m: 20.5,
                relative_humidity_2m: 65,
                apparent_temperature: 19.0,
                precipitation: 0.5,
                weather_code: 3,
                wind_speed_10m: 15.0,
                wind_direction_10m: 180,
                wind_gusts_10m: 25.0,
                cloud_cover: 75,
                pressure_msl: 1013.25,
                uv_index: 5.0,
                is_day: 1,
            },
            hourly: OpenMeteoHourly {
                time: vec![
                    "2024-01-01T00:00".to_string(),
                    "2024-01-01T01:00".to_string(),
                    "2024-01-01T02:00".to_string(),
                ],
                temperature_2m: vec![18.0, 17.5, 17.0],
                apparent_temperature: vec![16.0, 15.5, 15.0],
                precipitation_probability: vec![10, 20, 30],
                weather_code: vec![0, 1, 2],
                wind_speed_10m: vec![10.0, 12.0, 14.0],
            },
            daily: OpenMeteoDaily {
                time: vec!["2024-01-01".to_string(), "2024-01-02".to_string()],
                weather_code: vec![3, 61],
                temperature_2m_max: vec![22.0, 20.0],
                temperature_2m_min: vec![15.0, 12.0],
                apparent_temperature_max: vec![21.0, 19.0],
                apparent_temperature_min: vec![14.0, 11.0],
                sunrise: vec!["2024-01-01T07:00".to_string(), "2024-01-02T07:01".to_string()],
                sunset: vec!["2024-01-01T17:00".to_string(), "2024-01-02T17:01".to_string()],
                precipitation_sum: vec![0.0, 5.5],
                precipitation_probability_max: vec![10, 80],
                wind_speed_10m_max: vec![20.0, 35.0],
                uv_index_max: vec![4.0, 2.0],
            },
        }
    }

    #[test]
    fn test_current_weather_conversion() {
        let response = create_test_response();
        let weather_data: WeatherData = response.into();

        assert_eq!(weather_data.current.temperature, 20.5);
        assert_eq!(weather_data.current.humidity, 65);
        assert_eq!(weather_data.current.apparent_temperature, 19.0);
        assert_eq!(weather_data.current.weather_code, 3);
        assert_eq!(weather_data.current.wind_speed, 15.0);
        assert_eq!(weather_data.current.wind_direction, 180);
        assert_eq!(weather_data.current.wind_gusts, 25.0);
        assert_eq!(weather_data.current.cloud_cover, 75);
        assert_eq!(weather_data.current.pressure, 1013.25);
        assert_eq!(weather_data.current.precipitation, 0.5);
        assert_eq!(weather_data.current.uv_index, 5.0);
        assert!(weather_data.current.is_day);
    }

    #[test]
    fn test_is_day_conversion() {
        let mut response = create_test_response();
        
        // Test is_day = 1 (true)
        response.current.is_day = 1;
        let weather_data: WeatherData = response.clone().into();
        assert!(weather_data.current.is_day);

        // Test is_day = 0 (false)
        response.current.is_day = 0;
        let weather_data: WeatherData = response.into();
        assert!(!weather_data.current.is_day);
    }

    #[test]
    fn test_hourly_forecast_conversion() {
        let response = create_test_response();
        let weather_data: WeatherData = response.into();

        assert_eq!(weather_data.hourly.len(), 3);

        let first_hour = &weather_data.hourly[0];
        assert_eq!(first_hour.time, "2024-01-01T00:00");
        assert_eq!(first_hour.temperature, 18.0);
        assert_eq!(first_hour.apparent_temperature, 16.0);
        assert_eq!(first_hour.precipitation_probability, 10);
        assert_eq!(first_hour.weather_code, 0);
        assert_eq!(first_hour.wind_speed, 10.0);

        let last_hour = &weather_data.hourly[2];
        assert_eq!(last_hour.time, "2024-01-01T02:00");
        assert_eq!(last_hour.temperature, 17.0);
        assert_eq!(last_hour.precipitation_probability, 30);
    }

    #[test]
    fn test_daily_forecast_conversion() {
        let response = create_test_response();
        let weather_data: WeatherData = response.into();

        assert_eq!(weather_data.daily.len(), 2);

        let first_day = &weather_data.daily[0];
        assert_eq!(first_day.date, "2024-01-01");
        assert_eq!(first_day.weather_code, 3);
        assert_eq!(first_day.temp_max, 22.0);
        assert_eq!(first_day.temp_min, 15.0);
        assert_eq!(first_day.apparent_temp_max, 21.0);
        assert_eq!(first_day.apparent_temp_min, 14.0);
        assert_eq!(first_day.sunrise, "2024-01-01T07:00");
        assert_eq!(first_day.sunset, "2024-01-01T17:00");
        assert_eq!(first_day.precipitation_sum, 0.0);
        assert_eq!(first_day.precipitation_probability, 10);
        assert_eq!(first_day.wind_speed_max, 20.0);
        assert_eq!(first_day.uv_index_max, 4.0);

        let second_day = &weather_data.daily[1];
        assert_eq!(second_day.date, "2024-01-02");
        assert_eq!(second_day.weather_code, 61);
        assert_eq!(second_day.precipitation_probability, 80);
    }

    #[test]
    fn test_empty_hourly_data() {
        let response = OpenMeteoResponse {
            current: create_test_response().current,
            hourly: OpenMeteoHourly {
                time: vec![],
                temperature_2m: vec![],
                apparent_temperature: vec![],
                precipitation_probability: vec![],
                weather_code: vec![],
                wind_speed_10m: vec![],
            },
            daily: create_test_response().daily,
        };
        let weather_data: WeatherData = response.into();
        assert!(weather_data.hourly.is_empty());
    }

    #[test]
    fn test_empty_daily_data() {
        let response = OpenMeteoResponse {
            current: create_test_response().current,
            hourly: create_test_response().hourly,
            daily: OpenMeteoDaily {
                time: vec![],
                weather_code: vec![],
                temperature_2m_max: vec![],
                temperature_2m_min: vec![],
                apparent_temperature_max: vec![],
                apparent_temperature_min: vec![],
                sunrise: vec![],
                sunset: vec![],
                precipitation_sum: vec![],
                precipitation_probability_max: vec![],
                wind_speed_10m_max: vec![],
                uv_index_max: vec![],
            },
        };
        let weather_data: WeatherData = response.into();
        assert!(weather_data.daily.is_empty());
    }
}
