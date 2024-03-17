use serde::{ Deserialize, Serialize };
use std::collections::HashMap;

// Pre Game Player-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct PreGamePlayerResponse {
    subject: String,
    #[serde(rename = "MatchID")]
    match_id: String,
    version: i64,
}

// Pre Game Match-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct PreGameMatchResponse {
    #[serde(rename = "ID")]
    id: String,
    version: i64,
    teams: Vec<Team>,
    ally_team: Option<Team>,
    enemy_team: Option<Team>,
    observer_subjects: Vec<Value>,
    match_coaches: Vec<Value>,
    enemy_team_size: i32,
    enemy_team_lock_count: i32,
    pregame_state: PregameState,
    last_updated: String,
    #[serde(rename = "MapID")]
    map_id: String,
    map_select_pool: Vec<Value>,
    #[serde(rename = "BannedMapIDs")]
    banned_map_ids: Vec<Value>,
    casted_votes: Option<Vec<Value>>,
    map_select_steps: Vec<Value>,
    map_select_step: i32,
    team1: TeamID,
    #[serde(rename = "GamePodID")]
    game_pod_id: String,
    mode: String,
    #[serde(rename = "VoiceSessionID")]
    voice_session_id: String,
    muc_name: String,
    team_match_token: String,
    queue_id: Option<String>,
    #[serde(rename = "ProvisioningFlowID")]
    provisioning_flow_id: String,
    is_ranked: bool,
    #[serde(rename = "PhaseTimeRemainingNS")]
    phase_time_remaining_ns: i64,
    #[serde(rename = "StepTimeNS")]
    step_time_remaining_ns: i64,
    alt_modes_flag_ada: bool,
    tournament_metadata: Option<Value>,
    roster_metadata: Option<Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
enum TeamID {
    Blue,
    Red,
    #[serde(other)]
    Other(String),
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
struct PlayerIdentity {
    subject: String,
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
struct Player {
    subject: String,
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
    team_id: TeamID,
    players: Vec<Player>,
}

// Pre Game Loadouts-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PreGameLoadoutsResponse {
    loadouts: Vec<Loadout>,
    loadouts_valid: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Loadout {
    #[serde(rename = "SubjectID")]
    subject_id: String,
    sprays: SprayContainer,
    expressions: ExpressionContainer,
    items: HashMap<String, Item>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SprayContainer {
    spray_selections: Vec<SpraySelection>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SpraySelection {
    #[serde(rename = "SocketID")]
    socket_id: String,
    #[serde(rename = "SprayID")]
    spray_id: String,
    #[serde(rename = "LevelID")]
    level_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ExpressionContainer {
    aes_selections: Vec<AESSelection>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AESSelection {
    #[serde(rename = "SocketID")]
    socket_id: String,
    #[serde(rename = "AssetID")]
    asset_id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Item {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
    sockets: HashMap<String, Socket>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Socket {
    #[serde(rename = "ID")]
    id: String,
    item: SocketItem,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SocketItem {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "TypeID")]
    type_id: String,
}

// Select Character-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SelectCharacterResponse {
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
    map_select_pool: Vec<serde_json::Value>, // unknown types as flexible JSON
    #[serde(rename = "BannedMapIds")]
    banned_map_ids: Vec<serde_json::Value>, // unknown types as flexible JSON
    casted_votes: Option<Vec<serde_json::Value>>, // Optional and unknown types as flexible JSON
    map_select_steps: Vec<serde_json::Value>, // unknown types as flexible JSON
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
    tournament_metadata: Option<serde_json::Value>, // Null or unknown types as flexible JSON
    roster_metadata: Option<serde_json::Value>, // Null or unknown types as flexible JSON
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
    #[serde(other)]
    Other(String),
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


// Lock Character-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
enum TeamColor {
    Blue,
    Red,
    #[serde(other)]
    Other(String),
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
enum ProvisioningFlowID {
    Matchmaking,
    CustomGame,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct LockCharacterResponse {
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
