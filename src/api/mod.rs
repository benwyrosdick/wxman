pub mod geocoding;
pub mod geolocation;
pub mod weather;

pub use geocoding::lookup_zipcode;
pub use geolocation::get_location_from_ip;
pub use weather::fetch_weather;
