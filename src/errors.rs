use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

pub type AppResult<T> = Result<T, AppError>;

#[derive(thiserror::Error, Debug)]
pub enum AppError {
    #[error("Authentication is required to access this resource")]
    Unauthorized,
    #[error("User is not authorized to access this resource")]
    Forbidden,
    #[error("{0}")]
    BadRequest(String),
    #[error("Unprocessable entity request")]
    UnprocessableEntity,
    #[error("{0}")]
    NotFound(String),
    #[error("{0}")]
    Conflict(String),
    #[error("Unexpected error occurred")]
    InternalServerError,
    #[error("{0}")]
    InternalServerErrorWithMessage(String),
    #[error(transparent)]
    AnyhowError(#[from] anyhow::Error),
}

/// converts an AppError into an Axum response
impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status_code, error_message, status_text) = match self {
            AppError::Unauthorized => (StatusCode::UNAUTHORIZED, AppError::Unauthorized.to_string(), "Unauthorized"),
            AppError::Forbidden => (StatusCode::FORBIDDEN, AppError::Forbidden.to_string(), "Forbidden"),
            AppError::BadRequest(err) => (StatusCode::BAD_REQUEST, err, "Bad Request"),
            AppError::UnprocessableEntity => (StatusCode::UNPROCESSABLE_ENTITY, AppError::UnprocessableEntity.to_string(), "Unprocessable Entity"),
            AppError::NotFound(err) => (StatusCode::NOT_FOUND, err, "Not Found"),
            AppError::Conflict(err) => (StatusCode::CONFLICT, err, "Conflict"),
            AppError::InternalServerErrorWithMessage(err) => (StatusCode::INTERNAL_SERVER_ERROR, err, "Internal Server Error"),
            _ => (StatusCode::INTERNAL_SERVER_ERROR, "Internal Server Error".to_string(), "Internal Server Error"),
        };

        let body = Json(ApiError::new(status_code.as_u16(), &error_message, status_text));

        (status_code, body).into_response()
    }
}


/// https://cloud.google.com/apis/design/errors
#[derive(Serialize, Deserialize, Debug, ToSchema)]
pub struct ApiError {
    #[schema(example = "500")]
    pub code: u16,
    #[schema(example = "Internal Server Error")]
    pub message: String,
    #[schema(example = "500")]
    pub status: String,
}

impl ApiError {
    pub fn new(code: u16, message: &str, status: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            status: status.to_string(),
        }
    }
}

