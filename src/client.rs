use anyhow::{ Result, anyhow };
use base64::{engine::general_purpose::STANDARD, Engine};
use crate::api_config::ApiConfig;
use crate::endpoint::BaseUrls;
use reqwest::{ Client as HttpClient, cookie::Jar, ClientBuilder, Method, RequestBuilder };
use std::{path::PathBuf, sync::Arc};
use dirs;
use std::fs;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ValorantClient {
    pub client: HttpClient,
    pub config: ApiConfig,
}

impl ValorantClient {
    pub async fn new() -> Result<Self> {
        let jar = Arc::new(Jar::default());
        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)

            .user_agent("RiotClient/58.0.0.4640299.4552318 %s (Windows;10;;Professional, x64)")
            .cookie_provider(Arc::clone(&jar))
            .cookie_store(true)
            .build()?;
        let client_version = Self::client_version().await?;
        let client_platform = Self::client_platform();
        Ok(ValorantClient {
            client,
            config: ApiConfig { region: String::new(), shard: String::new(), port: 0, base_urls: BaseUrls::default(), lockfile_password: String::new(), entitlement_token: String::new(), client_version, client_platform, auth_token: String::new(), puuid: String::new()}
        })
    }

    pub async fn remote_authentication(self, username: &str, password: &str) -> Result<Self> {
        self.auth_cookies().await?;
        self.auth_request(username,password).await?;
        Ok(self)
    }

    pub async fn local_authentication(mut self) -> Result<Self> {
        let (lockfile_password, port) = Self::extract_lockfile_content()
            .ok_or(anyhow!("Unable to extract lockfile content"))?;
        let (region, shard) = Self::extract_region_and_shard().ok_or(anyhow!("Unable to extract region and shard from ShooterGame.log"))?;


        let res = self.get_entitlements_token().await?;

        self.config.base_urls = BaseUrls {
            shared: format!("https://shared.{}.a.pvp.net", shard),
            pd: format!("https://pd.{}.a.pvp.net", shard),
            glz: format!("https://glz-{}-1.{}.a.pvp.net", region, shard),
            localhost: format!("https://127.0.0.1:{}", port),
        };

        self.config.lockfile_password = lockfile_password;
        self.config.region = region;
        self.config.shard = shard;
        self.config.entitlement_token = res.entitlement_token;
        self.config.auth_token = res.auth_token;
        self.config.puuid = res.puuid;

        Ok(self)
    }

    pub(crate) fn create_base_request<T>(&self, method: Method, url: T) -> RequestBuilder where T: Into<String> {
        self.client.request(method, url.into())
            .header("Authorization", format!("Bearer {}", self.config.auth_token))
            .header("X-Riot-Entitlements-JWT", &self.config.entitlement_token)
    }

    fn create_appdata_local_path<T>(path: T) -> Option<PathBuf> where T: Into<String> {
        if let Some(mut appdata_local) = dirs::data_local_dir() {
            appdata_local.push(path.into());
            Some(appdata_local)
        } else {
            None
        }
    }

    fn extract_lockfile_content() -> Option<(String, u16)> {
        if let Some(lockfile_path) = Self::create_appdata_local_path("Riot Games\\Riot Client\\Config\\lockfile") {
            if let Ok(content) = std::fs::read_to_string(lockfile_path) {
                let parts: Vec<&str> = content.split(':').collect();
                if parts.len() >= 4 {
                    let lockfile_password = parts[3].to_string();
                    let port = parts[2].parse::<u16>().unwrap_or(0);
                    return Some((lockfile_password, port));
                }
            }
        }
        None
    }

    fn extract_region_and_shard() -> Option<(String, String)> {
        if let Some(log_file_path) = Self::create_appdata_local_path("VALORANT\\Saved\\Logs\\ShooterGame.log") {
            if let Ok(contents) = fs::read_to_string(log_file_path) {
                let regex = Regex::new(r"glz-(.+?)-1\.(.+?)\.a\.pvp\.net").unwrap();

                if let Some(caps) = regex.captures(&contents) {
                    if caps.len() == 3 {
                        let region = caps.get(1).map_or("", |m| m.as_str()).to_string();
                        let shard = caps.get(2).map_or("", |m| m.as_str()).to_string();
                        return Some((region, shard));
                    }
                }
            }
        }
        None
    }

    pub(crate) async fn client_version() -> Result<String> {
        let res = Self::get_version().await?;
        Ok(res.data.riot_client_version)
    }

    pub(crate) fn client_platform() -> String {
        let client_platform = r#"{
            "platformType": "PC",
            "platformOS": "Windows",
            "platformOSVersion": "10.0.19042.1.256.64bit",
            "platformChipset": "Unknown"
        }"#;
        STANDARD.encode(client_platform)
    }
}