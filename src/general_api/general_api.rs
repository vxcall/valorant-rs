use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use super::agents::AgentsResponse;
use super::version::VersionResponse;
use anyhow::Result;


impl ValorantClient {
    pub async fn get_agents(&self) -> Result<AgentsResponse> {
        let endpoint = Endpoint::Agents;
        let (_, url) = endpoint.url(&self.config);

        let response = self.client.get(&url).send().await.map_err(anyhow::Error::from)?;
        let agents_response = response.json::<AgentsResponse>().await.map_err(anyhow::Error::from)?;

        Ok(agents_response)
    }

    pub async fn get_version() -> Result<VersionResponse> {
        let endpoint = Endpoint::Version;
        let (_, url) = endpoint.url_no_config();

        let response = reqwest::get(&url).await.map_err(anyhow::Error::from)?;
        let version_response = response.json::<VersionResponse>().await.map_err(anyhow::Error::from)?;

        Ok(version_response)
    }
}