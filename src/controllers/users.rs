extern crate chrono;
use super::types::{CheckUserResp, PunchResp, SyncResp};
use chrono::NaiveDateTime;
use reqwest::header::{HeaderMap, HeaderValue};
use serde_json::json;
use std::collections::HashMap;
use std::process;

pub async fn check_user(url: String) -> Result<CheckUserResp, Box<dyn std::error::Error>> {
    let resp = reqwest::get(&url).await?;

    if resp.status().is_success() == false {
        println!("Something went wrong");
        process::exit(1);
    } else if resp.status().is_server_error() {
        println!("Failed to get user status");
        process::exit(1);
    }
    let parsed_resp: CheckUserResp = serde_json::from_str(&resp.text().await?)?;

    let first_name: Vec<&str> = parsed_resp.funcionario.nome.split_whitespace().collect();

    // This means there is a valid user associated with the
    // provided employer and pin codes
    if parsed_resp.status == "SUCCESS" {
        println!("Hi there, {}!", first_name[0]);
    } else {
        println!(
            "Sorry, {}, something went wrong",
            parsed_resp.funcionario.nome
        );
        process::exit(1);
    }

    Ok(parsed_resp)
}

pub async fn is_allowed(url: String) -> Result<bool, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut payload = HashMap::new();
    payload.insert("deviceId", "null");

    let resp = client.post(&url).json(&payload).send().await?;

    if resp.status().is_success() {
        let parsed_resp: PunchResp = serde_json::from_str(&resp.text().await?)?;
        if parsed_resp.allowAll == true {
            println!("You are allowed to proceed.");
        }
    } else if resp.status().is_server_error() {
        println!("You are not allowed to proceed");
        process::exit(1);
    }

    Ok(true)
}

pub async fn punch_record(
    url: String,
    parsed_check_user_resp: CheckUserResp,
    employer_code: &String,
    pin: &String,
    tangerino_basic_token: &String,
) -> Result<SyncResp, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let mut headers = HeaderMap::new();
    let employee_id = &parsed_check_user_resp.funcionario.id.to_string();
    let raw_dt = &parsed_check_user_resp
        .funcionario
        .selectedDataInicio
        .replace("T", " ");
    let dt = NaiveDateTime::parse_from_str(raw_dt, "%Y-%m-%d %H:%M:%S")?;
    let punch_in_date = dt.format("%d/%m/%Y %H:%M:%S").to_string();
    let punch_payload = json!({
        "horaInicio": &punch_in_date,
        "deviceId": null,
        "online": "true",
        "codigoEmpregador": employer_code,
        "pin": pin,
        "horaFim": "",
        "tipo": "WEB",
        "foto": "",
        "intervalo": "",
        "validFingerprint": false,
        "versao": "registra-ponto-fingerprint",
        "plataforma": "WEB",
        "funcionarioid": employee_id,
        "idAtividade": 6,
        "latitude": null,
        "longitude": null
    });

    headers.insert(
        reqwest::header::CONTENT_TYPE,
        HeaderValue::from_str("application/json").unwrap(),
    );
    headers.insert("empregador", HeaderValue::from_str(employer_code).unwrap());
    headers.insert(
        "funcionarioid",
        HeaderValue::from_str(&employee_id).unwrap(),
    );
    headers.insert("username", HeaderValue::from_str(&employee_id).unwrap());
    headers.insert(
        "authorization",
        HeaderValue::from_str(tangerino_basic_token).unwrap(),
    );
    headers.insert(
        "origin",
        HeaderValue::from_str("https://app.tangerino.com.br").unwrap(),
    );
    headers.insert(
        "referer", 
        HeaderValue::from_str("https://app.tangerino.com.br/Tangerino/?wicket:interface=wicket-0:2:loginForm:baterPonto::ILinkListener::").unwrap()
    );
    headers.insert("sec-fetch-dest", HeaderValue::from_str("empty").unwrap());
    headers.insert("sec-fetch-mode", HeaderValue::from_str("cors").unwrap());
    headers.insert(
        "sec-fetch-site",
        HeaderValue::from_str("same-origin").unwrap(),
    );
    headers.insert(
        "x-requested-with",
        HeaderValue::from_str("XMLHttpRequest").unwrap(),
    );
    headers.insert(
        "user-agent",
        HeaderValue::from_str(&randua::new().chrome().desktop().to_string()).unwrap(),
    );

    assert!(headers.contains_key("empregador"));
    assert!(headers.contains_key("funcionarioid"));
    assert!(headers.contains_key("username"));
    assert!(headers.contains_key("authorization"));

    let resp = client
        .post(&url)
        .headers(headers)
        .body(punch_payload.to_string())
        .send()
        .await?;

    if resp.status().is_success() == false {
        process::exit(1);
    } else if resp.status().is_server_error() {
        println!("Failed to punch the card");
        process::exit(1);
    }

    let parsed_resp: SyncResp = serde_json::from_str(&resp.text().await?)?;
    if parsed_resp.sucesso == false {
        println!(
            "{}",
            parsed_resp
                .tipoRetornoRegistroApontamentoEnum
                .replace("_", " ")
        );
        process::exit(1);
    }

    if parsed_resp.tipoRetornoRegistroApontamentoEnum == "NOVO_PONTO_ABERTO" {
        println!("Punched in successfully!");
    }

    if parsed_resp.tipoRetornoRegistroApontamentoEnum == "ULTIMO_PONTO_FECHADO_NOVO_ABERTO" {
        println!("Punched out successfully");
    }

    Ok(parsed_resp)
}
