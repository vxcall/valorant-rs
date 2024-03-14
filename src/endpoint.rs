use crate::api_config::ApiConfig;

pub(crate) enum Endpoint {
    ContentService, // shared
    PlayerMMR { puuid: String }, // pd
    CurrentGamePlayer { puuid: String }, // glz
    CurrentGameLoadouts { current_game_match_id: String }, // glz
    EntitlementsToken, // 127
}

impl Endpoint {
    pub fn url(&self, config: &ApiConfig) -> String {
        match self {
            Endpoint::ContentService => {
                format!("https://shared.{}.a.pvp.net/content-service/v3/content", config.shard)
            },
            Endpoint::PlayerMMR { puuid } => {
                format!("https://pd.{}.a.pvp.net/mmr/v1/players/{}", config.shard, puuid)
            },
            Endpoint::CurrentGamePlayer { puuid } => {
                format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/players/{}", config.region, config.shard, puuid)
            },
            Endpoint::CurrentGameLoadouts { current_game_match_id } => {
                format!("https://glz-{}-1.{}.a.pvp.net/core-game/v1/matches/{}/loadouts", config.region, config.shard, current_game_match_id)
            },
            Endpoint::EntitlementsToken => {
                format!("https://127.0.0.1:{}/entitlements/v1/token", config.port)
            },
        }
    }
}