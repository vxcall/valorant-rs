use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EntitlementsTokenResponse {
    pub access_token: String,
    pub entitlements: Vec<serde_json::Value>,
    pub issuer: String,
    pub subject: String,
    pub token: String,
}
