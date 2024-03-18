use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CurrentGamePlayerResponse {
    pub subject: String,
    #[serde(rename = "MatchID")]
    pub match_id: String,
    pub version: i64,
}