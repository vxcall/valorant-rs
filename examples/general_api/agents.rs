use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().await.expect("Client generation failed").local_authentication().await.unwrap();
    let res = client.get_agents().await.unwrap();
    if let Some(first_agent) = res.data.first() {
        println!("{:#?}", first_agent.uuid);
    } else {
        println!("No agents found");
    }
}