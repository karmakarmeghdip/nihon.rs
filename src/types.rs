/// Shared data types used across the application

#[derive(Debug, Clone)]
pub struct DeckInfo {
    pub id: String,
    pub name: String,
    pub total_cards: usize,
    pub due_cards: usize,
    pub new_cards: usize,
}

#[derive(Debug, Clone)]
pub struct TextInfo {
    pub id: String,
    pub title: String,
    pub preview: String,
    pub created_at: String,
}
