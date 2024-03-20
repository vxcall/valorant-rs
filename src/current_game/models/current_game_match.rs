use serde::{ Deserialize, Serialize };

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CurrentGameMatchResponse {
    #[serde(rename = "MatchID")]
    pub match_id: String,
    pub version: i64,
    pub state: String, // Considering flexibility, kept as String
    #[serde(rename = "MapID")]
    pub map_id: String,
    #[serde(rename = "ModeID")]
    pub mode_id: String,
    pub provisioning_flow: String, // Could be an enum for "Matchmaking" | "CustomGame"
    #[serde(rename = "GamePodID")]
    pub game_pod_id: String,
    #[serde(rename = "AllMUCName")]
    pub all_muc_name: String,
    #[serde(rename = "TeamMUCName")]
    pub team_muc_name: String,
    #[serde(rename = "TeamVoiceID")]
    pub team_voice_id: String,
    pub team_match_token: String,
    pub is_reconnectable: bool,
    pub connection_details: ConnectionDetails,
    pub post_game_details: Option<serde_json::Value>, // Nullable field
    pub players: Vec<Player>,
    pub matchmaking_data: Option<serde_json::Value>, // Nullable field
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConnectionDetails {
    pub game_server_hosts: Vec<String>,
    pub game_server_host: String,
    pub game_server_port: u32,
    #[serde(rename = "GameServerObfuscatedIP")]
    pub game_server_obfuscated_ip: u64,
    pub game_client_hash: u64,
    pub player_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub subject: String,
    #[serde(rename = "TeamID")]
    pub team_id: String, // Can be "Blue" | "Red" or other strings
    #[serde(rename = "CharacterID")]
    pub character_id: String,
    pub player_identity: PlayerIdentity,
    pub seasonal_badge_info: SeasonalBadgeInfo,
    pub is_coach: bool,
    pub is_associated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerIdentity {
    pub subject: String,
    #[serde(rename = "PlayerCardID")]
    pub player_card_id: String,
    #[serde(rename = "PlayerTitleID")]
    pub player_title_id: String,
    pub account_level: u32,
    #[serde(rename = "PreferredLevelBorderID")]
    pub preferred_level_border_id: String,
    pub incognito: bool,
    pub hide_account_level: bool,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct SeasonalBadgeInfo {
    #[serde(rename = "SeasonID")]
    pub season_id: String,
    pub number_of_wins: u32,
    pub wins_by_tier: Option<serde_json::Value>, // Nullable field
    pub rank: u32,
    pub leaderboard_rank: u32,
}
