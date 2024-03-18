use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use super::models::{CurrentGameLoadoutsResponse, CurrentGameMatchResponse, CurrentGamePlayerResponse};
use reqwest::Method;
use anyhow::Result;


impl ValorantClient {
    pub async fn get_current_game_player(&self) -> Result<CurrentGamePlayerResponse> {
        let endpoint = Endpoint::CurrentGamePlayer { puuid: &self.config.puuid };
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let player_response = response.json::<CurrentGamePlayerResponse>().await.map_err(anyhow::Error::from)?;

        Ok(player_response)
    }

     pub async fn get_current_game_match(&self, match_id: &str) -> Result<CurrentGameMatchResponse> {
        let endpoint = Endpoint::CurrentGameMatch { current_game_match_id: match_id };
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let match_response = response.json::<CurrentGameMatchResponse>().await.map_err(anyhow::Error::from)?;

        Ok(match_response)
    }

     pub async fn get_current_game_loadouts(&self, match_id: &str) -> Result<CurrentGameLoadoutsResponse> {
        let endpoint = Endpoint::CurrentGameLoadouts { current_game_match_id: match_id };
        let (method, url) = endpoint.url(&self.config);

        let response = self.send_request(method, &url).await?;
        let loadouts_response = response.json::<CurrentGameLoadoutsResponse>().await.map_err(anyhow::Error::from)?;

        Ok(loadouts_response)
    }

    // haven't tested yet
    pub async fn quit_current_match(&self, match_id: &str) -> Result<()> {
        let endpoint = Endpoint::CurrentGameQuit { puuid: &self.config.puuid, current_game_match_id: match_id };
        let (method, url) = endpoint.url(&self.config);

        self.send_request(method, &url).await?;

        Ok(())
    }
}