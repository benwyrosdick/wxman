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
            80 | 81 | 82 => Self::Rain,
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
            Self::ClearDay => "☀",
            Self::ClearNight => "☾",
            Self::PartlyCloudyDay => "⛅",
            Self::PartlyCloudyNight => "☁",
            Self::Overcast => "☁",
            Self::Fog => "🌫",
            Self::Drizzle => "🌧",
            Self::Rain => "🌧",
            Self::HeavyRain => "🌧",
            Self::Snow => "❄",
            Self::HeavySnow => "❄",
            Self::Thunderstorm => "⛈",
            Self::Unknown => "?",
        }
    }
}

/// Get color for temperature display
pub fn temperature_color(temp: f64, is_fahrenheit: bool) -> Color {
    // Convert to Fahrenheit for consistent thresholds
    let temp_f = if is_fahrenheit {
        temp
    } else {
        temp * 9.0 / 5.0 + 32.0
    };

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
    match degrees {
        0..=22 => "N",
        23..=67 => "NE",
        68..=112 => "E",
        113..=157 => "SE",
        158..=202 => "S",
        203..=247 => "SW",
        248..=292 => "W",
        293..=337 => "NW",
        _ => "N",
    }
}
