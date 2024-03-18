use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use super::models;
use reqwest::Method;
use anyhow::Result;


impl ValorantClient {
    pub async fn get_pre_game_player(&self) -> Result<PreGamePlayer> {
        let endpoint = Endpoint::PreGamePlayer { puuid: &self.config.puuid };
        let url = endpoint.url(&self.config);

        let response = self.send_request(Method::GET, &url).await?;
        let player_response = response.json::<PreGamePlayer>().await.map_err(anyhow::Error::from)?;

        Ok(player_response)
    }
}