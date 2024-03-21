use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct AgentsResponse {
    pub status: i32,
    pub data: Vec<AgentData>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct AgentData {
    pub uuid: String,
    pub display_name: String,
    pub description: String,
    pub developer_name: String,
    pub character_tags: Option<Vec<String>>,
    pub display_icon: Option<String>,
    pub display_icon_small: String,
    pub bust_portrait: Option<String>,
    pub full_portrait: Option<String>,
    pub full_portrait_v2: Option<String>,
    pub killfeed_portrait: String,
    pub background: String,
    pub background_gradient_colors: Vec<String>,
    pub asset_path: String,
    pub is_full_portrait_right_facing: bool,
    pub is_playable_character: bool,
    pub is_available_for_test: bool,
    pub is_base_content: bool,
    pub role: Role,
    pub recruitment_data: Option<RecruitmentData>,
    pub abilities: Vec<Ability>,
    pub voice_line: Option<VoiceLine>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Role {
    pub uuid: String,
    pub display_name: String,
    pub description: String,
    pub display_icon: Option<String>,
    pub asset_path: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct RecruitmentData {
    pub counter_id: String,
    pub milestone_id: String,
    pub milestone_threshold: i32,
    pub use_level_vp_cost_override: bool,
    pub level_vp_cost_override: i32,
    pub start_date: String, // dateTime format, assuming String for simplicity
    pub end_date: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Ability {
    pub slot: String,
    pub display_name: String,
    pub description: String,
    pub display_icon: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct VoiceLine {
    pub min_duration: f32, // single in Rust is represented by f32
    pub max_duration: f32,
    pub media_list: Vec<Media>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Media {
    pub id: i32,
    pub wwise: String,
    pub wave: String,
}
