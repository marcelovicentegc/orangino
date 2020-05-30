extern crate dotenv;

use dotenv::dotenv;
use serde_json::Value;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let tangerino_url = format!(
        "https://app.tangerino.com.br/Tangerino/ws/fingerprintWS/funcionario/empregador/{}/pin/{}",
        &env::var("EMPLOYER_CODE").unwrap(),
        &env::var("PIN").unwrap()
    );

    let resp = reqwest::get(&tangerino_url).await?;

    if resp.status().is_success() {
        println!("success!");
    } else if resp.status().is_server_error() {
        println!("server error!");
    } else {
        println!("Something else happened. Status: {:?}", resp.status());
    }

    let parsed_resp: Value = serde_json::from_str(&resp.text().await?)?;

    println!("{:#?}", parsed_resp["status"]);
    Ok(())
}
