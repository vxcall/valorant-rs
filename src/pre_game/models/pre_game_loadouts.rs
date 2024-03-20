use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PreGameLoadoutsResponse {
    pub loadouts: Vec<Loadout>,
    pub loadouts_valid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Loadout {
    pub subject: String,
    pub sprays: SprayContainer,
    pub expressions: ExpressionContainer,
    pub items: HashMap<String, Item>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SprayContainer {
    pub spray_selections: Vec<SpraySelection>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SpraySelection {
    #[serde(rename = "SocketID")]
    pub socket_id: String,
    #[serde(rename = "SprayID")]
    pub spray_id: String,
    #[serde(rename = "LevelID")]
    pub level_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ExpressionContainer {
    #[serde(rename = "AESSelections")]
    pub aes_selections: Vec<AESSelection>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AESSelection {
    #[serde(rename = "SocketID")]
    pub socket_id: String,
    #[serde(rename = "AssetID")]
    pub asset_id: String,
    #[serde(rename = "TypeID")]
    pub type_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "TypeID")]
    pub type_id: String,
    pub sockets: HashMap<String, Socket>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Socket {
    #[serde(rename = "ID")]
    pub id: String,
    pub item: SocketItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SocketItem {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "TypeID")]
    pub type_id: String,
}