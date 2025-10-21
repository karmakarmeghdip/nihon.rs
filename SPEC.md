# nihon.rs - Japanese Learning Tool Specification

## Overview
nihon.rs is a Japanese language learning application built with Rust and iced-rs, combining AI-powered tutoring with spaced repetition practice. Users can paste any Japanese text to learn vocabulary and grammar through interactive cards and AI-guided explanations.

---

## Core Technologies

### Framework & Architecture
- **UI Framework**: `iced-rs` (Elm architecture pattern)
- **LLM Integration**: `rig` (v0.x) - Rust LLM framework similar to Vercel AI SDK
- **Japanese Text Processing**: 
  - **Tokenization**: `lindera` (v1.4.1) - Morphological analysis library for Japanese
  - **Furigana Generation**: `furigana` (v0.1.12) - Maps furigana to words given readings
  - **Script Conversion**: `japanese` (v0.1.2) - Hiragana/Katakana conversion
- **Database**: `native_db` - Embedded database for local storage
- **Async Runtime**: `tokio` - For LLM API calls

---

## Application Structure (Elm Pattern)

### Model (State)
```rust
struct App {
    // Application mode
    mode: AppMode,
    
    // Practice mode state
    practice_state: PracticeState,
    
    // Learning mode state
    learning_state: LearningState,
    
    // User data & settings
    user_profile: UserProfile,
    decks: Vec<Deck>,
    
    // UI state
    input_text: String,
    selected_deck_id: Option<DeckId>,
}

enum AppMode {
    Home,
    Practice,
    Learning,
    Settings,
}
```

### Messages
```rust
enum Message {
    // Navigation
    NavigateTo(AppMode),
    
    // Input handling
    InputTextChanged(String),
    SubmitText,
    
    // Practice mode
    SelectAnswer(usize),
    NextCard,
    PreviousCard,
    ToggleRomaji,
    
    // Learning mode
    RequestExplanation(String),  // User question
    LLMResponseReceived(String),
    AddToFlashcards(WordCard),
    NextWord,
    
    // Deck management
    SelectDeck(DeckId),
    CreateDeck(String),
    
    // Settings
    UpdateFontSize(u16),
    UpdateUserProfile(String),
    ToggleTheme,
    
    // LLM background tasks
    LLMTaskComplete(LLMResult),
}
```

### Update (Logic)
```rust
fn update(app: &mut App, message: Message) -> Command<Message> {
    match message {
        // Handle state transitions, spawn async tasks for LLM calls
    }
}
```

### View (UI)
```rust
fn view(app: &App) -> Element<Message> {
    match app.mode {
        AppMode::Home => home_view(app),
        AppMode::Practice => practice_view(app),
        AppMode::Learning => learning_view(app),
        AppMode::Settings => settings_view(app),
    }
}
```

---

## Features Specification

### 1. Home Screen

**UI Components:**
- Large text input area for pasting Japanese text
- "Practice Mode" button
- "Learning Mode" button
- List of saved decks (for practice mode)
- List of saved texts (for learning mode)
- Settings button (top-right)

**Functionality:**
- Paste Japanese text → triggers parsing and mode selection
- Display deck statistics (cards due, new cards, etc.)
- Quick access to recent texts

---

### 2. Practice Mode (Flashcard SRS System)

#### 2.1 Text Processing Pipeline
1. User pastes Japanese text
2. Tokenization with `lindera` → extracts words/morphemes
3. For each word:
   - Extract kanji form
   - Get reading (hiragana)
   - Generate furigana mapping with `furigana` crate
   - Convert to romaji using `japanese` crate
   - Query LLM for:
     - English meaning
     - 3 semantically related wrong answers
     - Context-appropriate explanation

