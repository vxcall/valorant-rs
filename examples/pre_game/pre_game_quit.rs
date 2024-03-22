
use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().await.expect("Client generation failed").local_authentication().await.unwrap();
    let player_response = client.get_pre_game_player().await.unwrap();
    client.pre_game_quit(&player_response.match_id).await.unwrap();
}