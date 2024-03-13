use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EntitlementsTokenResponse {
    #[serde(rename = "accessToken")]
    pub access_token: String,                 //auth token
    pub entitlements: Vec<serde_json::Value>,
    pub issuer: String,
    pub subject: String,                      // Player uuid
    pub token: String,                        // entitlement token
}
