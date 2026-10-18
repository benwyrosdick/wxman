pub mod geocoding;
pub mod geolocation;
pub mod weather;

pub use geocoding::lookup_zipcode;
pub use geolocation::get_location_from_ip;
pub use weather::fetch_weather;

/// Reuse connections and bound how long a request can block the UI.
pub fn client() -> anyhow::Result<reqwest::Client> {
    Ok(reqwest::Client::builder()
        .user_agent(concat!("wxman/", env!("CARGO_PKG_VERSION")))
        .connect_timeout(std::time::Duration::from_secs(10))
        .timeout(std::time::Duration::from_secs(30))
        .build()?)
}
