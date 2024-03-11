use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct AuthCookiesBody {
    pub client_id: String,
    pub nonce: String,
    pub redirect_uri: String,
    pub response_type: String,
    pub scope: String,
}

#[derive(Serialize, Deserialize)]
pub struct AuthRequestBody {
    #[serde(rename = "type")]
    pub auth_type: String,
    pub username: String,
    pub password: String,
    pub remember: bool,
    pub language: String,
}
