//! Reusable UI components
//!
//! This module contains reusable widgets and UI elements that are used
//! across multiple views to avoid code duplication.

pub mod jlpt_badge;
pub mod example_display;
pub mod quiz_state;

// Re-export commonly used components
pub use jlpt_badge::jlpt_badge;
pub use example_display::example_sentences;
pub use quiz_state::QuizState;
