# Slack API

Check out the [CLI documentation!](./cli/README.md)

```rust
use slack_api_client::{SlackClient, CreateMessage};
let client = SlackClient::new("<SLACK_BEARER_TOKEN>");
let response = CreateMessage::Text("Hello World".to_string())
    .send_to_channel(&client, "<YOUR_CHANNEL_ID>".to_string())
    .await?;
```

## Examples

### Create a Block Message

```rust
use slack_api_client::*;

pub fn hello_world() -> CreateMessage {
    CreateMessage::Blocks(serde_json::json!([{
        "type": "section",
        "text": {
            "type": "plain_text",
            "text": "hello world",
            "emoji": true
        }
    }]))
}

pub fn main () {
    let client = SlackClient::new("<SLACK_BEARER_TOKEN>");
    let response = hello_world()
        .send_to_channel(&client, "<YOUR_CHANNEL_ID>".to_string())
        .await?;
}
```

### Open Modal in Slack

```rust
pub struct SlackModal<'a> {
    pub callback_id: &'a str,
}

impl<'a> SlackModal<'a> {
    // https://api.slack.com/surfaces/modals#updating_views
    pub async fn open(
        &self,
        client: &slack_api_client::SlackClient,
        trigger_id: &str,
        title: &str,
        blocks: serde_json::Value,
    ) -> anyhow::Result<()> {
        let response = client
            .client
            .post("https://slack.com/api/views.open")
            .header("Content-Type", "application/json")
            .body(
                serde_json::json!({
                  "trigger_id": trigger_id,
                  "view": {
                    "type": "modal",
                    "callback_id": self.callback_id,
                    "title": {
                      "type": "plain_text",
                      "text": title
                    },
                    "blocks": blocks
                  }
                })
                .to_string(),
            )
            .send()
            .await?;

        let response: SlackResponse = response.json().await?;
        response.is_ok()?;

        Ok(())
    }
}

#[derive(serde::Deserialize)]
pub struct SlackResponse {
    ok: bool,
    warning: Option<serde_json::Value>,
    error: Option<serde_json::Value>,
    response_metadata: Option<serde_json::Value>,
}

impl SlackResponse {
    pub fn is_ok(&self) -> anyhow::Result<()> {
        if self.ok {
            return Ok(());
        }

        Err(anyhow::Error::msg(serde_json::json!({
            "error": self.error,
            "warning": self.warning,
            "response_metadata": self.response_metadata,
        })))
    }
}

```

# Slack Api Reference

Slack provides a feature-rich API to format messages sent to slack. It is called [`Block Kit`](https://api.slack.com/block-kit), and can be used to construct beautiful messages.

- [block kit builder](https://app.slack.com/block-kit-builder/)
- [elements reference](https://api.slack.com/reference/block-kit/block-elements#button)
- [layout reference](https://api.slack.com/reference/block-kit/blocks)
