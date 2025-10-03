pub mod models;
use crate::{
    AppState,
    api::types::{ApiError, ApiResponse},
    db,
    events::models::{Event, PartialEvent},
    utility::parse_form,
};
use axum::{
    Router,
    extract::{Multipart, Path, State},
    routing::{delete, get, patch, post},
};
use std::sync::Arc;

pub fn event_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/events", post(create_event))
        .route("/events/{id}", get(get_event))
        .route("/events/{id}", patch(update_event))
        .route("/events/{id}", delete(delete_event))
        .with_state(app_state)
}

async fn create_event(
    State(app_state): State<Arc<AppState>>,
    multipart: Multipart,
) -> Result<ApiResponse<String>, ApiError<String>> {
    let event = parse_form::<Event>(multipart).await?;

    db::crud::create(event, app_state.events.clone()).await
}

async fn get_event(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<Event>, ApiError<String>> {
    db::crud::read(id, app_state.events.clone()).await
}

async fn delete_event(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<Event>, ApiError<String>> {
    db::crud::delete(id, app_state.events.clone()).await
}

async fn update_event(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    multipart: Multipart,
) -> Result<ApiResponse<Event>, ApiError<String>> {
    let event = parse_form::<PartialEvent>(multipart).await?;

    db::crud::update(event, id, app_state.events.clone()).await
}
