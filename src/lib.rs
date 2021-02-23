use anyhow::{Context, Result};
use reqwest::Client;
#[cfg(feature = "serialize")]
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};

mod responses;

#[derive(Deserialize)]
#[cfg_attr(feature = "serialize", derive(Serialize))]
#[cfg_attr(feature = "debug_attr", derive(Debug))]
#[cfg_attr(feature = "deny_unknown_fields", serde(deny_unknown_fields))]
struct AuthResponse {
    access_token: String,
    token_type: String,
    expires_in: u32,
    scope: String,
}

#[cfg_attr(feature = "debug_attr", derive(Debug))]
pub struct Rucola {
    client: Client,
    access_token: String,
}

impl Rucola {
    pub async fn new(
        username: &str,
        password: &str,
        client_id: &str,
        secret_token: &str,
    ) -> Result<Self> {
        let form = [
            ("grant_type", "password"),
            ("username", username),
            ("password", password),
        ];

        let client = Client::builder()
            .user_agent("ApiTest/0.1.0")
            .build()
            .context("Failed to build client")?;

        let res: AuthResponse = client
            .post("https://www.reddit.com/api/v1/access_token")
            .form(&form)
            .basic_auth(client_id, Some(secret_token))
            .send()
            .await?
            .json()
            .await?;

        Ok(Rucola {
            client,
            access_token: res.access_token,
        })
    }

    pub async fn get(&self, url: &str) -> Result<reqwest::Response> {
        let response = self
            .client
            .get(url)
            .header("Authorization", format!("bearer {}", self.access_token))
            .send()
            .await?;

        Ok(response)
    }

    pub async fn get_json<T: DeserializeOwned>(&self, url: &str) -> Result<T> {
        let response = self.get(url).await?.json::<T>().await?;

        Ok(response)
    }

    pub async fn me(&self) -> Result<responses::me::Me> {
        let response = self
            .get_json::<responses::me::Me>("https://oauth.reddit.com/api/v1/me")
            .await?;

        Ok(response)
    }

    pub async fn me_karma(&self) -> Result<responses::me::Karma> {
        let response = self
            .get_json::<responses::me::Karma>("https://oauth.reddit.com/api/v1/karma")
            .await?;

        Ok(response)
    }

    pub async fn me_prefs(&self) -> Result<responses::me::Prefs> {
        let response = self
            .get_json::<responses::me::Prefs>("https://oauth.reddit.com/api/v1/prefs")
            .await?;

        Ok(response)
    }

    pub async fn me_trophies(&self) -> Result<responses::me::Trophies> {
        let response = self
            .get_json::<responses::me::Trophies>("https://oauth.reddit.com/api/v1/trophies")
            .await?;

        Ok(response)
    }
}
