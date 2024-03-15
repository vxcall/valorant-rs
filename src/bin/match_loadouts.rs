use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new(String::from("ap"), String::from("ap")).expect("Client generation failed").init_tokens().await.unwrap();
    let match_response = client.get_current_game_player().await.unwrap();
    let loadouts = client.get_current_game_loadouts(match_response.match_id).await.unwrap();

    println!("{:#?}", loadouts);
}
