use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CurrentGameLoadoutsResponse {
    pub loadouts: Vec<LoadoutEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct LoadoutEntry {
    #[serde(rename = "CharacterID")]
    pub character_id: String,
    pub loadout: Loadout,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Loadout {
    pub subject: String,
    pub sprays: SprayContainer,
    pub expressions: ExpressionContainer,
    pub items: HashMap<String, Item>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SprayContainer {
    pub spray_selections: Vec<SpraySelection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpraySelection {
    #[serde(rename = "SocketID")]
    pub socket_id: String,
    #[serde(rename = "SprayID")]
    pub spray_id: String,
    #[serde(rename = "LevelID")]
    pub level_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpressionContainer {
    #[serde(rename = "AESSelections")]
    pub aes_selections: Vec<AESSelection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AESSelection {
    #[serde(rename = "SocketID")]
    pub socket_id: String,
    #[serde(rename = "AssetID")]
    pub asset_id: String,
    #[serde(rename = "TypeID")]
    pub type_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "TypeID")]
    pub type_id: String,
    pub sockets: HashMap<String, SocketEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SocketEntry {
    #[serde(rename = "ID")]
    pub id: String,
    pub item: NestedItem,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NestedItem {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "TypeID")]
    pub type_id: String,
}
