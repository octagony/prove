use serde::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct Data {
    config: Config,
}

#[derive(Deserialize)]
pub struct Config {
    ip: String,
    port: u16,
}

impl Data {
    pub fn read_file(filename: &str) {
        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => {
                // IMPLEMENT CREATE_FILE FUNCTION
                eprintln!("Could not read file `{}`", filename);
                exit(1);
            }
        };

        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from `{}`", filename);
                exit(1);
            }
        };

        println!("{}", data.config.ip);
        println!("{}", data.config.port);
    }

    // IMPLEMENT CREATE_FILE FUNCTION
    pub fn create_file() {}
}

