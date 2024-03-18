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
    character_id: String,
    loadout: Loadout,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Loadout {
    subject: String,
    sprays: SprayContainer,
    expressions: ExpressionContainer,
    items: HashMap<String, Item>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SprayContainer {
    spray_selections: Vec<SpraySelection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SpraySelection {
    #[serde(rename = "SocketID")]
    socket_id: String,
    #[serde(rename = "SprayID")]
    spray_id: String,
    #[serde(rename = "LevelID")]
    level_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ExpressionContainer {
    #[serde(rename = "AESSelections")]
    aes_selections: Vec<AESSelection>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AESSelection {
    #[serde(rename = "SocketID")]
    socket_id: String,
    #[serde(rename = "AssetID")]
    asset_id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
    sockets: HashMap<String, SocketEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SocketEntry {
    #[serde(rename = "ID")]
    id: String,
    item: NestedItem,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct NestedItem {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
}

//-------------------------------------------------------------