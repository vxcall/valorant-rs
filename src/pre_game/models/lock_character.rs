use serde::{ Deserialize, Serialize };

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct LockCharacterResponse {
    #[serde(rename = "ID")]
    pub id: String,
    pub version: i64,
    pub teams: Vec<TeamColor>,
    pub ally_team: Option<TeamColor>,
    pub enemy_team: Option<TeamColor>,
    pub observer_subjects: Vec<serde_json::Value>,
    pub match_coaches: Vec<serde_json::Value>,
    pub enemy_team_size: i32,
    pub enemy_team_lock_count: i32,
    pub pregame_state: PregameState,
    pub last_updated: String,
    #[serde(rename = "MapID")]
    pub map_id: String,
    pub map_select_pool: Vec<serde_json::Value>,
    #[serde(rename = "BannedMapIDs")]
    pub banned_map_ids: Vec<serde_json::Value>,
    pub casted_votes: Option<Vec<serde_json::Value>>,
    pub map_select_steps: Vec<serde_json::Value>,
    pub map_select_step: i32,
    pub team1: TeamColor,
    #[serde(rename = "GamePodID")]
    pub game_pod_id: String,
    pub mode: String,
    #[serde(rename = "VoiceSessionID")]
    pub voice_session_id: String,
    pub muc_name: String,
    pub team_match_token: String,
    #[serde(rename = "QueueID")]
    pub queue_id: Option<String>,
    #[serde(rename = "ProvisioningFlowID")]
    pub provisioning_flow_id: ProvisioningFlowID,
    pub is_ranked: bool,
    pub phase_time_remaining_ns: i64,
    pub step_time_remaining_ns: i64,
    #[serde(rename = "altModesFlagADA")]
    pub alt_modes_flag_ada: bool,
    pub tournament_metadata: Option<serde_json::Value>,
    pub roster_metadata: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum TeamColor {
    Blue,
    Red,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum CharacterSelectionState {
    Selected,
    Locked,
    #[serde(rename = "")]
    None,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum PregameState {
    CharacterSelectionState,
    Provisioned,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub enum ProvisioningFlowID {
    Matchmaking,
    CustomGame,
}

