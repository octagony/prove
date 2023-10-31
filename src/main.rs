mod api;
mod config;
mod env;
use api::schema::WeatherMap;
use config::Data;

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    //Read config file
    let config_file = Data::check_config_file().expect("Unable to read prove.toml file");

    //Get info from server
    let result = WeatherMap::get_info(&config_file).await?;

    //Convert sunrise and sunset
    let (sunrise, sunset) = WeatherMap::convert_sunrise_and_sunset(
        result.sys.sunrise,
        result.sys.sunset,
        result.timezone,
    );

    let (weather_data, temp_data) = (
        result
            .weather
            .get(0)
            .expect("Unable to retrieve weather data")
            .main
            .to_owned(),
        result.main.temp,
    );

    //Get weather info
    let weather_info = WeatherMap::get_weather(weather_data, temp_data, &config_file);

    //Get wind speed info
    let wind_speed_info = WeatherMap::get_wind_info(result.wind.speed, &config_file);

    //Print all info
    WeatherMap::print_info(result, sunrise, sunset, weather_info, wind_speed_info);

    Ok(())
}
