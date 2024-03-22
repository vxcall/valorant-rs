use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().await.expect("Client generation failed").local_authentication().await.unwrap();
    let res = client.get_player_loadout().await.unwrap();
    println!("{:#?}", res);
}
