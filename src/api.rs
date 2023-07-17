use serde::{Deserialize, Serialize};

#[derive(Clone)]
pub struct SteamAPI {
    steam_key: String,
}

impl SteamAPI {
    pub fn new(key: String) -> SteamAPI {
        SteamAPI { steam_key: key }
    }

    pub fn get_owned_games(self) {
        todo!()
    }
}
