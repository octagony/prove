use crate::env::enviroment;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::exit;
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub enum UnitsEnum {
    Metric,
    Imperial,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub config: Config,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub city: String,
    pub units: UnitsEnum,
    pub color: (u8, u8, u8),
}

impl Data {
    //Read config file
    fn read_file(filepath: &str) -> Self {
        let contents = match fs::read_to_string(filepath) {
            Ok(c) => c,
            //If config file does not exist, create it
            Err(_) => self::Data::create_file(filepath),
        };

        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(err) => {
                eprintln!("{:?}", err.message());
                exit(1);
            }
        };
        data
    }

    //Create config file
    fn create_file(filepath: &str) -> String {
        let config = Self {
            config: Config {
                api_key: "test_api".to_string(),
                city: "Minsk".to_string(),
                units: UnitsEnum::Metric,
                color: (0, 255, 136),
            },
        };

        let config = toml::to_string(&config).expect("Unable to read prove.toml file");

        match fs::write(filepath, config) {
            Ok(_data) => (),
            Err(_) => {
                eprintln!("Unable to create config file");
                exit(1);
            }
        }
        let contents = match fs::read_to_string(filepath) {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Unable to read file");
                exit(1);
            }
        };
        contents
    }

    //Check configuration file, if exist -> read it, else -> create it
    pub fn check_config_file() -> std::io::Result<Self> {
        let config_dir = enviroment::read_env();
        let format_path = format!("{}/prove/", config_dir);

        let prove_dir = Path::new(&format_path);

        if !prove_dir.exists() {
            fs::create_dir_all(format!("{}/prove", config_dir))?;
        }

        Ok(Self::read_file(
            prove_dir
                .join("prove.toml")
                .to_str()
                .expect("Unable to retrieve data from prove.toml file"),
        ))
    }
}
