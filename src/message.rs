use crate::client::SlackClient;

pub enum CreateMessage {
    /// https://api.slack.com/methods/chat.postMessage#arg_attachments
    #[allow(dead_code)]
    Attachments(serde_json::Value),
    /// https://api.slack.com/methods/chat.postMessage#arg_blocks
    Blocks(serde_json::Value),
    /// https://api.slack.com/methods/chat.postMessage#arg_text
    Text(String),
}

impl CreateMessage {
    fn key_value(self) -> (&'static str, serde_json::Value) {
        match self {
            CreateMessage::Attachments(value) => ("attachments", value),
            CreateMessage::Blocks(value) => ("blocks", value),
            CreateMessage::Text(value) => ("text", serde_json::Value::String(value)),
        }
    }

    pub fn to_request(self) -> serde_json::Value {
        let (message_type, value) = self.key_value();

        serde_json::json!({
            //"response_type": "in_channel",
            message_type: value,
        })
    }

    pub async fn send_to_channel(
        self,
        client: &SlackClient,
        channel: String,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let mut request = self.to_request();
        request["channel"] = serde_json::Value::String(channel);
        client.send_message(&request, None).await
    }

    pub async fn send_response_url(
        self,
        client: &SlackClient,
        response_url: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let request = self.to_request();
        // request["response_type"] = serde_json::Value::String("in_channel");
        client.send_message(&request, Some(response_url)).await
    }
}
