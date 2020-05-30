import os
from dotenv import load_dotenv
from slack import WebClient
from slack.errors import SlackApiError

load_dotenv()

client = WebClient(token=os.environ['SLACK_API_TOKEN'])

try:
    response = client.chat_postMessage(
        channel=os.environ['SLACK_CHANNEL'],
        text=os.environ['GREETING_MESSAGE'])
    assert response["message"]["text"] == os.environ['GREETING_MESSAGE']
except SlackApiError as error:    
    # You will get a SlackApiError if "ok" is False
    assert error.response["ok"] is False
    assert error.response["error"]  # str like 'invalid_auth', 'channel_not_found'
    print(f"Got an error: {error.response['error']}")