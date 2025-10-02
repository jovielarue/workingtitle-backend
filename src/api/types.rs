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
    OK,
    Created,
    Deleted,
    JsonData(Vec<T>),
}

pub enum ApiError<T> {
    BadRequest,
    NotFound,
    Unauthorized,
    InternalServerError(Vec<T>),
}

impl<T> IntoResponse for ApiResponse<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Self::OK => (StatusCode::OK).into_response(),
            Self::Created => (StatusCode::CREATED).into_response(),
            Self::Deleted => (StatusCode::OK).into_response(),
            Self::JsonData(data) => (StatusCode::OK, Json(data)).into_response(),
        }
    }
}

impl<T> IntoResponse for ApiError<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match self {
            Self::BadRequest => (StatusCode::BAD_REQUEST).into_response(),
            Self::NotFound => (StatusCode::NOT_FOUND).into_response(),
            Self::Unauthorized => (StatusCode::UNAUTHORIZED).into_response(),
            Self::InternalServerError(data) => {
                (StatusCode::INTERNAL_SERVER_ERROR, Json(data)).into_response()
            }
        }
    }
}
