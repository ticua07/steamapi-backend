use std::collections::HashMap;

use reqwest::header::CONTENT_TYPE;
use serde::Serialize;
use serde_json::Value;

#[derive(Clone)]
pub struct SteamAPI {
    steam_key: String,
    client: reqwest::Client,
}
#[derive(Debug, Serialize, Clone)]
pub struct Game {
    pub appid: u64,
    pub name: String,
    pub playtime: u64,    // Minutes
    pub icon_url: String, // Image ID, not full url
}

impl SteamAPI {
    pub fn new(key: String) -> Result<SteamAPI, reqwest::Error> {
        let client = reqwest::Client::builder().build()?;
        Ok(SteamAPI {
            steam_key: key,
            client,
        })
    }

    pub async fn get_owned_games(self, steam_id: String) -> Result<Vec<Game>, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("steamid", steam_id);
        params.insert("include_appinfo", "true".into());
        params.insert("include_played_free_games", "true".into());

        let body = self
            .make_request(
                params,
                "https://api.steampowered.com/IPlayerService/GetOwnedGames/v1/",
            )
            .await?;

        let mut games: Vec<Game> = Vec::new();

        for el in body["response"]["games"].as_array().unwrap() {
            games.push(Game {
                appid: el["appid"].as_u64().unwrap().clone(),
                name: el["name"].as_str().unwrap().to_string().clone(),
                playtime: el["playtime_forever"].as_u64().unwrap().clone(),
                icon_url: el["img_icon_url"].as_str().unwrap().to_string().clone(),
            })
        }

        Ok(games)
    }

    pub async fn get_game_achievements(
        self,
        steam_id: String,
        app_id: String,
    ) -> Result<Value, reqwest::Error> {
        let mut params = HashMap::new();
        params.insert("steamid", steam_id);
        params.insert("appid", app_id);

        // There is a v2 of this API, but returns more data that doesn't matter.
        let body = self
            .make_request(
                params,
                "https://api.steampowered.com/ISteamUserStats/GetPlayerAchievements/v0001/",
            )
            .await?;

        Ok(body)
    }

    async fn make_request(
        self,
        params: HashMap<&str, String>,
        url: &str,
    ) -> Result<Value, reqwest::Error> {
        let mut p = params.clone();
        p.insert("key", self.steam_key);
        let request = self
            .client
            .get(url)
            .header(CONTENT_TYPE, "application/json")
            .query(&p)
            .send();

        let body = request.await?.json::<Value>().await?;

        Ok(body)
    }
}
