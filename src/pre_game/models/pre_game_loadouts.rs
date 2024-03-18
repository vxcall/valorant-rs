use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PreGameLoadoutsResponse {
    loadouts: Vec<Loadout>,
    loadouts_valid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Loadout {
    #[serde(rename = "SubjectID")]
    subject_id: String,
    sprays: SprayContainer,
    expressions: ExpressionContainer,
    items: HashMap<String, Item>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SprayContainer {
    spray_selections: Vec<SpraySelection>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SpraySelection {
    #[serde(rename = "SocketID")]
    socket_id: String,
    #[serde(rename = "SprayID")]
    spray_id: String,
    #[serde(rename = "LevelID")]
    level_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ExpressionContainer {
    aes_selections: Vec<AESSelection>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AESSelection {
    #[serde(rename = "SocketID")]
    socket_id: String,
    #[serde(rename = "AssetID")]
    asset_id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
    sockets: HashMap<String, Socket>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Socket {
    #[serde(rename = "ID")]
    id: String,
    item: SocketItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SocketItem {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
}