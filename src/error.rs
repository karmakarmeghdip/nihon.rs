//! Application-wide error types and utilities

use std::fmt;

/// Application result type alias
pub type AppResult<T> = Result<T, AppError>;

/// Main application error type
#[derive(Debug, Clone)]
pub enum AppError {
    /// LLM service errors
    LLM(String),
    
    /// Database errors
    Database(String),
    
    /// Tokenizer errors
    Tokenizer(String),
    
    /// Configuration errors
    Config(String),
    
    /// Input validation errors
    Validation(String),
    
    /// Network errors
    Network(String),
    
    /// Generic error
    Other(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::LLM(msg) => write!(f, "LLM Error: {}", msg),
            AppError::Database(msg) => write!(f, "Database Error: {}", msg),
            AppError::Tokenizer(msg) => write!(f, "Tokenizer Error: {}", msg),
            AppError::Config(msg) => write!(f, "Configuration Error: {}", msg),
            AppError::Validation(msg) => write!(f, "Validation Error: {}", msg),
            AppError::Network(msg) => write!(f, "Network Error: {}", msg),
            AppError::Other(msg) => write!(f, "Error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

// Conversions from service errors
impl From<crate::services::llm::LLMError> for AppError {
    fn from(err: crate::services::llm::LLMError) -> Self {
        AppError::LLM(err.to_string())
    }
}

impl From<crate::services::database::DatabaseError> for AppError {
    fn from(err: crate::services::database::DatabaseError) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<crate::services::tokenizer::TokenizerError> for AppError {
    fn from(err: crate::services::tokenizer::TokenizerError) -> Self {
        AppError::Tokenizer(err.to_string())
    }
}
