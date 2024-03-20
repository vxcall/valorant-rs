use anyhow::{ Result, anyhow };
use crate::api_config::ApiConfig;
use crate::endpoint::BaseUrls;
use reqwest::{ Client as HttpClient, ClientBuilder, Method, RequestBuilder };
use std::path::PathBuf;
use dirs;
use std::fs;
use regex::Regex;

#[derive(Debug, Clone)]
pub struct ValorantClient {
    pub client: HttpClient,
    pub config: ApiConfig,
}

impl ValorantClient {
    pub fn new() -> Result<Self> {
        let (lockfile_password, port) = Self::extract_lockfile_content()
            .ok_or(anyhow!("Unable to extract lockfile content"))?;
        let (region, shard) = Self::extract_region_and_shard().ok_or(anyhow!("Unable to extract region and shard from ShooterGame.log"))?;

        let base_urls = BaseUrls {
            shared: format!("https://shared.{}.a.pvp.net", shard),
            pd: format!("https://pd.{}.a.pvp.net", shard),
            glz: format!("https://glz-{}-1.{}.a.pvp.net", region, shard),
            localhost: format!("https://127.0.0.1:{}", port),
        };

        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()?;

        Ok(ValorantClient {
            client,
            config: ApiConfig { region, shard, port, base_urls, lockfile_password, entitlement_token: String::new(), auth_token: String::new(), puuid: String::new()}
        })
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

}