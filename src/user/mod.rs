use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Path, State},
    routing::{delete, get, patch, post},
};
use mongodb::bson::{doc, oid::ObjectId, to_document};
use serde::{Deserialize, Serialize};

use crate::{
    AppState,
    api::types::{ApiError, ApiResponse},
};

#[derive(Debug, Deserialize, Serialize)]
pub struct User {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    user_id: Option<ObjectId>,
    first_name: String,
    last_name: String,
    email: String,
}

async fn create_user(
    State(app_state): State<Arc<AppState>>,
    Json(input): Json<User>,
) -> Result<ApiResponse<User>, ApiError> {
    let result = app_state.users.insert_one(input).await;

    match result {
        Ok(_) => Ok(ApiResponse::Created),
        Err(_) => Err(ApiError::InternalServerError),
    }
}

async fn get_user(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<User>, ApiError> {
    match ObjectId::parse_str(id) {
        Ok(oid) => match app_state.users.find_one(doc! {"_id": oid}).await {
            Ok(Some(u)) => {
                println!("User found:\n{:?}", u);
                Ok(ApiResponse::JsonData::<User>(vec![u as User]))
            }
            Ok(None) => {
                println!("User not found.");
                Err(ApiError::NotFound)
            }
            Err(e) => {
                println!("Database error: {:?}", e);
                Err(ApiError::InternalServerError)
            }
        },
        Err(e) => {
            println!("Error parsing object id from path: {:?}", e);
            Err(ApiError::InternalServerError)
        }
    }
}

async fn delete_user(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<User>, ApiError> {
    let oid = ObjectId::parse_str(id).map_err(|e| {
        println!("Error parsing object id from request path: {:?}", e);
        ApiError::InternalServerError
    })?;

    let user = app_state
        .users
        .find_one(doc! {"_id": oid})
        .await
        .map_err(|e| {
            println!("Database error: {:?}", e);
            ApiError::InternalServerError
        })?
        .ok_or_else(|| {
            println!("A user with object id of {} was not found.", oid);
            ApiError::NotFound
        })?;

    println!("User found:\n{:?}\nDeleting now...", user);

    // must convert User type to mongo document for deletion
    let user_doc = to_document(&user).unwrap();
    app_state.users.delete_one(user_doc).await.map_err(|e| {
        println!("Error during user deletion: {:?}", e);
        ApiError::InternalServerError
    })?;

    Ok(ApiResponse::Deleted)
}

async fn update_user() {}

pub fn users_router(app_state: Arc<AppState>) -> Router {
    Router::new()
        .route("/users/create", post(create_user))
        .route("/users/{id}", get(get_user))
        .route("/users/{id}", patch(update_user))
        .route("/users/{id}", delete(delete_user))
        .with_state(app_state)
}
