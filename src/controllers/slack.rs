use super::types::SyncResp;
use pyo3::prelude::*;

pub async fn filter_and_publish(
    parsed_sync_resp: SyncResp,
    slack_api_token: String,
    slack_channel: String,
    greeting_message: String,
    goodbye_message: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    publish_to_slack(
        py,
        parsed_sync_resp.tipoRetornoRegistroApontamentoEnum == "NOVO_PONTO_ABERTO",
        slack_api_token,
        slack_channel,
        greeting_message,
        goodbye_message,
    )
    .map_err(|e| {
        e.print_and_set_sys_last_vars(py);
    });

    Ok(())
}

fn publish_to_slack(
    py: Python,
    greet: bool,
    slack_api_token: String,
    slack_channel: String,
    greeting_message: String,
    goodbye_message: String,
) -> PyResult<()> {
    let slack_client = PyModule::from_code(
        py,
        r#"
import os
from slack import WebClient
from slack.errors import SlackApiError
            
def publish(greet, slack_api_token, slack_channel, greeting_message, goodbye_message):
    client = WebClient(token=slack_api_token)   

    if greet:
        message = greeting_message
    else:
        message = goodbye_message

    try:
        response = client.chat_postMessage(
            channel=slack_channel,
            text=message)
        assert response["message"]["text"] == message
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

    let publish_result: String = slack_client
        .call1(
            "publish",
            (
                greet,
                slack_api_token,
                slack_channel,
                greeting_message,
                goodbye_message,
            ),
        )?
        .extract()?;
    println!("{}", publish_result);

    Ok(())
}
