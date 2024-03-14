use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EntitlementsTokenResponse {
    #[serde(rename = "accessToken")]
    pub auth_token: String,                 //auth token
    pub entitlements: Vec<serde_json::Value>,
    pub issuer: String,
    #[serde(rename = "subject")]
    pub puuid: String,                      // Player uuid
    #[serde(rename = "token")]
    pub entitlement_token: String,                        // entitlement token
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RiotGeoResponse {
    pub token: String,
    pub affinities: Affinities,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Affinities {
    pub pbe: String,
    pub live: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentGamePlayerResponse {
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "MatchID")]
    pub match_id: String,
    #[serde(rename = "Version")]
    pub version: i64,
}