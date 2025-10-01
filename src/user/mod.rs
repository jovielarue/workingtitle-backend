use axum::{
    Router,
    extract::Path,
    routing::{delete, get, patch, post},
};

async fn get_user(Path(id): Path<i32>) -> String {
    format!("one user here with an id of {}", id)
}

async fn create_user() {}
async fn delete_user() {}
async fn update_user() {}

pub fn users_router() -> Router {
    Router::new()
        .route("/users/create", post(create_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", patch(update_user))
        .route("/users/{id}", delete(delete_user))
}
