use crate::models::EntitlementsTokenResponse;
use base64;
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::{Client as HttpClient, Error};

pub struct ValorantAuthClient {
    pub client: HttpClient,
    pub base_url: String,
    pub lockfile_password: String,
    pub port: u16,
}

impl ValorantAuthClient {
    pub async fn new(lockfile_password: String, port: u16) -> Self {
        ValorantAuthClient {
            client: HttpClient::new(),
            base_url: format!("https://127.0.0.1:{}", port),
            lockfile_password,
            port,
        }
    }

    pub async fn get_entitlements_token(&self) -> Result<EntitlementsTokenResponse, Error> {
        let url = format!("{}/entitlements/v1/token", self.base_url);
        let auth_value = format!(
            "Basic {}",
            STANDARD.encode(format!("riot:{}", self.lockfile_password))
        );
        self.client
            .get(&url)
            .header("Authorization", auth_value)
            .send()
            .await?
            .json::<EntitlementsTokenResponse>()
            .await
    }

    pub fn parse_lockfile_content(content: &str) -> (String, u16) {
        let parts: Vec<&str> = content.split(':').collect();
        if parts.len() >= 4 {
            (parts[3].to_string(), parts[2].parse::<u16>().unwrap_or(0))
        } else {
            (String::new(), 0)
        }
    }
}
