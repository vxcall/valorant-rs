use dotenv::dotenv;
use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let lockfile_content = std::env::var("LOCKFILE_PASSWORD").unwrap_or_default();
    let (password, port) = ValorantClient::parse_lockfile_content(&lockfile_content);
    let client = ValorantClient::new(password, port).await;
    let response = client.get_entitlements_token().await;

    match response {
        Ok(token_response) => println!("{:?}", token_response),
        Err(e) => println!("Error fetching token: {:?}", e),
    }
}
