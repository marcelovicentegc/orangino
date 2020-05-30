extern crate dotenv;

use dotenv::dotenv;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::Value;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let check_user_status_url = format!(
        "https://app.tangerino.com.br/Tangerino/ws/fingerprintWS/funcionario/empregador/{}/pin/{}",
        &env::var("EMPLOYER_CODE").unwrap(),
        &env::var("PIN").unwrap()
    );
    let punch_url = format!(
        "https://app.tangerino.com.br/Tangerino/ws/autorizaDipositivoWS/verifica/web/empregador/{}/pin/{}", 
        &env::var("EMPLOYER_CODE").unwrap(),
        &env::var("PIN").unwrap()
    );
    let sync_punch_url =
        "https://app.tangerino.com.br/Tangerino/ws/pontoWS/ponto/sincronizacaoPontos/1.2";
    let check_user_resp = reqwest::get(&check_user_status_url).await?;

    if check_user_resp.status().is_success() {
        println!("Success getting user status");
    } else if check_user_resp.status().is_server_error() {
        println!("Failed to get user status");
    } else {
        println!(
            "Something else happened while trying to get user status. Status: {:?}",
            check_user_resp.status()
        );
    }

    let parsed_check_user_resp: Value = serde_json::from_str(&check_user_resp.text().await?)?;

    // This means there is a valid
    // user associated with the
    // provided employer and pin
    // codes
    if parsed_check_user_resp["status"] == "SUCCESS" {
        let client = reqwest::Client::new();

        let mut map = HashMap::new();
        map.insert("deviceId", "null");

        let punch_resp = client.post(&punch_url).json(&map).send().await?;
        if punch_resp.status().is_success() {
            println!("Success on punching the card");
        } else if punch_resp.status().is_server_error() {
            println!("Failed to punch the card");
        } else {
            println!(
                "Something else happened while trying to punch the card. Status: {:?}",
                punch_resp.status()
            );
        }

        let parsed_punch_resp: Value = serde_json::from_str(&punch_resp.text().await?)?;

        if parsed_punch_resp["allowAll"] == true {
            let mut headers = HeaderMap::new();

            let employer = HeaderValue::from_str(&parsed_check_user_resp["empregador"].to_string());
            let employee =
                HeaderValue::from_str(&parsed_check_user_resp["funcionario"].to_string());
            let username = HeaderValue::from_str(&parsed_check_user_resp["username"].to_string());

            headers.insert("empregador", employer.unwrap());
            headers.insert("funcionario", employee.unwrap());
            headers.insert("username", username.unwrap());
            headers.insert(
                "authorization",
                HeaderValue::from_str(&env::var("TANGERINO_BASIC_TOKEN").unwrap().to_string())
                    .unwrap(),
            );

            assert!(headers.contains_key("empregador"));
            assert!(headers.contains_key("funcionario"));
            assert!(headers.contains_key("username"));

            let sync_resp = client.post(sync_punch_url).headers(headers).send().await?;

            if sync_resp.status().is_success() {
                println!("Success on synchronizing the punch record");
            } else if sync_resp.status().is_server_error() {
                println!("Failed to syncrhonize the pucnh record");
            } else {
                println!(
                    "Something else happened while trying to synchronize the punch record. Status: {:?}",
                    sync_resp.status()
                );
            }
        }
    }
    Ok(())
}
