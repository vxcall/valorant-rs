use anyhow::Result;
use crate::endpoint::Endpoint;
use base64::{ engine::general_purpose::STANDARD, Engine };
use base64;
use crate::client::ValorantClient;

use super::entitlements_token::EntitlementsTokenResponse;
use super::sessions::SessionsResponse;

impl ValorantClient {
    /// local_authentication initializes the auth token and entitlement token.
    pub async fn get_entitlements_token(&self) -> Result<EntitlementsTokenResponse> {
        let endpoint = Endpoint::EntitlementsToken;
        let (_, url) = endpoint.url(&self.config);
        let header_value = format!(
            "Basic {}",
            STANDARD.encode(format!("riot:{}", self.config.lockfile_password))
        );
        let response = self.client
            .get(&url)
            .header("Authorization", header_value)
            .send()
            .await
            .map_err(|e| anyhow::Error::from(e))?;

        let token_response = response
            .json::<EntitlementsTokenResponse>()
            .await
            .map_err(|e| anyhow::Error::from(e))?;

        Ok(token_response)
    }

    pub async fn get_sessions(&self) -> Result<SessionsResponse> {
        let endpoint = Endpoint::Sessions;
        let (_, url) = endpoint.url(&self.config);

        let header_value = format!(
            "Basic {}",
            STANDARD.encode(format!("riot:{}", self.config.lockfile_password))
        );
        let response = self.client
            .get(&url)
            .header("Authorization", header_value)
            .send()
            .await?
            .json::<SessionsResponse>()
            .await
            .map_err(|e| anyhow::Error::from(e))?;

        Ok(response)
    }
}