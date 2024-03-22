use crate::endpoint::Endpoint;
use crate::client::ValorantClient;
use anyhow::Result;

impl ValorantClient {
    pub async fn auth_cookies(&self) -> Result<()> {
        let endpoint = Endpoint::AuthCookies;
        let (method, url) = endpoint.url(&self.config);

        let body = serde_json::json!({
            "client_id": "play-valorant-web-prod",
            "nonce": "1",
            "redirect_uri": "https://playvalorant.com/opt_in",
            "response_type": "token id_token",
            "scope": "account openid"
        });

        let request = self.client.request(method, url)
            .header("Content-Type", "application/json")
            .json(&body);

        let res = request.send().await.map_err(anyhow::Error::from)?;
        println!("{:?}", res.status());

        Ok(())
    }

    pub async fn auth_request(&self, username: &str, password: &str) -> Result<()> {
        let endpoint = Endpoint::AuthCookies;
        let (method, url) = endpoint.url(&self.config);

        let body = serde_json::json!({
            "type": "auth",
            "username": username,
            "password": password,
            "remember": true,
            "language": "en_US"
        });

        let request = self.create_base_request(method, url)
            .header("Content-Type", "application/json")
            .json(&body);

        let res = request.send().await.map_err(anyhow::Error::from)?;

        println!("{:?}", res.status());
        Ok(())
    }
}