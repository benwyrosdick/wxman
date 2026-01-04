use ratatui::style::Color;

/// Weather condition based on WMO code
#[derive(Debug, Clone, Copy)]
pub enum WeatherCondition {
    ClearDay,
    ClearNight,
    PartlyCloudyDay,
    PartlyCloudyNight,
    Overcast,
    Fog,
    Drizzle,
    Rain,
    HeavyRain,
    Snow,
    HeavySnow,
    Thunderstorm,
    Unknown,
}

impl WeatherCondition {
    pub fn from_wmo_code(code: i32, is_day: bool) -> Self {
        match code {
            0 => {
                if is_day {
                    Self::ClearDay
                } else {
                    Self::ClearNight
                }
            }
            1 | 2 => {
                if is_day {
                    Self::PartlyCloudyDay
                } else {
                    Self::PartlyCloudyNight
                }
            }
            3 => Self::Overcast,
            45 | 48 => Self::Fog,
            51 | 53 | 55 | 56 | 57 => Self::Drizzle,
            61 | 63 | 66 => Self::Rain,
            65 | 67 => Self::HeavyRain,
            71 | 73 | 77 => Self::Snow,
            75 | 85 | 86 => Self::HeavySnow,
            80..=82 => Self::Rain,
            95 | 96 | 99 => Self::Thunderstorm,
            _ => Self::Unknown,
        }
    }

    pub fn description(&self) -> &'static str {
        match self {
            Self::ClearDay => "Clear",
            Self::ClearNight => "Clear",
            Self::PartlyCloudyDay => "Partly Cloudy",
            Self::PartlyCloudyNight => "Partly Cloudy",
            Self::Overcast => "Overcast",
            Self::Fog => "Foggy",
            Self::Drizzle => "Drizzle",
            Self::Rain => "Rain",
            Self::HeavyRain => "Heavy Rain",
            Self::Snow => "Snow",
            Self::HeavySnow => "Heavy Snow",
            Self::Thunderstorm => "Thunderstorm",
            Self::Unknown => "Unknown",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            Self::ClearDay => Color::Yellow,
            Self::ClearNight => Color::Blue,
            Self::PartlyCloudyDay => Color::LightYellow,
            Self::PartlyCloudyNight => Color::LightBlue,
            Self::Overcast => Color::Gray,
            Self::Fog => Color::DarkGray,
            Self::Drizzle => Color::LightCyan,
            Self::Rain => Color::Cyan,
            Self::HeavyRain => Color::Blue,
            Self::Snow => Color::White,
            Self::HeavySnow => Color::White,
            Self::Thunderstorm => Color::Magenta,
            Self::Unknown => Color::Gray,
        }
    }

    /// Returns ASCII art for the weather condition (5 lines, max 11 chars wide)
    pub fn icon(&self) -> [&'static str; 5] {
        match self {
            Self::ClearDay => [
                "    \\   /   ",
                "     .-.    ",
                "  - (   ) - ",
                "     `-'    ",
                "    /   \\   ",
            ],
            Self::ClearNight => [
                "            ",
                "     .--.   ",
                "    (    )  ",
                "     `--'   ",
                "            ",
            ],
            Self::PartlyCloudyDay => [
                "   \\  /     ",
                " _ /\"\".-.   ",
                "   \\_( _ ). ",
                "   /(___(__)",
                "            ",
            ],
            Self::PartlyCloudyNight => [
                "     .--.   ",
                "  _ (    ). ",
                "  _(___(__) ",
                "            ",
                "            ",
            ],
            Self::Overcast => [
                "            ",
                "    .--.    ",
                " .-(    ).  ",
                "(___.__)__) ",
                "            ",
            ],
            Self::Fog => [
                "            ",
                " _ - _ - _  ",
                "  _ - _ -   ",
                " _ - _ - _  ",
                "            ",
            ],
            Self::Drizzle => [
                "    .-.     ",
                "   (   ).   ",
                "  (___(__) ",
                "   ' ' ' '  ",
                "  ' ' ' '   ",
            ],
            Self::Rain => [
                "    .-.     ",
                "   (   ).   ",
                "  (___(__)  ",
                "  ,',',','  ",
                "  ,',',','  ",
            ],
            Self::HeavyRain => [
                "    .-.     ",
                "   (   ).   ",
                "  (___(__)  ",
                " ,',',',',' ",
                " ,',',',',' ",
            ],
            Self::Snow => [
                "    .-.     ",
                "   (   ).   ",
                "  (___(__)  ",
                "   * * * *  ",
                "  * * * *   ",
            ],
            Self::HeavySnow => [
                "    .-.     ",
                "   (   ).   ",
                "  (___(__)  ",
                "  * * * * * ",
                " * * * * *  ",
            ],
            Self::Thunderstorm => [
                "    .-.     ",
                "   (   ).   ",
                "  (___(__)  ",
                "   ⚡⚡⚡    ",
                "  ,',',','  ",
            ],
            Self::Unknown => [
                "            ",
                "    ???     ",
                "            ",
                "    ???     ",
                "            ",
            ],
        }
    }

    /// Returns a small icon (single character or short string)
    pub fn small_icon(&self) -> &'static str {
        match self {
            Self::ClearDay => "☀️",
            Self::ClearNight => "🌙",
            Self::PartlyCloudyDay => "🌤️",
            Self::PartlyCloudyNight => "☁️",
            Self::Overcast => "☁️",
            Self::Fog => "🌫️",
            Self::Drizzle => "🌧️",
            Self::Rain => "🌧️",
            Self::HeavyRain => "🌧️",
            Self::Snow => "❄️",
            Self::HeavySnow => "❄️",
            Self::Thunderstorm => "🌩️",
            Self::Unknown => "❓",
        }
    }
}

