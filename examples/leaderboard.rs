use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().expect("Client generation failed").init_tokens().await.unwrap();
    //let res = client.get_competitive_updates(None, None, None).await.unwrap();
    let res = client.get_leaderboard("22d10d66-4d2a-a340-6c54-408c7bd53807", "0", "510", None).await.unwrap();
    println!("{:#?}", res);
}