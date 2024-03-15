use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use crate::models::{CurrentGameLoadoutsResponse, CurrentGamePlayerResponse};
use anyhow::Result;


impl ValorantClient {
    pub async fn get_current_game_player(&self) -> Result<CurrentGamePlayerResponse> {
        let endpoint = Endpoint::CurrentGamePlayer { puuid: &self.config.puuid };
        let url = endpoint.url(&self.config);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.auth_token))
            .header("X-Riot-Entitlements-JWT", &self.config.entitlement_token)
            .send()
            .await
            .map_err(|e| anyhow::Error::from(e))?; // Convert reqwest::Error to anyhow::Error

        let response = response
            .json::<CurrentGamePlayerResponse>()
            .await
            .map_err(|e| anyhow::Error::from(e))?; // Convert reqwest::Error to anyhow::Error

        Ok(response)
    }

    pub async fn get_current_game_loadouts(&self, match_id: &str) -> Result<CurrentGameLoadoutsResponse> {
        let endpoint = Endpoint::CurrentGameLoadouts { current_game_match_id: match_id };
        let url = endpoint.url(&self.config);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", self.config.auth_token))
            .header("X-Riot-Entitlements-JWT", &self.config.entitlement_token)
            .send()
            .await
            .map_err(|e| anyhow::Error::from(e))?; // Convert reqwest::Error to anyhow::Error

        let response = response
            .json::<CurrentGameLoadoutsResponse>()
            .await
            .map_err(|e| anyhow::Error::from(e))?; // Convert reqwest::Error to anyhow::Error

        Ok(response)
    }
}