/// Get color for temperature display based on Celsius value.
/// Uses Fahrenheit thresholds internally for consistent color mapping.
pub fn temperature_color_celsius(temp_c: f64) -> Color {
    let temp_f = temp_c * 9.0 / 5.0 + 32.0;
    temperature_color_fahrenheit(temp_f)
}

/// Get color for temperature display based on Fahrenheit value
pub fn temperature_color_fahrenheit(temp_f: f64) -> Color {
    match temp_f as i32 {
        ..=32 => Color::LightBlue,      // Freezing
        33..=50 => Color::Cyan,         // Cold
        51..=65 => Color::Green,        // Cool
        66..=75 => Color::LightGreen,   // Comfortable
        76..=85 => Color::Yellow,       // Warm
        86..=95 => Color::Rgb(255, 165, 0), // Hot (Orange)
        _ => Color::Red,                // Very hot
    }
}

/// Get UV index description and color
pub fn uv_info(uv_index: f64) -> (&'static str, Color) {
    match uv_index as i32 {
        0..=2 => ("Low", Color::Green),
        3..=5 => ("Moderate", Color::Yellow),
        6..=7 => ("High", Color::Rgb(255, 165, 0)),
        8..=10 => ("Very High", Color::Red),
        _ => ("Extreme", Color::Magenta),
    }
}

