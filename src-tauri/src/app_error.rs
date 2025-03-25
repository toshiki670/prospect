use axum::http::StatusCode;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum AppError {
    #[error("Invalid validation: {0}")]
    ImvalidValidation(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::ImvalidValidation(_) => StatusCode::UNPROCESSABLE_ENTITY,
            AppError::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<AppError> for String {
    fn from(error: AppError) -> Self {
        match error {
            AppError::ImvalidValidation(message) => message,
            AppError::InternalServerError(message) => message,
        }
    }
}

impl From<anyhow::Error> for AppError {
    fn from(error: anyhow::Error) -> Self {
        if let Some(app_error) = error.downcast_ref::<AppError>() {
            app_error.clone()
        } else {
            AppError::InternalServerError(error.to_string())
        }
    }
}
