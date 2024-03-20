use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().expect("Client generation failed").init_tokens().await.unwrap();
    let res = client.get_match_details("e3f81220-3340-436b-addf-48b4968822f9").await.unwrap();
    println!("{:#?}", res);
}