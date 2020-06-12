<p align="center">
  <img alt="orangino logo" src="assets/orangino.png" height="300" />
  <h3 align="center">ORANGINO</h3>
  <p align="center">An amazing Tangerino plugin for Slack to punch in and out from your CLI, while automatically letting your team know if you're available or not.</p>
</p>

---

Orangino uses the Reqwest crate to interact with the [Tangerino](https://app.tangerino.com.br/) API, while binding with Python to benefit from the [official Slack client](https://github.com/slackapi/python-slackclient) to publish messages to the channel of your choice.

## Installation

### Package Managers

```console
# Cargo
cargo install orangino
# Debian
curl -LJO https://github.com/marcelovicentegc/orangino/releases/download/0.1.1/orangino_0.1.1_amd64.deb
sudo dpkg -i ./orangino_0.1.1_amd64.deb
```

## Configuration

Orangino has a [configuration](.orangino.example.toml) file that allows you to change default behaviour.
The file must be named `.orangino.toml`. Currently, Orangino looks for
this file in your home directory (`/home/marcelo` on Linux, `/Users/Marcelo` on macOs, `C:\Users\Alice` on Windows)

## How to use Orangino

To have Tangerino punch your card, you just need to call it:

```shell
$ orangino
```

### Demo

#### Punching in

![Punching in](./assets/orangino-in.gif)

#### Punching out

![Punching in](./assets/orangino-out.gif)

## Development directions ▶️

You will need:

1. A [Slack app](https://api.slack.com/apps) installed on your workspace with the following features and permissions:
   - Incoming webhooks
   - Bot
     - `chat:write`
     - `incoming-webhook` - `pins:read`
     - `pins:write`
2. Python version 3.5 up installed.
3. Rust's nightly version installed.
4. A `.orangino.toml` file with your credentials, following the `.orangino.example.toml` model on your home path:

```toml
employer_code = "12345"
pin = "9876"
tangerino_basic_token = "Basic xeAxZyEwTOsPZKdlIA=="

slack_channel = "#random"
slack_api_token = "xoxp-22f3f6aa-1a75-452c-b023-5365db9409ae"
greetings_message = "Hello world!"
goodbye_message = "Goodbye world!"

```

5. To install the dependencies: `pip install -r requirements.txt && cargo build`

You are good to go now, make changes to the app and run it: `cargo run`
