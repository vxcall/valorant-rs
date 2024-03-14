use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use crate::models::CurrentGamePlayerResponse;
use anyhow::Result;


impl ValorantClient {
    pub async fn get_current_game_player(&self, entitlement_token: String, auth_token: String, puuid: String) -> Result<CurrentGamePlayerResponse> {
        let endpoint = Endpoint::CurrentGamePlayer { puuid };
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
}