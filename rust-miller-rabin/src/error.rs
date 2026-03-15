//! Error types for Miller-Rabin primality testing

use thiserror::Error;

/// Errors that can occur during primality testing
#[derive(Error, Debug, Clone, PartialEq)]
pub enum PrimalityError {
    /// The input number is less than 2
    #[error("input must be >= 2, got {0}")]
    InvalidInput(String),

    /// The base is invalid (must be >= 2 and < n)
    #[error("invalid base: {0}")]
    InvalidBase(String),

    /// Thread-related error during parallel execution
    #[error("parallel execution failed: {0}")]
    ParallelError(String),
}

/// Result type for primality testing operations
pub type Result<T> = std::result::Result<T, PrimalityError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = PrimalityError::InvalidInput("1".to_string());
        assert_eq!(err.to_string(), "input must be >= 2, got 1");

        let err = PrimalityError::InvalidBase("0".to_string());
        assert_eq!(err.to_string(), "invalid base: 0");
    }
}
