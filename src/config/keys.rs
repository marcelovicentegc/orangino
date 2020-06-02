extern crate dotenv;
use dotenv::dotenv;
use std::env;

pub fn get_keys() -> [String; 3] {
    dotenv().ok();

    #[allow(non_snake_case)]
    let EMPLOYER_CODE = env::var("EMPLOYER_CODE").unwrap();
    #[allow(non_snake_case)]
    let PIN = env::var("PIN").unwrap();
    #[allow(non_snake_case)]
    let TANGERINO_BASIC_TOKEN = env::var("TANGERINO_BASIC_TOKEN").unwrap();

    return [EMPLOYER_CODE, PIN, TANGERINO_BASIC_TOKEN];
}
