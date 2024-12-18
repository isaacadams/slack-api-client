# Slack CLI

This is a CLI tool for sending messages to Slack channels. It supports sending text messages, blocks, and attachments using the Slack API.

## Installation

1. Clone the repository.
2. Install dependencies and compile the code using `cargo build --release`.
3. Make sure the `SLACK_TOKEN` environment variable is set with your Slack bot token.
   
   ```bash
   export SLACK_TOKEN=xoxb-your-slack-bot-token
   ```

## Usage

### General Command

```bash
slack <COMMAND> [OPTIONS]
```

### Commands

#### `slack send <message>`
Send a message to a Slack channel.

##### Arguments

- `--channel` (`-c`) - The Slack channel to send the message to. Accepts either the channel name (e.g., `#general`) or the channel ID.
- `--kind` (`-k`) - The type of message to send. Options are:
  - `text` - A simple text message.
  - `block` - A message with block elements (provide a JSON object).
  - `attachment` - A message with attachments (provide a JSON object).
- `<message>` - The content of the message. For `block` or `attachment`, provide a valid JSON object.

##### Example Usage

Send a text message:

```bash
slack send --channel general --kind text "Hello, Slack!"
slack send --channel=#general --kind text "Hello, Slack!"
slack send --channel "#general" --kind text "Hello, Slack!"
```

Send a block message:

```bash
slack send --channel general --kind block '[{"type": "section", "text": {"type": "mrkdwn", "text": "*Hello, Slack!*"}}]'
```

Send an attachment message:

```bash
slack send --channel general --kind attachment '[{"color":"#f2c744","blocks":[{"type":"section","text":{"type":"mrkdwn","text":"*Hello, Slack!*"}}]}]'
```

## Environment Variables

- `SLACK_TOKEN`: Required. The token for authenticating with the Slack API. Set this in your environment or use a `.env` file for local development.

## Error Handling

Ensure that all required options are provided and that the `SLACK_TOKEN` environment variable is set. The CLI will display meaningful error messages if these requirements are not met.

