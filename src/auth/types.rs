use std::fmt::Display;

use axum::{RequestPartsExt, extract::FromRequestParts, http::request::Parts};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

use crate::{api::types::ApiError, auth::KEYS};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub first_name: String,
    pub last_name: String,
    pub exp: usize,
}
impl Display for Claims {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "First Name: {}\nLast Name: {}",
            self.first_name, self.last_name
        )
    }
}
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = ApiError<String>;

    /// Get claims from JWT
    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        // Extract the token from the authorization header
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| ApiError::Unauthorized(Some(vec!["Invalid JWT.".to_string()])))?;
        // Decode the user data
        let token_data =
            decode::<Claims>(bearer.token(), &KEYS.decoding, &Validation::default())
                .map_err(|_| ApiError::Unauthorized(Some(vec!["Invalid JWT.".to_string()])))?;

        Ok(token_data.claims)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    access_token: String,
    token_type: String,
}
impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub email: String,
    pub password: String,
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
