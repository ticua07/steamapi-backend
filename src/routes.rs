use axum::{
    extract::{Query, State},
    Json,
};

use crate::{api::Game, AppState};
use serde::Deserialize;
// basic handler that responds with a static string
#[derive(Debug, Deserialize)]
pub struct GetOwnedGames {
    steam_id: String,
}

use axum_macros::debug_handler;

#[debug_handler]
pub async fn get_owned_games(
    State(state): State<AppState>,
    Query(query): Query<GetOwnedGames>,
) -> Json<Vec<Game>> {
    let mut games = state
        .steam_api
        .get_owned_games(query.steam_id)
        .await
        .unwrap();
    games.sort_by_cached_key(|x| x.playtime);
    games.reverse();
    axum::Json(games[0..10].to_vec())
}
// 76561198861877701
