use crate::api_config::ApiConfig;

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
                format!("https://shared.{}.a.pvp.net/content-service/v3/content", config.shard)
            },
            Endpoint::Prices => {
                format!("https://pd.{}.a.pvp.net/store/v1/offers", config.shard)
            },
            Endpoint::Storefront { puuid } => {
                format!("https://pd.{}.a.pvp.net/store/v2/storefront/{}", config.shard, puuid)
            },
            Endpoint::Wallet { puuid } => {
                format!("https://pd.{}.a.pvp.net/store/v1/wallet/{}", config.shard, puuid)
            },
            Endpoint::OwnedItems { puuid, item_type_id} => {
                format!("https://pd.{}.a.pvp.net/store/v1/entitlements/{}/{}", config.shard, puuid, item_type_id)
            },
            Endpoint::CurrentGamePlayer { puuid } => {
                format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/players/{}", config.region, config.shard, puuid)
            },
            Endpoint::CurrentGameMatch { current_game_match_id } => {
                format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/matches/{}", config.region, config.shard, current_game_match_id)
            },
            Endpoint::CurrentGameLoadouts { current_game_match_id } => {
                format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/matches/{}/loadouts", config.region, config.shard, current_game_match_id)
            },
            Endpoint::CurrentGameQuit { puuid, current_game_match_id } => {
                format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/players/{}/disassociate/{}", config.region, config.shard, puuid, current_game_match_id)
            },
            Endpoint::EntitlementsToken => {
                format!("https://127.0.0.1:{}/entitlements/v1/token", config.port)
            },
        }
    }
}