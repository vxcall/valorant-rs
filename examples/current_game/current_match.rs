use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().expect("Client generation failed").init_tokens().await.unwrap();
    let player_response = client.get_current_game_player().await.unwrap();
    let match_response = client.get_current_game_match(&player_response.match_id).await.unwrap();
    println!("{:#?}", match_response);
}