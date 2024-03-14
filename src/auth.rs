use anyhow::Result;
use crate::endpoint::Endpoint;
use crate::models::EntitlementsTokenResponse;
use base64::{ engine::general_purpose::STANDARD, Engine };
use base64;
use crate::client::ValorantClient;

impl ValorantClient {
    pub async fn get_entitlements_token(&self) -> Result<EntitlementsTokenResponse> {
        let endpoint = Endpoint::EntitlementsToken;
        let url = endpoint.url(&self.config);
        let auth_value = format!(
            "Basic {}",
            STANDARD.encode(format!("riot:{}", self.config.lockfile_password))
        );
        let response = self.client
            .get(&url)
            .header("Authorization", auth_value)
            .send()
            .await
            .map_err(|e| anyhow::Error::from(e))?; // Convert reqwest::Error to anyhow::Error

        let token_response = response
            .json::<EntitlementsTokenResponse>()
            .await
            .map_err(|e| anyhow::Error::from(e))?; // Convert reqwest::Error to anyhow::Error

        Ok(token_response)
    }
}