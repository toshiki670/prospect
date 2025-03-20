use axum::response::{IntoResponse, Response};

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AppError {
    #[error("Bad request: {0}")]
    BadRequest(String),
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        match error {
            AppError::BadRequest(message) => message,
        }
    }
}

impl TryFrom<anyhow::Error> for AppError {
    type Error = anyhow::Error;

    fn try_from(error: anyhow::Error) -> Result<Self, anyhow::Error> {
        if let Some(app_error) = error.downcast_ref::<AppError>() {
            Ok(app_error.clone())
        } else {
            anyhow::bail!("Not found app error");
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::BadRequest(message) => {
                (axum::http::StatusCode::BAD_REQUEST, message).into_response()
            }
        }
    }
}

impl From<AppError> for Response {
    fn from(error: AppError) -> Self {
        error.into_response()
    }
}
