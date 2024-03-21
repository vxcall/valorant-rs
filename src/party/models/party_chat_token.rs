#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PartyChatTokenResponse {
    token: String,
    room: String,
}
