#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
struct PartyVoiceTokenResponse {
    token: String,
    room: String,
}
