use anyhow::Result;
use crate::endpoint::Endpoint;
use crate::models::EntitlementsTokenResponse;
use base64::{ engine::general_purpose::STANDARD, Engine };
use base64;
use crate::client::ValorantClient;

impl ValorantClient {
    /// init_tokens initializes the auth token and entitlement token.
    pub async fn init_tokens(mut self) -> Result<Self> {
        let endpoint = Endpoint::EntitlementsToken;
        let (method, url) = endpoint.url(&self.config);
        let header_value = format!(
            "Basic {}",
            STANDARD.encode(format!("riot:{}", self.config.lockfile_password))
        );
        let response = self.client
            .get(&url)
            .header("Authorization", header_value)
            .send()
            .await
            .map_err(|e| anyhow::Error::from(e))?; // Convert reqwest::Error to anyhow::Error

        let token_response = response
            .json::<EntitlementsTokenResponse>()
            .await
            .map_err(|e| anyhow::Error::from(e))?; // Convert reqwest::Error to anyhow::Error

        self.config.entitlement_token = token_response.entitlement_token;
        self.config.auth_token = token_response.auth_token;
        self.config.puuid = token_response.puuid;

        Ok(self)
    }
}