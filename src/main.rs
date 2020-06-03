mod config;
mod controllers;
use config::emojis::{ASTRONAUT, LINK, PUNCH, TAKING_NOTE};
use config::keys::get_keys;
use console::style;
use controllers::slack::filter_and_publish;
use controllers::users::{check_user, is_allowed, punch_record};

#[tokio::main]
async fn start() -> Result<(), Box<dyn std::error::Error>> {
    let keys = get_keys();
    let employer_code = &keys[0];
    let pin = &keys[1];
    let tangerino_basic_token = &keys[2];

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

    println!("{} {}Punching card...", style("[2/4]").bold().dim(), PUNCH);
    is_allowed(is_allowed_url).await.unwrap();

    println!(
        "{} {}Syncing punch record...",
        style("[3/4]").bold().dim(),
        LINK
    );
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
        "{} {}Publishing on Slack...",
        style("[4/4]").bold().dim(),
        TAKING_NOTE
    );
    filter_and_publish(parsed_punch_record_resp).await.unwrap();

    Ok(())
}

fn main() {
    #[allow(unused_must_use)]
    start();
}
