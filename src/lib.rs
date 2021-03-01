use anyhow::{Context, Result};
use reqwest::Client;
#[cfg(feature = "serialize")]
use serde::Serialize;
use serde::{de::DeserializeOwned, Deserialize};

pub mod responses;
use responses::ThingKind;

/// Helper function to generate user agent
///
/// It has the following format `<platform>:<app ID>:<version string> (by /u/<reddit username>)`
/// and is fully compliant with [reddi api rules](https://github.com/reddit-archive/reddit/wiki/API)
///
/// Idealy app_id and version could be figured out at build time but I have not had the occasion
/// to try.
pub fn user_agent(app_id: &str, version: &str, username: &str) -> String {
    format!(
        "{}:{}:{} (by /u/{})",
        std::env::consts::OS,
        app_id,
        version,
        username
    )
}

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
        user_agent: &str,
    ) -> Result<Self> {
        let form = [
            ("grant_type", "password"),
            ("username", username),
            ("password", password),
        ];

        let client = Client::builder()
            .user_agent(user_agent)
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

        // This stuff is here just for debbuging reason
        // There's probably a better way but is a quick hack and it works so ¯\_(ツ)_/¯
        // let response = self.get(url).await?.text().await?;
        // tokio::fs::write("temp/res.json", &response).await?;
        // let response = serde_json::from_str(&response)?;

        Ok(response)
    }

    pub async fn user_overview(&self, username: &str) -> Result<responses::ThingKind> {
        let response = self
            .get_json::<responses::ThingKind>(&format!(
                "https://oauth.reddit.com/user/{}/overview",
                username
            ))
            .await?;

        Ok(response)
    }

    pub async fn username_available(&self, username: &str) -> Result<bool> {
        let response = self
            .get_json::<bool>(&format!(
                "https://oauth.reddit.com/api/username_available?user={}",
                username
            ))
            .await?;

        Ok(response)
    }

    pub async fn me(&self) -> Result<responses::Account> {
        let response = self
            .get_json::<responses::Account>("https://oauth.reddit.com/api/v1/me")
            .await?;

        Ok(response)
    }

    pub async fn me_karma(&self) -> Result<responses::ThingKind> {
        let response = self
            .get_json::<responses::ThingKind>("https://oauth.reddit.com/api/v1/me/karma")
            .await?;

        Ok(response)
    }

    pub async fn me_prefs(&self) -> Result<responses::Prefs> {
        let response = self
            .get_json::<responses::Prefs>("https://oauth.reddit.com/api/v1/me/prefs")
            .await?;

        Ok(response)
    }

    pub async fn me_trophies(&self) -> Result<responses::Trophies> {
        let response = self
            .get_json::<responses::Trophies>("https://oauth.reddit.com/api/v1/me/trophies")
            .await?;

        Ok(response)
    }

    pub async fn subreddits_popular(&self) -> Result<ThingKind> {
        let response = self
            .get_json::<ThingKind>("https://oauth.reddit.com/subreddits/popular")
            .await?;

        Ok(response)
    }

    pub async fn subreddits_new(&self) -> Result<ThingKind> {
        let response = self
            .get_json::<ThingKind>("https://oauth.reddit.com/subreddits/new")
            .await?;

        Ok(response)
    }

    pub async fn subreddits_gold(&self) -> Result<ThingKind> {
        let response = self
            .get_json::<ThingKind>("https://oauth.reddit.com/subreddits/gold")
            .await?;

        Ok(response)
    }

    pub async fn subreddits_default(&self) -> Result<ThingKind> {
        let response = self
            .get_json::<ThingKind>("https://oauth.reddit.com/subreddits/default")
            .await?;

        Ok(response)
    }

    pub async fn subreddits_mine_subscriber(&self) -> Result<ThingKind> {
        let response = self
            .get_json::<ThingKind>("https://oauth.reddit.com/subreddits/mine/subscriber")
            .await?;

        Ok(response)
    }

    pub async fn subreddits_mine_contributor(&self) -> Result<ThingKind> {
        let response = self
            .get_json::<ThingKind>("https://oauth.reddit.com/subreddits/mine/contributor")
            .await?;

        Ok(response)
    }

    pub async fn subreddits_mine_moderator(&self) -> Result<ThingKind> {
        let response = self
            .get_json::<ThingKind>("https://oauth.reddit.com/subreddits/mine/moderator")
            .await?;

        Ok(response)
    }

    pub async fn about_moderators(&self, subreddit: &str) -> Result<ThingKind> {
        let response = self
            .get_json::<ThingKind>(&format!(
                " https://oauth.reddit.com/r/{}/about/moderators",
                subreddit
            ))
            .await?;

        Ok(response)
    }

    pub async fn wiki_pages(&self, subreddit: &str) -> Result<ThingKind> {
        let response = self
            .get_json::<ThingKind>(&format!(
                " https://oauth.reddit.com/r/{}/wiki/pages",
                subreddit
            ))
            .await?;

        Ok(response)
    }
}
