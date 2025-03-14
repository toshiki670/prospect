pub mod shared_kernel;

pub mod tokyo_stock_exchange;

#[derive(Debug, Clone, PartialEq, Eq, thiserror::Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    ValidationError(String),
}