#### 2.2 Card Structure
```rust
enum FlashCard {
    Vocabulary(VocabularyCard),
    Grammar(GrammarCard),
}

struct VocabularyCard {
    id: CardId,
    kanji: String,           // 食べる
    hiragana: String,        // たべる
    furigana: Vec<FuriganaSpan>, // Mapped positions
    romaji: String,          // taberu
    meaning: String,         // "to eat"
    wrong_answers: Vec<String>, // ["to drink", "to cook", "to buy"]
    
    // Example sentences
    example_sentences: Vec<ExampleSentence>,
    
    // Difficulty
    jlpt_level: JLPTLevel,   // N5, N4, N3, N2, N1
    
    // SRS data
    ease_factor: f32,
    interval: u32,
    repetitions: u32,
    next_review: DateTime<Utc>,
    
    // Metadata
    deck_id: DeckId,
    source_text: String,
    created_at: DateTime<Utc>,
}

struct GrammarCard {
    id: CardId,
    pattern: String,         // 〜てもいい
    pattern_reading: String, // てもいい
    explanation: String,     // "Permission: 'it's okay to...'"
    wrong_answers: Vec<String>, // Other grammar meanings
    
    // Example sentences showing usage
    example_sentences: Vec<ExampleSentence>,
    
    // Difficulty
    jlpt_level: JLPTLevel,
    
    // SRS data
    ease_factor: f32,
    interval: u32,
    repetitions: u32,
    next_review: DateTime<Utc>,
    
    // Metadata
    deck_id: DeckId,
    created_at: DateTime<Utc>,
}

struct ExampleSentence {
    japanese: String,
    furigana: Vec<FuriganaSpan>,
    english: String,
}

struct FuriganaSpan {
    text: String,      // Base text (kanji or kana)
    reading: Option<String>, // Furigana if applicable
}

enum JLPTLevel {
    N5,  // Beginner
    N4,
    N3,  // Intermediate
    N2,
    N1,  // Advanced
    Unknown,
}
```

#### 2.3 Card Display
- **Front of card:**
  - Kanji (large text)
  - Furigana (small text above kanji, shown on hover/tooltip)
  - "Show Romaji" button (spoiler toggle)
  
- **Back of card:**
  - 4 multiple choice buttons for meanings
  - After selection: show correct answer with visual feedback
  - "Next" button to continue

#### 2.4 SRS Algorithm
- **Correct answer**: Increase interval using SM-2 algorithm
- **Incorrect answer**: Reset interval, add card to review queue
- Track statistics: correct/incorrect counts, streak, accuracy %

#### 2.5 Progress Tracking
```rust
struct UserProgress {
    card_id: CardId,
    user_id: UserId,
    times_correct: u32,
    times_incorrect: u32,
    last_reviewed: DateTime<Utc>,
    // Injected into LLM context for personalized explanations
}
```

---

### 3. Learning Mode (AI Tutor)

#### 3.1 Guided Learning Flow
1. User pastes Japanese text
2. Tokenize immediately with `lindera` (fast, local operation)
3. Display parsed words/segments to user
4. For each segment (on-demand when user clicks):
   - Show loading spinner
   - LLM receives:
     - Current word/segment
     - Surrounding context from original text
     - User's skill level profile
     - User's learning history (from `user_report.md`)
   - LLM generates:
     - Explanation in English
     - Relevant grammar notes (if applicable)
     - Example sentences with translations
     - JLPT difficulty level
   - Cache LLM response in database
5. User interactions:
   - Read LLM explanation
   - Ask follow-up questions (triggers new LLM call)
   - Click "Add to Flashcards" → creates vocabulary or grammar card
   - Click "Next" to move to next segment (triggers LLM for that word)

#### 3.2 LLM Context Structure
```rust
struct LLMContext {
    user_profile: String,           // User's stated skill level
    learning_history: Vec<String>,   // Past mistakes, strengths
    user_report: String,            // Content from user_report.md
    current_progress: UserProgress, // Real-time statistics
}
```

#### 3.3 AI Agent Memory System
- **File**: `user_report.md` (stored in app data directory)
- **Content**: AI-generated assessment of user's current level
  - Vocabulary strengths/weaknesses
  - Grammar concepts mastered/struggling
  - Recommended focus areas
  - Updated after each learning session

#### 3.4 Interactive Learning UI
- **Layout:**
  - Top: Japanese text segment with furigana tooltips
  - Middle: LLM explanation text box (scrollable)
  - Bottom: User input for questions
  - Buttons: "Next Word/Line", "Add to Flashcards", "Ask Question"

#### 3.5 Adaptive Teaching
- LLM adjusts explanation complexity based on user profile
- References user's past mistakes in explanations
- Suggests related grammar points or vocabulary

---

### 4. Deck Management

```rust
struct Deck {
    id: DeckId,
    name: String,
    description: String,
    cards: Vec<CardId>,
    created_at: DateTime<Utc>,
    
    // Statistics
    total_cards: usize,
    new_cards: usize,
    due_cards: usize,
}
```

**Functionality:**
- Create new deck manually
- Auto-create deck from imported text (named by first line or user input)
- Move cards between decks
- Delete decks
- Rename decks

---

### 5. Settings Panel

**Options:**
- **Font Size**: Slider (12px - 32px)
- **Theme**: Light/Dark mode toggle
- **User Profile**: Text area for skill level description
  - E.g., "Beginner, know hiragana and katakana, learning N5 vocabulary"
