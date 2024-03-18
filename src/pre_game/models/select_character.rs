use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SelectCharacterResponse {
    #[serde(rename = "ID")]
    id: String,
    version: i64,
    teams: Vec<Team>,
    ally_team: Option<Team>,
    enemy_team: Option<Team>,
    observer_subjects: Vec<serde_json::Value>, // unknown types as flexible JSON
    match_coaches: Vec<serde_json::Value>, // unknown types as flexible JSON
    enemy_team_size: i32,
    enemy_team_lock_count: i32,
    pregame_state: PregameState,
    last_updated: String,
    #[serde(rename = "MapID")]
    map_id: String,
    map_select_pool: Vec<serde_json::Value>,
    #[serde(rename = "BannedMapIds")]
    banned_map_ids: Vec<serde_json::Value>,
    casted_votes: Option<Vec<serde_json::Value>>,
    map_select_steps: Vec<serde_json::Value>,
    map_select_step: i32,
    team1: TeamColor,
    #[serde(rename = "GamePodID")]
    game_pod_id: String,
    mode: String,
    #[serde(rename = "VoiceSessionID")]
    voice_session_id: String,
    muc_name: String,
    team_match_token: String,
    #[serde(rename = "QueueID")]
    queue_id: Option<String>,
    provisioning_flow_id: ProvisioningFlowID,
    is_ranked: bool,
    #[serde(rename = "PhaseTimeRemainingNS")]
    phase_time_remaining_ns: i64,
    #[serde(rename = "StepTimeNS")]
    step_time_remaining_ns: i64,
    alt_modes_flag_ADA: bool,
    tournament_metadata: Option<serde_json::Value>,
    roster_metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
enum ProvisioningFlowID {
    Matchmaking,
    CustomGame,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
enum TeamColor {
    Blue,
    Red,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
enum CharacterSelectionState {
    Selected,
    Locked,
    #[serde(rename = "")]
    None,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
enum PregameState {
    CharacterSelectActive,
    Provisioned,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Player {
    #[serde(rename = "SubjectID")]
    subject_id: String,
    #[serde(rename = "CharacterID")]
    character_id: String,
    character_selection_state: CharacterSelectionState,
    pregame_player_state: String,
    competitive_tier: i32,
    player_identity: PlayerIdentity,
    seasonal_badge_info: SeasonalBadgeInfo,
    is_captain: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct PlayerIdentity {
    #[serde(rename = "SubjectID")]
    subject_id: String,
    #[serde(rename = "PlayerCardID")]
    player_card_id: String,
    #[serde(rename = "PlayerTitleID")]
    player_title_id: String,
    account_level: i32,
    #[serde(rename = "PreferredLevelBorderID")]
    preferred_level_border_id: Option<String>,
    incognito: bool,
    hide_account_level: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SeasonalBadgeInfo {
    #[serde(rename = "SeasonID")]
    season_id: Option<String>,
    number_of_wins: i32,
    wins_by_tier: Option<serde_json::Value>,
    rank: i32,
    leaderboard_rank: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Team {
    #[serde(rename = "TeamID")]
    team_id: TeamColor,
    players: Vec<Player>,
}