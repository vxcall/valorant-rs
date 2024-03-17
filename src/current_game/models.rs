use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

//-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrentGamePlayerResponse {
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "MatchID")]
    pub match_id: String,
    #[serde(rename = "Version")]
    pub version: i64,
}

//-------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentGameLoadoutsResponse {
    #[serde(rename = "Loadouts")]
    pub loadouts: Vec<LoadoutEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoadoutEntry {
    #[serde(rename = "CharacterID")]
    character_id: String,
    #[serde(rename = "Loadout")]
    loadout: Loadout,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Loadout {
    #[serde(rename = "Subject")]
    subject: String,
    #[serde(rename = "Sprays")]
    sprays: SprayContainer,
    #[serde(rename = "Expressions")]
    expressions: ExpressionContainer,
    #[serde(rename = "Items")]
    items: HashMap<String, Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SprayContainer {
    #[serde(rename = "SpraySelections")]
    spray_selections: Vec<SpraySelection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SpraySelection {
    #[serde(rename = "SocketID")]
    socket_id: String,
    #[serde(rename = "SprayID")]
    spray_id: String,
    #[serde(rename = "LevelID")]
    level_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ExpressionContainer {
    #[serde(rename = "AESSelections")]
    aes_selections: Vec<AESSelection>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AESSelection {
    #[serde(rename = "SocketID")]
    socket_id: String,
    #[serde(rename = "AssetID")]
    asset_id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
    #[serde(rename = "Sockets")]
    sockets: HashMap<String, SocketEntry>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SocketEntry {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "Item")]
    item: NestedItem,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NestedItem {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
}

//-------------------------------------------------------------

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrentGameMatchResponse {
    #[serde(rename = "MatchID")]
    pub match_id: String,
    #[serde(rename = "Version")]
    pub version: i64,
    #[serde(rename = "State")]
    pub state: String, // Considering flexibility, kept as String
    #[serde(rename = "MapID")]
    pub map_id: String,
    #[serde(rename = "ModeID")]
    pub mode_id: String,
    #[serde(rename = "ProvisioningFlow")]
    pub provisioning_flow: String, // Could be an enum for "Matchmaking" | "CustomGame"
    #[serde(rename = "GamePodID")]
    pub game_pod_id: String,
    #[serde(rename = "AllMUCName")]
    pub all_muc_name: String,
    #[serde(rename = "TeamMUCName")]
    pub team_muc_name: String,
    #[serde(rename = "TeamVoiceID")]
    pub team_voice_id: String,
    #[serde(rename = "TeamMatchToken")]
    pub team_match_token: String,
    #[serde(rename = "IsReconnectable")]
    pub is_reconnectable: bool,
    #[serde(rename = "ConnectionDetails")]
    pub connection_details: ConnectionDetails,
    #[serde(rename = "PostGameDetails")]
    pub post_game_details: Option<serde_json::Value>, // Nullable field
    #[serde(rename = "Players")]
    pub players: Vec<Player>,
    #[serde(rename = "MatchmakingData")]
    pub matchmaking_data: Option<serde_json::Value>, // Nullable field
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ConnectionDetails {
    #[serde(rename = "GameServerHosts")]
    pub game_server_hosts: Vec<String>,
    #[serde(rename = "GameServerHost")]
    pub game_server_host: String,
    #[serde(rename = "GameServerPort")]
    pub game_server_port: u32,
    #[serde(rename = "GameServerObfuscatedIP")]
    pub game_server_obfuscated_ip: u64,
    #[serde(rename = "GameClientHash")]
    pub game_client_hash: u64,
    #[serde(rename = "PlayerKey")]
    pub player_key: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Player {
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "TeamID")]
    pub team_id: String, // Can be "Blue" | "Red" or other strings
    #[serde(rename = "CharacterID")]
    pub character_id: String,
    #[serde(rename = "PlayerIdentity")]
    pub player_identity: PlayerIdentity,
    #[serde(rename = "SeasonalBadgeInfo")]
    pub seasonal_badge_info: SeasonalBadgeInfo,
    #[serde(rename = "IsCoach")]
    pub is_coach: bool,
    #[serde(rename = "IsAssociated")]
    pub is_associated: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PlayerIdentity {
    #[serde(rename = "Subject")]
    pub subject: String,
    #[serde(rename = "PlayerCardID")]
    pub player_card_id: String,
    #[serde(rename = "PlayerTitleID")]
    pub player_title_id: String,
    #[serde(rename = "AccountLevel")]
    pub account_level: u32,
    #[serde(rename = "PreferredLevelBorderID")]
    pub preferred_level_border_id: String,
    #[serde(rename = "Incognito")]
    pub incognito: bool,
    #[serde(rename = "HideAccountLevel")]
    pub hide_account_level: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SeasonalBadgeInfo {
    #[serde(rename = "SeasonID")]
    pub season_id: String,
    #[serde(rename = "NumberOfWins")]
    pub number_of_wins: u32,
    #[serde(rename = "WinsByTier")]
    pub wins_by_tier: Option<serde_json::Value>, // Nullable field
    #[serde(rename = "Rank")]
    pub rank: u32,
    #[serde(rename = "LeaderboardRank")]
    pub leaderboard_rank: u32,
}

//-------------------------------------------------------------