- **LLM Configuration**:
  - API key input
  - Model selection (GPT-4, Claude, etc.)
  - Temperature/creativity slider
- **SRS Settings**:
  - Daily review limit
  - New cards per day

---

## Data Models

### Database Schema (using native_db)

#### Tables/Collections:

1. **users**
```rust
struct User {
    id: UserId,
    profile_description: String,
    created_at: DateTime<Utc>,
}
```

2. **decks**
```rust
struct Deck {
    id: DeckId,
    user_id: UserId,
    name: String,
    description: String,
    created_at: DateTime<Utc>,
}
```

3. **cards**
```rust
struct Card {
    id: CardId,
    deck_id: DeckId,
    card_type: CardType, // Vocabulary or Grammar
    
    // For vocabulary cards
    kanji: Option<String>,
    hiragana: Option<String>,
    romaji: Option<String>,
    
    // For grammar cards
    grammar_pattern: Option<String>,
    
    // Common fields
    meaning: String,
    wrong_answers: Vec<String>,
    example_sentences: String, // JSON serialized Vec<ExampleSentence>
    furigana_data: String, // JSON serialized
    jlpt_level: String, // N5, N4, N3, N2, N1, Unknown
    
    source_text: String,
    created_at: DateTime<Utc>,
}

enum CardType {
    Vocabulary,
    Grammar,
}
```

4. **srs_progress**
```rust
struct SRSProgress {
    card_id: CardId,
    ease_factor: f32,
    interval: u32,
    repetitions: u32,
    next_review: DateTime<Utc>,
    times_correct: u32,
    times_incorrect: u32,
    last_reviewed: DateTime<Utc>,
}
```

5. **learning_texts**
```rust
struct LearningText {
    id: TextId,
    user_id: UserId,
    content: String,
    title: String, // First 50 chars or user-defined
    processed_words: Vec<ProcessedWord>,
    created_at: DateTime<Utc>,
}

struct ProcessedWord {
    word: String,
    explanation: String,
    added_to_deck: bool,
}
```

---

## LLM Integration Details

### Using `rig` Crate with Gemini

#### 1. Setup
```rust
use rig::{completion::Prompt, providers::gemini};

// Initialize Gemini client
let client = gemini::Client::from_env(); // Uses GEMINI_API_KEY
let model = client.agent("gemini-2.5-pro").build();

// For caching and rate limiting
struct LLMService {
    model: Agent,
    cache: LRUCache<String, CachedResponse>,
}
```

#### 2. Generate Flashcard Data
```rust
async fn generate_flashcard_data(
    word: &str, 
    reading: &str, 
    sentence_context: &str,
    context: &LLMContext
) -> CardData {
    let prompt = format!(
        "For the Japanese word '{}' (reading: {}), appearing in context: '{}'
        
        Provide a JSON response with:
        1. English meaning (string)
        2. Three semantically similar but incorrect meanings (array of strings)
        3. JLPT level (N5, N4, N3, N2, N1, or Unknown)
        4. Two example sentences in Japanese with English translations:
           - sentence_jp: Japanese sentence
           - sentence_en: English translation
        5. Indicate if this is primarily a grammar pattern (boolean)
        6. If grammar, provide the pattern explanation
        
        User level: {}
        
        Format as JSON matching this schema:
        {{
          \"meaning\": string,
          \"wrong_answers\": [string, string, string],
          \"jlpt_level\": string,
          \"examples\": [
            {{\"japanese\": string, \"english\": string}},
            {{\"japanese\": string, \"english\": string}}
          ],
          \"is_grammar\": boolean,
          \"grammar_explanation\": string | null
        }}",
        word, reading, sentence_context, context.user_profile
    );
    
    let response = model.prompt(&prompt).await?;
    serde_json::from_str(&response)?
}
```

#### 3. Generate Learning Explanation
```rust
async fn generate_explanation(
    text_segment: &str,
    full_sentence: &str,
    user_context: &LLMContext
) -> String {
    let prompt = format!(
        "You are teaching Japanese to a learner.
        
        Word/Phrase: {}
        Full sentence context: {}
        
        Learner profile: {}
        Recent mistakes: {:?}
        
        Provide a clear explanation covering:
        1. Meaning in English
        2. Any relevant grammar points
        3. Usage notes or nuances
        4. How it fits in this specific sentence
        
        Adjust complexity to match the learner's level.
        Keep explanation concise but informative.",
        text_segment,
        full_sentence,
        user_context.user_profile,
        user_context.learning_history
    );
    
    model.prompt(&prompt).await?
}
```

