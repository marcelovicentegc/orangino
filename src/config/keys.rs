use serde::{Deserialize, Serialize};
extern crate directories;
use directories::UserDirs;
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub employer_code: String,
    pub pin: String,
    pub tangerino_basic_token: String,
    pub slack_channel: String,
    pub slack_api_token: String,
    pub greetings_message: String,
    pub goodbye_message: String,
}

pub fn get_config() -> Config {
    let path = UserDirs::new()
        .unwrap()
        .home_dir()
        .join(".orangino.toml")
        .into_os_string()
        .into_string()
        .unwrap();
    let raw_config =
        fs::read_to_string(&path).expect("Something went wrong reading the config file");

    let config: Config = toml::from_str(&raw_config).unwrap();

    return config;
}
