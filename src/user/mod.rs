use crate::api::types::{ApiError, ApiResponse};
use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, patch, post},
};
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    user_id: u32,
    first_name: String,
    last_name: bool,
    email: String,
}

async fn create_user(
    State(db): State<Collection<User>>,
    Json(input): Json<User>,
) -> Result<ApiResponse, ApiError> {
    let result = db.insert_one(input).await;

    match result {
        Ok(_) => Ok(ApiResponse::Created),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

async fn get_user(Path(id): Path<i32>) -> String {
    format!("one user here with an id of {}", id)
}

async fn delete_user() {}
async fn update_user() {}

pub fn users_router() -> Router {
    Router::new()
        .route("/users/create", post(create_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", patch(update_user))
        .route("/users/{id}", delete(delete_user))
}
