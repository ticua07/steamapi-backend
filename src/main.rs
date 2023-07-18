use dotenv::dotenv;
mod api;
mod routes;
use crate::api::SteamAPI;
use crate::routes::get_owned_games;
use axum::{routing::get, Router};
use std::net::SocketAddr;

// TODO: Remove this later, just send SteamAPI
#[derive(Clone)]
pub struct AppState {
    pub steam_api: SteamAPI,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let steam_key = std::env::var("STEAM_KEY").expect("STEAM_KEY not found in enviroment.");

    let state = SteamAPI::new(steam_key).expect("Error initializing Steam API service.");

    let app = Router::new()
        .route("/GetOwnedGames", get(get_owned_games))
        .with_state(AppState { steam_api: state });

    // `GET /` goes to `root`
    // `POST /users` goes to `create_user`
    // .route("/users", post(create_user));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
