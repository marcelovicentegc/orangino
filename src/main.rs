extern crate chrono;
extern crate dotenv;
use chrono::NaiveDateTime;
use console::{style, Emoji};
use dotenv::dotenv;
use pyo3::prelude::*;
use reqwest::header::{HeaderMap, HeaderValue};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::env;

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct PunchResp {
    allowAll: bool,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct Employee {
    id: u32,
    nome: String,
    usuario: User,
    selectedDataInicio: String,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct CheckUserResp {
    status: String,
    funcionario: Employee,
}

#[allow(non_snake_case)]
#[derive(Serialize, Deserialize)]
struct SyncResp {
    tipoRetornoRegistroApontamentoEnum: String,
    mensagem: String,
    statusPonto: i32,
    sucesso: bool,
    excluirPonto: bool,
}

static ASTRONAUT: Emoji<'_, '_> = Emoji("👩‍🚀 ", "");
static PUNCH: Emoji<'_, '_> = Emoji("👊 ", "");
static LINK: Emoji<'_, '_> = Emoji("🔗 ", "");
static TAKING_NOTE: Emoji<'_, '_> = Emoji("📝 ", "");

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    #[allow(non_snake_case)]
    let EMPLOYER_CODE = env::var("EMPLOYER_CODE").unwrap();
    #[allow(non_snake_case)]
    let PIN = env::var("PIN").unwrap();

    let check_user_status_url = format!(
        "https://app.tangerino.com.br/Tangerino/ws/fingerprintWS/funcionario/empregador/{}/pin/{}",
        EMPLOYER_CODE, PIN
    );
    let punch_url = format!(
        "https://app.tangerino.com.br/Tangerino/ws/autorizaDipositivoWS/verifica/web/empregador/{}/pin/{}", 
        EMPLOYER_CODE,
        PIN
    );
    let sync_punch_url =
        "https://app.tangerino.com.br/Tangerino/ws/pontoWS/ponto/sincronizacaoPontos/1.2";
    println!(
        "{} {}Checking user status...",
        style("[1/4]").bold().dim(),
        ASTRONAUT
    );
    let check_user_resp = reqwest::get(&check_user_status_url).await?;

    if check_user_resp.status().is_success() {
        let parsed_check_user_resp: CheckUserResp =
            serde_json::from_str(&check_user_resp.text().await?)?;

        // This means there is a valid user associated with the
        // provided employer and pin codes
        if parsed_check_user_resp.status == "SUCCESS" {
            let client = reqwest::Client::new();
            let mut payload = HashMap::new();
            payload.insert("deviceId", "null");
            println!("Hi there, {}!", parsed_check_user_resp.funcionario.nome);
            println!("{} {}Punching card...", style("[2/4]").bold().dim(), PUNCH);

            let punch_resp = client.post(&punch_url).json(&payload).send().await?;

            if punch_resp.status().is_success() {
                println!("Success on punching the card");

                let parsed_punch_resp: PunchResp = serde_json::from_str(&punch_resp.text().await?)?;

                if parsed_punch_resp.allowAll == true {
                    let mut headers = HeaderMap::new();
                    let employee_id = &parsed_check_user_resp.funcionario.id.to_string();
                    let raw_dt = &parsed_check_user_resp
                        .funcionario
                        .selectedDataInicio
                        .replace("T", " ");
                    println!("raw_dt: {}", raw_dt);
                    let dt = NaiveDateTime::parse_from_str(raw_dt, "%Y-%m-%d %H:%M:%S")?;
                    println!("dt: {}", dt);
                    let punch_in_date = dt.format("%d/%m/%y %H:%M:%S").to_string();
                    let punch_payload = format!("{{\"horaInicio\": \"{}\",\"deviceId\": null,\"online\": \"true\",\"codigoEmpregador\": \"{}\",\"pin\": \"{}\",\"horaFim\": \"\",\"tipo\": \"WEB\",\"foto\": \"\",\"intervalo\": \"\",\"validFingerprint\": false,\"versao\": \"registra-ponto-fingerprint\",\"plataforma\": \"WEB\",\"funcionarioid\": {},\"idAtividade\": 1,\"latitude\": null,\"longitude\": null}}",
                        &punch_in_date, &EMPLOYER_CODE, &PIN, employee_id
                    );

                    headers.insert(
                        reqwest::header::CONTENT_TYPE,
                        HeaderValue::from_str("application/json").unwrap(),
                    );
                    headers.insert("empregador", HeaderValue::from_str(&EMPLOYER_CODE).unwrap());
                    headers.insert(
                        "funcionarioid",
                        HeaderValue::from_str(&employee_id).unwrap(),
                    );
                    headers.insert("username", HeaderValue::from_str(&employee_id).unwrap());
                    headers.insert(
                        "authorization",
                        HeaderValue::from_str(
                            &env::var("TANGERINO_BASIC_TOKEN").unwrap().to_string(),
                        )
                        .unwrap(),
                    );

                    assert!(headers.contains_key("empregador"));
                    assert!(headers.contains_key("funcionarioid"));
                    assert!(headers.contains_key("username"));
                    assert!(headers.contains_key("authorization"));

                    println!(
                        "{} {}Syncing punch record...",
                        style("[3/4]").bold().dim(),
                        LINK
                    );

                    let sync_resp = client
                        .post(sync_punch_url)
                        .headers(headers)
                        .body(punch_payload)
                        .send()
                        .await?;

                    if sync_resp.status().is_success() {
                        let parsed_sync_resp: SyncResp =
                            serde_json::from_str(&sync_resp.text().await?)?;

                        if parsed_sync_resp.sucesso == false {
                            println!(
                                "{}",
                                parsed_sync_resp
                                    .tipoRetornoRegistroApontamentoEnum
                                    .replace("_", " ")
                            );
                            println!("{}", parsed_sync_resp.mensagem);
                        } else {
                            let gil = Python::acquire_gil();
                            let py = gil.python();
                            println!(
                                "{} {}Publishing on Slack...",
                                style("[4/4]").bold().dim(),
                                TAKING_NOTE
                            );
                            publish_to_slack(py).map_err(|e| {
                                // We can't display Python exceptions via std::fmt::Display,
                                // so print the error here manually.
                                e.print_and_set_sys_last_vars(py);
                            });
                        }
                    } else if sync_resp.status().is_server_error() {
                        println!("Failed to syncrhonize the punch record");
                    } else {
                        println!(
                        "Something else happened while trying to synchronize the punch record. Status: {:?}",
                        sync_resp.status()
                    );
                    }
                }
            } else if punch_resp.status().is_server_error() {
                println!("Failed to punch the card");
            } else {
                println!(
                    "Something else happened while trying to punch the card. Status: {:?}",
                    punch_resp.status()
                );
            }
        }
    } else if check_user_resp.status().is_server_error() {
        println!("Failed to get user status");
    } else {
        println!(
            "Something else happened while trying to get user status. Status: {:?}",
            check_user_resp.status()
        );
    }
    Ok(())
}

fn publish_to_slack(py: Python) -> PyResult<()> {
    let slack_client = PyModule::from_code(
        py,
        r#"
import os
from dotenv import load_dotenv
from slack import WebClient
from slack.errors import SlackApiError
            
def publish():
    load_dotenv()
    client = WebClient(token=os.environ['SLACK_API_TOKEN'])   

    try:
        response = client.chat_postMessage(
            channel=os.environ['SLACK_CHANNEL'],
            text=os.environ['GREETING_MESSAGE'])
        assert response["message"]["text"] == os.environ['GREETING_MESSAGE']
        return f"Published: {response['message']['text']}"
    except SlackApiError as error:    
        # You will get a SlackApiError if "ok" is False
        assert error.response["ok"] is False
        assert error.response["error"]  # str like 'invalid_auth', 'channel_not_found'
        return f"Got an error: {error.response['error']}"
    "#,
        "slack_client.py",
        "slack_client",
    )?;

    let publish_result: String = slack_client.call0("publish")?.extract()?;
    println!("{}", publish_result);

    Ok(())
}
