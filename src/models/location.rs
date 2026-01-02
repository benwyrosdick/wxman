use serde::Deserialize;

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Location {
    pub latitude: f64,
    pub longitude: f64,
    pub city: String,
    pub region: Option<String>,
    pub country: String,
    pub timezone: String,
}

impl Location {
    pub fn display_name(&self) -> String {
        match &self.region {
            Some(region) => format!("{}, {}", self.city, region),
            None => format!("{}, {}", self.city, self.country),
        }
    }
}

/// Response from ipapi.co for IP geolocation
#[derive(Debug, Deserialize)]
pub struct IpApiResponse {
    pub latitude: f64,
    pub longitude: f64,
    pub city: String,
    pub region: Option<String>,
    pub country_name: String,
    pub timezone: String,
}

impl From<IpApiResponse> for Location {
    fn from(resp: IpApiResponse) -> Self {
        Self {
            latitude: resp.latitude,
            longitude: resp.longitude,
            city: resp.city,
            region: resp.region,
            country: resp.country_name,
            timezone: resp.timezone,
        }
    }
}

/// Response from Open-Meteo geocoding API
#[derive(Debug, Deserialize)]
pub struct GeocodingResponse {
    pub results: Option<Vec<GeocodingResult>>,
}

#[derive(Debug, Deserialize)]
pub struct GeocodingResult {
    pub name: String,
    pub latitude: f64,
    pub longitude: f64,
    pub timezone: String,
    pub country: String,
    pub admin1: Option<String>,
}

impl From<GeocodingResult> for Location {
    fn from(result: GeocodingResult) -> Self {
        Self {
            latitude: result.latitude,
            longitude: result.longitude,
            city: result.name,
            region: result.admin1,
            country: result.country,
            timezone: result.timezone,
        }
    }
}
