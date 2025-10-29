# Native DB Integration Guide

## Overview

nihon.rs now uses **native_db** for persistent storage of all application data. This embedded database provides fast, type-safe storage with automatic serialization/deserialization.

## Dependencies

- **native_db** (v0.8.2): Embedded database built on redb
- **native_model** (v0.4.20): Serialization framework (must match native_db version)
- **serde** (v1.0): Serialization/deserialization traits
- **chrono** (v0.4): Date/time handling with serde support
- **once_cell** (v1.20): Lazy static initialization for models

## Database Models

All database models are defined in `src/models/` and use native_db macros:

### 1. FlashCard (`src/models/flashcard.rs`)

Stores individual flashcards with SRS (Spaced Repetition System) metadata.

```rust
#[derive(Serialize, Deserialize)]
#[native_model(id = 1, version = 1)]
#[native_db]
pub struct FlashCard {
    #[primary_key]
    pub id: String,
    #[secondary_key]
    pub deck_id: String,         // Indexed for fast deck queries
    pub card_type: CardType,     // Vocabulary or Grammar
    pub srs_data: SRSData,       // SM-2 algorithm data
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

**SRS Data Structure:**
```rust
pub struct SRSData {
    pub ease_factor: f32,        // Difficulty multiplier (default 2.5)
    pub interval: u32,           // Days until next review
    pub repetitions: u32,        // Consecutive correct answers
    pub next_review: DateTime<Utc>,
    pub is_new: bool,            // Never reviewed before
}
```

### 2. Deck (`src/models/deck.rs`)

Organizes flashcards into collections.

```rust
#[derive(Serialize, Deserialize)]
#[native_model(id = 2, version = 1)]
#[native_db]
pub struct Deck {
    #[primary_key]
    pub id: String,
    pub name: String,
    pub description: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### 3. LearningText (`src/models/deck.rs`)

Stores user-submitted Japanese texts with tokenization.

```rust
#[derive(Serialize, Deserialize)]
#[native_model(id = 3, version = 1)]
#[native_db]
pub struct LearningText {
    #[primary_key]
    pub id: String,
    pub title: String,
    pub original_text: String,
    pub tokenized_segments: Vec<WordSegment>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

### 4. CachedResponse (`src/models/deck.rs`)

Caches LLM responses to minimize API costs.

```rust
#[derive(Serialize, Deserialize)]
#[native_model(id = 4, version = 1)]
#[native_db]
pub struct CachedResponse {
    #[primary_key]
    pub cache_key: String,
    pub response: String,
    pub created_at: DateTime<Utc>,
}
```

### 5. UserSetting (`src/models/deck.rs`)

Stores application settings and preferences.

```rust
#[derive(Serialize, Deserialize)]
#[native_model(id = 5, version = 1)]
#[native_db]
pub struct UserSetting {
    #[primary_key]
    pub key: String,
    pub value: String,
    pub updated_at: DateTime<Utc>,
}
```

## Database Service API

The `DatabaseService` (`src/services/database.rs`) provides a high-level API for database operations.

### Initialization

```rust
use std::path::PathBuf;
use crate::services::database::DatabaseService;

// Production: create database file
let db_path = PathBuf::from("./data/nihonrs.db");
let db = DatabaseService::new(db_path)?;

// Testing: in-memory database
let db = DatabaseService::new_in_memory()?;
```

### Flashcard Operations

```rust
// Create a flashcard
db.save_card(
    "card-123".to_string(),
    "deck-1".to_string(),
    CardType::Vocabulary(VocabularyCard {
        kanji: "日本".to_string(),
        hiragana: "にほん".to_string(),
        romaji: "nihon".to_string(),
        meaning: "Japan".to_string(),
        wrong_answers: vec!["China".to_string(), "Korea".to_string()],
        example_sentences: vec![...],
        jlpt_level: JLPTLevel::N5,
    })
)?;

// Update SRS data after review
db.update_card_srs("card-123", SRSData {
    ease_factor: 2.6,
    interval: 7,
    repetitions: 2,
    next_review: Utc::now() + Duration::days(7),
    is_new: false,
})?;

// Get all cards in a deck
let cards = db.get_deck_cards("deck-1")?;
```

### Deck Operations

```rust
// Create a deck
db.create_deck(
    "deck-1".to_string(),
    "JLPT N5 Vocabulary".to_string(),
    "Basic vocabulary for JLPT N5".to_string()
)?;

// Load all decks with statistics
let decks = db.load_decks()?;
for deck in decks {
    println!("{}: {} cards ({} due, {} new)",
        deck.name, deck.total_cards, deck.due_cards, deck.new_cards);
}
```

### Text Operations

```rust
// Save a learning text
db.save_text(LearningText {
    id: "text-1".to_string(),
    title: "My First Article".to_string(),
    original_text: "日本語を勉強しています。".to_string(),
    tokenized_segments: vec![...],
    created_at: Utc::now(),
    updated_at: Utc::now(),
})?;

// Load all texts
let texts = db.load_texts()?;

// Get specific text
let text = db.get_text("text-1")?;
```

### Settings Operations

```rust
// Save settings
db.save_settings("theme", "dark")?;
db.save_settings("api_key", "sk-...")?;

// Load settings
let theme = db.load_settings("theme")?;
```

### LLM Cache Operations

```rust
// Cache an LLM response
let cache_key = format!("explain:食べる");
db.cache_llm_response(&cache_key, "{\"meaning\": \"to eat\", ...}")?;

// Retrieve cached response
if let Some(cached) = db.get_cached_response(&cache_key)? {
    // Use cached response (no API call needed)
}
```

## Key Design Patterns

### 1. Static Model Registry

All models are registered once in a static `MODELS` variable using `once_cell`:

```rust
static MODELS: Lazy<Models> = Lazy::new(|| {
    let mut models = Models::new();
    models.define::<FlashCard>().unwrap();
    models.define::<Deck>().unwrap();
    models.define::<LearningText>().unwrap();
    models.define::<CachedResponse>().unwrap();
    models.define::<UserSetting>().unwrap();
    models
});
```

### 2. Transaction Pattern

All database operations use explicit transactions:

```rust
// Read-write transaction
let rw = db.rw_transaction()?;
rw.insert(data)?;
rw.commit()?;

// Read-only transaction
let r = db.r_transaction()?;
let data = r.get().primary(id)?;
```

### 3. Secondary Index Queries

Use secondary keys for efficient filtering:

```rust
// Get all cards in a specific deck (uses secondary index)
let cards = r
    .scan()
    .secondary(FlashCardKey::deck_id)?
    .start_with("deck-1".to_string())?
    .collect()?;
```

## Data Migration

When updating model structures, native_db supports automatic migration:

### Example: Adding a field to FlashCard

```rust
// Version 1 (original)
#[native_model(id = 1, version = 1)]
pub struct FlashCard {
    pub id: String,
    pub deck_id: String,
    pub card_type: CardType,
}

// Version 2 (with new field)
#[native_model(id = 1, version = 2, from = v1::FlashCard)]
pub struct FlashCard {
    pub id: String,
    pub deck_id: String,
    pub card_type: CardType,
    pub srs_data: SRSData,  // New field
}

// Implement From trait for migration
impl From<v1::FlashCard> for FlashCard {
    fn from(old: v1::FlashCard) -> Self {
        Self {
            id: old.id,
            deck_id: old.deck_id,
            card_type: old.card_type,
            srs_data: SRSData::default(),  // Default for new field
        }
    }
}

// Run migration
let rw = db.rw_transaction()?;
rw.migrate::<FlashCard>()?;
rw.commit()?;
```

## Database File Location

Recommended locations by platform:

- **Linux**: `~/.local/share/nihonrs/nihonrs.db`
- **macOS**: `~/Library/Application Support/nihonrs/nihonrs.db`
- **Windows**: `%APPDATA%\nihonrs\nihonrs.db`

Use the `directories` crate for cross-platform paths:

```rust
use directories::ProjectDirs;

if let Some(proj_dirs) = ProjectDirs::from("rs", "nihonrs", "nihonrs") {
    let data_dir = proj_dirs.data_dir();
    std::fs::create_dir_all(data_dir)?;
    let db_path = data_dir.join("nihonrs.db");
    let db = DatabaseService::new(db_path)?;
}
```

## Performance Considerations

1. **Batch Operations**: Use transactions for multiple inserts/updates
2. **Secondary Indexes**: Only add when needed for queries (adds overhead)
3. **Cache LLM Responses**: Always check cache before making API calls
4. **Compact Database**: Periodically call `db.compact()` to reclaim space

## Error Handling

All database operations return `Result<T, DatabaseError>`:

```rust
pub enum DatabaseError {
    ConnectionError(String),
    QueryError(String),
    SerializationError(String),
}
```

Use `?` operator for error propagation or `match` for custom handling:

```rust
match db.save_card(id, deck_id, card_type) {
    Ok(_) => println!("Card saved!"),
    Err(DatabaseError::ConnectionError(msg)) => {
        eprintln!("Database connection failed: {}", msg);
    }
    Err(e) => eprintln!("Error: {}", e),
}
```

## Testing

Example test using in-memory database:

```rust
#[test]
fn test_save_and_load_card() {
    let db = DatabaseService::new_in_memory().unwrap();
    
    db.save_card("test-1".to_string(), "deck-1".to_string(), card_type).unwrap();
    
    let cards = db.get_deck_cards("deck-1").unwrap();
    assert_eq!(cards.len(), 1);
    assert_eq!(cards[0].id, "test-1");
}
```

## Next Steps

1. **Integrate with App**: Initialize database in `App::new()`
2. **LLM Integration**: Use cache operations when implementing `LLMService`
3. **Tokenizer Integration**: Store `WordSegment` results in `LearningText`
4. **SRS Algorithm**: Implement SM-2 algorithm for flashcard reviews
5. **Settings UI**: Load/save user preferences via `UserSetting`

## Resources

- [native_db Documentation](https://docs.rs/native_db/latest/native_db/)
- [native_model Documentation](https://docs.rs/native_model/latest/native_model/)
- [GitHub Repository](https://github.com/vincent-herlemont/native_db)
- [SM-2 Algorithm](https://www.supermemo.com/en/archives1990-2015/english/ol/sm2)
