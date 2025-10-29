//! Database service for persistent storage
//!
//! This service will use `native_db` for local storage of:
//! - Flashcards and SRS progress
//! - Saved texts and cached LLM responses
//! - User settings and preferences

use crate::models::{DeckInfo, TextInfo, CardType};

/// Database service for persistent storage
pub struct DatabaseService {
    // TODO: Add native_db connection
}

impl DatabaseService {
    /// Initialize the database service
    pub fn new() -> Result<Self, DatabaseError> {
        // TODO: Initialize native_db
        Ok(Self {})
    }

    /// Save a flashcard to the database
    pub fn save_card(&self, _card: CardType) -> Result<(), DatabaseError> {
        // TODO: Implement
        Ok(())
    }

    /// Load all decks
    pub fn load_decks(&self) -> Result<Vec<DeckInfo>, DatabaseError> {
        // TODO: Implement
        Ok(vec![])
    }

    /// Load all saved texts
    pub fn load_texts(&self) -> Result<Vec<TextInfo>, DatabaseError> {
        // TODO: Implement
        Ok(vec![])
    }

    /// Save user settings
    pub fn save_settings(&self, _key: &str, _value: &str) -> Result<(), DatabaseError> {
        // TODO: Implement
        Ok(())
    }

    /// Load user settings
    pub fn load_settings(&self, _key: &str) -> Result<Option<String>, DatabaseError> {
        // TODO: Implement
        Ok(None)
    }

    /// Cache an LLM response
    pub fn cache_llm_response(
        &self,
        _key: &str,
        _response: &str,
    ) -> Result<(), DatabaseError> {
        // TODO: Implement
        Ok(())
    }

    /// Get cached LLM response
    pub fn get_cached_response(&self, _key: &str) -> Result<Option<String>, DatabaseError> {
        // TODO: Implement
        Ok(None)
    }
}

impl Default for DatabaseService {
    fn default() -> Self {
        Self::new().expect("Failed to initialize database")
    }
}

/// Database service errors
#[derive(Debug, Clone)]
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    SerializationError(String),
}

impl std::fmt::Display for DatabaseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DatabaseError::ConnectionError(msg) => write!(f, "Database connection error: {}", msg),
            DatabaseError::QueryError(msg) => write!(f, "Query error: {}", msg),
            DatabaseError::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}
