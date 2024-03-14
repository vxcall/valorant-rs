use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use crate::models::{CurrentGameLoadoutsResponse, CurrentGamePlayerResponse};
use anyhow::Result;


impl ValorantClient {
    pub async fn get_current_game_player(&self, entitlement_token: &String, auth_token: &String, puuid: &String) -> Result<CurrentGamePlayerResponse> {
        let endpoint = Endpoint::CurrentGamePlayer { puuid: puuid.to_string() };
        let url = endpoint.url(&self.config);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", auth_token))
            .header("X-Riot-Entitlements-JWT", entitlement_token)
            .send()
            .await
            .map_err(|e| anyhow::Error::from(e))?; // Convert reqwest::Error to anyhow::Error

        let response = response
            .json::<CurrentGamePlayerResponse>()
            .await
            .map_err(|e| anyhow::Error::from(e))?; // Convert reqwest::Error to anyhow::Error

        Ok(response)
    }

    pub async fn get_current_game_loadouts(&self, entitlement_token: &String, auth_token: &String, match_id: &String) -> Result<CurrentGameLoadoutsResponse> {
        let endpoint = Endpoint::CurrentGameLoadouts { current_game_match_id: match_id.to_string() };
        let url = endpoint.url(&self.config);

        let response = self.client
            .get(&url)
            .header("Authorization", format!("Bearer {}", auth_token))
            .header("X-Riot-Entitlements-JWT", entitlement_token)
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