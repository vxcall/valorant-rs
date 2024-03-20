use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct OwnedItemsResponse {
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    #[serde(rename = "Entitlements")]
    pub entitlements: Vec<Entitlement>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Entitlement {
    #[serde(rename = "TypeID")]
    pub type_id: String,
    #[serde(rename = "ItemID")]
    pub item_id: String,
    #[serde(rename = "InstanceID")]
    pub instance_id: Option<String>,
}