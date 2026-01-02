use anyhow::Result;
use chrono::{DateTime, Local};

use crate::api;
use crate::config::{Config, PrecipitationUnit, PressureUnit, TemperatureUnit, WindSpeedUnit};
use crate::models::{Location, WeatherData};
use crate::ui::hourly::get_max_hourly_scroll;

pub enum AppState {
    Loading,
    Ready,
    Error(String),
}

#[derive(Clone, Copy, PartialEq)]
pub enum UnitMenuField {
    Temperature,
    WindSpeed,
    Precipitation,
    Pressure,
}

pub struct App {
    pub config: Config,
    pub state: AppState,
    pub location: Option<Location>,
    pub weather: Option<WeatherData>,
    pub last_updated: Option<DateTime<Local>>,
    pub hourly_scroll: usize,
    pub show_help: bool,
    pub show_units_menu: bool,
    pub units_menu_selection: UnitMenuField,
    pub units_changed: bool,
    pub should_quit: bool,
}

impl App {
    pub fn new(config: Config) -> Self {
        Self {
            config,
            state: AppState::Loading,
            location: None,
            weather: None,
            last_updated: None,
            hourly_scroll: 0,
            show_help: false,
            show_units_menu: false,
            units_menu_selection: UnitMenuField::Temperature,
            units_changed: false,
            should_quit: false,
        }
    }

    pub async fn load_weather(&mut self) -> Result<()> {
        self.state = AppState::Loading;

        // Get location
        let location = self.get_location().await?;
        self.location = Some(location.clone());

        // Fetch weather
        let weather = api::fetch_weather(
            location.latitude,
            location.longitude,
            &self.config.units,
        )
        .await?;

        self.weather = Some(weather);
        self.last_updated = Some(Local::now());
        self.hourly_scroll = 0;
        self.state = AppState::Ready;

        Ok(())
    }

    async fn get_location(&self) -> Result<Location> {
        // Check if zipcode is configured
        if let Some(zipcode) = &self.config.location.zipcode {
            return api::lookup_zipcode(zipcode).await;
        }

        // Check if coordinates are configured
        if let (Some(lat), Some(lon)) = (
            self.config.location.latitude,
            self.config.location.longitude,
        ) {
            return Ok(Location {
                latitude: lat,
                longitude: lon,
                city: self.config.location.city.clone().unwrap_or_else(|| "Unknown".to_string()),
                region: None,
                country: "".to_string(),
                timezone: "auto".to_string(),
            });
        }

        // Fall back to IP geolocation
        api::get_location_from_ip().await
    }

    pub fn toggle_units_menu(&mut self) {
        self.show_units_menu = !self.show_units_menu;
        if !self.show_units_menu && self.units_changed {
            // Save config when closing menu if units changed
            let _ = self.config.save();
        }
    }

    pub fn units_menu_up(&mut self) {
        self.units_menu_selection = match self.units_menu_selection {
            UnitMenuField::Temperature => UnitMenuField::Pressure,
            UnitMenuField::WindSpeed => UnitMenuField::Temperature,
            UnitMenuField::Precipitation => UnitMenuField::WindSpeed,
            UnitMenuField::Pressure => UnitMenuField::Precipitation,
        };
    }

    pub fn units_menu_down(&mut self) {
        self.units_menu_selection = match self.units_menu_selection {
            UnitMenuField::Temperature => UnitMenuField::WindSpeed,
            UnitMenuField::WindSpeed => UnitMenuField::Precipitation,
            UnitMenuField::Precipitation => UnitMenuField::Pressure,
            UnitMenuField::Pressure => UnitMenuField::Temperature,
        };
    }

    pub fn units_menu_toggle_selected(&mut self) {
        self.units_changed = true;
        match self.units_menu_selection {
            UnitMenuField::Temperature => {
                self.config.units.temperature = match self.config.units.temperature {
                    TemperatureUnit::Fahrenheit => TemperatureUnit::Celsius,
                    TemperatureUnit::Celsius => TemperatureUnit::Fahrenheit,
                };
            }
            UnitMenuField::WindSpeed => {
                self.config.units.wind_speed = match self.config.units.wind_speed {
                    WindSpeedUnit::Mph => WindSpeedUnit::Kmh,
                    WindSpeedUnit::Kmh => WindSpeedUnit::Ms,
                    WindSpeedUnit::Ms => WindSpeedUnit::Knots,
                    WindSpeedUnit::Knots => WindSpeedUnit::Mph,
                };
            }
            UnitMenuField::Precipitation => {
                self.config.units.precipitation = match self.config.units.precipitation {
                    PrecipitationUnit::Inch => PrecipitationUnit::Mm,
                    PrecipitationUnit::Mm => PrecipitationUnit::Inch,
                };
            }
            UnitMenuField::Pressure => {
                self.config.units.pressure = match self.config.units.pressure {
                    PressureUnit::Hpa => PressureUnit::InHg,
                    PressureUnit::InHg => PressureUnit::Hpa,
                };
            }
        }
    }

    pub fn close_units_menu(&mut self) -> bool {
        if self.show_units_menu {
            self.show_units_menu = false;
            if self.units_changed {
                let _ = self.config.save();
                self.units_changed = false;
                return true; // Signal that we need to reload weather
            }
        }
        false
    }

    pub fn scroll_hourly_up(&mut self) {
        if self.hourly_scroll > 0 {
            self.hourly_scroll -= 1;
        }
    }

    pub fn scroll_hourly_down(&mut self) {
        if let Some(weather) = &self.weather {
            // Approximate visible height (will be adjusted by actual render area)
            let max_scroll = get_max_hourly_scroll(&weather.hourly, 12);
            if self.hourly_scroll < max_scroll {
                self.hourly_scroll += 1;
            }
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
    }

    pub fn set_error(&mut self, message: String) {
        self.state = AppState::Error(message);
    }
}
