use tokio;
use valorant_rs::ValorantAuthClient;

#[tokio::main]
async fn main() {
    let (password, port) = ValorantAuthClient::parse_lockfile_content();
    let client = ValorantAuthClient::new(password, port).await;
    let response = client.get_entitlements_token().await;

    match response {
        Ok(token_response) => println!("{:?}", token_response),
        Err(e) => println!("Error fetching token: {:?}\nAre you sure Valorant is running now?", e),
    }
}
