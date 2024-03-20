use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().expect("Client generation failed").init_tokens().await.unwrap();
    let res = client.get_match_history(None, None, None).await.unwrap();
    println!("{:#?}", res);
}