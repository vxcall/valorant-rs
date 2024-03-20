use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use anyhow::Result;

use super::current_game_loadouts::CurrentGameLoadoutsResponse;
use super::current_game_match::CurrentGameMatchResponse;
use super::current_game_player::CurrentGamePlayerResponse;


impl ValorantClient {
    /// get_current_game_player returns following
    /// - player uuid
    /// - match id
    /// - version
    pub async fn get_current_game_player(&self) -> Result<CurrentGamePlayerResponse> {
        let endpoint = Endpoint::CurrentGamePlayer;
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let player_response = response.json::<CurrentGamePlayerResponse>().await.map_err(anyhow::Error::from)?;

        Ok(player_response)
    }

    /// get_current_game_match returns match state, map id, players info participating in the match and many more stuff.
     pub async fn get_current_game_match(&self, match_id: &str) -> Result<CurrentGameMatchResponse> {
        let endpoint = Endpoint::CurrentGameMatch { current_game_match_id: match_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let match_response = response.json::<CurrentGameMatchResponse>().await.map_err(anyhow::Error::from)?;

        Ok(match_response)
    }

    /// get_current_game_loadouts returns many things including agent info,skins, sprays and many more about participating players.
     pub async fn get_current_game_loadouts(&self, match_id: &str) -> Result<CurrentGameLoadoutsResponse> {
        let endpoint = Endpoint::CurrentGameLoadouts { current_game_match_id: match_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;
        let loadouts_response = response.json::<CurrentGameLoadoutsResponse>().await.map_err(anyhow::Error::from)?;

        Ok(loadouts_response)
    }

    /// quit_current_match allows u to quit the current match.
    pub async fn quit_current_match(&self, match_id: &str) -> Result<()> {
        let endpoint = Endpoint::CurrentGameQuit { current_game_match_id: match_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;

        Ok(())
    }
}