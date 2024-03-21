#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct CustomGameConfigsResponse {
    enabled: bool,
    enabled_maps: Vec<String>,
    enabled_modes: Vec<String>,
    queues: Vec<Queue>,
    game_pod_ping_service_info: std::collections::HashMap<String, GamePodPingServiceInfo>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Queue {
    #[serde(rename = "ID")]
    queue_id: String,
    enabled: bool,
    team_size: u32,
    num_teams: u32,
    max_party_size: u32,
    min_party_size: u32,
    invalid_party_sizes: Vec<u32>,
    max_party_size_high_skill: u32,
    high_skill_tier: u32,
    max_skill_tier: u32,
    allow_full_party_bypass_skill_restrictions: bool,
    mode: String,
    is_ranked: bool,
    is_tournament: bool,
    require_roster: bool,
    priority: i32,
    party_max_competitive_tier_range: u32,
    party_max_competitive_tier_range_placement_buffer: u32,
    full_party_max_competitive_tier_range: u32,
    party_skill_disparity_competitive_tiers_ceilings: std::collections::HashMap<String, u32>,
    use_account_level_requirement: bool,
    minimum_account_level_required: u32,
    game_rules: std::collections::HashMap<String, String>,
    supported_platform_types: Vec<String>,
    disabled_content: Vec<serde_json::Value>,
    queue_field_a: Vec<serde_json::Value>,
    next_schedule_change_seconds: u32,
    time_until_next_schedule_change_seconds: u32,
    map_weights: Vec<String>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct GamePodPingServiceInfo {
    security_hash: u64,
    obfuscated_ip: u64,
    ping_proxy_address: String,
    ping_proxy_addresses: Vec<String>,
}