#### 4. Interactive Q&A
```rust
async fn answer_question(
    question: &str,
    context_text: &str,
    user_context: &LLMContext
) -> String {
    let prompt = format!(
        "Student question about Japanese text:
        
        Text: {}
        Question: {}
        Student level: {}
        
        Provide a clear answer.",
        context_text, question, user_context.user_profile
    );
    
    model.prompt(&prompt).await?
}
```

---

## Japanese Text Processing Pipeline

### Using `lindera` for Tokenization
```rust
use lindera::tokenizer::Tokenizer;

fn tokenize_japanese(text: &str) -> Vec<Token> {
    let tokenizer = Tokenizer::new().unwrap();
    let tokens = tokenizer.tokenize(text).unwrap();
    
    tokens.into_iter()
        .map(|token| Token {
            surface: token.text.to_string(),
            reading: token.detail[7].to_string(), // Reading field
            base_form: token.detail[6].to_string(),
        })
        .collect()
}
```

### Using `furigana` for Furigana Mapping
```rust
use furigana::furigana;

fn generate_furigana(word: &str, reading: &str) -> Vec<FuriganaSpan> {
    furigana(word, reading)
        .map(|spans| spans.into_iter()
            .map(|span| FuriganaSpan {
                text: span.text,
                reading: span.ruby,
            })
            .collect())
        .unwrap_or_else(|_| vec![FuriganaSpan {
            text: word.to_string(),
            reading: Some(reading.to_string()),
        }])
}
```

### Using `japanese` for Romaji Conversion
```rust
use japanese;

fn to_romaji(hiragana: &str) -> String {
    japanese::to_romaji(hiragana)
}
```

---

## UI Component Hierarchy

```
App
├── HomeScreen
│   ├── TextInput (Japanese text paste area)
│   ├── ModeSelector (Practice/Learning buttons)
│   ├── DeckList
│   │   └── DeckCard (name, stats, select button)
│   └── TextList
│       └── TextCard (title, date, open button)
│
├── PracticeScreen
│   ├── ProgressBar (cards remaining)
│   ├── FlashCard
│   │   ├── CardContent (Vocabulary or Grammar)
│   │   │   ├── VocabularyDisplay
│   │   │   │   ├── KanjiDisplay (with furigana tooltip)
│   │   │   │   └── RomajiToggle
│   │   │   └── GrammarDisplay
│   │   │       └── PatternDisplay (grammar pattern)
│   │   ├── ExampleSentences (collapsible)
│   │   ├── JLPTBadge (difficulty indicator)
│   │   └── MultipleChoice (4 buttons)
│   └── Controls (Next, Previous, Exit)
│
├── LearningScreen
│   ├── SegmentList (scrollable list of parsed words)
│   ├── SelectedSegment
│   │   ├── LoadingSpinner (while fetching LLM explanation)
│   │   ├── TextSegment (Japanese with furigana)
│   │   ├── ExplanationPanel (LLM-generated)
│   │   │   ├── MeaningSection
│   │   │   ├── GrammarNotes (if applicable)
│   │   │   └── ExampleSentences
│   │   ├── QuestionInput (user asks follow-up)
│   │   └── Controls (Next, Add to Flashcards, Add Grammar Card)
│
└── SettingsScreen
    ├── FontSizeSlider
    ├── ThemeToggle
    ├── UserProfileInput
    ├── LLMConfig
    │   ├── APIKeyInput (for Gemini)
    │   └── ModelSelector
    ├── SRSSettings
    │   ├── DailyReviewLimit
    │   └── NewCardsPerDay
    └── FilterSettings
        └── JLPTLevelFilter (N5-N1 checkboxes)
```

---

## MVP Development Phases

### Phase 1: Basic UI & Text Processing ✓
- [ ] Set up iced application skeleton
- [ ] Create home screen with text input
- [ ] Integrate `lindera` for tokenization
- [ ] Integrate `furigana` and `japanese` crates
- [ ] Display parsed words in console (testing)

### Phase 2: LLM Integration ✓
- [ ] Set up `rig` with OpenAI/Claude
- [ ] Create prompt templates
- [ ] Test flashcard data generation
- [ ] Test explanation generation
- [ ] Handle async LLM calls in iced

### Phase 3: Practice Mode ✓
- [ ] Implement flashcard UI (vocabulary cards)
- [ ] Multiple choice quiz logic
- [ ] Furigana tooltip display
- [ ] Romaji toggle functionality
- [ ] Basic SRS algorithm (SM-2)
- [ ] Grammar card support
- [ ] Example sentences display (collapsible)
- [ ] JLPT level badge display

