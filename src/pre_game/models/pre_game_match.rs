use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PreGameMatchResponse {
    #[serde(rename = "ID")]
    pub id: String,
    pub version: i64,
    pub teams: Vec<Team>,
    pub ally_team: Option<Team>,
    pub enemy_team: Option<Team>,
    pub observer_subjects: Vec<serde_json::Value>,
    pub match_coaches: Vec<serde_json::Value>,
    pub enemy_team_size: i32,
    pub enemy_team_lock_count: i32,
    pub pregame_state: String,
    pub last_updated: String,
    #[serde(rename = "MapID")]
    pub map_id: String,
    pub map_select_pool: Vec<serde_json::Value>,
    #[serde(rename = "BannedMapIDs")]
    pub banned_map_ids: Vec<serde_json::Value>,
    pub casted_votes: Option<serde_json::Value>,
    pub map_select_steps: Vec<serde_json::Value>,
    pub map_select_step: i32,
    pub team1: String,
    #[serde(rename = "GamePodID")]
    pub game_pod_id: String,
    pub mode: String,
    #[serde(rename = "VoiceSessionID")]
    pub voice_session_id: String,
    #[serde(rename = "MUCName")]
    pub muc_name: String,
    pub team_match_token: String,
    #[serde(rename = "QueueID")]
    pub queue_id: String,
    #[serde(rename = "ProvisioningFlowID")]
    pub provisioning_flow_id: String,
    pub is_ranked: bool,
    #[serde(rename = "PhaseTimeRemainingNS")]
    pub phase_time_remaining_ns: i64,
    #[serde(rename = "StepTimeRemainingNS")]
    pub step_time_remaining_ns: i64,
    #[serde(rename = "altModesFlagADA")]
    pub alt_modes_flag_ada: bool,
    pub tournament_metadata: Option<serde_json::Value>,
    pub roster_metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Team {
    #[serde(rename = "TeamID")]
    pub team_id: String,
    pub players: Vec<Player>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Player {
    pub subject: String,
    #[serde(rename = "CharacterID")]
    pub character_id: String,
    pub character_selection_state: String,
    pub pregame_player_state: String,
    pub competitive_tier: i32,
    pub player_identity: PlayerIdentity,
    pub seasonal_badge_info: SeasonalBadgeInfo,
    pub is_captain: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerIdentity {
    pub subject: String,
    #[serde(rename = "PlayerCardID")]
    pub player_card_id: String,
    #[serde(rename = "PlayerTitleID")]
    pub player_title_id: String,
    pub account_level: i32,
    #[serde(rename = "PreferredLevelBorderID")]
    pub preferred_level_border_id: String,
    pub incognito: bool,
    pub hide_account_level: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SeasonalBadgeInfo {
    #[serde(rename = "SeasonID")]
    pub season_id: String,
    pub number_of_wins: i32,
    pub wins_by_tier: Option<serde_json::Value>,
    pub rank: i32,
    pub leaderboard_rank: i32,
}
