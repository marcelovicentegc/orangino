use super::types::SyncResp;
use pyo3::prelude::*;

pub async fn filter_and_publish(
    parsed_sync_resp: SyncResp,
) -> Result<(), Box<dyn std::error::Error>> {
    let gil = Python::acquire_gil();
    let py = gil.python();

    if parsed_sync_resp.tipoRetornoRegistroApontamentoEnum == "NOVO_PONTO_ABERTO" {
        publish_to_slack(py, true).map_err(|e| {
            e.print_and_set_sys_last_vars(py);
        });
    } else if parsed_sync_resp.tipoRetornoRegistroApontamentoEnum
        == "ULTIMO_PONTO_FECHADO_NOVO_ABERTO"
    {
        publish_to_slack(py, false).map_err(|e| {
            e.print_and_set_sys_last_vars(py);
        });
    }
    Ok(())
}

fn publish_to_slack(py: Python, greet: bool) -> PyResult<()> {
    let slack_client = PyModule::from_code(
        py,
        r#"
import os
from dotenv import load_dotenv
from slack import WebClient
from slack.errors import SlackApiError
            
def publish(greet):
    load_dotenv()
    client = WebClient(token=os.environ['SLACK_API_TOKEN'])   

    if greet:
        message = os.environ['GREETING_MESSAGE']
    else:
        message = os.environ['GOODBYE_MESSAGE']

    try:
        response = client.chat_postMessage(
            channel=os.environ['SLACK_CHANNEL'],
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

    let publish_result: String = slack_client.call1("publish", (greet,))?.extract()?;
    println!("{}", publish_result);

    Ok(())
}
