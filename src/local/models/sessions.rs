use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
pub struct SessionsResponse(pub HashMap<String, Session>);

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Session {
    pub exit_code: i32,
    #[serde(default)] // Handles `null` by using the default value, which is `None` for `Option<T>`
    pub exit_reason: Option<serde_json::Value>, // Using serde_json::Value for potential future expansion; use Option<()> if strictly null
    pub is_internal: bool,
    #[serde(rename = "launchConfiguration")]
    pub launch_configuration: LaunchConfiguration,
    #[serde(rename = "patchlineFullName")]
    pub patchline_full_name: PatchlineFullName,
    #[serde(rename = "patchlineId")]
    pub patchline_id: PatchlineId,
    pub phase: String,
    pub product_id: ProductId,
    pub version: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LaunchConfiguration {
    pub arguments: Vec<String>,
    pub executable: String,
    #[serde(default)] // Nullable field
    pub locale: Option<String>,
    #[serde(default)] // Nullable field
    pub voice_locale: Option<serde_json::Value>, // Similarly, use Option<()> if strictly null
    #[serde(rename = "workingDirectory")]
    pub working_directory: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PatchlineFullName {
    #[serde(rename = "VALORANT")]
    Valorant,
    #[serde(rename = "riot_client")]
    RiotClient,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum PatchlineId {
    #[serde(rename = "")]
    Empty,
    #[serde(rename = "live")]
    Live,
    #[serde(rename = "pbe")]
    Pbe,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ProductId {
    #[serde(rename = "valorant")]
    Valorant,
    #[serde(rename = "riot_client")]
    RiotClient,
}
