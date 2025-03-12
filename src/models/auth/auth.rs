use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct Auth {
    pub token_type: String,
    pub access_token: String,
    pub expires_in: i64, // or u64, depending on the expected type
    pub scope: String,
}