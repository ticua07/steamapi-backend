use dotenv::dotenv;
mod api;
mod routes;
use crate::api::SteamAPI;
use crate::routes::hello_world;
use axum::{routing::get, Router};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    dotenv().ok();
    let steam_key = std::env::var("STEAM_KEY").expect("STEAM_KEY not found in enviroment");

    let app = Router::new()
        .route("/", get(hello_world))
        .with_state(SteamAPI::new(steam_key));

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
