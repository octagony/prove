mod api;
mod config;
mod env;
use api::schema::WeatherMap;
use config::Data;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let path = config::Data::check_config_file();
    let prove = path.expect("Unable to load prove.toml");

    println!("{}:{}", prove.config.api_key, prove.config.city);

    // let format_url = format!(
    //     "https://api.openweathermap.org/data/1.5/weather?q={}&APPID={}",
    //     prove.config.city, prove.config.api_key
    // );
    //
    // let response: WeatherMap = reqwest::Client::new()
    //     .get(format_url)
    //     .send()
    //     .await?
    //     .json()
    //     .await?;
    //
    // println!("{:#?}", response);
    //
    Ok(())
}
