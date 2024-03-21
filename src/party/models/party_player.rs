#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PartyPlayerResponse {
    #[serde(rename = "ID")]
    subject: String,
    version: u32,
    #[serde(rename = "ID")]
    current_party_id: String,
    invites: Option<()>,
    requests: Vec<Request>,
    platform_info: PlatformInfo,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct Request {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "ID")]
    party_id: String,
    #[serde(rename = "ID")]
    requested_by_subject: String,
    subjects: Vec<String>,
    #[serde(rename = "ID")]
    created_at: String,
    #[serde(rename = "ID")]
    refreshed_at: String,
    expires_in: u32,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PlatformInfo {
    platform_type: String,
    platform_os: String,
    platform_os_version: String,
    platform_chipset: String,
}
