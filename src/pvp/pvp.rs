use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use anyhow::Result;
use base64::{ engine::general_purpose::STANDARD, Engine };
use base64;

use super::account_xp::AccountXPResponse;
use super::competitive_updates::CompetitiveUpdatesResponse;
use super::config::ConfigResponse;
use super::fetch_content::FetchContentResponse;
use super::leaderboard::LeaderboardResponse;
use super::match_details::MatchDetailsResponse;
use super::match_history::MatchHistoryResponse;
use super::penalties::PenaltiesResponse;
use super::player_loadout::PlayerLoadoutResponse;
use super::player_mmr::PlayerMMRResponse;


impl ValorantClient {
    pub async fn fetch_content(&self) -> Result<FetchContentResponse> {
        let client_platform = r#"{
    "platformType": "PC",
    "platformOS": "Windows",
    "platformOSVersion": "10.0.19042.1.256.64bit",
    "platformChipset": "Unknown"
}"#;

        let endpoint = Endpoint::FetchContent;
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request
            .header("X-Riot-ClientPlatform", STANDARD.encode(client_platform))
            .header("X-Riot-ClientVersion", "release-08.04-shipping-4-2324912")
            .send()
            .await
            .map_err(anyhow::Error::from)?;

        let fetch_content_response = response.json::<FetchContentResponse>().await.map_err(anyhow::Error::from)?;

        Ok(fetch_content_response)
    }

    pub async fn get_account_xp(&self) -> Result<AccountXPResponse> {
        let endpoint = Endpoint::AccountXP;
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let account_xp_response = response.json::<AccountXPResponse>().await.map_err(anyhow::Error::from)?;

        Ok(account_xp_response)
    }

    pub async fn get_player_loadout(&self) -> Result<PlayerLoadoutResponse> {
        let endpoint = Endpoint::PlayerLoadout;
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let loadout_response = response.json::<PlayerLoadoutResponse>().await.map_err(anyhow::Error::from)?;

        Ok(loadout_response)
    }

    pub async fn get_player_mmr(&self) -> Result<PlayerMMRResponse> {
        let endpoint = Endpoint::PlayerMMR;
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let mmr_response = response.json::<>().await.map_err(anyhow::Error::from)?;

        Ok(mmr_response)
    }

    pub async fn get_match_history(&self, start_index: &str, end_index: &str, queue: &str) -> Result<MatchHistoryResponse> {
        let endpoint = Endpoint::MatchHistory { start_index, end_index, queue };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let match_history_response = response.json::<>().await.map_err(anyhow::Error::from)?;

        Ok(match_history_response)
    }

    pub async fn get_match_details(&self, match_id: &str) -> Result<MatchDetailsResponse> {
        let endpoint = Endpoint::MatchDetails { match_id: match_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let match_details_response = response.json::<>().await.map_err(anyhow::Error::from)?;

        Ok(match_details_response)
    }

    pub async fn get_competitive_updates(&self, start_index: &str, end_index: &str, queue: &str) -> Result<CompetitiveUpdatesResponse> {
        let endpoint = Endpoint::CompetitiveUpdates { start_index, end_index, queue };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let competitive_updates_response = response.json::<>().await.map_err(anyhow::Error::from)?;

        Ok(competitive_updates_response)
    }

    pub async fn get_leaderboard(&self, season_id: &str, start_index: &str, size: &str, query: &str) -> Result<LeaderboardResponse> {
        let endpoint = Endpoint::Leaderboard { season_id, start_index, size, query };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let leaderboard_response = response.json::<>().await.map_err(anyhow::Error::from)?;

        Ok(leaderboard_response)
    }

    pub async fn get_penalties(&self) -> Result<PenaltiesResponse> {
        let endpoint = Endpoint::Penalties;
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let leaderboard_response = response.json::<>().await.map_err(anyhow::Error::from)?;

        Ok(leaderboard_response)
    }

    pub async fn get_config(&self) -> Result<ConfigResponse> {
        let endpoint = Endpoint::Config;
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let leaderboard_response = response.json::<>().await.map_err(anyhow::Error::from)?;

        Ok(leaderboard_response)
    }
}