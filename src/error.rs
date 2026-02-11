use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[allow(clippy::enum_variant_names)]
#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("MongoDB error: {0}")]
    MongoError(#[from] mongodb::error::Error),

    #[error("JWT error: {0}")]
    JwtError(#[from] jsonwebtoken::errors::Error),

    #[error("BCrypt error: {0}")]
    BcryptError(#[from] bcrypt::BcryptError),

    #[error("BSON OID error: {0}")]
    BsonOidError(#[from] bson::oid::Error),

    #[error("Chrono parse error: {0}")]
    ChronoParseError(#[from] chrono::format::ParseError),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Authentication error: {0}")]
    AuthError(String),
    // #[error("Not found: {0}")]
    // NotFound(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = match self {
            AppError::MongoError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::JwtError(_) => StatusCode::UNAUTHORIZED,
            AppError::BcryptError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::BsonOidError(_) => StatusCode::BAD_REQUEST,
            AppError::ChronoParseError(_) => StatusCode::BAD_REQUEST,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::AuthError(_) => StatusCode::UNAUTHORIZED,
            // AppError::NotFound(_) => StatusCode::NOT_FOUND,
        };

        let body = Json(json!({
            "error": self.to_string(),
        }));

        (status, body).into_response()
    }
}
