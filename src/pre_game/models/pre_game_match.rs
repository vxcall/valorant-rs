use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PreGameMatchResponse {
    #[serde(rename = "ID")]
    id: String,
    version: i64,
    teams: Vec<Team>,
    ally_team: Option<Team>,
    enemy_team: Option<Team>,
    observer_subjects: Vec<serde_json::Value>,
    match_coaches: Vec<serde_json::Value>,
    enemy_team_size: i32,
    enemy_team_lock_count: i32,
    pregame_state: String,
    last_updated: String,
    #[serde(rename = "MapID")]
    map_id: String,
    map_select_pool: Vec<serde_json::Value>,
    #[serde(rename = "BannedMapIds")]
    banned_map_ids: Vec<serde_json::Value>,
    casted_votes: Option<serde_json::Value>,
    map_select_steps: Vec<serde_json::Value>,
    map_select_step: i32,
    team1: String,
    #[serde(rename = "GamePodID")]
    game_pod_id: String,
    mode: String,
    #[serde(rename = "VoiceSessionID")]
    voice_session_id: String,
    #[serde(rename = "MUCName")]
    muc_name: String,
    team_match_token: String,
    #[serde(rename = "QueueID")]
    queue_id: String,
    #[serde(rename = "ProvisioningFlowID")]
    provisioning_flow_id: String,
    is_ranked: bool,
    #[serde(rename = "PhaseTimeRemainingNS")]
    phase_time_remaining_ns: i64,
    #[serde(rename = "StepTimeRemainingNS")]
    step_time_remaining_ns: i64,
    #[serde(rename = "AltModesFlagADA")]
    alt_modes_flag_ada: bool,
    tournament_metadata: Option<serde_json::Value>,
    roster_metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Team {
    #[serde(rename = "TeamID")]
    team_id: String,
    players: Vec<Player>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Player {
    #[serde(rename = "Subject")]
    subject: String,
    #[serde(rename = "CharacterID")]
    character_id: String,
    character_selection_state: String,
    pregame_player_state: String,
    competitive_tier: i32,
    player_identity: PlayerIdentity,
    seasonal_badge_info: SeasonalBadgeInfo,
    is_captain: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PlayerIdentity {
    #[serde(rename = "Subject")]
    subject: String,
    #[serde(rename = "PlayerCardID")]
    player_card_id: String,
    #[serde(rename = "PlayerTitleID")]
    player_title_id: String,
    account_level: i32,
    #[serde(rename = "PreferredLevelBorderID")]
    preferred_level_border_id: String,
    incognito: bool,
    hide_account_level: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct SeasonalBadgeInfo {
    #[serde(rename = "SeasonID")]
    season_id: String,
    number_of_wins: i32,
    wins_by_tier: Option<serde_json::Value>,
    rank: i32,
    leaderboard_rank: i32,
}