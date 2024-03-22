use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().await.expect("Client generation failed").local_authentication().await.unwrap();
    let res = client.get_competitive_updates(None, None, None).await.unwrap();
    println!("{:#?}", res);
}