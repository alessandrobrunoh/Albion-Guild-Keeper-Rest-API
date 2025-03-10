use serde::Deserialize;

#[derive(Deserialize)]
pub struct AuthDiscord {
    pub token_type: String,
    pub token: String,
}