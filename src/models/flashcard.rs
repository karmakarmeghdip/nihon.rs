//! Flashcard data models for SRS practice

use super::word::{ExampleSentence, JLPTLevel};

/// Type of flashcard
#[derive(Debug, Clone)]
pub enum CardType {
    Vocabulary(VocabularyCard),
    Grammar(GrammarCard),
}

/// Vocabulary flashcard
#[derive(Debug, Clone)]
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
#[derive(Debug, Clone)]
pub struct GrammarCard {
    pub pattern: String,
    pub pattern_reading: String,
    pub explanation: String,
    pub wrong_answers: Vec<String>,
    pub example_sentences: Vec<ExampleSentence>,
    pub jlpt_level: JLPTLevel,
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
