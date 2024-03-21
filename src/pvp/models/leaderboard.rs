use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LeaderboardResponse {
    #[serde(rename = "Deployment")]
    pub deployment: String,
    #[serde(rename = "QueueID")]
    pub queue_id: String,
    #[serde(rename = "SeasonID")]
    pub season_id: String,
    pub players: Vec<Player>,
    #[serde(rename = "totalPlayers")]
    pub total_players: i32,
    #[serde(rename = "immortalStartingPage")]
    pub immortal_starting_page: i32,
    #[serde(rename = "immortalStartingIndex")]
    pub immortal_starting_index: i32,
    #[serde(rename = "topTierRRThreshold")]
    pub top_tier_rr_threshold: i32,
    #[serde(rename = "tierDetails")]
    pub tier_details: HashMap<String, TierDetail>,
    #[serde(rename = "startIndex")]
    pub start_index: i32,
    #[serde(rename = "query")]
    pub query: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    #[serde(rename = "PlayerCardID")]
    pub player_card_id: String,
    #[serde(rename = "TitleID")]
    pub title_id: String,
    pub is_banned: bool,
    pub is_anonymized: bool,
    #[serde(rename = "puuid")]
    pub puuid: String,
    #[serde(rename = "gameName")]
    pub game_name: String,
    #[serde(rename = "tagLine")]
    pub tag_line: String,
    #[serde(rename = "leaderboardRank")]
    pub leaderboard_rank: i32,
    #[serde(rename = "rankedRating")]
    pub ranked_rating: i32,
    #[serde(rename = "numberOfWins")]
    pub number_of_wins: i32,
    #[serde(rename = "competitiveTier")]
    pub competitive_tier: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierDetail {
    pub ranked_rating_threshold: i32,
    pub starting_page: i32,
    pub starting_index: i32,
}
