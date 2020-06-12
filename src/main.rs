mod config;
mod controllers;
mod utils;
use config::keys::get_config;
use console::style;
use controllers::slack::filter_and_publish;
use controllers::users::{check_user, is_allowed, punch_record};
use utils::emojis::{ASTRONAUT, LOCK, PUNCH, TAKING_NOTE};

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let config = get_config();
    let employer_code = &config.employer_code;
    let pin = &config.pin;
    let tangerino_basic_token = &config.tangerino_basic_token;

    let check_user_status_url = format!(
        "https://app.tangerino.com.br/Tangerino/ws/fingerprintWS/funcionario/empregador/{}/pin/{}",
        employer_code, pin
    );
    let is_allowed_url = format!(
        "https://app.tangerino.com.br/Tangerino/ws/autorizaDipositivoWS/verifica/web/empregador/{}/pin/{}", 
        employer_code,
        pin
    );
    let punch_record_url =
        "https://app.tangerino.com.br/Tangerino/ws/pontoWS/ponto/sincronizacaoPontos/1.2";

    println!(
        "{} {}Checking user status...",
        style("[1/4]").bold().dim(),
        ASTRONAUT
    );
    let parsed_check_user_resp = check_user(check_user_status_url).await.unwrap();

    println!(
        "{} {}Checking user's permissions...",
        style("[2/4]").bold().dim(),
        LOCK
    );
    is_allowed(is_allowed_url).await.unwrap();

    println!("{} {}Punching card...", style("[3/4]").bold().dim(), PUNCH);
    let parsed_punch_record_resp = punch_record(
        punch_record_url.to_string(),
        parsed_check_user_resp,
        &employer_code,
        &pin,
        &tangerino_basic_token,
    )
    .await
    .unwrap();

    println!(
        "{} {}Publishing to Slack...",
        style("[4/4]").bold().dim(),
        TAKING_NOTE
    );
    filter_and_publish(
        parsed_punch_record_resp,
        config.slack_api_token,
        config.slack_channel,
        config.greetings_message,
        config.goodbye_message,
    )
    .await
    .unwrap();

    Ok(())
}

fn main() {
    start();
}