/// Convert wind direction degrees to cardinal direction
pub fn wind_direction_str(degrees: i32) -> &'static str {
    // Normalize to 0-359 range
    let normalized = degrees.rem_euclid(360);
    match normalized {
        0..=22 => "N",
        23..=67 => "NE",
        68..=112 => "E",
        113..=157 => "SE",
        158..=202 => "S",
        203..=247 => "SW",
        248..=292 => "W",
        293..=337 => "NW",
        338..=359 => "N",
        _ => "N", // Should never reach here after normalization
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod weather_condition {
        use super::*;

        #[test]
        fn test_clear_day_night() {
            assert!(matches!(
                WeatherCondition::from_wmo_code(0, true),
                WeatherCondition::ClearDay
            ));
            assert!(matches!(
                WeatherCondition::from_wmo_code(0, false),
                WeatherCondition::ClearNight
            ));
        }

        #[test]
        fn test_partly_cloudy() {
            assert!(matches!(
                WeatherCondition::from_wmo_code(1, true),
                WeatherCondition::PartlyCloudyDay
            ));
            assert!(matches!(
                WeatherCondition::from_wmo_code(2, false),
                WeatherCondition::PartlyCloudyNight
            ));
        }

        #[test]
        fn test_overcast() {
            assert!(matches!(
                WeatherCondition::from_wmo_code(3, true),
                WeatherCondition::Overcast
            ));
        }

        #[test]
        fn test_fog() {
            assert!(matches!(
                WeatherCondition::from_wmo_code(45, true),
                WeatherCondition::Fog
            ));
            assert!(matches!(
                WeatherCondition::from_wmo_code(48, true),
                WeatherCondition::Fog
            ));
        }

        #[test]
        fn test_rain_codes() {
            // Drizzle codes
            for code in [51, 53, 55, 56, 57] {
                assert!(matches!(
                    WeatherCondition::from_wmo_code(code, true),
                    WeatherCondition::Drizzle
                ));
            }
            // Rain codes
            for code in [61, 63, 66, 80, 81, 82] {
                assert!(matches!(
                    WeatherCondition::from_wmo_code(code, true),
                    WeatherCondition::Rain
                ));
            }
            // Heavy rain codes
            for code in [65, 67] {
                assert!(matches!(
                    WeatherCondition::from_wmo_code(code, true),
                    WeatherCondition::HeavyRain
                ));
            }
        }

        #[test]
        fn test_snow_codes() {
            for code in [71, 73, 77] {
                assert!(matches!(
                    WeatherCondition::from_wmo_code(code, true),
                    WeatherCondition::Snow
                ));
            }
            for code in [75, 85, 86] {
                assert!(matches!(
                    WeatherCondition::from_wmo_code(code, true),
                    WeatherCondition::HeavySnow
                ));
            }
        }

        #[test]
        fn test_thunderstorm() {
            for code in [95, 96, 99] {
                assert!(matches!(
                    WeatherCondition::from_wmo_code(code, true),
                    WeatherCondition::Thunderstorm
                ));
            }
        }

        #[test]
        fn test_unknown_code() {
            assert!(matches!(
                WeatherCondition::from_wmo_code(999, true),
                WeatherCondition::Unknown
            ));
        }

        #[test]
        fn test_description_not_empty() {
            let conditions = [
                WeatherCondition::ClearDay,
                WeatherCondition::ClearNight,
                WeatherCondition::Rain,
                WeatherCondition::Snow,
                WeatherCondition::Thunderstorm,
            ];
            for condition in conditions {
                assert!(!condition.description().is_empty());
            }
        }

        #[test]
        fn test_icon_dimensions() {
            let conditions = [
                WeatherCondition::ClearDay,
                WeatherCondition::Rain,
                WeatherCondition::Snow,
            ];
            for condition in conditions {
                let icon = condition.icon();
                assert_eq!(icon.len(), 5, "Icon should have 5 lines");
            }
        }
    }

    mod temperature_color {
        use super::*;

        #[test]
        fn test_freezing() {
            // 0°C = 32°F (freezing)
            assert_eq!(temperature_color_celsius(0.0), Color::LightBlue);
            // -10°C = 14°F (freezing)
            assert_eq!(temperature_color_celsius(-10.0), Color::LightBlue);
        }

        #[test]
        fn test_cold() {
            // 5°C = 41°F (cold)
            assert_eq!(temperature_color_celsius(5.0), Color::Cyan);
        }

        #[test]
        fn test_cool() {
            // 15°C = 59°F (cool)
            assert_eq!(temperature_color_celsius(15.0), Color::Green);
        }

        #[test]
        fn test_comfortable() {
            // 22°C ≈ 72°F (comfortable)
            assert_eq!(temperature_color_celsius(22.0), Color::LightGreen);
        }

        #[test]
        fn test_warm() {
            // 27°C ≈ 80°F (warm)
            assert_eq!(temperature_color_celsius(27.0), Color::Yellow);
        }

        #[test]
        fn test_hot() {
            // 32°C ≈ 90°F (hot)
            assert_eq!(temperature_color_celsius(32.0), Color::Rgb(255, 165, 0));
        }

        #[test]
        fn test_very_hot() {
            // 38°C ≈ 100°F (very hot)
            assert_eq!(temperature_color_celsius(38.0), Color::Red);
        }

        #[test]
        fn test_fahrenheit_function() {
            assert_eq!(temperature_color_fahrenheit(32.0), Color::LightBlue);
            assert_eq!(temperature_color_fahrenheit(70.0), Color::LightGreen);
            assert_eq!(temperature_color_fahrenheit(100.0), Color::Red);
        }
    }

    mod uv_info_tests {
        use super::*;

        #[test]
        fn test_low_uv() {
            let (desc, color) = uv_info(1.0);
            assert_eq!(desc, "Low");
            assert_eq!(color, Color::Green);
        }

        #[test]
        fn test_moderate_uv() {
            let (desc, color) = uv_info(4.0);
            assert_eq!(desc, "Moderate");
            assert_eq!(color, Color::Yellow);
        }

        #[test]
        fn test_high_uv() {
            let (desc, color) = uv_info(7.0);
            assert_eq!(desc, "High");
            assert_eq!(color, Color::Rgb(255, 165, 0));
        }

        #[test]
        fn test_very_high_uv() {
            let (desc, color) = uv_info(9.0);
            assert_eq!(desc, "Very High");
            assert_eq!(color, Color::Red);
        }

        #[test]
        fn test_extreme_uv() {
            let (desc, color) = uv_info(12.0);
            assert_eq!(desc, "Extreme");
            assert_eq!(color, Color::Magenta);
        }
    }

    mod wind_direction {
        use super::*;

        #[test]
        fn test_cardinal_directions() {
            assert_eq!(wind_direction_str(0), "N");
            assert_eq!(wind_direction_str(45), "NE");
            assert_eq!(wind_direction_str(90), "E");
            assert_eq!(wind_direction_str(135), "SE");
            assert_eq!(wind_direction_str(180), "S");
            assert_eq!(wind_direction_str(225), "SW");
            assert_eq!(wind_direction_str(270), "W");
            assert_eq!(wind_direction_str(315), "NW");
        }

        #[test]
        fn test_boundary_values() {
            assert_eq!(wind_direction_str(22), "N");
            assert_eq!(wind_direction_str(23), "NE");
            assert_eq!(wind_direction_str(337), "NW");
            assert_eq!(wind_direction_str(338), "N");
        }

        #[test]
        fn test_full_rotation() {
            assert_eq!(wind_direction_str(360), "N");
            assert_eq!(wind_direction_str(359), "N");
        }

        #[test]
        fn test_negative_degrees() {
            // -90 degrees should be same as 270 degrees (West)
            assert_eq!(wind_direction_str(-90), "W");
            // -45 degrees should be same as 315 degrees (NW)
            assert_eq!(wind_direction_str(-45), "NW");
        }

        #[test]
        fn test_large_values() {
            // 450 degrees = 90 degrees (East)
            assert_eq!(wind_direction_str(450), "E");
            // 720 degrees = 0 degrees (North)
            assert_eq!(wind_direction_str(720), "N");
        }
    }
}
