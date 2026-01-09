use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Config {
    #[serde(default)]
    pub location: LocationConfig,
    #[serde(default)]
    pub units: UnitsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct LocationConfig {
    pub zipcode: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub city: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UnitsConfig {
    #[serde(default = "default_temperature")]
    pub temperature: TemperatureUnit,
    #[serde(default = "default_wind_speed")]
    pub wind_speed: WindSpeedUnit,
    #[serde(default = "default_precipitation")]
    pub precipitation: PrecipitationUnit,
    #[serde(default = "default_pressure")]
    pub pressure: PressureUnit,
}

impl Default for UnitsConfig {
    fn default() -> Self {
        Self {
            temperature: TemperatureUnit::Fahrenheit,
            wind_speed: WindSpeedUnit::Mph,
            precipitation: PrecipitationUnit::Inch,
            pressure: PressureUnit::InHg,
        }
    }
}

fn default_temperature() -> TemperatureUnit {
    TemperatureUnit::Fahrenheit
}

fn default_wind_speed() -> WindSpeedUnit {
    WindSpeedUnit::Mph
}

fn default_precipitation() -> PrecipitationUnit {
    PrecipitationUnit::Inch
}

fn default_pressure() -> PressureUnit {
    PressureUnit::InHg
}

impl PrecipitationUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Inch => "in",
            Self::Cm => "cm",
        }
    }

    /// Convert from mm (API base unit) to the selected unit
    pub fn convert(&self, mm: f64) -> f64 {
        match self {
            Self::Cm => mm / 10.0,
            Self::Inch => mm / 25.4,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum TemperatureUnit {
    Fahrenheit,
    Celsius,
}

impl TemperatureUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Fahrenheit => "°F",
            Self::Celsius => "°C",
        }
    }

    /// Convert from Celsius (API base unit) to the selected unit
    pub fn convert(&self, celsius: f64) -> f64 {
        match self {
            Self::Celsius => celsius,
            Self::Fahrenheit => celsius * 9.0 / 5.0 + 32.0,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum WindSpeedUnit {
    Mph,
    Kmh,
    Ms,
    Knots,
}

impl WindSpeedUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Mph => "mph",
            Self::Kmh => "km/h",
            Self::Ms => "m/s",
            Self::Knots => "kn",
        }
    }

    /// Convert from km/h (API base unit) to the selected unit
    pub fn convert(&self, kmh: f64) -> f64 {
        match self {
            Self::Kmh => kmh,
            Self::Mph => kmh * 0.621371,
            Self::Ms => kmh / 3.6,
            Self::Knots => kmh * 0.539957,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrecipitationUnit {
    Inch,
    Cm,
}

impl<'de> Deserialize<'de> for PrecipitationUnit {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let s = String::deserialize(deserializer)?;
        match s.to_lowercase().as_str() {
            "inch" | "in" => Ok(PrecipitationUnit::Inch),
            "cm" | "centimeter" | "centimeters" => Ok(PrecipitationUnit::Cm),
            // Backward compatibility: treat mm as cm (since we're changing the unit)
            "mm" | "millimeter" | "millimeters" => Ok(PrecipitationUnit::Cm),
            _ => Err(serde::de::Error::custom(format!(
                "invalid precipitation unit: {}. Expected 'inch', 'cm', or 'mm' (treated as cm)",
                s
            ))),
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PressureUnit {
    #[serde(alias = "hpa")]
    Hpa,
    #[serde(alias = "inhg")]
    InHg,
}

impl PressureUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Hpa => "hPa",
            Self::InHg => "inHg",
        }
    }

    /// Convert from hPa (API base unit) to the selected unit
    pub fn convert(&self, hpa: f64) -> f64 {
        match self {
            Self::Hpa => hpa,
            Self::InHg => hpa * 0.02953,
        }
    }

    /// Format pressure value with appropriate decimal places
    pub fn format(&self, hpa: f64) -> String {
        let value = self.convert(hpa);
        match self {
            Self::Hpa => format!("{:.0}", value),
            Self::InHg => format!("{:.2}", value),
        }
    }
}

impl Config {
    pub fn config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .context("Could not find config directory")?
            .join("wxman");
        Ok(config_dir.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let path = Self::config_path()?;

        if !path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&path)
            .with_context(|| format!("Failed to read config file: {}", path.display()))?;

        let config: Config =
            toml::from_str(&content).with_context(|| "Failed to parse config file")?;

        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;

        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).with_context(|| {
                format!("Failed to create config directory: {}", parent.display())
            })?;
        }

        let content = toml::to_string_pretty(self).context("Failed to serialize config")?;

        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod temperature_unit {
        use super::*;

        #[test]
        fn test_celsius_to_celsius() {
            let unit = TemperatureUnit::Celsius;
            assert_eq!(unit.convert(0.0), 0.0);
            assert_eq!(unit.convert(100.0), 100.0);
            assert_eq!(unit.convert(-40.0), -40.0);
        }

        #[test]
        fn test_celsius_to_fahrenheit() {
            let unit = TemperatureUnit::Fahrenheit;
            // 0°C = 32°F
            assert!((unit.convert(0.0) - 32.0).abs() < 0.001);
            // 100°C = 212°F
            assert!((unit.convert(100.0) - 212.0).abs() < 0.001);
            // -40°C = -40°F (the crossover point)
            assert!((unit.convert(-40.0) - (-40.0)).abs() < 0.001);
            // 20°C = 68°F
            assert!((unit.convert(20.0) - 68.0).abs() < 0.001);
        }

        #[test]
        fn test_symbol() {
            assert_eq!(TemperatureUnit::Celsius.symbol(), "°C");
            assert_eq!(TemperatureUnit::Fahrenheit.symbol(), "°F");
        }
    }

    mod wind_speed_unit {
        use super::*;

        #[test]
        fn test_kmh_to_kmh() {
            let unit = WindSpeedUnit::Kmh;
            assert_eq!(unit.convert(100.0), 100.0);
            assert_eq!(unit.convert(0.0), 0.0);
        }

        #[test]
        fn test_kmh_to_mph() {
            let unit = WindSpeedUnit::Mph;
            // 100 km/h ≈ 62.14 mph
            assert!((unit.convert(100.0) - 62.1371).abs() < 0.001);
            // 1 km/h ≈ 0.621 mph
            assert!((unit.convert(1.0) - 0.621371).abs() < 0.001);
        }

        #[test]
        fn test_kmh_to_ms() {
            let unit = WindSpeedUnit::Ms;
            // 3.6 km/h = 1 m/s
            assert!((unit.convert(3.6) - 1.0).abs() < 0.001);
            // 36 km/h = 10 m/s
            assert!((unit.convert(36.0) - 10.0).abs() < 0.001);
        }

        #[test]
        fn test_kmh_to_knots() {
            let unit = WindSpeedUnit::Knots;
            // 1 km/h ≈ 0.54 knots
            assert!((unit.convert(1.0) - 0.539957).abs() < 0.001);
            // 100 km/h ≈ 54 knots
            assert!((unit.convert(100.0) - 53.9957).abs() < 0.001);
        }

        #[test]
        fn test_symbol() {
            assert_eq!(WindSpeedUnit::Mph.symbol(), "mph");
            assert_eq!(WindSpeedUnit::Kmh.symbol(), "km/h");
            assert_eq!(WindSpeedUnit::Ms.symbol(), "m/s");
            assert_eq!(WindSpeedUnit::Knots.symbol(), "kn");
        }
    }

    mod precipitation_unit {
        use super::*;

        #[test]
        fn test_mm_to_cm() {
            let unit = PrecipitationUnit::Cm;
            // 10 mm = 1 cm
            assert_eq!(unit.convert(10.0), 1.0);
            // 25.4 mm = 2.54 cm
            assert!((unit.convert(25.4) - 2.54).abs() < 0.001);
            assert_eq!(unit.convert(0.0), 0.0);
        }

        #[test]
        fn test_mm_to_inch() {
            let unit = PrecipitationUnit::Inch;
            // 25.4 mm = 1 inch
            assert!((unit.convert(25.4) - 1.0).abs() < 0.001);
            // 50.8 mm = 2 inches
            assert!((unit.convert(50.8) - 2.0).abs() < 0.001);
        }

        #[test]
        fn test_symbol() {
            assert_eq!(PrecipitationUnit::Cm.symbol(), "cm");
            assert_eq!(PrecipitationUnit::Inch.symbol(), "in");
        }
    }

    mod pressure_unit {
        use super::*;

        #[test]
        fn test_hpa_to_hpa() {
            let unit = PressureUnit::Hpa;
            assert_eq!(unit.convert(1013.25), 1013.25);
            assert_eq!(unit.convert(0.0), 0.0);
        }

        #[test]
        fn test_hpa_to_inhg() {
            let unit = PressureUnit::InHg;
            // 1013.25 hPa ≈ 29.92 inHg (standard atmosphere)
            assert!((unit.convert(1013.25) - 29.921).abs() < 0.01);
            // 1000 hPa ≈ 29.53 inHg
            assert!((unit.convert(1000.0) - 29.53).abs() < 0.01);
        }

        #[test]
        fn test_format_hpa() {
            let unit = PressureUnit::Hpa;
            assert_eq!(unit.format(1013.25), "1013");
            assert_eq!(unit.format(1000.0), "1000");
        }

        #[test]
        fn test_format_inhg() {
            let unit = PressureUnit::InHg;
            assert_eq!(unit.format(1013.25), "29.92");
            assert_eq!(unit.format(1000.0), "29.53");
        }

        #[test]
        fn test_symbol() {
            assert_eq!(PressureUnit::Hpa.symbol(), "hPa");
            assert_eq!(PressureUnit::InHg.symbol(), "inHg");
        }
    }

    mod units_config {
        use super::*;

        #[test]
        fn test_default_units() {
            let units = UnitsConfig::default();
            assert_eq!(units.temperature, TemperatureUnit::Fahrenheit);
            assert_eq!(units.wind_speed, WindSpeedUnit::Mph);
            assert_eq!(units.precipitation, PrecipitationUnit::Cm);
            assert_eq!(units.pressure, PressureUnit::InHg);
        }
    }

    mod config_serialization {
        use super::*;

        #[test]
        fn test_deserialize_empty_config() {
            let config: Config = toml::from_str("").unwrap();
            assert_eq!(config.units.temperature, TemperatureUnit::Fahrenheit);
            assert!(config.location.zipcode.is_none());
        }

        #[test]
        fn test_deserialize_partial_config() {
            let toml_str = r#"
                [units]
                temperature = "celsius"
            "#;
            let config: Config = toml::from_str(toml_str).unwrap();
            assert_eq!(config.units.temperature, TemperatureUnit::Celsius);
            // Other units should use defaults
            assert_eq!(config.units.wind_speed, WindSpeedUnit::Mph);
        }

        #[test]
        fn test_deserialize_full_config() {
            let toml_str = r#"
                [location]
                zipcode = "10001"
                latitude = 40.7128
                longitude = -74.0060
                city = "New York"

                [units]
                temperature = "celsius"
                wind_speed = "kmh"
                precipitation = "cm"
                pressure = "hpa"
            "#;
            let config: Config = toml::from_str(toml_str).unwrap();
            assert_eq!(config.location.zipcode, Some("10001".to_string()));
            assert_eq!(config.location.city, Some("New York".to_string()));
            assert_eq!(config.units.temperature, TemperatureUnit::Celsius);
            assert_eq!(config.units.wind_speed, WindSpeedUnit::Kmh);
            assert_eq!(config.units.precipitation, PrecipitationUnit::Cm);
            assert_eq!(config.units.pressure, PressureUnit::Hpa);
        }

        #[test]
        fn test_serialize_config() {
            let config = Config {
                location: LocationConfig {
                    zipcode: Some("90210".to_string()),
                    latitude: Some(34.0901),
                    longitude: Some(-118.4065),
                    city: Some("Beverly Hills".to_string()),
                },
                units: UnitsConfig {
                    temperature: TemperatureUnit::Celsius,
                    wind_speed: WindSpeedUnit::Ms,
                    precipitation: PrecipitationUnit::Cm,
                    pressure: PressureUnit::Hpa,
                },
            };
            let toml_str = toml::to_string(&config).unwrap();
            assert!(toml_str.contains("zipcode = \"90210\""));
            assert!(toml_str.contains("temperature = \"celsius\""));
        }

        #[test]
        fn test_backward_compatibility_mm_to_cm() {
            let toml_str = r#"
                [units]
                precipitation = "mm"
            "#;
            let config: Config = toml::from_str(toml_str).unwrap();
            // "mm" should be converted to "cm" for backward compatibility
            assert_eq!(config.units.precipitation, PrecipitationUnit::Cm);
        }
    }
}
