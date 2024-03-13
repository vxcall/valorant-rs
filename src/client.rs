use std::path::PathBuf;

use crate::models::EntitlementsTokenResponse;
use base64;
use base64::{engine::general_purpose::STANDARD, Engine};
use reqwest::{Client as HttpClient, Error, ClientBuilder};

pub struct ValorantAuthClient {
    pub client: HttpClient,
    pub base_url: String,
    pub lockfile_password: String,
    pub port: u16,
}

impl ValorantAuthClient {
    pub async fn new(lockfile_password: String, port: u16) -> Self {
        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()
            .expect("Failed to build HTTP client");
        ValorantAuthClient {
            client,
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

    fn get_lockfile_path() -> Option<PathBuf> {
        if let Some(mut appdata_local) = dirs::data_local_dir() {
            appdata_local.push("Riot Games\\Riot Client\\Config\\lockfile");
            Some(appdata_local)
        } else {
            None
        }
    }

    pub fn parse_lockfile_content() -> (String, u16) {
        if let Some(lockfile_path) = Self::get_lockfile_path() {
            if let Ok(content) = std::fs::read_to_string(lockfile_path) {
                let parts: Vec<&str> = content.split(':').collect();
                if parts.len() >= 4 {
                    return (parts[3].to_string(), parts[2].parse::<u16>().unwrap_or(0));
                }
            }
        }
        (String::new(), 0)
    }
}
