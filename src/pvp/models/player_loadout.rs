use serde::{Serialize, Deserialize};
use serde_json::Value; // For representing the `unknown[]` type from TypeScript

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerLoadoutResponse {
    #[serde(rename = "Subject")]
    pub subject: String,
    pub version: i64,
    pub guns: Vec<Gun>,
    pub sprays: Vec<Spray>,
    pub identity: Identity,
    pub incognito: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Gun {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "CharmInstanceID")]
    pub charm_instance_id: Option<String>,
    #[serde(rename = "CharmID")]
    pub charm_id: Option<String>,
    #[serde(rename = "CharmLevelID")]
    pub charm_level_id: Option<String>,
    #[serde(rename = "SkinID")]
    pub skin_id: String,
    #[serde(rename = "SkinLevelID")]
    pub skin_level_id: String,
    #[serde(rename = "ChromaID")]
    pub chroma_id: String,
    pub attachments: Vec<Value>, // Using `Value` to represent `unknown[]` type from TypeScript
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Spray {
    #[serde(rename = "EquipSlotID")]
    pub equip_slot_id: String,
    #[serde(rename = "SprayID")]
    pub spray_id: String,
    pub spray_level_id: Option<String>, // Using `Option<String>` as it can be null
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Identity {
    #[serde(rename = "PlayerCardID")]
    pub player_card_id: String,
    #[serde(rename = "PlayerTitleID")]
    pub player_title_id: String,
    pub account_level: i32,
    #[serde(rename = "PreferredLevelBorderID")]
    pub preferred_level_border_id: String,
    pub hide_account_level: bool,
}
