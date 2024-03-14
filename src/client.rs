use anyhow::{ Result, anyhow };
use crate::api_config::ApiConfig;
use reqwest::{ Client as HttpClient, ClientBuilder };
use std::path::PathBuf;
use dirs;
pub struct ValorantClient {
    pub client: HttpClient,
    pub config: ApiConfig,
}

impl ValorantClient {
     pub fn new(shard: String, region: String) -> Result<Self> {
        let (lockfile_password, port) = Self::get_lockfile_content()
            .ok_or(anyhow!("Unable to parse lockfile content"))?;
        let client = ClientBuilder::new()
            .danger_accept_invalid_certs(true)
            .build()?;
        Ok(ValorantClient {
            client,
            config: ApiConfig { shard, region, port, lockfile_password }
        })
    }


    fn get_lockfile_path() -> Option<PathBuf> {
        if let Some(mut appdata_local) = dirs::data_local_dir() {
            appdata_local.push("Riot Games\\Riot Client\\Config\\lockfile");
            Some(appdata_local)
        } else {
            None
        }
    }

    fn get_lockfile_content() -> Option<(String, u16)> {
        if let Some(lockfile_path) = Self::get_lockfile_path() {
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
}