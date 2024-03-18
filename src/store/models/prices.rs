use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PricesResponse {
    offers: Vec<Offer>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Offer {
    #[serde(rename = "OfferID")]
    offer_id: String,
    is_direct_purchase: bool,
    start_date: String,
    cost: HashMap<String, f64>,
    rewards: Vec<Reward>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct Reward {
    #[serde(rename = "ItemTypeID")]
    item_type_id: String,
    #[serde(rename = "ItemID")]
    item_id: String,
    quantity: u32,
}