<p align="center">
  <img alt="orangino logo" src="assets/orangino.png" height="300" />
  <h3 align="center">ORANGINO</h3>
  <p align="center">An amazing Tangerino plugin for Slack to punch in and out from your CLI, while automatically letting your team know if you're available or not.</p>
</p>

---

Orangino uses the Reqwest crate to interact with the [Tangerino API](https://app.tangerino.com.br/), while binding with Python to benefit from the [official Slack client](https://github.com/slackapi/python-slackclient) to publish messages to the channel of your choice.

## Development directions ▶️

1. You will need Python > 3.5 and Rust's nightly version installed.
2. Create a `.env` file with your credentials, following the `.env.example` model:

```s
EMPLOYER_CODE=12345
PIN=9876
TANGERINO_BASIC_TOKEN="Basic xeAxZyEwTOsPZKdlIA=="

SLACK_CHANNEL="#random"
SLACK_API_TOKEN="xoxp-22f3f6aa-1a75-452c-b023-5365db9409ae"
GREETING_MESSAGE="Hello world!"
GOODBYE_MESSAGE="Goodbye world!"
```

3. Install dependencies: `pip install -r requirements.txt && cargo build`
4. Make changes to the app and run in: `cargo run`
