use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchDetailsResponse {
    pub match_info: MatchInfo,
    pub players: Vec<Player>,
    pub bots: Vec<serde_json::Value>,
    pub coaches: Vec<Coach>,
    pub teams: Option<Vec<Team>>,
    pub round_results: Option<Vec<RoundResult>>,
    pub kills: Option<Vec<KillDetail>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MatchInfo {
    pub match_id: String,
    pub map_id: String,
    pub game_pod_id: String,
    pub game_loop_zone: String,
    pub game_server_address: String,
    pub game_version: String,
    pub game_length_millis: Option<i64>,
    pub game_start_millis: i64,
    #[serde(rename = "provisioningFlowID")]
    pub provisioning_flow_id: String,
    pub is_completed: bool,
    pub custom_game_name: String,
    pub force_post_processing: bool,
    #[serde(rename = "queueID")]
    pub queue_id: String,
    pub game_mode: String,
    pub is_ranked: bool,
    pub is_match_sampled: bool,
    pub season_id: String,
    pub completion_state: String,
    pub platform_type: String,
    pub premier_match_info: serde_json::Value,
    #[serde(rename = "partyRRPenalties")]
    pub party_rr_penalties: Option<HashMap<String, i32>>,
    pub should_match_disable_penalties: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub subject: String,
    pub game_name: String,
    pub tag_line: String,
    pub platform_info: PlatformInfo,
    pub team_id: String,
    pub party_id: String,
    pub character_id: String,
    pub stats: Option<Stats>,
    pub round_damage: Option<Vec<RoundDamage>>,
    pub competitive_tier: i32,
    pub is_observer: bool,
    pub player_card: String,
    pub player_title: String,
    pub preferred_level_border: Option<String>,
    pub account_level: i32,
    pub session_playtime_minutes: Option<Option<i32>>,
    pub xp_modifications: Option<Vec<XPModification>>,
    pub behavior_factors: Option<BehaviorFactors>,
    pub new_player_experience_details: Option<NewPlayerExperienceDetails>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlatformInfo {
    pub platform_type: String,
    #[serde(rename = "platformOS")]
    pub platform_os: String,
    #[serde(rename = "platformOSVersion")]
    pub platform_os_version: String,
    pub platform_chipset: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Stats {
    pub score: i32,
    pub rounds_played: i32,
    pub kills: i32,
    pub deaths: i32,
    pub assists: i32,
    pub playtime_millis: i64,
    pub ability_casts: Option<AbilityCasts>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbilityCasts {
    pub grenade_casts: i32,
    pub ability1_casts: i32,
    pub ability2_casts: i32,
    pub ultimate_casts: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoundDamage {
    pub round: i32,
    pub receiver: String,
    pub damage: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct XPModification {
    pub value: i32,
    #[serde(rename = "ID")]
    pub id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BehaviorFactors {
    pub afk_rounds: i32,
    pub collisions: Option<i32>,
    pub comms_rating_recovery: i32,
    pub damage_participation_outgoing: i32,
    pub friendly_fire_incoming: Option<i32>,
    pub friendly_fire_outgoing: Option<i32>,
    pub mouse_movement: Option<i32>,
    pub stayed_in_spawn_rounds: Option<i32>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NewPlayerExperienceDetails {
    pub basic_movement: BasicMovement,
    pub basic_gun_skill: BasicGunSkill,
    pub adaptive_bots: AdaptiveBots,
    pub ability: AbilityExperience,
    pub bomb_plant: BombPlant,
    pub defend_bomb_site: DefendBombSite,
    pub setting_status: SettingStatus,
    pub version_string: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicMovement {
    pub idle_time_millis: i64,
    pub objective_complete_time_millis: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BasicGunSkill {
    pub idle_time_millis: i64,
    pub objective_complete_time_millis: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AdaptiveBots {
    pub adaptive_bot_average_duration_millis_all_attempts: i64,
    pub adaptive_bot_average_duration_millis_first_attempt: i64,
    pub kill_details_first_attempt: Option<serde_json::Value>, // Placeholder for null
    pub idle_time_millis: i64,
    pub objective_complete_time_millis: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbilityExperience {
    pub idle_time_millis: i64,
    pub objective_complete_time_millis: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BombPlant {
    pub idle_time_millis: i64,
    pub objective_complete_time_millis: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DefendBombSite {
    pub success: bool,
    pub idle_time_millis: i64,
    pub objective_complete_time_millis: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SettingStatus {
    pub is_mouse_sensitivity_default: bool,
    pub is_crosshair_default: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Coach {
    pub subject: String,
    pub team_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Team {
    pub team_id: String,
    pub won: bool,
    pub rounds_played: i32,
    pub rounds_won: i32,
    pub num_points: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RoundResult {
    pub round_num: i32,
    pub round_result: String,
    pub round_ceremony: String,
    pub winning_team: String,
    pub bomb_planter: Option<String>,
    pub bomb_defuser: Option<String>,
    pub plant_round_time: Option<i64>,
    pub plant_player_locations: Option<Vec<PlayerLocation>>,
    pub plant_location: Location,
    pub plant_site: String,
    pub defuse_round_time: Option<i64>,
    pub defuse_player_locations: Option<Vec<PlayerLocation>>,
    pub defuse_location: Location,
    pub player_stats: Vec<PlayerStats>,
    pub round_result_code: String,
    pub player_economies: Option<Vec<PlayerEconomy>>,
    pub player_scores: Option<Vec<PlayerScore>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerLocation {
    pub subject: String,
    pub view_radians: f64,
    pub location: Location,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerStats {
    pub subject: String,
    pub kills: Vec<KillDetail>,
    pub damage: Vec<Damage>,
    pub score: i32,
    pub economy: Economy,
    pub ability: AbilityEffects,
    pub was_afk: bool,
    pub was_penalized: bool,
    pub stayed_in_spawn: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct KillDetail {
    pub game_time: i64,
    pub round_time: i64,
    pub killer: String,
    pub victim: String,
    pub victim_location: Location,
    pub assistants: Vec<String>,
    pub player_locations: Vec<PlayerLocation>,
    pub finishing_damage: FinishingDamage,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Damage {
    pub receiver: String,
    pub damage: i32,
    pub legshots: i32,
    pub bodyshots: i32,
    pub headshots: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Economy {
    pub loadout_value: i32,
    pub weapon: String,
    pub armor: String,
    pub remaining: i32,
    pub spent: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbilityEffects {
    pub grenade_effects: Option<serde_json::Value>,
    pub ability1_effects: Option<serde_json::Value>,
    pub ability2_effects: Option<serde_json::Value>,
    pub ultimate_effects: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FinishingDamage {
    pub damage_type: String,
    pub damage_item: String,
    pub is_secondary_fire_mode: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerEconomy {
    pub subject: String,
    pub loadout_value: i32,
    pub weapon: String,
    pub armor: String,
    pub remaining: i32,
    pub spent: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PlayerScore {
    pub subject: String,
    pub score: i32,
}
