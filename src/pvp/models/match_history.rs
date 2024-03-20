use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MatchHistoryResponse {
    pub subject: String,
    pub begin_index: i32,
    pub end_index: i32,
    pub total: i32,
    pub history: Vec<MatchHistoryEntry>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MatchHistoryEntry {
    #[serde(rename = "MatchID")]
    pub match_id: String,
    pub game_start_time: i64, // Using i64 to represent milliseconds since epoch
    #[serde(rename = "QueueID")]
    pub queue_id: String,
}
