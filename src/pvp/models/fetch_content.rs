use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FetchContentResponse {
    #[serde(rename = "DisabledIDs")]
    pub disabled_ids: Vec<Value>,
    pub seasons: Vec<Season>,
    pub events: Vec<Event>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Season {
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
    #[serde(rename = "Type")]
    pub type_field: SeasonType,
    #[serde(rename = "StartTime")]
    pub start_time: String,
    #[serde(rename = "EndTime")]
    pub end_time: String,
    pub is_active: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum SeasonType {
    Episode,
    Act,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Event {
    #[serde(rename = "ID")]
    pub id: String,
    pub name: String,
    #[serde(rename = "StartTime")]
    pub start_time: String,
    #[serde(rename = "EndTime")]
    pub end_time: String,
    pub is_active: bool,
}
