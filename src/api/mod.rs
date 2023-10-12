pub mod schema;
use crate::Data;
use schema::WeatherMap;

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
}
