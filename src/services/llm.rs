//! LLM service for AI-powered explanations
//!
//! This service will integrate with Gemini API via the `rig` crate
//! to provide context-aware explanations for Japanese words and grammar.

use crate::models::{WordExplanation, ExampleSentence};

/// LLM service for generating explanations
pub struct LLMService {
    api_key: Option<String>,
    user_context: String,
}

impl LLMService {
    /// Create a new LLM service
    pub fn new(api_key: Option<String>, user_context: String) -> Self {
        Self {
            api_key,
            user_context,
        }
    }

    /// Request an explanation for a Japanese word
    ///
    /// # Arguments
    /// * `surface` - The surface form of the word (kanji/kana)
    /// * `reading` - The hiragana reading
    /// * `base_form` - Dictionary form of the word
    ///
    /// # Returns
    /// A `WordExplanation` with meaning, grammar notes, examples, and JLPT level
    ///
    /// # Future Implementation
    /// - Use `rig` crate to call Gemini API
    /// - Include user context in the prompt
    /// - Implement caching to avoid redundant API calls
    /// - Add exponential backoff retry logic
    pub async fn explain_word(
        &self,
        surface: &str,
        reading: &str,
        base_form: &str,
    ) -> Result<WordExplanation, LLMError> {
        // TODO: Implement actual LLM integration
        // For now, return a placeholder
        Ok(WordExplanation {
            meaning: format!("Meaning of '{}'", surface),
            grammar_notes: Some(format!("Grammar notes for '{}'", base_form)),
            examples: vec![
                ExampleSentence {
                    japanese: format!("{}の例文", surface),
                    english: format!("Example sentence with {}", surface),
                }
            ],
            jlpt_level: "N5".to_string(),
        })
    }

    /// Answer a user's question about the text
    ///
    /// # Arguments
    /// * `question` - The user's question
    /// * `context` - The current text being studied
    ///
    /// # Returns
    /// An answer string from the LLM
    pub async fn answer_question(
        &self,
        question: &str,
        context: &str,
    ) -> Result<String, LLMError> {
        // TODO: Implement actual LLM integration
        Ok(format!("Answer to: '{}' (with context: {})", question, context))
    }

    /// Check if the service is configured (has API key)
    pub fn is_configured(&self) -> bool {
        self.api_key.is_some()
    }

    /// Update the API key
    pub fn set_api_key(&mut self, api_key: String) {
        self.api_key = Some(api_key);
    }

    /// Update user context for personalized responses
    pub fn set_user_context(&mut self, context: String) {
        self.user_context = context;
    }
}

/// LLM service errors
#[derive(Debug, Clone)]
pub enum LLMError {
    NotConfigured,
    NetworkError(String),
    ApiError(String),
    ParseError(String),
}

impl std::fmt::Display for LLMError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LLMError::NotConfigured => write!(f, "LLM service not configured. Please add your API key in settings."),
            LLMError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            LLMError::ApiError(msg) => write!(f, "API error: {}", msg),
            LLMError::ParseError(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}
