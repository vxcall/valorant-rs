use tokio;
use valorant_rs::ValorantClient;

#[tokio::main]
async fn main() {
    let client = ValorantClient::new().await.expect("Client generation failed").local_authentication().await.unwrap();
    let sessions = client.get_sessions().await.unwrap();
    println!("{:#?}", sessions.0.values());
    let first_session_version = sessions.0.values()
                .next()
                .map(|session| &session.version);

    match first_session_version {
        Some(version) => println!("Version: {}", version),
        None => println!("No sessions found."),
    }
}