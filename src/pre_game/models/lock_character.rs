use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LockCharacterResponse {
    #[serde(rename = "ID")]
    id: String,
    version: i64,
    teams: Vec<TeamColor>,
    ally_team: Option<TeamColor>,
    enemy_team: Option<TeamColor>,
    observer_subjects: Vec<serde_json::Value>,
    match_coaches: Vec<serde_json::Value>,
    enemy_team_size: i32,
    enemy_team_lock_count: i32,
    pregame_state: PregameState,
    last_updated: String,
    #[serde(rename = "MapID")]
    map_id: String,
    map_select_pool: Vec<serde_json::Value>,
    #[serde(rename = "BannedMapIDs")]
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
    #[serde(rename = "ProvisioningFlowID")]
    provisioning_flow_id: ProvisioningFlowID,
    is_ranked: bool,
    phase_time_remaining_ns: i64,
    step_time_remaining_ns: i64,
    #[serde(rename = "AltModesFlagADA")]
    alt_modes_flag_ada: bool,
    tournament_metadata: Option<serde_json::Value>,
    roster_metadata: Option<serde_json::Value>,
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
    CharacterSelectionState,
    Provisioned,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
enum ProvisioningFlowID {
    Matchmaking,
    CustomGame,
}

