use crate::api_config::ApiConfig;

#[derive(Debug, Clone)]
pub struct BaseUrls {
    pub shared: String,
    pub pd: String,
    pub glz: String,
    pub local: String,
}

pub(crate) enum Endpoint<'a> {
    ContentService,
    Prices,
    Storefront { puuid: &'a str },
    Wallet { puuid: &'a str },
    OwnedItems { puuid: &'a str, item_type_id: &'a str },
    CurrentGamePlayer { puuid: &'a str },
    CurrentGameMatch { current_game_match_id: &'a str },
    CurrentGameLoadouts { current_game_match_id: &'a str },
    CurrentGameQuit { puuid: &'a str, current_game_match_id: &'a str },
    EntitlementsToken,
}

impl<'a> Endpoint<'a> {
    pub fn url(&self, config: &ApiConfig) -> String {
        match self {
            Endpoint::ContentService => {
                format!("{}/content-service/v3/content", config.base_urls.shared)
            },
            Endpoint::Prices => {
                format!("{}/store/v1/offers", config.base_urls.pd)
            },
            Endpoint::Storefront { puuid } => {
                format!("{}/store/v2/storefront/{}", config.base_urls.pd, puuid)
            },
            Endpoint::Wallet { puuid } => {
                format!("{}/store/v1/wallet/{}", config.base_urls.pd, puuid)
            },
            Endpoint::OwnedItems { puuid, item_type_id } => {
                format!("{}/store/v1/entitlements/{}/{}", config.base_urls.pd, puuid, item_type_id)
            },
            Endpoint::CurrentGamePlayer { puuid } => {
                format!("{}/core-game/v1/players/{}", config.base_urls.glz, puuid)
            },
            Endpoint::CurrentGameMatch { current_game_match_id } => {
                format!("{}/core-game/v1/matches/{}", config.base_urls.glz, current_game_match_id)
            },
            Endpoint::CurrentGameLoadouts { current_game_match_id } => {
                format!("{}/core-game/v1/matches/{}/loadouts", config.base_urls.glz, current_game_match_id)
            },
            Endpoint::CurrentGameQuit { puuid, current_game_match_id } => {
                format!("{}/core-game/v1/players/{}/disassociate/{}", config.base_urls.glz, puuid, current_game_match_id)
            },
            Endpoint::EntitlementsToken => {
                format!("{}/entitlements/v1/token", config.base_urls.local)
            },
        }
    }
}
