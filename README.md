# Slack API

```rust
let client = SlackClient::new("<SLACK_BEARER_TOKEN>");
let response = CreateMessage::Text("Hello World".to_string())
    .send_to_channel(&client, "<YOUR_CHANNEL_ID>".to_string())
    .await?;
```
