pub mod schema;
use crate::Data;
use schema::WeatherMap;
use uts2ts::uts2ts;

impl WeatherMap {
    pub async fn get_info() -> Result<WeatherMap, reqwest::Error> {
        let path = Data::check_config_file();
        let prove = path.expect("Unable to load prove.toml");

        let format_url = format!(
            "https://api.openweathermap.org/data/2.5/weather?q={}&APPID={}&units={}",
            prove.config.city, prove.config.api_key, prove.config.units
        );

        let response: WeatherMap = reqwest::Client::new()
            .get(format_url)
            .send()
            .await?
            .json()
            .await?;

        Ok(response)
    }

    pub fn convert_sunrise_and_sunshine(
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
            .expect("Unable to retrieve sunsrise data")
            .to_string();

        (get_sunrise, get_sunset)
    }
    pub fn print_info(data: WeatherMap) {
        //Print Place
        println!("Place: {},{}", data.sys.country, data.name);

        //Print Weather #TODO Implement metric
        println!(
            "Weather: {},{}°",
            data.weather
                .get(0)
                .expect("Unable to retrieve weather data")
                .main,
            data.main.temp
        );

        //Print Wind speed #TODO Implement metric
        println!("Wind speed: {} m/s", data.wind.speed);

        let (sunsrise, sunset) = self::WeatherMap::convert_sunrise_and_sunshine(
            data.sys.sunrise,
            data.sys.sunset,
            data.timezone,
        );

        //Print Sunrise/Sunset
        println!("Sunrise: {}", sunsrise);
        println!("Sunset: {}", sunset);
    }
}
