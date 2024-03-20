use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigResponse {
    pub last_application: String,
    pub collapsed: HashMap<String, String>,
}

