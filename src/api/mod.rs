pub mod schema;
use crate::config::{Data, UnitsEnum};
use colored::Colorize;
use schema::WeatherMap;
use uts2ts::uts2ts;

impl WeatherMap {
    // Get info from server
    pub async fn get_info(config_file: &Data) -> Result<WeatherMap, reqwest::Error> {
        let format_url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&APPID={}&units={:?}",
            config_file.config.city, config_file.config.api_key, config_file.config.units
        );

        let response: WeatherMap = reqwest::Client::new()
            .get(format_url)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    //Convert sunrise and sunset values
    pub fn convert_sunrise_and_sunset(
        sunrise: i64,
        sunset: i64,
        timezone: i64,
    ) -> (String, String) {
        // Print Sunrise/Sunset #TODO Implement convertation
        let sunrise_convert = &uts2ts(sunrise + (timezone)).as_string();
        let sunset_convert = &uts2ts(sunset + (timezone)).as_string();

        let sunrise_convert: Vec<&str> = sunrise_convert.split(" ").collect();
        let sunset_convert: Vec<&str> = sunset_convert.split(" ").collect();

        let get_sunrise = sunrise_convert
            .get(1)
            .expect("Unable to retrieve sunrise data")
            .to_string();
        let get_sunset = sunset_convert
            .get(1)
            .expect("Unable to retrieve sunset data")
            .to_string();

        (get_sunrise, get_sunset)
    }

    // Convert weather info depending on config
    pub fn get_weather(weather: &String, temp: f64, config_file: &Data) -> String {
        match config_file.config.units {
            UnitsEnum::Metric => return format!("{weather}, {temp}°C"),
            UnitsEnum::Imperial => return format!("{weather}, {temp}°F"),
        };
    }

    // Convert wind info depending on config
    pub fn get_wind_info(wind_speed: f64, config_file: &Data) -> String {
        match config_file.config.units {
            UnitsEnum::Metric => return format!("{} m/s", wind_speed),
            UnitsEnum::Imperial => return format!("{} m/h", wind_speed),
        }
    }

    //Print all info
    pub fn print_info(
        data: WeatherMap,
        sunrise: String,
        sunset: String,
        weather_info: String,
        wind_speed: String,
        config_file: &Data,
    ) {
        //Print Place
        let (country, name) = (data.sys.country, data.name);
        let (r, g, b) = (
            config_file.config.color.0,
            config_file.config.color.1,
            config_file.config.color.2,
        );

        //Print Weather
        println!("{}: {name}, {country}", "City".truecolor(r, g, b));

        //Print Weather
        println!("{}: {weather_info}", "Weather".truecolor(r, g, b));

        //Print Wind speed
        println!("{}: {wind_speed}", "Wind speed".truecolor(r, g, b));

        //Print Sunrise/Sunset
        println!("{}: {sunrise}", "Sunrise".truecolor(r, g, b));
        println!("{}: {sunset}", "Sunset".truecolor(r, g, b));
    }
}
