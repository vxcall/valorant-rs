use serde::{Serialize, Deserialize};
use serde_json::Value;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AccountXPResponse {
    pub version: i64,
    pub subject: String,
    pub progress: Progress,
    pub history: Vec<HistoryEntry>,
    pub last_time_granted_first_win: String,
    pub next_time_first_win_available: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Progress {
    pub level: i32,
    #[serde(rename = "XP")]
    pub xp: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct HistoryEntry {
    #[serde(rename = "ID")]
    pub id: String,
    pub match_start: String,
    pub start_progress: Progress,
    pub end_progress: Progress,
    pub xp_delta: i32,
    pub xp_sources: Vec<XpSource>,
    pub xp_multipliers: Vec<Value>, // Using `Value` to represent `unknown` type from TypeScript
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct XpSource {
    #[serde(rename = "ID")]
    pub id: XpSourceId,
    pub amount: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum XpSourceId {
    TimePlayed,
    MatchWin,
    FirstWinOfTheDay,
}
