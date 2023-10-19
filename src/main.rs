mod api;
mod config;
mod env;
use api::schema::WeatherMap;
use config::Data;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let result = WeatherMap::get_info().await?;
    WeatherMap::print_info(result);
    Ok(())
}
