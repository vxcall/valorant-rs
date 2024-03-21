use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PartyResponse {
    #[serde(rename = "ID")]
    id: String,
    muc_name: String,
    #[serde(rename = "ID")]
    voice_room_id: String,
    version: u32,
    client_version: String,
    members: Vec<Member>,
    state: String,
    previous_state: String,
    state_transition_reason: String,
    accessibility: String,
    custom_game_data: CustomGameData,
    matchmaking_data: MatchmakingData,
    invites: Option<()>,
    requests: Vec<serde_json::Value>,
    #[serde(rename = "ID")]
    queue_entry_time: String,
    error_notification: ErrorNotification,
    restricted_seconds: u32,
    eligible_queues: Vec<String>,
    queue_ineligibilities: Vec<String>,
    cheat_data: CheatData,
    xp_bonuses: Vec<serde_json::Value>,
    #[serde(rename = "ID")]
    invite_code: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Member {
    #[serde(rename = "ID")]
    subject: String,
    competitive_tier: u32,
    player_identity: PlayerIdentity,
    seasonal_badge_info: Option<()>,
    is_owner: Option<bool>,
    queue_eligible_remaining_account_levels: u32,
    pings: Vec<Ping>,
    is_ready: bool,
    is_moderator: bool,
    use_broadcast_hud: bool,
    platform_type: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PlayerIdentity {
    #[serde(rename = "ID")]
    subject: String,
    #[serde(rename = "ID")]
    player_card_id: String,
    #[serde(rename = "ID")]
    player_title_id: String,
    account_level: u32,
    #[serde(rename = "ID")]
    preferred_level_border_id: String,
    incognito: bool,
    hide_account_level: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Ping {
    ping: u32,
    #[serde(rename = "ID")]
    game_pod_id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CustomGameData {
    settings: Settings,
    membership: Membership,
    max_party_size: u32,
    autobalance_enabled: bool,
    autobalance_min_players: u32,
    has_recovery_data: bool,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Settings {
    #[serde(rename = "ID")]
    map: String,
    mode: String,
    use_bots: bool,
    game_pod: String,
    game_rules: Option<GameRules>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GameRules {
    allow_game_modifiers: Option<String>,
    is_overtime_win_by_two: Option<String>,
    play_out_all_rounds: Option<String>,
    skip_match_history: Option<String>,
    tournament_mode: Option<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Membership {
    team_one: Option<Vec<TeamMember>>,
    team_two: Option<Vec<TeamMember>>,
    team_spectate: Option<Vec<TeamMember>>,
    team_one_coaches: Option<Vec<TeamMember>>,
    team_two_coaches: Option<Vec<TeamMember>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct TeamMember {
    #[serde(rename = "ID")]
    subject: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct MatchmakingData {
    #[serde(rename = "ID")]
    queue_id: String,
    preferred_game_pods: Vec<String>,
    skill_disparity_rr_penalty: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ErrorNotification {
    error_type: String,
    errored_players: Option<Vec<ErroredPlayer>>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct ErroredPlayer {
    #[serde(rename = "ID")]
    subject: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CheatData {
    game_pod_override: String,
    force_post_game_processing: bool,
}
