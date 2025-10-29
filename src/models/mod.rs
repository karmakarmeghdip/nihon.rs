//! Domain models and data structures
//!
//! This module contains all domain models used throughout the application,
//! including flashcards, word segments, JLPT levels, and example sentences.

pub mod flashcard;
pub mod word;
pub mod deck;

// Re-export commonly used types
pub use flashcard::{CardType, GrammarCard, VocabularyCard};
pub use word::{ExampleSentence, JLPTLevel, WordExplanation, WordSegment};
pub use deck::{DeckInfo, TextInfo};
