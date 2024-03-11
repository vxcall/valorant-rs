use crate::models::{AuthCookiesBody, AuthRequestBody};
use reqwest::{header, Client};

pub async fn get_auth_cookies() -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let auth_cookies_body = AuthCookiesBody {
        client_id: "play-valorant-web-prod".to_string(),
        nonce: "1".to_string(),
        redirect_uri: "https://playvalorant.com/opt_in".to_string(),
        response_type: "token id_token".to_string(),
        scope: "account openid".to_string(),
    };

    let _res = client
        .post("https://auth.riotgames.com/api/v1/authorization")
        .header(header::CONTENT_TYPE, "application/json")
        .json(&auth_cookies_body)
        .send()
        .await?;

    // Handle the response, e.g., extract the cookie
    Ok(())
}

pub async fn perform_authorization(
    username: String,
    password: String,
    remember: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let client = Client::new();
    let auth_request_body = AuthRequestBody {
        auth_type: "auth".to_string(),
        username,
        password,
        remember,
        language: "en_US".to_string(),
    };

    let _res = client
        .post("https://auth.riotgames.com/api/v1/authorization")
        .header(header::CONTENT_TYPE, "application/json")
        // Add the cookie to the request
        .json(&auth_request_body)
        .send()
        .await?;

    // Handle the response
    Ok(())
}
