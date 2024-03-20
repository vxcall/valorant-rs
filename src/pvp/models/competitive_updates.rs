use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct CompetitiveUpdatesResponse {
    pub version: i64,
    pub subject: String,
    pub matches: Vec<MatchUpdate>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct MatchUpdate {
    #[serde(rename = "MatchID")]
    pub match_id: String,
    #[serde(rename = "MapID")]
    pub map_id: String,
    #[serde(rename = "SeasonID")]
    pub season_id: String,
    pub match_start_time: i64,
    pub tier_after_update: i32,
    pub tier_before_update: i32,
    pub ranked_rating_after_update: i32,
    pub ranked_rating_before_update: i32,
    pub ranked_rating_earned: i32,
    pub ranked_rating_performance_bonus: i32,
    pub competitive_movement: String,
    #[serde(rename = "AFKPenalty")]
    pub afk_penalty: i32,
}
