use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Prices-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct PricesResponse {
    offers: Vec<OfferForPrices>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct OfferForPrices {
    #[serde(rename = "OfferID")]
    offer_id: String,
    is_direct_purchase: bool,
    start_date: String,
    cost: HashMap<String, f64>,
    rewards: Vec<RewardForPrices>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct RewardForPrices {
    #[serde(rename = "ItemTypeID")]
    item_type_id: String,
    #[serde(rename = "ItemID")]
    item_id: String,
    quantity: u32,
}

// Storefront-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct StorefrontResponse {
    featured_bundle: FeaturedBundle,
    skins_panel_layout: SkinsPanelLayout,
    upgrade_currency_store: UpgradeCurrencyStore,
    accessory_store: AccessoryStore,
    bonus_store: Option<BonusStore>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct FeaturedBundle {
    bundle: BundleDetails,
    bundles: Vec<BundleDetails>,
    bundle_remaining_duration_in_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct BundleDetails {
    #[serde(rename = "ID")]
    id: String,
    #[serde(rename = "DataAssetID")]
    data_asset_id: String,
    #[serde(rename = "CurrencyID")]
    currency_id: String,
    items: Vec<ItemBundle>,
    item_offers: Option<Vec<ItemOffer>>,
    total_base_cost: Option<HashMap<String, f64>>,
    total_discounted_cost: Option<HashMap<String, f64>>,
    total_discount_percent: f64,
    duration_remaining_in_seconds: i64,
    wholesale_only: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ItemBundle {
    item: ItemDetails,
    base_price: f64,
    #[serde(rename = "CurrencyID")]
    currency_id: String,
    discount_percent: f64,
    discounted_price: f64,
    is_promo_item: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ItemDetails {
    #[serde(rename = "ItemTypeID")]
    item_type_id: String,
    #[serde(rename = "ItemID")]
    item_id: String,
    amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct ItemOffer {
    #[serde(rename = "BundleItemOfferID")]
    bundle_item_offer_id: String,
    offer: OfferForStorefront,
    discount_percent: f64,
    discounted_cost: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct OfferForStorefront {
    #[serde(rename = "OfferID")]
    offer_id: String,
    is_direct_purchase: bool,
    start_date: String,
    cost: HashMap<String, f64>,
    rewards: Vec<RewardForStorefront>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct RewardForStorefront {
    #[serde(rename = "ItemTypeID")]
    item_type_id: String,
    #[serde(rename = "ItemID")]
    item_id: String,
    quantity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct SkinsPanelLayout {
    single_item_offers: Vec<String>,
    single_item_store_offers: Vec<OfferForStorefront>,
    single_item_offers_remaining_duration_in_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct UpgradeCurrencyStore {
    upgrade_currency_offers: Vec<UpgradeCurrencyOffer>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct UpgradeCurrencyOffer {
    #[serde(rename = "OfferID")]
    offer_id: String,
    #[serde(rename = "StorefrontItemID")]
    storefront_item_id: String,
    offer: OfferForStorefront,
    discounted_percent: f64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct AccessoryStore {
    accessory_store_offers: Vec<AccessoryStoreOffer>,
    accessory_store_remaining_duration_in_seconds: i64,
    #[serde(rename = "StorefrontID")]
    storefront_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct AccessoryStoreOffer {
    offer: OfferForStorefront,
    #[serde(rename = "ContractID")]
    contract_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct BonusStore {
    bonus_store_offers: Vec<BonusStoreOffer>,
    bonus_store_remaining_duration_in_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
struct BonusStoreOffer {
    #[serde(rename = "BonusOfferID")]
    bonus_offer_id: String,
    offer: OfferForStorefront,
    discount_percent: f64,
    discount_costs: HashMap<String, f64>,
    is_seen: bool,
}

// Wallet-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "PascalCase")]
pub struct WalletResponse {
    balances: HashMap<String, f64>,
}

// Owned Items-------------------------------------------------------------

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

//-------------------------------------------------------------