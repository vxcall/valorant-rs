use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().await.expect("Client generation failed").local_authentication().await.unwrap();
    let res = client.get_agents().await.unwrap();
    let sova_id = res.data.iter().find(|agent| agent.display_name == "Sova").map(|agent| &agent.uuid);
    let pre_game_match_id = client.get_pre_game_player().await.unwrap().match_id;
    match sova_id {
        Some(uuid) => {
            client.select_character(&pre_game_match_id, uuid).await.unwrap();
            client.lock_character(&pre_game_match_id, uuid).await.unwrap();
        },
        None => println!("Sova not found"),
    }
}