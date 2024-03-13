use dirs;
use tokio;
use valorant_rs::ValorantAuthClient;

#[tokio::main]
async fn main() {
    let mut appdata_local = dirs::data_local_dir().expect("Could not find the local data directory");
    appdata_local.push("Riot Games\\Riot Client\\Config\\lockfile");

    let (password, port) = ValorantAuthClient::parse_lockfile_content();
    let client = ValorantAuthClient::new(password, port).await;
    let response = client.get_entitlements_token().await;

    match response {
        Ok(token_response) => println!("auth response ----> {:?}", token_response),
        Err(e) => println!("Error fetching token: {:?}\nAre you sure Valorant is running right now?", e),
    }
}
