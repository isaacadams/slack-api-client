use std::marker::PhantomData;

use super::CreateMessage;
use reqwest::header;

pub struct Default;
pub struct Channel;

pub struct SlackMessageBuilder<T = Default> {
    request: serde_json::Value,
    kind: PhantomData<T>,
}

impl SlackMessageBuilder {
    pub fn new(kind: CreateMessage) -> Self {
        Self {
            request: kind.to_request(),
            kind: PhantomData::<Default>,
        }
    }
}

impl<T> SlackMessageBuilder<T> {
    // https://api.slack.com/methods/chat.postMessage#arg_channel
    pub fn set_channel<I: Into<String>>(mut self, channel: I) -> SlackMessageBuilder<Channel> {
        self.request["channel"] = serde_json::Value::String(channel.into());
        SlackMessageBuilder::<Channel> {
            request: self.request,
            kind: PhantomData::<Channel>,
        }
    }

    /// https://api.slack.com/methods/chat.postMessage
    /// errors: https://api.slack.com/methods/chat.postMessage#errors
    async fn send_message(
        request: String,
        // w/ slack interactions/commands, a message can be sent to a provided response_url
        // https://api.slack.com/interactivity/handling#message_responses
        response_url: Option<&str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let response = SLACK
            .client
            .post(response_url.unwrap_or("https://slack.com/api/chat.postMessage"))
            .body(request)
            .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .send()
            .await?;

        log::info!("{}", response.status());

        Ok(response)
    }
}

impl SlackMessageBuilder<Channel> {
    pub async fn send(self) -> Result<reqwest::Response, reqwest::Error> {
        Self::send_message(self.request.to_string(), None).await
    }
}

impl SlackMessageBuilder<Default> {
    pub async fn send_to_response_url(
        self,
        response_url: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        Self::send_message(self.request.to_string(), Some(response_url)).await
    }
}
