use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
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
}

impl Default for UnitsConfig {
    fn default() -> Self {
        Self {
            temperature: TemperatureUnit::Fahrenheit,
            wind_speed: WindSpeedUnit::Mph,
            precipitation: PrecipitationUnit::Inch,
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

    pub fn api_value(&self) -> &'static str {
        match self {
            Self::Fahrenheit => "fahrenheit",
            Self::Celsius => "celsius",
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

    pub fn api_value(&self) -> &'static str {
        match self {
            Self::Mph => "mph",
            Self::Kmh => "kmh",
            Self::Ms => "ms",
            Self::Knots => "kn",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum PrecipitationUnit {
    Inch,
    Mm,
}

impl PrecipitationUnit {
    pub fn symbol(&self) -> &'static str {
        match self {
            Self::Inch => "in",
            Self::Mm => "mm",
        }
    }

    pub fn api_value(&self) -> &'static str {
        match self {
            Self::Inch => "inch",
            Self::Mm => "mm",
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            location: LocationConfig::default(),
            units: UnitsConfig::default(),
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
        
        let config: Config = toml::from_str(&content)
            .with_context(|| "Failed to parse config file")?;
        
        Ok(config)
    }

    pub fn save(&self) -> Result<()> {
        let path = Self::config_path()?;
        
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent)
                .with_context(|| format!("Failed to create config directory: {}", parent.display()))?;
        }

        let content = toml::to_string_pretty(self)
            .context("Failed to serialize config")?;
        
        fs::write(&path, content)
            .with_context(|| format!("Failed to write config file: {}", path.display()))?;
        
        Ok(())
    }
}
