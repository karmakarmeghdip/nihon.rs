//! Word and language-related data models

use iced::Color;

/// Example sentence with Japanese and English
#[derive(Debug, Clone)]
pub struct ExampleSentence {
    pub japanese: String,
    pub english: String,
}

/// JLPT difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JLPTLevel {
    N5, // Beginner
    N4,
    N3, // Intermediate
    N2,
    N1, // Advanced
    Unknown,
}

impl JLPTLevel {
    pub fn as_str(&self) -> &str {
        match self {
            JLPTLevel::N5 => "N5",
            JLPTLevel::N4 => "N4",
            JLPTLevel::N3 => "N3",
            JLPTLevel::N2 => "N2",
            JLPTLevel::N1 => "N1",
            JLPTLevel::Unknown => "?",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            JLPTLevel::N5 => Color::from_rgb(0.4, 0.8, 0.4),  // Green
            JLPTLevel::N4 => Color::from_rgb(0.6, 0.8, 0.4),  // Light green
            JLPTLevel::N3 => Color::from_rgb(0.9, 0.8, 0.3),  // Yellow
            JLPTLevel::N2 => Color::from_rgb(0.9, 0.6, 0.3),  // Orange
            JLPTLevel::N1 => Color::from_rgb(0.9, 0.3, 0.3),  // Red
            JLPTLevel::Unknown => Color::from_rgb(0.6, 0.6, 0.6), // Gray
        }
    }

    /// Parse JLPT level from string (e.g., "N5", "N4")
    pub fn from_str(s: &str) -> Self {
        match s.to_uppercase().as_str() {
            "N5" => JLPTLevel::N5,
            "N4" => JLPTLevel::N4,
            "N3" => JLPTLevel::N3,
            "N2" => JLPTLevel::N2,
            "N1" => JLPTLevel::N1,
            _ => JLPTLevel::Unknown,
        }
    }
}

/// A parsed word segment from Japanese text
#[derive(Debug, Clone)]
pub struct WordSegment {
    pub surface: String,      // Original text (kanji/kana)
    pub reading: String,      // Hiragana reading
    pub base_form: String,    // Dictionary form
    pub explanation: Option<WordExplanation>,
    pub is_selected: bool,
}

/// LLM-generated explanation for a word
#[derive(Debug, Clone)]
pub struct WordExplanation {
    pub meaning: String,
    pub grammar_notes: Option<String>,
    pub examples: Vec<ExampleSentence>,
    pub jlpt_level: String,
}

/// Represents a single furigana span
#[derive(Debug, Clone)]
pub struct FuriganaSpan {
    pub text: String,
    pub reading: Option<String>,
}