### Phase 4: Database & Persistence ✓
- [ ] Set up `native_db`
- [ ] Create database schema
- [ ] Implement CRUD operations for cards
- [ ] Save/load user progress
- [ ] Deck management

### Phase 5: Learning Mode ✓
- [ ] On-demand word processing UI
- [ ] Loading states for LLM calls
- [ ] LLM context building
- [ ] Cache LLM responses in database
- [ ] Interactive Q&A system
- [ ] Add to vocabulary flashcards
- [ ] Add to grammar flashcards
- [ ] Grammar pattern detection and explanation
- [ ] Update user_report.md

### Phase 6: Polish & Settings ✓
- [ ] Settings panel implementation
- [ ] Theme switching
- [ ] Font size adjustment
- [ ] User profile management
- [ ] JLPT level filtering
- [ ] Statistics dashboard (by difficulty level)
- [ ] Gemini API key configuration

---

## Key Design Decisions & Trade-offs

### LLM vs Dictionary
**Decision**: Use LLM for definitions and explanations

**Advantages:**
- Context-aware definitions (considers surrounding text)
- Adaptive difficulty (matches user level)
- Generates semantically similar wrong answers automatically
- Can explain nuances, grammar, and usage
- No need to maintain large dictionary databases
- Can provide examples and cultural context

**Disadvantages:**
- Requires API calls (latency, cost)
- Offline functionality limited
- Potential for incorrect information (mitigated by using reliable models)
- Rate limits may affect bulk processing

**Mitigation Strategy:**
- Cache LLM responses in database
- Allow offline mode with cached data
- Batch process new texts to minimize API calls
- Provide manual correction option for wrong LLM outputs

### Furigana Display
**Decision**: Tooltip-based furigana (hover to show)

**Reasoning:**
- Cleaner UI, less cluttered
- Encourages users to try reading without furigana first
- Better for intermediate learners
- Can always show on-demand

---

## Dependencies Summary

```toml
[dependencies]
# UI
iced = { version = "0.13", features = ["tokio"] }

# LLM
rig-core = "0.5"
tokio = { version = "1", features = ["full"] }

# Japanese processing
lindera = "1.4"
furigana = "0.1"
japanese = "0.1"

# Database
native_db = "0.8"
native_model = "0.4"

# Caching
lru = "0.12"

# Serialization
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"

# Date/Time
chrono = { version = "0.4", features = ["serde"] }

# Error handling
anyhow = "1.0"
thiserror = "1.0"

# UUID for IDs
uuid = { version = "1.0", features = ["v4", "serde"] }
```

---

## Design Decisions - Final

### 1. LLM Provider ✓
**Decision**: Google Gemini 2.5 Pro (via `rig` crate)
- Excellent multilingual support
- Good at Japanese language understanding
- Cost-effective for MVP

### 2. Text Processing Strategy ✓
**Decision**: On-demand processing during learning mode
- Parse text with `lindera` immediately (fast, local)
- Generate LLM content (definitions, examples) only when user reaches that word
- Show loading spinner while LLM processes current word
- Cache results in database to avoid re-processing

**Benefits:**
- Faster initial load time
- Lower API costs (only process what user actually studies)
- Better UX (no waiting for long texts to fully process)

### 3. Offline Mode ✓
**Decision**: Not a priority for MVP
- Require internet connection
- Focus on core learning features first
- Can add offline support in future versions with cached data

### 4. Card Difficulty Levels ✓
**Decision**: Yes, implement JLPT difficulty tagging
- LLM assigns JLPT level (N5-N1) to each card
- Users can filter cards by difficulty
- Track progress across difficulty levels
- Useful for organizing study sessions

### 5. Audio Pronunciation ✗
**Decision**: Not implementing for MVP
- Adds complexity with TTS integration
- Can be added as future enhancement
- Text-based learning is sufficient for MVP

### 6. Example Sentences ✓
**Decision**: Yes, include example sentences
- Each card shows 1-2 example sentences from LLM
- Helps understand word usage in context
- Sentences also have furigana and English translation
- Better retention through contextual learning

### 7. Grammar Integration ✓
**Decision**: Dual approach
- **Grammar Flashcards**: Separate card type for grammar patterns
  - Question: Grammar pattern (e.g., "〜てもいい")
  - Answer: Multiple choice for meaning/usage
  - Examples showing the pattern
- **Learning Mode**: LLM explains relevant grammar when teaching vocabulary
  - E.g., when teaching "食べている", explain 〜ている progressive form
  - Creates holistic learning experience

### 8. Text Input Method ✓
**Decision**: Paste-only for MVP
- Simple text area on home screen
- Can add file import (txt, epub) in future versions
- Focus on core functionality first
