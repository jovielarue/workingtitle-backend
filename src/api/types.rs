use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;

pub enum ApiResponse<T>
where
    T: Serialize,
{
    OK(Option<Vec<T>>),
    Created(Option<Vec<T>>),
    Deleted(Option<Vec<T>>),
}

pub enum ApiError<T>
where
    T: Serialize,
{
    BadRequest(Option<Vec<T>>),
    NotFound(Option<Vec<T>>),
    Unauthorized(Option<Vec<T>>),
    InternalServerError(Option<Vec<T>>),
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Self::OK(data) => match data {
                Some(d) => (StatusCode::OK, Json(d)).into_response(),
                None => (StatusCode::OK).into_response(),
            },
            Self::Created(data) => match data {
                Some(d) => (StatusCode::CREATED, Json(d)).into_response(),
                None => (StatusCode::CREATED).into_response(),
            },
            Self::Deleted(data) => match data {
                Some(d) => (StatusCode::OK, Json(d)).into_response(),
                None => (StatusCode::OK).into_response(),
            },
        }
    }
}

impl<T> IntoResponse for ApiError<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest(data) => match data {
                Some(d) => (StatusCode::BAD_REQUEST, Json(d)).into_response(),
                None => (StatusCode::BAD_REQUEST).into_response(),
            },
            Self::NotFound(data) => match data {
                Some(d) => (StatusCode::NOT_FOUND, Json(d)).into_response(),
                None => (StatusCode::NOT_FOUND).into_response(),
            },
            Self::Unauthorized(data) => match data {
                Some(d) => (StatusCode::UNAUTHORIZED, Json(d)).into_response(),
                None => (StatusCode::UNAUTHORIZED).into_response(),
            },
            Self::InternalServerError(data) => match data {
                Some(d) => (StatusCode::INTERNAL_SERVER_ERROR, Json(d)).into_response(),
                None => (StatusCode::INTERNAL_SERVER_ERROR).into_response(),
            },
        }
    }
}
