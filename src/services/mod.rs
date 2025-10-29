//! Service layer for external integrations
//!
//! This module contains services for:
//! - LLM integration (Gemini API)
//! - Database operations (native_db)
//! - Text tokenization (lindera)

pub mod llm;
pub mod database;
pub mod tokenizer;

// Re-export service interfaces
pub use llm::LLMService;
pub use database::DatabaseService;
pub use tokenizer::TokenizerService;
