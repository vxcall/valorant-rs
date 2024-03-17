use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct PricesResponse {
    #[serde(rename = "Offers")]
    offers: Vec<OfferForPrices>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OfferForPrices {
    #[serde(rename = "OfferID")]
    offer_id: String,

    #[serde(rename = "IsDirectPurchase")]
    is_direct_purchase: bool,

    #[serde(rename = "StartDate")]
    start_date: String,

    #[serde(rename = "Cost")]
    cost: HashMap<String, f64>,

    #[serde(rename = "Rewards")]
    rewards: Vec<RewardForPrices>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RewardForPrices {
    #[serde(rename = "ItemTypeID")]
    item_type_id: String,

    #[serde(rename = "ItemID")]
    item_id: String,

    #[serde(rename = "Quantity")]
    quantity: u32,
}

//-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct StorefrontResponse {
    #[serde(rename = "FeaturedBundle")]
    featured_bundle: FeaturedBundle,

    #[serde(rename = "SkinsPanelLayout")]
    skins_panel_layout: SkinsPanelLayout,

    #[serde(rename = "UpgradeCurrencyStore")]
    upgrade_currency_store: UpgradeCurrencyStore,

    #[serde(rename = "AccessoryStore")]
    accessory_store: AccessoryStore,

    #[serde(rename = "BonusStore")]
    bonus_store: Option<BonusStore>,
}

#[derive(Serialize, Deserialize, Debug)]
struct FeaturedBundle {
    #[serde(rename = "Bundle")]
    bundle: BundleDetails,

    #[serde(rename = "Bundles")]
    bundles: Vec<BundleDetails>,

    #[serde(rename = "BundleRemainingDurationInSeconds")]
    bundle_remaining_duration_in_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct BundleDetails {
    #[serde(rename = "ID")]
    id: String,

    #[serde(rename = "DataAssetID")]
    data_asset_id: String,

    #[serde(rename = "CurrencyID")]
    currency_id: String,

    #[serde(rename = "Items")]
    items: Vec<ItemBundle>,

    #[serde(rename = "ItemOffers")]
    item_offers: Option<Vec<ItemOffer>>,

    #[serde(rename = "TotalBaseCost")]
    total_base_cost: Option<HashMap<String, f64>>,

    #[serde(rename = "TotalDiscountedCost")]
    total_discounted_cost: Option<HashMap<String, f64>>,

    #[serde(rename = "TotalDiscountPercent")]
    total_discount_percent: f64,

    #[serde(rename = "DurationRemainingInSeconds")]
    duration_remaining_in_seconds: i64,

    #[serde(rename = "WholesaleOnly")]
    wholesale_only: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemBundle {
    #[serde(rename = "Item")]
    item: ItemDetails,

    #[serde(rename = "BasePrice")]
    base_price: f64,

    #[serde(rename = "CurrencyID")]
    currency_id: String,

    #[serde(rename = "DiscountPercent")]
    discount_percent: f64,

    #[serde(rename = "DiscountedPrice")]
    discounted_price: f64,

    #[serde(rename = "IsPromoItem")]
    is_promo_item: bool,
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemDetails {
    #[serde(rename = "ItemTypeID")]
    item_type_id: String,

    #[serde(rename = "ItemID")]
    item_id: String,

    #[serde(rename = "Amount")]
    amount: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct ItemOffer {
    #[serde(rename = "BundleItemOfferID")]
    bundle_item_offer_id: String,

    #[serde(rename = "Offer")]
    offer: OfferForStorefront,

    #[serde(rename = "DiscountPercent")]
    discount_percent: f64,

    #[serde(rename = "DiscountedCost")]
    discounted_cost: HashMap<String, f64>,
}

#[derive(Serialize, Deserialize, Debug)]
struct OfferForStorefront {
    #[serde(rename = "OfferID")]
    offer_id: String,

    #[serde(rename = "IsDirectPurchase")]
    is_direct_purchase: bool,

    #[serde(rename = "StartDate")]
    start_date: String,

    #[serde(rename = "Cost")]
    cost: HashMap<String, f64>,

    #[serde(rename = "Rewards")]
    rewards: Vec<RewardForStorefront>,
}

#[derive(Serialize, Deserialize, Debug)]
struct RewardForStorefront {
    #[serde(rename = "ItemTypeID")]
    item_type_id: String,

    #[serde(rename = "ItemID")]
    item_id: String,

    #[serde(rename = "Quantity")]
    quantity: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct SkinsPanelLayout {
    #[serde(rename = "SingleItemOffers")]
    single_item_offers: Vec<String>,

    #[serde(rename = "SingleItemStoreOffers")]
    single_item_store_offers: Vec<OfferForStorefront>,

    #[serde(rename = "SingleItemOffersRemainingDurationInSeconds")]
    single_item_offers_remaining_duration_in_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpgradeCurrencyStore {
    #[serde(rename = "UpgradeCurrencyOffers")]
    upgrade_currency_offers: Vec<UpgradeCurrencyOffer>,
}

#[derive(Serialize, Deserialize, Debug)]
struct UpgradeCurrencyOffer {
    #[serde(rename = "OfferID")]
    offer_id: String,

    #[serde(rename = "StorefrontItemID")]
    storefront_item_id: String,

    #[serde(rename = "Offer")]
    offer: OfferForStorefront,

    #[serde(rename = "DiscountedPercent")]
    discounted_percent: f64,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccessoryStore {
    #[serde(rename = "AccessoryStoreOffers")]
    accessory_store_offers: Vec<AccessoryStoreOffer>,

    #[serde(rename = "AccessoryStoreRemainingDurationInSeconds")]
    accessory_store_remaining_duration_in_seconds: i64,

    #[serde(rename = "StorefrontID")]
    storefront_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct AccessoryStoreOffer {
    #[serde(rename = "Offer")]
    offer: OfferForStorefront,

    #[serde(rename = "ContractID")]
    contract_id: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct BonusStore {
    #[serde(rename = "BonusStoreOffers")]
    bonus_store_offers: Vec<BonusStoreOffer>,

    #[serde(rename = "BonusStoreRemainingDurationInSeconds")]
    bonus_store_remaining_duration_in_seconds: i64,
}

#[derive(Serialize, Deserialize, Debug)]
struct BonusStoreOffer {
    #[serde(rename = "BonusOfferID")]
    bonus_offer_id: String,

    #[serde(rename = "Offer")]
    offer: OfferForStorefront,

    #[serde(rename = "DiscountPercent")]
    discount_percent: f64,

    #[serde(rename = "DiscountCosts")]
    discount_costs: HashMap<String, f64>,

    #[serde(rename = "IsSeen")]
    is_seen: bool,
}

//-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct WalletResponse {
    #[serde(rename = "Balances")]
    balances: HashMap<String, f64>,
}

//-------------------------------------------------------------

#[derive(Serialize, Deserialize, Debug)]
pub struct OwnedItemsResponse {
    #[serde(rename = "ItemTypeID")]
    item_type_id: String,

    #[serde(rename = "Entitlements")]
    entitlements: Vec<Entitlement>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Entitlement {
    #[serde(rename = "TypeID")]
    type_id: String,

    #[serde(rename = "ItemID")]
    item_id: String,

    #[serde(rename = "InstanceID")]
    instance_id: Option<String>, // Optional to handle the undefined case
}

//-------------------------------------------------------------