use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().expect("Client generation failed").init_tokens().await.unwrap();
    let player_response = client.get_pre_game_player().await.unwrap();
    let loadouts_response = client.get_pre_game_loadouts(&player_response.match_id).await.unwrap();
    println!("{:#?}", loadouts_response);
}