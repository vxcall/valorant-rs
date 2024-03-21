use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().expect("Client generation failed").init_tokens().await.unwrap();
    let res = client.get_agents().await.unwrap();
    if let Some(first_agent) = res.data.first() {
        println!("{:#?}", first_agent.uuid);
    } else {
        println!("No agents found");
    }
}