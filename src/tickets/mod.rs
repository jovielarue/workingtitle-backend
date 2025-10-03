pub mod models;
use crate::{
    AppState,
    api::types::{ApiError, ApiResponse},
    db,
    tickets::models::{PartialTicket, Ticket},
    utility::parse_form,
};
use axum::{
    Router,
    extract::{Multipart, Path, State},
    routing::{delete, get, patch, post},
};
use std::sync::Arc;

pub fn ticket_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/tickets", post(create_ticket))
        .route("/tickets/{id}", get(get_ticket))
        .route("/tickets/{id}", patch(update_ticket))
        .route("/tickets/{id}", delete(delete_ticket))
        .with_state(app_state)
}

async fn create_ticket(
    State(app_state): State<Arc<AppState>>,
    multipart: Multipart,
) -> Result<ApiResponse<String>, ApiError<String>> {
    let ticket = parse_form::<Ticket>(multipart).await?;

    db::crud::create(ticket, app_state.tickets.clone()).await
}

async fn get_ticket(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<Ticket>, ApiError<String>> {
    db::crud::read(id, app_state.tickets.clone()).await
}

async fn delete_ticket(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<Ticket>, ApiError<String>> {
    db::crud::delete(id, app_state.tickets.clone()).await
}

async fn update_ticket(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
    multipart: Multipart,
) -> Result<ApiResponse<Ticket>, ApiError<String>> {
    let ticket = parse_form::<PartialTicket>(multipart).await?;

    db::crud::update(ticket, id, app_state.tickets.clone()).await
}
