mod api;
mod config;
mod env;
use config::Data;
use uts2ts::{uts2ts, Timestamp};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let result = api::schema::WeatherMap::get_info().await?;
    //Print Place
    println!("Place: {},{}", result.sys.country, result.name);

    //Print Weather #TODO Implement metric
    println!(
        "Weather: {},{}°",
        result.weather.get(0).unwrap().main,
        result.main.temp
    );

    //Print Wind speed #TODO Implement metric
    println!("Wind speed: {} m/s", result.wind.speed);

    //Print Sunrise/Sunset #TODO Implement convertation
    // let sunrise_convert: Timestamp = uts2ts(result.sys.sunrise);
    // let sunset_convert: Timestamp = uts2ts(result.sys.sunset);
    //
    // println!("Sunrise: {:?}", sunrise_convert);
    // println!("Sunset: {:?}", sunset_convert);
    Ok(())
}
