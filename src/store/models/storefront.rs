use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StorefrontResponse {
    pub featured_bundle: FeaturedBundle,
    pub skins_panel_layout: SkinsPanelLayout,
    pub upgrade_currency_store: UpgradeCurrencyStore,
    pub accessory_store: AccessoryStore,
    pub bonus_store: Option<BonusStore>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct FeaturedBundle {
    pub bundle: BundleDetails,
    pub bundles: Vec<BundleDetails>,
    pub bundle_remaining_duration_in_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BundleDetails {
    #[serde(rename = "ID")]
    pub id: String,
    #[serde(rename = "DataAssetID")]
    pub data_asset_id: String,
    #[serde(rename = "CurrencyID")]
    pub currency_id: String,
    pub items: Vec<ItemBundle>,
    pub item_offers: Option<Vec<ItemOffer>>,
    pub total_base_cost: Option<HashMap<String, f64>>,
    pub total_discounted_cost: Option<HashMap<String, f64>>,
    pub total_discount_percent: f64,
    pub duration_remaining_in_seconds: i64,
    pub wholesale_only: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ItemBundle {
    pub item: ItemDetails,
    pub base_price: f64,
    #[serde(rename = "CurrencyID")]
    pub currency_id: String,
    pub discount_percent: f64,
    pub discounted_price: f64,
    pub is_promo_item: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ItemDetails {
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    #[serde(rename = "ItemID")]
    pub item_id: String,
    pub amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct ItemOffer {
    #[serde(rename = "BundleItemOfferID")]
    pub bundle_item_offer_id: String,
    pub offer: Offer,
    pub discount_percent: f64,
    pub discounted_cost: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Offer {
    #[serde(rename = "OfferID")]
    pub offer_id: String,
    pub is_direct_purchase: bool,
    pub start_date: String,
    pub cost: HashMap<String, f64>,
    pub rewards: Vec<Reward>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct Reward {
    #[serde(rename = "ItemTypeID")]
    pub item_type_id: String,
    #[serde(rename = "ItemID")]
    pub item_id: String,
    pub quantity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct SkinsPanelLayout {
    pub single_item_offers: Vec<String>,
    pub single_item_store_offers: Vec<Offer>,
    pub single_item_offers_remaining_duration_in_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UpgradeCurrencyStore {
    pub upgrade_currency_offers: Vec<UpgradeCurrencyOffer>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct UpgradeCurrencyOffer {
    #[serde(rename = "OfferID")]
    pub offer_id: String,
    #[serde(rename = "StorefrontItemID")]
    pub storefront_item_id: String,
    pub offer: Offer,
    pub discounted_percent: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AccessoryStore {
    pub accessory_store_offers: Vec<AccessoryStoreOffer>,
    pub accessory_store_remaining_duration_in_seconds: i64,
    #[serde(rename = "StorefrontID")]
    pub storefront_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct AccessoryStoreOffer {
    pub offer: Offer,
    #[serde(rename = "ContractID")]
    pub contract_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BonusStore {
    pub bonus_store_offers: Vec<BonusStoreOffer>,
    pub bonus_store_remaining_duration_in_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct BonusStoreOffer {
    #[serde(rename = "BonusOfferID")]
    pub bonus_offer_id: String,
    pub offer: Offer,
    pub discount_percent: f64,
    pub discount_costs: HashMap<String, f64>,
    pub is_seen: bool,
}
