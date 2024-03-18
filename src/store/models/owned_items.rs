use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct OwnedItemsResponse {
    #[serde(rename = "ItemTypeID")]
    item_type_id: String,
    #[serde(rename = "Entitlements")]
    entitlements: Vec<Entitlement>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Entitlement {
    #[serde(rename = "TypeID")]
    type_id: String,
    #[serde(rename = "ItemID")]
    item_id: String,
    #[serde(rename = "InstanceID")]
    instance_id: Option<String>,
}