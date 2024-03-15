#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub shard: String,
    pub region: String,
    pub port: u16,
    pub lockfile_password: String,
    pub entitlement_token: String,
    pub auth_token: String,
    pub puuid: String,
}