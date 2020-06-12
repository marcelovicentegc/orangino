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
# Arch Linux
pacman -S orangino
# Cargo
cargo install orangino
# MacOS (Homebrew)
brew install orangino
```

## Configuration

Orangino has a [configuration](orangino.example.toml) file that allows you to change default behaviour.
The file can be named `orangino.toml` or `.oranginorc`. Currently, Orangino looks for
this file in three different places. The current directory, your home directory,
and your configuration directory.

## How to use Orangino

To have Tangerino punch your card, you just need to call it:

```shell
$ orangino
```

## Development directions ▶️

You will need:

1. A [Slack app](https://api.slack.com/apps) installed on your workspace with the following features and permissions:
	- Incoming webhooks
	- Bot
	    - `chat:write`
	    - `incoming-webhook`
            - `pins:read`
	    - `pins:write`
2. Python version 3.5 up installed.
3. Rust's nightly version installed.
4. A `.env` file with your credentials, following the `.env.example` model:

```s
EMPLOYER_CODE=12345
PIN=9876
TANGERINO_BASIC_TOKEN="Basic xeAxZyEwTOsPZKdlIA=="

SLACK_CHANNEL="#random"
SLACK_API_TOKEN="xoxp-22f3f6aa-1a75-452c-b023-5365db9409ae"
GREETING_MESSAGE="Hello world!"
GOODBYE_MESSAGE="Goodbye world!"
```

5. To install the dependencies: `pip install -r requirements.txt && cargo build`

You are good to go now, make changes to the app and run it: `cargo run`
