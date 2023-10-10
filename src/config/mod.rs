use crate::env::enviroment;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::exit;
use toml;

#[derive(Deserialize, Serialize, Debug)]
pub struct Data {
    pub config: Config,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct Config {
    pub api_key: String,
    pub city: String,
}

impl Data {
    pub fn read_file(filepath: &str) -> Self {
        let contents = match fs::read_to_string(filepath) {
            Ok(c) => c,
            Err(_) => self::Data::create_file(filepath),
        };

        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from `{}`", filepath);
                exit(1);
            }
        };
        println!("{}", data.config.api_key);
        data
    }

    fn create_file(filepath: &str) -> String {
        let config = Self {
            config: Config {
                api_key: "test_api".to_string(),
                city: "Minsk".to_string(),
            },
        };
        let config = toml::to_string(&config).unwrap();
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
    pub fn check_config_file() {
        let config_dir = enviroment::read_env();

        let prove_dir = format!("{}/prove", config_dir);
        let prove_file = format!("{}/prove/prove.toml", config_dir);

        let check_path = Path::new(&prove_dir);

        let is_exist = check_path.exists();

        if is_exist {
            Self::read_file(&prove_file);
        } else {
            Self::create_file(&prove_file);
        }
    }
}
