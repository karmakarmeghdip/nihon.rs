//! Deck and text management models

use chrono::{DateTime, Utc};
use native_db::{native_db, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

use super::word::WordSegment;

/// Information about a flashcard deck (UI model)
#[derive(Debug, Clone)]
pub struct DeckInfo {
    pub id: String,
    pub name: String,
    pub total_cards: usize,
    pub due_cards: usize,
    pub new_cards: usize,
}

/// A flashcard deck - database model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[native_model(id = 2, version = 1)]
#[native_db]
pub struct Deck {
    #[primary_key]
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Information about a saved learning text (UI model)
#[derive(Debug, Clone)]
pub struct TextInfo {
    pub id: String,
    pub title: String,
    pub preview: String,
    pub created_at: String,
}

/// A saved learning text with tokenization - database model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[native_model(id = 3, version = 1)]
#[native_db]
pub struct LearningText {
    #[primary_key]
    pub id: String,
    pub title: String,
    pub original_text: String,
    pub tokenized_segments: Vec<WordSegment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Cached LLM response - database model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[native_model(id = 4, version = 1)]
#[native_db]
pub struct CachedResponse {
    #[primary_key]
    pub cache_key: String,
    pub response: String,
    pub created_at: DateTime<Utc>,
}

/// User settings - database model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[native_model(id = 5, version = 1)]
#[native_db]
pub struct UserSetting {
    #[primary_key]
    pub key: String,
    pub value: String,
    pub updated_at: DateTime<Utc>,
}
