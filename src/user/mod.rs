pub mod models;
pub mod user_service;
use crate::{
    AppState,
    api::types::{ApiError, ApiResponse},
    user::models::{PartialUser, User},
    utility::parse_form,
};
use axum::{
    Router,
    extract::{Multipart, Path, State},
    routing::{delete, get, patch, post},
};
use std::sync::Arc;

pub fn users_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/users", post(create_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", patch(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(app_state)
}

async fn create_user(
    State(app_state): State<Arc<AppState>>,
    multipart: Multipart,
) -> Result<ApiResponse<String>, ApiError<String>> {
    let user = parse_form::<User>(multipart).await?;

    user_service::create(user, app_state.users.clone()).await
}

async fn get_user(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<User>, ApiError<String>> {
    user_service::read(id, app_state.users.clone()).await
}

async fn delete_user(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<User>, ApiError<String>> {
    user_service::delete(id, app_state.users.clone()).await
}

async fn update_user(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    multipart: Multipart,
) -> Result<ApiResponse<User>, ApiError<String>> {
    let user = parse_form::<PartialUser>(multipart).await?;

    user_service::update(user, id, app_state.users.clone()).await
}
