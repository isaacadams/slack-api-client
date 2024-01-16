use crate::user;
use reqwest::header;

pub struct SlackClient {
    pub client: reqwest::Client,
}

impl SlackClient {
    pub fn new(bearer_token: String) -> Self {
        let mut headers = header::HeaderMap::new();
        headers.insert(
            header::AUTHORIZATION,
            header::HeaderValue::from_str(format!("Bearer {}", bearer_token).as_str())
                .expect("failed to add Bearer token for slack client"),
        );

        let client = reqwest::ClientBuilder::new()
            .default_headers(headers)
            .build()
            .expect("failed to build reqwest client");

        SlackClient { client }
    }

    /// https://api.slack.com/methods/conversations.list
    #[allow(dead_code)]
    pub async fn get_conversation_list(&self) -> Result<String, reqwest::Error> {
        self.client
            .get("https://slack.com/api/conversations.list")
            .send()
            .await?
            .text()
            .await
    }

    /// https://api.slack.com/methods/chat.postMessage
    /// errors: https://api.slack.com/methods/chat.postMessage#errors
    pub async fn send_message(
        &self,
        request: String,
        // w/ slack interactions/commands, a message can be sent to a provided response_url
        // https://api.slack.com/interactivity/handling#message_responses
        response_url: Option<&str>,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let response = self
            .client
            .post(response_url.unwrap_or("https://slack.com/api/chat.postMessage"))
            .body(request)
            .header(header::CONTENT_TYPE, "application/json; charset=utf-8")
            .send()
            .await?;

        log::info!("{}", response.status());

        Ok(response)
    }

    /// https://api.slack.com/methods/chat.delete
    pub async fn delete_message(
        &self,
        channel_id: &str,
        message_ts: &str,
    ) -> Result<reqwest::Response, reqwest::Error> {
        let response = self
            .client
            .post("https://slack.com/api/chat.delete")
            .query(&[("channel", channel_id), ("ts", message_ts)])
            .send()
            .await?;

        log::info!("{}", response.status());

        Ok(response)
    }

    pub async fn get_user_profile(
        &self,
        user_id: &str,
    ) -> Result<user::GetUserProfileResponse, reqwest::Error> {
        let response = self
            .client
            .post("https://slack.com/api/users.profile.get")
            .query(&[("user", user_id)])
            .send()
            .await?;

        log::info!("{}", response.status());

        response.json().await
    }
}
