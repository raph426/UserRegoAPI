use axum::{Json, extract::State, http::StatusCode};

// accesses 'this' crate and imports modules
use crate::{
    auth::create_jwt,
    models::{RegiReq, RegiResp, User},
    state::AppState,
};

pub async fn register(
    State(state): State<AppState>,

    Json(payload): Json<RegiReq>,
) -> Result<Json<RegiResp>, StatusCode> {
    // acquire qrite lock for user map
    let mut users = state.users.write().await;

    if users.contains_key(&payload.username) {
        return Err(StatusCode::CONFLICT);
    }

    let user = User {
        username: payload.username.clone(),
        pwd: payload.pwd.clone(),
    };

    users.insert(payload.username.clone(), user);

    let token = create_jwt(&payload.username, &state.jwt_secret)
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(RegiResp { token }))
}
