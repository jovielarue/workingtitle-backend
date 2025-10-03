use mongodb::{
    Collection,
    bson::{doc, oid::ObjectId, to_document},
};
use serde::{Serialize, de::DeserializeOwned};

use crate::api::types::{ApiError, ApiResponse};

/// Creates a event document in the database from a struct.
pub async fn create<T>(
    item: T,
    collection: Collection<T>,
) -> Result<ApiResponse<String>, ApiError<String>>
where
    T: Serialize + Send + Sync,
{
    collection.insert_one(item).await.map_err(|e| {
        let err = format!("Unable to create document in database: {:?}", e);
        println!("{}", err);

        ApiError::InternalServerError(Some(vec![err.to_string()]))
    })?;

    let message = format!("Successfully created document in database.");
    println!("{}", message);
    Ok(ApiResponse::Created(Some(vec![message.to_string()])))
}

/// Gets a specified event document by id from the database and returns a struct.
pub async fn read<T>(
    id: String,
    collection: Collection<T>,
) -> Result<ApiResponse<T>, ApiError<String>>
where
    T: Serialize + DeserializeOwned + Send + Sync,
{
    println!("Here");
    println!("id: {}", id);
    let oid = get_oid(id)?;
    println!("oid: {}", oid);

    let item = collection
        .find_one(doc! {"_id": oid})
        .await
        .map_err(|e| {
            let err = format!("Database error: {:?}", e);
            println!("{}", err);

            ApiError::InternalServerError(Some(vec![err.to_string()]))
        })?
        .ok_or_else(|| {
            let err = format!("A document with an ObjectID of {} was not found.", oid);
            ApiError::NotFound(Some(vec![err.to_string()]))
        })?;

    Ok(ApiResponse::OK(Some(vec![item])))
}

/// Updates a document in the database by id and returns the updated struct.
/// T: full struct, P: partial struct
pub async fn update<T, P>(
    item: P,
    id: String,
    collection: Collection<T>,
) -> Result<ApiResponse<T>, ApiError<String>>
where
    T: Serialize + DeserializeOwned + Send + Sync,
    P: Serialize,
{
    let event_doc = to_document(&item).unwrap();
    let updated_event = doc! { "$set": event_doc };
    let oid = get_oid(id.clone())?;

    collection
        .update_one(doc! {"_id": oid}, updated_event)
        .await
        .map_err(|e| {
            let err = format!("Database error: {:?}", e);
            println!("{}", e);

            ApiError::InternalServerError(Some(vec![err.to_string()]))
        })?;

    read::<T>(id, collection).await
}

// Deletes a event document in the database by id.
pub async fn delete<T>(
    id: String,
    collection: Collection<T>,
) -> Result<ApiResponse<T>, ApiError<String>>
where
    T: Serialize + DeserializeOwned + Send + Sync,
{
    let oid = get_oid(id)?;

    let item = collection
        .find_one(doc! {"_id": oid})
        .await
        .map_err(|e| {
            let err = format!("Database error: {:?}", e);
            println!("{}", err);

            ApiError::InternalServerError(Some(vec![err.to_string()]))
        })?
        .ok_or_else(|| {
            let err = format!("A document with an ObjectID of {} was not found.", oid);
            ApiError::NotFound(Some(vec![err.to_string()]))
        })?;

    println!("Document found. Deleting now...");

    // must convert type to mongo document for deletion
    let event_doc = to_document(&item).unwrap();
    collection.delete_one(event_doc).await.map_err(|e| {
        let err = format!("Error during document deletion: {:?}", e);
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
            let err = format!("Error parsing ObjectID from {}.", id);
            Err(ApiError::InternalServerError(Some(vec![err.to_string()])))
        }
    }
}
