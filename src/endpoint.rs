use crate::api_config::ApiConfig;

#[derive(Debug, Clone)]
pub struct BaseUrls {
    pub shared: String,
    pub pd: String,
    pub glz: String,
    pub localhost: String,
}

pub(crate) enum Endpoint<'a> {
    ContentService,
    Prices,
    Storefront { puuid: &'a str },
    Wallet { puuid: &'a str },
    OwnedItems { puuid: &'a str, item_type_id: &'a str },
    PreGamePlayer { puuid: &'a str },
    PreGameMatch { pre_game_match_id: &'a str },
    PreGameLoadouts { pre_game_match_id: &'a str },
    SelectCharacter { pre_game_match_id: &'a str, agent_id: &'a str },
    LockCharacter { pre_game_match_id: &'a str, agent_id: &'a str },
    PreGameQuit { pre_game_match_id: &'a str },
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
            Endpoint::PreGamePlayer { puuid } => {
                format!("{}/pregame/v1/players/{}", config.base_urls.glz, puuid)
            },
            Endpoint::PreGameMatch { pre_game_match_id } => {
                format!("{}/pregame/v1/matches/{}", config.base_urls.glz, pre_game_match_id)
            },
            Endpoint::PreGameLoadouts { pre_game_match_id } => {
                format!("{}/pregame/v1/matches/{}/loadouts", config.base_urls.glz, pre_game_match_id)
            },
            Endpoint::SelectCharacter { pre_game_match_id, agent_id } => {
                format!("{}/pregame/v1/matches/{}/select/{}", config.base_urls.glz, pre_game_match_id, agent_id)
            },
            Endpoint::LockCharacter { pre_game_match_id, agent_id } => {
                format!("{}/pregame/v1/matches/{}/lock/{}", config.base_urls.glz, pre_game_match_id, agent_id)
            },
            Endpoint::PreGameQuit { pre_game_match_id } => {
                format!("{}/pregame/v1/matches/{}/quit", config.base_urls.glz, pre_game_match_id)
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
                format!("{}/entitlements/v1/token", config.base_urls.localhost)
            },
        }
    }
}
