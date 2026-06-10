/// Error type for prime generation failures
#[derive(Debug)]
pub enum PrimeGenError {
    /// Worker thread panicked during parallel execution
    WorkerThreadPanic(String),
    /// Invalid input parameter
    InvalidInput(String),
}

impl std::fmt::Display for PrimeGenError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PrimeGenError::WorkerThreadPanic(msg) => {
                write!(f, "worker thread panicked: {}", msg)
            }
            PrimeGenError::InvalidInput(msg) => write!(f, "invalid input: {}", msg),
        }
    }
}

impl std::error::Error for PrimeGenError {}
