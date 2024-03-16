use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use crate::models::{CurrentGameLoadoutsResponse, CurrentGameMatchResponse, CurrentGamePlayerResponse};
use reqwest::Method;
use anyhow::Result;


impl ValorantClient {
    pub async fn get_current_game_player(&self) -> Result<CurrentGamePlayerResponse> {
        let endpoint = Endpoint::CurrentGamePlayer { puuid: &self.config.puuid };
        let url = endpoint.url(&self.config);

        let response = self.send_request(Method::GET, &url).await?;
        let player_response = response.json::<CurrentGamePlayerResponse>().await.map_err(anyhow::Error::from)?;

        Ok(player_response)
    }

     pub async fn get_current_game_match(&self, match_id: &str) -> Result<CurrentGameMatchResponse> {
        let endpoint = Endpoint::CurrentGameMatch { current_game_match_id: match_id };
        let url = endpoint.url(&self.config);

        let response = self.send_request(Method::GET, &url).await?;
        let match_response = response.json::<CurrentGameMatchResponse>().await.map_err(anyhow::Error::from)?;

        Ok(match_response)
    }

     pub async fn get_current_game_loadouts(&self, match_id: &str) -> Result<CurrentGameLoadoutsResponse> {
        let endpoint = Endpoint::CurrentGameLoadouts { current_game_match_id: match_id };
        let url = endpoint.url(&self.config);

        let response = self.send_request(Method::GET, &url).await?;
        let loadouts_response = response.json::<CurrentGameLoadoutsResponse>().await.map_err(anyhow::Error::from)?;

        Ok(loadouts_response)
    }
}