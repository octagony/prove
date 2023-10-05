use serde::{Deserialize,Serialize};
use std::fs;
use std::process::exit;
use toml;


#[derive(Deserialize,Serialize,Debug)]
pub struct Data {
   pub config: Config,
}

#[derive(Deserialize,Serialize,Debug)]
pub struct Config {
    pub api_key: String,
    pub city: String,
}

impl Data {
    pub fn read_file(filename: &str)->Self {
        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => {
              self::Data::create_file(filename)
            }
        };

        let data: Data = match toml::from_str(&contents) {
            Ok(d) => d,
            Err(_) => {
                eprintln!("Unable to load data from `{}`", filename);
                exit(1);
            }
        };
        data
    }

     fn create_file(filename:&str)->String {
        let config = Self{
            config:Config { api_key: "test_api".to_string(), city: "Minsk".to_string() }
        };
        let config = toml::to_string(&config).unwrap();
        match fs::write(filename, config){
            Ok(_data)=>(),
            Err(_)=>{
                eprintln!("Unable to create config file");
                exit(1);
            }
        }
        let contents = match fs::read_to_string(filename) {
            Ok(c) => c,
            Err(_) => {
               eprintln!("Unable to read file");
               exit(1);
            }
        };
        contents
    }
}

