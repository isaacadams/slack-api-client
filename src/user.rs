use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Profile {
    pub title: String,
    pub phone: String,
    pub skype: String,
    pub real_name: String,
    pub real_name_normalized: String,
    pub display_name: String,
    pub display_name_normalized: String,
    pub fields: serde_json::Map<String, serde_json::Value>,
    pub status_text: String,
    pub status_emoji: String,
    pub status_emoji_display_info: Vec<serde_json::Value>,
    pub status_expiration: i64,
    pub avatar_hash: String,
    pub image_original: String,
    pub is_custom_image: bool,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub image_24: String,
    pub image_32: String,
    pub image_48: String,
    pub image_72: String,
    pub image_192: String,
    pub image_512: String,
    pub image_1024: String,
    pub status_text_canonical: String,
}

#[derive(Debug, Deserialize)]
pub struct GetUserProfileResponse {
    pub ok: bool,
    pub profile: Option<Profile>,
    pub error: Option<String>,
}
