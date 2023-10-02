use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct SlackCommandRequest {
    pub token: String,
    pub command: String,
    pub text: String,
    pub response_url: String,
    pub trigger_id: String,
    pub user_id: String,
    pub user_name: String,
    pub team_id: String,
    pub enterprise_id: Option<String>,
    pub is_enterprise_install: bool,
    pub channel_id: String,
    pub team_domain: String,
    pub channel_name: String,
    pub api_app_id: String,
}
