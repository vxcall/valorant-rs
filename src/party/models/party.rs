use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PartyResponse {
    #[serde(rename = "ID")]
    pub id: String,
    pub muc_name: String,
    #[serde(rename = "ID")]
    pub voice_room_id: String,
    pub version: u32,
    pub client_version: String,
    pub members: Vec<Member>,
    pub state: String,
    pub previous_state: String,
    pub state_transition_reason: String,
    pub accessibility: String,
    pub custom_game_data: CustomGameData,
    pub matchmaking_data: MatchmakingData,
    pub invites: Option<()>,
    pub requests: Vec<serde_json::Value>,
    #[serde(rename = "ID")]
    pub queue_entry_time: String,
    pub error_notification: ErrorNotification,
    pub restricted_seconds: u32,
    pub eligible_queues: Vec<String>,
    pub queue_ineligibilities: Vec<String>,
    pub cheat_data: CheatData,
    pub xp_bonuses: Vec<serde_json::Value>,
    #[serde(rename = "ID")]
    pub invite_code: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Member {
    #[serde(rename = "ID")]
    pub subject: String,
    pub competitive_tier: u32,
    pub player_identity: PlayerIdentity,
    pub seasonal_badge_info: Option<()>,
    pub is_owner: Option<bool>,
    pub queue_eligible_remaining_account_levels: u32,
    pub pings: Vec<Ping>,
    pub is_ready: bool,
    pub is_moderator: bool,
    pub use_broadcast_hud: bool,
    pub platform_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlayerIdentity {
    #[serde(rename = "ID")]
    pub subject: String,
    #[serde(rename = "ID")]
    pub player_card_id: String,
    #[serde(rename = "ID")]
    pub player_title_id: String,
    pub account_level: u32,
    #[serde(rename = "ID")]
    pub preferred_level_border_id: String,
    pub incognito: bool,
    pub hide_account_level: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Ping {
    pub ping: u32,
    #[serde(rename = "ID")]
    pub game_pod_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CustomGameData {
    pub settings: Settings,
    pub membership: Membership,
    pub max_party_size: u32,
    pub autobalance_enabled: bool,
    pub autobalance_min_players: u32,
    pub has_recovery_data: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Settings {
    #[serde(rename = "ID")]
    pub map: String,
    pub mode: String,
    pub use_bots: bool,
    pub game_pod: String,
    pub game_rules: Option<GameRules>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct GameRules {
    pub allow_game_modifiers: Option<String>,
    pub is_overtime_win_by_two: Option<String>,
    pub play_out_all_rounds: Option<String>,
    pub skip_match_history: Option<String>,
    pub tournament_mode: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Membership {
    pub team_one: Option<Vec<TeamMember>>,
    pub team_two: Option<Vec<TeamMember>>,
    pub team_spectate: Option<Vec<TeamMember>>,
    pub team_one_coaches: Option<Vec<TeamMember>>,
    pub team_two_coaches: Option<Vec<TeamMember>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct TeamMember {
    #[serde(rename = "ID")]
    pub subject: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct MatchmakingData {
    #[serde(rename = "ID")]
    pub queue_id: String,
    pub preferred_game_pods: Vec<String>,
    pub skill_disparity_rr_penalty: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErrorNotification {
    pub error_type: String,
    pub errored_players: Option<Vec<ErroredPlayer>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ErroredPlayer {
    #[serde(rename = "ID")]
    pub subject: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct CheatData {
    pub game_pod_override: String,
    pub force_post_game_processing: bool,
}
