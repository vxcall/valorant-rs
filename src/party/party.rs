use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use anyhow::Result;

use super::models::party::PartyResponse;
use super::party_player::PartyPlayerResponse;

impl ValorantClient {

    pub async fn party(&self, party_id: &str) -> Result<PartyResponse> {

        let endpoint = Endpoint::Party { party_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request.send().await.map_err(anyhow::Error::from)?;

        let party_response = response.json::<PartyResponse>().await.map_err(anyhow::Error::from)?;

        Ok(party_response)
    }

    pub async fn party_player(&self) -> Result<PartyPlayerResponse> {

        let endpoint = Endpoint::PartyPlayer;
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        let response = request
            .header("X-Riot-ClientVersion", Self::client_version().await?)
            .send()
            .await
            .map_err(anyhow::Error::from)?;

        let party_player_response = response.json::<PartyPlayerResponse>().await.map_err(anyhow::Error::from)?;

        Ok(party_player_response)
    }

    pub async fn remove_party_player(&self) -> Result<()> {

        let endpoint = Endpoint::PartyRemovePlayer;
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        request.send().await.map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn refresh_competitive_tier(&self, party_id: &str) -> Result<()> {

        let endpoint = Endpoint::RefreshCompetitiveTier { party_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        request.send().await.map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn refresh_player_identity(&self, party_id: &str) -> Result<()> {

        let endpoint = Endpoint::RefreshPlayerIdentity { party_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        request.send().await.map_err(anyhow::Error::from)?;

        Ok(())
    }

    pub async fn refresh_pings(&self, party_id: &str) -> Result<()> {

        let endpoint = Endpoint::RefreshPings { party_id };
        let (method, url) = endpoint.url(&self.config);

        let request = self.create_base_request(method, url);
        request.send().await.map_err(anyhow::Error::from)?;

        Ok(())
    }

    /*
        let request = self.create_base_request(method, url);
        let response = request
            .header("X-Riot-ClientPlatform", Self::client_platform())
            .header("X-Riot-ClientVersion", Self::client_version().await?)
            .send()
            .await
            .map_err(anyhow::Error::from)?;
*/

}