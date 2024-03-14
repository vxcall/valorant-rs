use tokio;
use valorant_rs::ValorantAuthClient;

#[tokio::main]
async fn main() {
    let client = ValorantAuthClient::new().expect("client generation failed");
    let response = client.get_entitlements_token().await;

    match response {
        Ok(token_response) => println!("auth response ----> {:?}", token_response),
        Err(e) => println!("Error fetching token: {:?}\nAre you sure Valorant is running right now?", e),
    }
}
