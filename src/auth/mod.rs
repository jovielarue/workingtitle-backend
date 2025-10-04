use std::sync::LazyLock;

use axum::extract::Multipart;
use dotenv::dotenv;
use jsonwebtoken::{Header, encode};

use crate::{
    api::types::{ApiError, ApiResponse},
    auth::types::{AuthBody, AuthPayload, Claims, Keys},
    utility::parse_form,
};

pub mod types;

static KEYS: LazyLock<Keys> = LazyLock::new(|| {
    dotenv().ok();
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

/// Takes in a multipart/form-data in the form of the AuthPayload struct and returns a JWT
pub async fn authorize(multipart: Multipart) -> Result<ApiResponse<AuthBody>, ApiError<String>> {
    let payload = parse_form::<AuthPayload>(multipart).await?;
    // Check if the user sent the credentials
    if payload.email.is_empty() || payload.password.is_empty() {
        return Err(ApiError::Unauthorized(Some(vec![
            "Email or password is missing.".to_string(),
        ])));
    }
    // Here you can check the user credentials from a database
    if payload.email != "test@email.com" || payload.password != "password" {
        return Err(ApiError::Unauthorized(Some(vec![
            "Bad credentials. Please try again.".to_string(),
        ])));
    }

    // here is where you'd grab info from the database to fulfill first_ and last_name
    let claims = Claims {
        first_name: "Jovie".to_string(),
        last_name: "LaRue".to_string(),
        // Mandatory expiry time as UTC timestamp
        // generate this, don't just use this placeholder
        exp: 3000000000,
    };
    // Create the authorization token
    let token = encode(&Header::default(), &claims, &KEYS.encoding).map_err(|_| {
        ApiError::Unauthorized(Some(vec!["Unable to generate auth token.".to_string()]))
    })?;

    // Send the authorized token
    Ok(ApiResponse::OK(Some(vec![AuthBody::new(token)])))
}

pub async fn protected_test(_claims: Claims) -> Result<ApiResponse<String>, ApiError<String>> {
    Ok(ApiResponse::OK(Some(vec![
        "You've successfully accessed the protected route!".to_string(),
    ])))
}
