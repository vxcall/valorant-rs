use dotenv::dotenv;
use std::env;
use tokio;
use valorant_rs::client::{get_auth_cookies, perform_authorization};

#[tokio::main]
async fn main() {
    dotenv().ok();

    if let Err(e) = get_auth_cookies().await {
        eprintln!("Failed to get auth cookies: {}", e);
        return;
    }

    let username = env::var("VALORANT_USERNAME").expect("VALORANT_USERNAME not set");
    let password = env::var("VALORANT_PASSWORD").expect("VALORANT_PASSWORD not set");
    let remember = true; // or false, depending on your needs.

    if let Err(e) = perform_authorization(username, password, remember).await {
        eprintln!("Failed to perform authorization: {}", e);
        return;
    }

    println!("Authorization successful (or handle response accordingly)");
}
