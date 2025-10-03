use std::collections::HashMap;

use axum::extract::Multipart;
use serde::{Serialize, de::DeserializeOwned};

use crate::api::types::ApiError;

pub async fn parse_form<T>(mut multipart: Multipart) -> Result<T, ApiError<String>>
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

        ApiError::InternalServerError(Some(vec![err.to_string()]))
    })?;

    let form_struct: T = serde_json::from_str(&json_string).map_err(|e| {
        let err = format!("Unable to create struct from JSON string: {:?}", e);
        println!("{}", err);

        ApiError::InternalServerError(Some(vec![err.to_string()]))
    })?;

    Ok(form_struct)
}
