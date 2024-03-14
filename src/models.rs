use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

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

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentGameLoadoutsResponse {
    pub loadouts: Vec<LoadoutEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadoutEntry {
    #[serde(rename = "CharacterID")]
    character_id: String,
    loadout: Loadout,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Loadout {
    #[serde(rename = "Subject")]
    subject: String,
    sprays: SprayContainer,
    expressions: ExpressionContainer,
    items: HashMap<String, Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SprayContainer {
    #[serde(rename = "SpraySelections")]
    spray_selections: Vec<SpraySelection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpraySelection {
    #[serde(rename = "SocketID")]
    socket_id: String,
    #[serde(rename = "SprayID")]
    spray_id: String,
    #[serde(rename = "LevelID")]
    level_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpressionContainer {
    #[serde(rename = "AESSelections")]
    aes_selections: Vec<AESSelection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AESSelection {
    #[serde(rename = "SocketID")]
    socket_id: String,
    #[serde(rename = "AssetID")]
    asset_id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
    sockets: HashMap<String, SocketEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocketEntry {
    #[serde(rename = "ID")]
    id: String,
    item: NestedItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NestedItem {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
}
