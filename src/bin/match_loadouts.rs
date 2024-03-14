use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new(String::from("ap"), String::from("ap")).expect("Client generation failed");
    let token_response = client.get_entitlements_token().await.unwrap();
    let match_response = client.get_current_game_player(token_response.entitlement_token, token_response.auth_token, token_response.puuid).await.unwrap();
    println!("{:#?}", match_response);
}
