use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct NameServiceResponseItem {
    pub display_name: String,
    pub subject: String,
    pub game_name: String,
    pub tag_line: String,
}
