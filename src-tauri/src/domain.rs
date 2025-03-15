pub mod shared_kernel;

pub mod tokyo_stock_exchange;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),
}

impl From<DomainError> for String {
    fn from(error: DomainError) -> Self {
        match error {
            DomainError::ValidationError(message) => message,
        }
    }
}

impl TryFrom<anyhow::Error> for DomainError {
    type Error = anyhow::Error;

    fn try_from(error: anyhow::Error) -> Result<Self, anyhow::Error> {
        if let Some(domain_error) = error.downcast_ref::<DomainError>() {
            Ok(domain_error.clone())
        } else {
            anyhow::bail!("Not found domain error");
        }
    }
}
