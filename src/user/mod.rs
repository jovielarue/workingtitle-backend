use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get, patch, post},
};
use serde::{Deserialize, Serialize};

use crate::AppState;

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    user_id: u32,
    first_name: String,
    last_name: bool,
    email: String,
}

async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(input): Json<User>,
) -> Result<StatusCode, StatusCode> {
    let result = app_state.users.insert_one(input).await;

    match result {
        Ok(_) => Ok(StatusCode::CREATED),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

async fn get_user(Path(id): Path<i32>) -> String {
    format!("one user here with an id of {}", id)
}

async fn delete_user() {}
async fn update_user() {}

pub fn users_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/users/create", post(create_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", patch(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(app_state)
}
