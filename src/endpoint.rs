use crate::api_config::ApiConfig;
use reqwest::Method;

#[derive(Debug, Clone)]
pub struct BaseUrls {
    pub shared: String,
    pub pd: String,
    pub glz: String,
    pub localhost: String,
}

#[allow(dead_code)]
pub(crate) enum Endpoint<'a> {
    FetchContent,
    AccountXP,
    PlayerLoadout,
    SetPlayerLoadout,
    PlayerMMR,
    MatchHistory { start_index: Option<&'a str>, end_index: Option<&'a str>, queue: Option<&'a str> },
    MatchDetails { match_id: &'a str },
    CompetitiveUpdates { start_index: Option<&'a str>, end_index: Option<&'a str>, queue: Option<&'a str> },
    Leaderboard { season_id: &'a str, start_index: &'a str, size: &'a str, query: Option<&'a str> },
    Penalties,
    Config,
    NameService,

    Party { party_id: &'a str },
    PartyPlayer,
    PartyRemovePlayer,
    PartySetMemberReady { party_id: &'a str },
    RefreshCompetitiveTier { party_id: &'a str },
    RefreshPlayerIdentity { party_id: &'a str },
    RefreshPings { party_id: &'a str },
    ChangeQueue { party_id: &'a str }, 
    StartCustomGame { party_id: &'a str }, 
    EnterMatchmakingQueue { party_id: &'a str }, 
    LeaveMatchmakingQueue { party_id: &'a str },
    SetPartyAccessibility { party_id: &'a str },
    SetCustomGameSettings { party_id: &'a str },
    PartyInvite { party_id: &'a str, name: &'a str, tagline: &'a str },
    PartyRequest { party_id: &'a str },
    PartyDecline { party_id: &'a str, request_id: &'a str },
    CustomGameConfigs,
    PartyChatToken { party_id: &'a str },
    PartyVoiceToken { party_id: &'a str },
    PartyDisableCode { party_id: &'a str },
    PartyGenerateCode { party_id: &'a str },
    PartyJoinByCode { code: &'a str },

    Prices,
    Storefront,
    Wallet,
    OwnedItems { item_type_id: &'a str },

    PreGamePlayer,
    PreGameMatch { pre_game_match_id: &'a str },
    PreGameLoadouts { pre_game_match_id: &'a str },
    SelectCharacter { pre_game_match_id: &'a str, agent_id: &'a str },
    LockCharacter { pre_game_match_id: &'a str, agent_id: &'a str },
    PreGameQuit { pre_game_match_id: &'a str },

    CurrentGamePlayer,
    CurrentGameMatch { current_game_match_id: &'a str },
    CurrentGameLoadouts { current_game_match_id: &'a str },
    CurrentGameQuit { current_game_match_id: &'a str },

    ItemUpgrades,
    Contracts,
    ActiveContract { contract_id: &'a str },

    EntitlementsToken,
    Sessions,
}

impl<'a> Endpoint<'a> {
    pub fn url(&self, config: &ApiConfig) -> (Method, String) {
        match self {
            Endpoint::FetchContent => {
                (Method::GET, format!("{}/content-service/v3/content", config.base_urls.shared))
            },
            Endpoint::AccountXP => {
                (Method::GET, format!("{}/account-xp/v1/players/{}", config.base_urls.pd, config.puuid))
            },
            Endpoint::PlayerLoadout => {
                (Method::GET, format!("{}/personalization/v2/players/{}/playerloadout", config.base_urls.pd, config.puuid))
            },
            Endpoint::SetPlayerLoadout => {
                (Method::PUT, format!("{}/personalization/v2/players/{}/playerloadout", config.base_urls.pd, config.puuid))
            },
            Endpoint::PlayerMMR => {
                (Method::GET, format!("{}/mmr/v1/players/{}", config.base_urls.pd, config.puuid))
            },
            Endpoint::MatchHistory { start_index, end_index, queue } => {
                let start_index_query = start_index.map_or(String::new(), |start| format!("&startIndex={}", start));
                let end_index_query = end_index.map_or(String::new(), |end| format!("&endIndex={}", end));
                let queue_query = queue.map_or(String::new(), |q| format!("&queue={}", q));
                (Method::GET, format!("{}/match-history/v1/history/{}?{}{}{}", config.base_urls.pd, config.puuid, start_index_query, end_index_query, queue_query))
            },
            Endpoint::MatchDetails { match_id } => {
                (Method::GET, format!("{}/match-details/v1/matches/{}", config.base_urls.pd, match_id))
            },
            Endpoint::CompetitiveUpdates { start_index, end_index, queue } => {
                let start_index_query = start_index.map_or(String::new(), |start| format!("&startIndex={}", start));
                let end_index_query = end_index.map_or(String::new(), |end| format!("&endIndex={}", end));
                let queue_query = queue.map_or(String::new(), |q| format!("&queue={}", q));
                (Method::GET, format!("{}/mmr/v1/players/{}/competitiveupdates?{}{}{}", config.base_urls.pd, config.puuid, start_index_query, end_index_query, queue_query))
            },
            Endpoint::Leaderboard { season_id, start_index, size, query } => {
                let query_q = query.map_or(String::new(), |q| format!("&queue={}", q));
                (Method::GET, format!("{}/mmr/v1/leaderboards/affinity/na/queue/competitive/season/{}?startIndex={}&size={}{}", config.base_urls.pd, season_id, start_index, size, query_q))
            },
            Endpoint::Penalties => {
                (Method::GET, format!("{}/restrictions/v3/penalties", config.base_urls.pd))
            },
            Endpoint::Config => {
                (Method::GET, format!("{}/v1/config/{}", config.base_urls.pd, config.region))
            },
            Endpoint::NameService => {
                (Method::PUT, format!("{}/name-service/v2/players", config.base_urls.pd))
            },
            Endpoint::Party { party_id } => {
                (Method::GET, format!("{}/parties/v1/parties/{}", config.base_urls.glz, party_id))
            },
            Endpoint::PartyPlayer => {
                (Method::GET, format!("{}/parties/v1/players/{}", config.base_urls.glz, config.puuid))
            },
            Endpoint::PartyRemovePlayer => {
                (Method::DELETE, format!("{}/parties/v1/players/{}", config.base_urls.glz, config.puuid))
            },
            Endpoint::PartySetMemberReady { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/members/{}/setReady", config.base_urls.glz, party_id, config.puuid))
            },
            Endpoint::RefreshCompetitiveTier { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/members/{}/refreshCompetitiveTier", config.base_urls.glz, party_id, config.puuid))
            },
            Endpoint::RefreshPlayerIdentity { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/members/{}/refreshPlayerIdentity", config.base_urls.glz, party_id, config.puuid))
            },
            Endpoint::RefreshPings { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/members/{}/refreshPings", config.base_urls.glz, party_id, config.puuid))
            },
            Endpoint::ChangeQueue { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/queue", config.base_urls.glz, party_id))
            },
            Endpoint::StartCustomGame { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/startcustomgame", config.base_urls.glz, party_id))
            },
            Endpoint::EnterMatchmakingQueue { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/matchmaking/join", config.base_urls.glz, party_id))
            },
            Endpoint::LeaveMatchmakingQueue { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/matchmaking/leave", config.base_urls.glz, party_id))
            },
            Endpoint::SetPartyAccessibility { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/accessibility", config.base_urls.glz, party_id))
            },
            Endpoint::SetCustomGameSettings { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/customgamesettings", config.base_urls.glz, party_id))
            },
            Endpoint::PartyInvite { party_id, name, tagline } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/invites/name/{}/tag/{}", config.base_urls.glz, party_id, name, tagline))
            },
            Endpoint::PartyRequest { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/request", config.base_urls.glz, party_id))
            },
            Endpoint::PartyDecline { party_id, request_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/request/{}/decline", config.base_urls.glz, party_id, request_id))
            },
            Endpoint::CustomGameConfigs => {
                (Method::GET, format!("{}/parties/v1/parties/customgameconfigs", config.base_urls.glz))
            },
            Endpoint::PartyChatToken { party_id } => {
                (Method::GET, format!("{}/parties/v1/parties/{}/muctoken", config.base_urls.glz, party_id))
            },
            Endpoint::PartyVoiceToken { party_id } => {
                (Method::GET, format!("{}/parties/v1/parties/{}/voicetoken", config.base_urls.glz, party_id))
            },
            Endpoint::PartyDisableCode { party_id } => {
                (Method::DELETE, format!("{}/parties/v1/parties/{}/invitecode", config.base_urls.glz, party_id))
            },
            Endpoint::PartyGenerateCode { party_id } => {
                (Method::POST, format!("{}/parties/v1/parties/{}/invitecode", config.base_urls.glz, party_id))
            },
            Endpoint::PartyJoinByCode { code } => {
                (Method::POST, format!("{}/parties/v1/players/joinbycode/{}", config.base_urls.glz, code))
            },
            Endpoint::Prices => {
                (Method::GET, format!("{}/store/v1/offers", config.base_urls.pd))
            },
            Endpoint::Storefront => {
                (Method::GET, format!("{}/store/v2/storefront/{}", config.base_urls.pd, config.puuid))
            },
            Endpoint::Wallet => {
                (Method::GET, format!("{}/store/v1/wallet/{}", config.base_urls.pd, config.puuid))
            },
            Endpoint::OwnedItems { item_type_id } => {
                (Method::GET, format!("{}/store/v1/entitlements/{}/{}", config.base_urls.pd, config.puuid, item_type_id))
            },
            Endpoint::PreGamePlayer => {
                (Method::GET, format!("{}/pregame/v1/players/{}", config.base_urls.glz, config.puuid))
            },
            Endpoint::PreGameMatch { pre_game_match_id } => {
                (Method::GET, format!("{}/pregame/v1/matches/{}", config.base_urls.glz, pre_game_match_id))
            },
            Endpoint::PreGameLoadouts { pre_game_match_id } => {
                (Method::GET, format!("{}/pregame/v1/matches/{}/loadouts", config.base_urls.glz, pre_game_match_id))
            },
            Endpoint::SelectCharacter { pre_game_match_id, agent_id } => {
                (Method::POST, format!("{}/pregame/v1/matches/{}/select/{}", config.base_urls.glz, pre_game_match_id, agent_id))
            },
            Endpoint::LockCharacter { pre_game_match_id, agent_id } => {
                (Method::POST, format!("{}/pregame/v1/matches/{}/lock/{}", config.base_urls.glz, pre_game_match_id, agent_id))
            },
            Endpoint::PreGameQuit { pre_game_match_id } => {
                (Method::POST, format!("{}/pregame/v1/matches/{}/quit", config.base_urls.glz, pre_game_match_id))
            },
            Endpoint::CurrentGamePlayer => {
                (Method::GET, format!("{}/core-game/v1/players/{}", config.base_urls.glz, config.puuid))
            },
            Endpoint::CurrentGameMatch { current_game_match_id } => {
                (Method::GET, format!("{}/core-game/v1/matches/{}", config.base_urls.glz, current_game_match_id))
            },
            Endpoint::CurrentGameLoadouts { current_game_match_id } => {
                (Method::GET, format!("{}/core-game/v1/matches/{}/loadouts", config.base_urls.glz, current_game_match_id))
            },
            Endpoint::CurrentGameQuit { current_game_match_id } => {
                (Method::POST, format!("{}/core-game/v1/players/{}/disassociate/{}", config.base_urls.glz, config.puuid, current_game_match_id))
            },
            Endpoint::ItemUpgrades => {
                (Method::GET, format!("{}/contract-definitions/v3/item-upgrades", config.base_urls.pd))
            },
            Endpoint::Contracts => {
                (Method::GET, format!("{}/contracts/v1/contracts/{}", config.base_urls.pd, config.puuid))
            },
            Endpoint::ActiveContract { contract_id } => {
                (Method::POST, format!("{}/contracts/v1/contracts/{}/special/{}", config.base_urls.pd, config.puuid, contract_id))
            },
            Endpoint::EntitlementsToken => {
                (Method::GET, format!("{}/entitlements/v1/token", config.base_urls.localhost))
            },
            Endpoint::Sessions => {
                (Method::GET, format!("{}/product-session/v1/external-sessions", config.base_urls.localhost))
            },
        }
    }
}

