use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct VersionResponse {
    pub status: i32,
    pub data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all="camelCase")]
pub struct Data {
    pub manifest_id: String,
    pub branch: String,
    pub version: String,
    pub build_version: String,
    pub engine_version: String,
    pub riot_client_version: String,
    pub riot_client_build: String,
    pub build_date: String,
}