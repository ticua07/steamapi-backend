use axum::{
    extract::{Query, State},
    Json,
};
use serde_json::{json, Value};

use crate::{api::Game, AppState};
use axum_macros::debug_handler;
use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Deserialize)]
pub struct GetOwnedGames {
    steam_id: String,
}

// Hello! 76561198861877701

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
    // ! HANDLE UNWRAP LATER
    games.sort_by_cached_key(|x| x.playtime);
    games.reverse();
    axum::Json(games[0..10].to_vec())
}

#[derive(Debug, Deserialize)]
pub struct GetGameAchievements {
    steam_id: String,
    app_id: String,
}

#[derive(Debug, Serialize)]
struct GetAchievementsResponse {
    completed_achievements: usize,
    total_achievements: usize,
}

#[debug_handler]
pub async fn get_game_achievements(
    State(state): State<AppState>,
    Query(query): Query<GetGameAchievements>,
) -> Json<Value> {
    let achievements = match state
        .steam_api
        .get_game_achievements(query.steam_id, query.app_id)
        .await
    {
        Ok(ach) => ach,
        Err(_) => {
            return axum::Json(json!({
                "success": false
            }))
        }
    };
    // ! HANDLE UNWRAP LATER

    if achievements["playerstats"]["success"].as_bool().unwrap() == false {
        return axum::Json(json!({"success": false}));
    }
    let achievements = achievements["playerstats"]["achievements"]
        .as_array()
        .unwrap();

    let completed_achievements: usize = achievements
        .iter()
        .filter(|ach| ach["achieved"].as_u64().unwrap() == 1)
        .count();

    let total_achievements: usize = achievements.len();

    dbg!(completed_achievements);
    dbg!(total_achievements);

    axum::Json(json!({
        "completedAchievements": completed_achievements,
        "totalAchievements": total_achievements as usize,
    }))
}
