//! Deck and text management models

/// Information about a flashcard deck
#[derive(Debug, Clone)]
pub struct DeckInfo {
    pub id: String,
    pub name: String,
    pub total_cards: usize,
    pub due_cards: usize,
    pub new_cards: usize,
}

/// Information about a saved learning text
#[derive(Debug, Clone)]
pub struct TextInfo {
    pub id: String,
    pub title: String,
    pub preview: String,
    pub created_at: String,
}
