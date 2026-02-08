// import modules from src/
mod auth;
mod handlers;
mod models;
mod state;

use axum::{Router, routing::post};
use state::AppState;
use tower_http::trace::TraceLayer;

// inject secret-key variable from .env file in root folder
use dotenvy::dotenv;
use std::env;

// MAIN -----------------------------------------
#[tokio::main]
async fn main() {
    dotenv().ok();

    //value is refutable so let else has to be used so rustc wont complain lol
    let Ok(secret) = env::var("SECRET_KEY") else {
        return;
    };

    //initialise shared appstate
    let state = AppState::new(secret);

    //build axum router
    let app = Router::new()
        .route("/register", post(handlers::register)) // mapped to POST /register to register fn in handler module
        .with_state(state) // attach shared appstate to all routes
        .layer(TraceLayer::new_for_http()); // logger

    // bind tcp socket to localhost
    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();

    // start the http server
    axum::serve(listener, app).await.unwrap();
}

// test api
#[cfg(test)]
mod tests {
    use axum::{body::Body, http::Request};
    use tokio_test::block_on;
    use tower::ServiceExt;

    // use modules in 'this'
    use crate::{handlers, state::AppState};
    use axum::{Router, routing::post};

    #[test]
    fn register_user_success() {
        block_on(async {
            let state = AppState::new("test-secret".to_string());

            let app = Router::new()
                .route("/register", post(handlers::register))
                .with_state(state);

            let request = Request::builder()
                .method("POST")
                .uri("/register")
                .header("content-type", "application/json")
                .body(Body::from(r#"{"username":"reinhard","pwd":"kircheis"}"#))
                .unwrap();

            let response = app.oneshot(request).await.unwrap();

            // checks if response is == to 200 "success"
            assert_eq!(response.status(), 200);
        });
    }
}
