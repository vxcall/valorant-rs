use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct EntitlementsTokenResponse {
    #[serde(rename = "accessToken")]
    pub auth_token: String,                 //auth token
    pub entitlements: Vec<serde_json::Value>,
    pub issuer: String,
    #[serde(rename = "subject")]
    pub puuid: String,                      // Player uuid
    #[serde(rename = "token")]
    pub entitlement_token: String,          // entitlement token
}