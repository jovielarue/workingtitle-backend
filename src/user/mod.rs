use std::{collections::HashMap, sync::Arc};

use axum::{
    Json, Router,
    extract::{Multipart, Path, State},
    routing::{delete, get, patch, post},
};
use mongodb::bson::{doc, oid::ObjectId, to_document};
use serde::{Deserialize, Serialize, de::DeserializeOwned};

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
    multipart: Multipart,
) -> Result<ApiResponse<User>, ApiError<String>> {
    let user: User = parse_form::<User>(multipart).await?;

    app_state.users.insert_one(user).await.map_err(|e| {
        let err = format!("Unable to create user: {:?}", e);
        println!("{}", err);

        ApiError::InternalServerError(vec![err.to_string()])
    })?;

    println!("Successfully created user.");
    Ok(ApiResponse::Created)
}

async fn parse_form<T>(mut multipart: Multipart) -> Result<T, ApiError<String>>
where
    T: Serialize + DeserializeOwned,
{
    let mut form_data: HashMap<String, String> = HashMap::new();

    while let Some(field) = multipart.next_field().await.unwrap() {
        let key = field.name().unwrap().to_string();
        let value = String::from_utf8(field.bytes().await.unwrap().to_vec()).unwrap();

        println!("KEY {} VALUE {:?}", key, value);
        form_data.insert(key, value);
    }

    let json_string = serde_json::to_string(&form_data).map_err(|e| {
        let err = format!("Unable to create JSON string from form: {:?}", e);
        println!("{}", err);

        ApiError::InternalServerError(vec![err.to_string()])
    })?;

    let form_struct: T = serde_json::from_str(&json_string).map_err(|e| {
        let err = format!("Unable to create struct from JSON string: {:?}", e);
        println!("{}", err);

        ApiError::InternalServerError(vec![err.to_string()])
    })?;

    Ok(form_struct)
}

async fn get_user(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<User>, ApiError<String>> {
    let oid = ObjectId::parse_str(id).map_err(|e| {
        let err = format!("Error parsing object id from request path: {:?}", e);
        println!("{}", err);

        ApiError::InternalServerError(vec![err.to_string()])
    })?;

    let user = app_state
        .users
        .find_one(doc! {"_id": oid})
        .await
        .map_err(|e| {
            let err = format!("Database error: {:?}", e);
            println!("{}", err);

            ApiError::InternalServerError(vec![err.to_string()])
        })?
        .ok_or_else(|| {
            println!("A user with object id of {} was not found.", oid);
            ApiError::NotFound
        })?;

    Ok(ApiResponse::JsonData(vec![user]))
}

async fn delete_user(
    State(app_state): State<Arc<AppState>>,
    Path(id): Path<String>,
) -> Result<ApiResponse<User>, ApiError<String>> {
    let oid = ObjectId::parse_str(id).map_err(|e| {
        let err = format!("Error parsing object id from request path: {:?}", e);
        println!("{}", err);

        ApiError::InternalServerError(vec![err.to_string()])
    })?;

    let user = app_state
        .users
        .find_one(doc! {"_id": oid})
        .await
        .map_err(|e| {
            let err = format!("Database error: {:?}", e);
            println!("{}", err);

            ApiError::InternalServerError(vec![err.to_string()])
        })?
        .ok_or_else(|| {
            println!("A user with object id of {} was not found.", oid);
            ApiError::NotFound
        })?;

    println!("User found:\n{:?}\nDeleting now...", user);

    // must convert User type to mongo document for deletion
    let user_doc = to_document(&user).unwrap();
    app_state.users.delete_one(user_doc).await.map_err(|e| {
        let err = format!("Error during user deletion: {:?}", e);
        println!("{}", err);

        ApiError::InternalServerError(vec![err.to_string()])
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
