use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use super::{models::{ PreGameLoadoutsResponse, PreGameMatchResponse, PreGamePlayerResponse }, LockCharacterResponse, SelectCharacterResponse};
use anyhow::Result;


impl ValorantClient {
    pub async fn get_pre_game_player(&self) -> Result<PreGamePlayerResponse> {
        let endpoint = Endpoint::PreGamePlayer { puuid: &self.config.puuid };
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let player_response = response.json::<PreGamePlayerResponse>().await.map_err(anyhow::Error::from)?;

        Ok(player_response)
    }

    pub async fn get_pre_game_match(&self, match_id: &str) -> Result<PreGameMatchResponse> {
        let endpoint = Endpoint::PreGameMatch { pre_game_match_id: match_id };
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let match_response = response.json::<PreGameMatchResponse>().await.map_err(anyhow::Error::from)?;

        Ok(match_response)
    }

    pub async fn get_pre_game_loadouts(&self, match_id: &str) -> Result<PreGameLoadoutsResponse> {
        let endpoint = Endpoint::PreGameLoadouts { pre_game_match_id: match_id };
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let loadouts_response = response.json::<PreGameLoadoutsResponse>().await.map_err(anyhow::Error::from)?;

        Ok(loadouts_response)
    }

    pub async fn select_character(&self, match_id: &str, agent_id: &str) -> Result<SelectCharacterResponse> {
        let endpoint = Endpoint::SelectCharacter { pre_game_match_id: match_id, agent_id: agent_id };
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let loadouts_response = response.json::<SelectCharacterResponse>().await.map_err(anyhow::Error::from)?;

        Ok(loadouts_response)
    }

    pub async fn lock_character(&self, match_id: &str, agent_id: &str) -> Result<LockCharacterResponse> {
        let endpoint = Endpoint::LockCharacter { pre_game_match_id: match_id, agent_id: agent_id };
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let loadouts_response = response.json::<LockCharacterResponse>().await.map_err(anyhow::Error::from)?;

        Ok(loadouts_response)
    }
}