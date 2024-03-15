use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new(String::from("ap"), String::from("ap")).expect("Client generation failed").init_tokens().await;

    match client {
        Ok(client) => println!("auth response ----> {:#?}", client),
        Err(e) => println!("Error fetching token: {:?}\nAre you sure Valorant is running right now?", e),
    }
}