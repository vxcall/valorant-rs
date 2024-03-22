use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct PartyChatTokenResponse {
    pub token: String,
    pub room: String,
}
