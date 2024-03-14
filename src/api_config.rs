#[derive(Debug, Clone)]
pub struct ApiConfig {
    pub shard: String,
    pub region: String,
    pub port: u16,
    pub lockfile_password: String,
}