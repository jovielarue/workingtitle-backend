use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId, to_document},
};

use crate::{
    api::types::{ApiError, ApiResponse},
    user::models::{PartialUser, User},
};

/// Creates a user document in the database from a User struct.
pub async fn create(
    user: User,
    collection: Collection<User>,
) -> Result<ApiResponse<String>, ApiError<String>> {
    collection.insert_one(user).await.map_err(|e| {
        let err = format!("Unable to create user: {:?}", e);
        println!("{}", err);

        ApiError::InternalServerError(Some(vec![err.to_string()]))
    })?;

    println!("Successfully created user.");
    Ok(ApiResponse::Created(Some(vec![
        "Successfully created user.".to_string(),
    ])))
}

/// Gets a specified user document by id from the database and returns a User struct.
pub async fn read(
    id: String,
    collection: Collection<User>,
) -> Result<ApiResponse<User>, ApiError<String>> {
    let oid = get_oid(id)?;

    let user = collection
        .find_one(doc! {"_id": oid})
        .await
        .map_err(|e| {
            let err = format!("Database error: {:?}", e);
            println!("{}", err);

            ApiError::InternalServerError(Some(vec![err.to_string()]))
        })?
        .ok_or_else(|| {
            let err = format!("A user with object id of {} was not found.", oid);
            ApiError::NotFound(Some(vec![err.to_string()]))
        })?;

    Ok(ApiResponse::OK(Some(vec![user])))
}

/// Updates a user document in the database by id and returns the updated User struct.
pub async fn update(
    user: PartialUser,
    id: String,
    collection: Collection<User>,
) -> Result<ApiResponse<User>, ApiError<String>> {
    let user_doc = to_document(&user).unwrap();
    let updated_user = doc! { "$set": user_doc };
    let oid = get_oid(id.clone())?;

    collection
        .update_one(doc! {"_id": oid}, updated_user)
        .await
        .map_err(|e| {
            let err = format!("Database error: {:?}", e);
            println!("{}", e);

            ApiError::InternalServerError(Some(vec![err.to_string()]))
        })?;

    read(id, collection).await
}

// Deletes a user document in the database by id.
pub async fn delete(
    id: String,
    collection: Collection<User>,
) -> Result<ApiResponse<User>, ApiError<String>> {
    let oid = get_oid(id)?;

    let user = collection
        .find_one(doc! {"_id": oid})
        .await
        .map_err(|e| {
            let err = format!("Database error: {:?}", e);
            println!("{}", err);

            ApiError::InternalServerError(Some(vec![err.to_string()]))
        })?
        .ok_or_else(|| {
            let err = format!("A user with object id of {} was not found.", oid);
            ApiError::NotFound(Some(vec![err.to_string()]))
        })?;

    println!("User found:\n{:?}\nDeleting now...", user);

    // must convert User type to mongo document for deletion
    let user_doc = to_document(&user).unwrap();
    collection.delete_one(user_doc).await.map_err(|e| {
        let err = format!("Error during user deletion: {:?}", e);
        println!("{}", err);

        ApiError::InternalServerError(Some(vec![err.to_string()]))
    })?;

    Ok(ApiResponse::Deleted(None))
}

// Parses a Mongodb ObjectID from a String
fn get_oid(id: String) -> Result<ObjectId, ApiError<String>> {
    match ObjectId::parse_str(&id).ok() {
        Some(oid) => Ok(oid),
        None => {
            let err = format!("Error parsing objectid from {}.", id);
            Err(ApiError::InternalServerError(Some(vec![err.to_string()])))
        }
    }
}
