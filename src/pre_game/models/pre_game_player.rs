use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct PreGamePlayerResponse {
    subject: String,
    #[serde(rename = "MatchID")]
    match_id: String,
    version: i64,
}