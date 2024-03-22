use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PartyPlayerResponse {
    #[serde(rename = "ID")]
    pub subject: String,
    pub version: u32,
    #[serde(rename = "ID")]
    pub current_party_id: String,
    pub invites: Option<()>,
    pub requests: Vec<Request>,
    pub platform_info: PlatformInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct Request {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "ID")]
    pub party_id: String,
    #[serde(rename = "ID")]
    pub requested_by_subject: String,
    pub subjects: Vec<String>,
    #[serde(rename = "ID")]
    pub created_at: String,
    #[serde(rename = "ID")]
    pub refreshed_at: String,
    pub expires_in: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PlatformInfo {
    pub platform_type: String,
    pub platform_os: String,
    pub platform_os_version: String,
    pub platform_chipset: String,
}
