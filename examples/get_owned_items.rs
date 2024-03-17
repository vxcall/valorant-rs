use tokio;
use valorant_rs::ValorantClient;
use valorant_rs::store::ItemType;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().expect("Client generation failed").init_tokens().await.unwrap();
    let owned_item_response = client.get_owned_items(ItemType::Titles).await;
    println!("{:#?}", owned_item_response);
}