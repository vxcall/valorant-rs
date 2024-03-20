use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use super::lock_character::LockCharacterResponse;
use super::models::pre_game_player::PreGamePlayerResponse;
use super::pre_game_loadouts::PreGameLoadoutsResponse;
use super::pre_game_match::PreGameMatchResponse;
use super::select_character::SelectCharacterResponse;
use anyhow::Result;


impl ValorantClient {
    /// get_pre_game_player returns player data at agent selection phase.
    pub async fn get_pre_game_player(&self) -> Result<PreGamePlayerResponse> {
        let endpoint = Endpoint::PreGamePlayer;
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let player_response = response.json::<PreGamePlayerResponse>().await.map_err(anyhow::Error::from)?;

        Ok(player_response)
    }

    /// get_pre_game_match returns match data at agent selection phase.
    pub async fn get_pre_game_match(&self, pre_game_match_id: &str) -> Result<PreGameMatchResponse> {
        let endpoint = Endpoint::PreGameMatch { pre_game_match_id: pre_game_match_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let match_response = response.json::<PreGameMatchResponse>().await.map_err(anyhow::Error::from)?;

        Ok(match_response)
    }

    /// get_pre_game_loadouts returns loadouts data at agent selection phase.
    pub async fn get_pre_game_loadouts(&self, pre_game_match_id: &str) -> Result<PreGameLoadoutsResponse> {
        let endpoint = Endpoint::PreGameLoadouts { pre_game_match_id: pre_game_match_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let loadouts_response = response.json::<PreGameLoadoutsResponse>().await.map_err(anyhow::Error::from)?;

        Ok(loadouts_response)
    }

    /// select_character allows u to select a character at agent selection phase.
    /// DON'T use this to insta-lock a character, u may get banned.
    pub async fn select_character(&self, pre_game_match_id: &str, agent_id: &str) -> Result<SelectCharacterResponse> {
        let endpoint = Endpoint::SelectCharacter { pre_game_match_id: pre_game_match_id, agent_id: agent_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let select_character_response = response.json::<SelectCharacterResponse>().await.map_err(anyhow::Error::from)?;

        Ok(select_character_response)
    }

    /// select_character allows u to lock a character at agent selection phase.
    /// DON'T use this to insta-lock a character, u may get banned.
    pub async fn lock_character(&self, pre_game_match_id: &str, agent_id: &str) -> Result<LockCharacterResponse> {
        let endpoint = Endpoint::LockCharacter { pre_game_match_id: pre_game_match_id, agent_id: agent_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let lock_character_response = response.json::<LockCharacterResponse>().await.map_err(anyhow::Error::from)?;

        Ok(lock_character_response)
    }

    pub async fn pre_game_quit(&self, pre_game_match_id: &str) -> Result<()> {
        let endpoint = Endpoint::PreGameQuit { pre_game_match_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;

        Ok(())
    }
}