use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().await.expect("Client generation failed").local_authentication().await.unwrap();
    let prices_response = client.get_prices().await.unwrap();
    println!("{:#?}", prices_response);
    let storefront_response = client.get_storefront().await.unwrap();
    println!("{:#?}", storefront_response);
    let wallet_response = client.get_wallet().await.unwrap();
    println!("{:#?}", wallet_response);
}