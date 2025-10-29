//! Database service for persistent storage
//!
//! This service uses `native_db` for local storage of:
//! - Flashcards and SRS progress
//! - Saved texts and cached LLM responses
//! - User settings and preferences

use crate::models::{
    deck::{CachedResponse, Deck, LearningText, UserSetting},
    flashcard::{CardType, FlashCard, SRSData},
    DeckInfo, TextInfo,
};
use chrono::Utc;
use native_db::{Builder, Database, Models};
use once_cell::sync::Lazy;
use std::path::PathBuf;

// Define all database models
static MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<FlashCard>().unwrap();
    models.define::<Deck>().unwrap();
    models.define::<LearningText>().unwrap();
    models.define::<CachedResponse>().unwrap();
    models.define::<UserSetting>().unwrap();
    models
});

/// Database service for persistent storage
pub struct DatabaseService {
    db: Database<'static>,
}

impl DatabaseService {
    /// Initialize the database service with a file path
    pub fn new(db_path: PathBuf) -> Result<Self, DatabaseError> {
        let db = Builder::new()
            .create(&MODELS, db_path)
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        Ok(Self { db })
    }

    /// Initialize in-memory database (for testing)
    #[allow(dead_code)]
    pub fn new_in_memory() -> Result<Self, DatabaseError> {
        let db = Builder::new()
            .create_in_memory(&MODELS)
            .map_err(|e| DatabaseError::ConnectionError(e.to_string()))?;

        Ok(Self { db })
    }

    /// Save a flashcard to the database
    pub fn save_card(
        &self,
        id: String,
        deck_id: String,
        card_type: CardType,
    ) -> Result<(), DatabaseError> {
        let rw = self
            .db
            .rw_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let now = Utc::now();
        let card = FlashCard {
            id,
            deck_id,
            card_type,
            srs_data: SRSData {
                ease_factor: 2.5,
                interval: 0,
                repetitions: 0,
                next_review: now,
                is_new: true,
            },
            created_at: now,
            updated_at: now,
        };

        rw.insert(card)
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        rw.commit()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Update a flashcard's SRS data
    pub fn update_card_srs(&self, id: &str, srs_data: SRSData) -> Result<(), DatabaseError> {
        let rw = self
            .db
            .rw_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let old_card: FlashCard = rw
            .get()
            .primary(id.to_string())
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .ok_or_else(|| DatabaseError::QueryError(format!("Card not found: {}", id)))?;

        let mut new_card = old_card.clone();
        new_card.srs_data = srs_data;
        new_card.updated_at = Utc::now();

        rw.update(old_card, new_card)
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        rw.commit()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Get all cards in a deck
    pub fn get_deck_cards(&self, deck_id: &str) -> Result<Vec<FlashCard>, DatabaseError> {
        let r = self
            .db
            .r_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let cards: Result<Vec<FlashCard>, _> = r
            .scan()
            .secondary(crate::models::flashcard::FlashCardKey::deck_id)
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .start_with(deck_id.to_string())
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect();
        let cards = cards.map_err(|e: native_db::db_type::Error| DatabaseError::QueryError(e.to_string()))?;

        Ok(cards)
    }

    /// Create a new deck
    pub fn create_deck(&self, id: String, name: String, description: String) -> Result<(), DatabaseError> {
        let rw = self
            .db
            .rw_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let now = Utc::now();
        let deck = Deck {
            id,
            name,
            description,
            created_at: now,
            updated_at: now,
        };

        rw.insert(deck)
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        rw.commit()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Load all decks with statistics
    pub fn load_decks(&self) -> Result<Vec<DeckInfo>, DatabaseError> {
        let r = self
            .db
            .r_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let decks: Result<Vec<Deck>, _> = r
            .scan()
            .primary()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .all()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect();
        let decks = decks.map_err(|e: native_db::db_type::Error| DatabaseError::QueryError(e.to_string()))?;

        let mut deck_infos = Vec::new();
        for deck in decks {
            let cards = self.get_deck_cards(&deck.id)?;
            let total_cards = cards.len();
            let now = Utc::now();
            let due_cards = cards
                .iter()
                .filter(|c| c.srs_data.next_review <= now)
                .count();
            let new_cards = cards.iter().filter(|c| c.srs_data.is_new).count();

            deck_infos.push(DeckInfo {
                id: deck.id,
                name: deck.name,
                total_cards,
                due_cards,
                new_cards,
            });
        }

        Ok(deck_infos)
    }

    /// Save a learning text
    pub fn save_text(&self, text: LearningText) -> Result<(), DatabaseError> {
        let rw = self
            .db
            .rw_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        rw.insert(text)
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        rw.commit()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Load all saved texts
    pub fn load_texts(&self) -> Result<Vec<TextInfo>, DatabaseError> {
        let r = self
            .db
            .r_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let texts: Result<Vec<LearningText>, _> = r
            .scan()
            .primary()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .all()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?
            .collect();
        let texts = texts.map_err(|e: native_db::db_type::Error| DatabaseError::QueryError(e.to_string()))?;

        let text_infos = texts
            .into_iter()
            .map(|t| TextInfo {
                id: t.id,
                title: t.title,
                preview: t
                    .original_text
                    .chars()
                    .take(100)
                    .collect::<String>(),
                created_at: t.created_at.format("%Y-%m-%d %H:%M").to_string(),
            })
            .collect();

        Ok(text_infos)
    }

    /// Get a specific text by ID
    pub fn get_text(&self, id: &str) -> Result<Option<LearningText>, DatabaseError> {
        let r = self
            .db
            .r_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let text = r
            .get()
            .primary(id.to_string())
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(text)
    }

    /// Save or update user settings
    pub fn save_settings(&self, key: &str, value: &str) -> Result<(), DatabaseError> {
        let rw = self
            .db
            .rw_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let setting = UserSetting {
            key: key.to_string(),
            value: value.to_string(),
            updated_at: Utc::now(),
        };

        rw.upsert(setting)
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        rw.commit()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Load user settings
    pub fn load_settings(&self, key: &str) -> Result<Option<String>, DatabaseError> {
        let r = self
            .db
            .r_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let setting: Option<UserSetting> = r
            .get()
            .primary(key.to_string())
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(setting.map(|s| s.value))
    }

    /// Cache an LLM response
    pub fn cache_llm_response(&self, key: &str, response: &str) -> Result<(), DatabaseError> {
        let rw = self
            .db
            .rw_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let cached = CachedResponse {
            cache_key: key.to_string(),
            response: response.to_string(),
            created_at: Utc::now(),
        };

        rw.upsert(cached)
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;
        rw.commit()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(())
    }

    /// Get cached LLM response
    pub fn get_cached_response(&self, key: &str) -> Result<Option<String>, DatabaseError> {
        let r = self
            .db
            .r_transaction()
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        let cached: Option<CachedResponse> = r
            .get()
            .primary(key.to_string())
            .map_err(|e| DatabaseError::QueryError(e.to_string()))?;

        Ok(cached.map(|c| c.response))
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

impl std::error::Error for DatabaseError {}
