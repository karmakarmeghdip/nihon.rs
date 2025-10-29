//! Flashcard data models for SRS practice

use super::word::{ExampleSentence, JLPTLevel};
use chrono::{DateTime, Utc};
use native_db::{native_db, ToKey};
use native_model::{native_model, Model};
use serde::{Deserialize, Serialize};

/// Type of flashcard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CardType {
    Vocabulary(VocabularyCard),
    Grammar(GrammarCard),
}

/// Vocabulary flashcard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VocabularyCard {
    pub kanji: String,
    pub hiragana: String,
    pub romaji: String,
    pub meaning: String,
    pub wrong_answers: Vec<String>,
    pub example_sentences: Vec<ExampleSentence>,
    pub jlpt_level: JLPTLevel,
}

/// Grammar flashcard
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GrammarCard {
    pub pattern: String,
    pub pattern_reading: String,
    pub explanation: String,
    pub wrong_answers: Vec<String>,
    pub example_sentences: Vec<ExampleSentence>,
    pub jlpt_level: JLPTLevel,
}

/// A flashcard with SRS metadata - the database model
#[derive(Debug, Clone, Serialize, Deserialize)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct FlashCard {
    #[primary_key]
    pub id: String,
    #[secondary_key]
    pub deck_id: String,
    pub card_type: CardType,
    pub srs_data: SRSData,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

/// Spaced Repetition System data using SM-2 algorithm
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SRSData {
    /// Ease factor (difficulty multiplier)
    pub ease_factor: f32,
    /// Current interval in days
    pub interval: u32,
    /// Number of consecutive correct answers
    pub repetitions: u32,
    /// Next review date
    pub next_review: DateTime<Utc>,
    /// Is this card new (never reviewed)?
    pub is_new: bool,
}

impl CardType {
    /// Get the correct answer for this card
    pub fn correct_answer(&self) -> &str {
        match self {
            CardType::Vocabulary(card) => &card.meaning,
            CardType::Grammar(card) => &card.explanation,
        }
    }

    /// Get the wrong answers for this card
    pub fn wrong_answers(&self) -> &[String] {
        match self {
            CardType::Vocabulary(card) => &card.wrong_answers,
            CardType::Grammar(card) => &card.wrong_answers,
        }
    }

    /// Get the example sentences for this card
    pub fn example_sentences(&self) -> &[ExampleSentence] {
        match self {
            CardType::Vocabulary(card) => &card.example_sentences,
            CardType::Grammar(card) => &card.example_sentences,
        }
    }

    /// Get the JLPT level for this card
    pub fn jlpt_level(&self) -> JLPTLevel {
        match self {
            CardType::Vocabulary(card) => card.jlpt_level,
            CardType::Grammar(card) => card.jlpt_level,
        }
    }
}
