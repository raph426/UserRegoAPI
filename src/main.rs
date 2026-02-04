mod auth;
mod handlers;
mod models;
mod state;

use axum::{Router, routing::post};
use state::AppState;
use tower_http::trace::TraceLayer;

#[tokio::main]
async fn main() {
    //initialise shared appstate
    let state = AppState::new("secret-key".to_string());

    //build router
    let app = Router::new()
        .route("/register", post(handlers::register))
        .with_state(state) // attach shared appstate to all routes
        .layer(TraceLayer::new_for_http()); // logger

    // bind tcp socket
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    // start the http server
    axum::serve(listener, app).await.unwrap();
}
