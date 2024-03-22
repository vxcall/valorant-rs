
use tokio;
use valorant_rs::ValorantClient;


const USERNAME: &str ="";
const PASSWORD: &str ="";

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().await.expect("Client generation failed").remote_authentication(&USERNAME, &PASSWORD).await.unwrap();
}