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
    let EMPLOYER_CODE = keys[0];
    let PIN = keys[1];
    let TANGERINO_BASIC_TOKEN = keys[3];

    let check_user_status_url = format!(
        "https://app.tangerino.com.br/Tangerino/ws/fingerprintWS/funcionario/empregador/{}/pin/{}",
        EMPLOYER_CODE, PIN
    );
    let is_allowed_url = format!(
        "https://app.tangerino.com.br/Tangerino/ws/autorizaDipositivoWS/verifica/web/empregador/{}/pin/{}", 
        EMPLOYER_CODE,
        PIN
    );
    let punch_record_url =
        "https://app.tangerino.com.br/Tangerino/ws/pontoWS/ponto/sincronizacaoPontos/1.2";

    println!(
        "{} {}Checking user status...",
        style("[1/4]").bold().dim(),
        ASTRONAUT
    );
    check_user(check_user_status_url);

    println!("{} {}Punching card...", style("[2/4]").bold().dim(), PUNCH);
    is_allowed(is_allowed_url);

    println!(
        "{} {}Syncing punch record...",
        style("[3/4]").bold().dim(),
        LINK
    );
    punch_record(
        punch_record_url.to_string(),
        parsed_check_user_resp: CheckUserResp,
        &EMPLOYER_CODE,
        &PIN,
        &TANGERINO_BASIC_TOKEN,
    );

    println!(
        "{} {}Publishing on Slack...",
        style("[4/4]").bold().dim(),
        TAKING_NOTE
    );
    filter_and_publish();

    Ok(())
}

fn main() {
    start();
}
