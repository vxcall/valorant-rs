use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use super::models::{ PricesResponse, StorefrontResponse, WalletResponse, OwnedItemsResponse };
use reqwest::Method;
use anyhow::Result;

#[derive(Debug, Clone, Copy)]
pub enum ItemType {
    Agents,
    Contracts,
    Sprays,
    GunBuddies,
    Cards,
    Skins,
    SkinVariants,
    Titles,
}

impl ItemType {
    pub fn item_type_id(&self) -> &'static str {
        match self {
            ItemType::Agents => "01bb38e1-da47-4e6a-9b3d-945fe4655707",
            ItemType::Contracts => "f85cb6f7-33e5-4dc8-b609-ec7212301948",
            ItemType::Sprays => "d5f120f8-ff8c-4aac-92ea-f2b5acbe9475",
            ItemType::GunBuddies => "dd3bf334-87f3-40bd-b043-682a57a8dc3a",
            ItemType::Cards => "3f296c07-64c3-494c-923b-fe692a4fa1bd",
            ItemType::Skins => "e7c63390-eda7-46e0-bb7a-a6abdacd2433",
            ItemType::SkinVariants => "3ad1b2b2-acdb-4524-852f-954a76ddae0a",
            ItemType::Titles => "de7caa6b-adf7-4588-bbd1-143831e786c6",
        }
    }
}


impl ValorantClient {
    pub async fn get_prices(&self) -> Result<PricesResponse> {
        let endpoint = Endpoint::Prices;
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let prices_response = response.json::<PricesResponse>().await.map_err(anyhow::Error::from)?;

        Ok(prices_response)
    }

    pub async fn get_storefront(&self) -> Result<StorefrontResponse> {
        let endpoint = Endpoint::Storefront { puuid: &self.config.puuid };
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let storefront_response = response.json::<StorefrontResponse>().await.map_err(anyhow::Error::from)?;

        Ok(storefront_response)
    }

    pub async fn get_wallet(&self) -> Result<WalletResponse> {
        let endpoint = Endpoint::Wallet { puuid: &self.config.puuid };
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let wallet_response = response.json::<WalletResponse>().await.map_err(anyhow::Error::from)?;

        Ok(wallet_response)
    }

    pub async fn get_owned_items(&self, item_type: ItemType) -> Result<OwnedItemsResponse> {
        let endpoint = Endpoint::OwnedItems { puuid: &self.config.puuid, item_type_id: item_type.item_type_id() };
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let owned_items_response = response.json::<OwnedItemsResponse>().await.map_err(anyhow::Error::from)?;

        Ok(owned_items_response)
    }
}