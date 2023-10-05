use serde::{Deserialize,Serialize};
mod config;
use config::Data;
mod api;
use api::schema::Schema::WeatherMap;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let data =Data::read_file("prove.toml");

    let format_url = format!("https://api.openweathermap.org/data/2.5/weather?q={}&APPID={}",data.config.city,data.config.api_key);

    let response:WeatherMap = reqwest::Client::new()
    .get(format_url)
    .send()
    .await?
    .json()
    .await?;

    println!("{:#?}",response);

    Ok(())
}
