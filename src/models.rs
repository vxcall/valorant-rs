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

#[derive(Serialize, Deserialize, Debug)]
struct RiotGeoResponse {
    token: String,
    affinities: Affinities,
}

#[derive(Serialize, Deserialize, Debug)]
struct Affinities {
    pbe: String,
    live: String,
}