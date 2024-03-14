use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new(String::from("ap"), String::from("ap")).expect("Client generation failed");
    let response = client.get_entitlements_token().await;

    match response {
        Ok(token_response) => println!("auth response ----> {:?}", token_response),
        Err(e) => println!("Error fetching token: {:?}\nAre you sure Valorant is running right now?", e),
    }
}