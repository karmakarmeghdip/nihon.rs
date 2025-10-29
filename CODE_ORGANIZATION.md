# Code Organization Guide

This document describes the organized structure of the nihon.rs codebase after the refactoring to improve maintainability.

## Directory Structure

```
src/
├── app.rs              # Root application and routing
├── main.rs             # Entry point
├── constants.rs        # Application-wide constants
├── types.rs            # Legacy type re-exports (for backward compatibility)
├── error.rs            # Error types and Result aliases
│
├── models/             # Domain models and data structures
│   ├── mod.rs          # Module exports
│   ├── flashcard.rs    # CardType, VocabularyCard, GrammarCard
│   ├── word.rs         # WordSegment, WordExplanation, ExampleSentence, JLPTLevel
│   └── deck.rs         # DeckInfo, TextInfo
│
├── components/         # Reusable UI components
│   ├── mod.rs          # Component exports
│   ├── jlpt_badge.rs   # JLPT level badge widget
│   ├── example_display.rs  # Example sentence display
│   └── quiz_state.rs   # Quiz state management
│
├── ui/                 # UI styling and theming
│   ├── mod.rs          # Style exports
│   ├── button.rs       # Button styles
│   ├── input.rs        # Text input styles
│   ├── container.rs    # Container/card styles
│   ├── slider.rs       # Slider styles
│   ├── theme.rs        # Theme management
│   └── utils.rs        # UI utility functions (color mixing)
│
├── services/           # External integrations (stubs for future implementation)
│   ├── mod.rs          # Service exports
│   ├── llm.rs          # LLM service (Gemini API via rig)
│   ├── database.rs     # Database service (native_db)
│   └── tokenizer.rs    # Tokenizer service (lindera)
│
└── views/              # Application views (Elm architecture)
    ├── mod.rs          # View module exports
    ├── home.rs         # Home/landing page
    ├── practice.rs     # Flashcard practice mode
    ├── learning.rs     # Interactive learning mode
    └── settings.rs     # Settings and configuration
```

## Module Responsibilities

### `models/`
**Purpose**: Contains all domain models and data structures used throughout the application.

- **flashcard.rs**: Flashcard types for SRS practice
  - `CardType` enum (Vocabulary/Grammar)
  - `VocabularyCard` struct
  - `GrammarCard` struct
  - Helper methods for card operations

- **word.rs**: Japanese language-related models
  - `WordSegment` - Tokenized word with reading and explanation
  - `WordExplanation` - LLM-generated explanation
  - `ExampleSentence` - Japanese/English example pair
  - `JLPTLevel` - JLPT difficulty levels (N5-N1)
  - `FuriganaSpan` - Furigana reading annotations

- **deck.rs**: Deck and text management
  - `DeckInfo` - Flashcard deck metadata
  - `TextInfo` - Saved text metadata

### `components/`
**Purpose**: Reusable UI widgets that can be used across multiple views.

- **jlpt_badge.rs**: Renders a colored badge showing JLPT level
- **example_display.rs**: Displays a list of example sentences in styled containers
- **quiz_state.rs**: Manages quiz state (Question/AnswerCorrect/AnswerIncorrect)

### `ui/`
**Purpose**: All UI styling functions organized by widget type, following Catppuccin/shadcn design system.

- **button.rs**: Button styles with hover/active/disabled states
- **input.rs**: Text input styles with focus states
- **container.rs**: Container/card styles for sections
- **slider.rs**: Slider styles for settings
- **theme.rs**: Theme selection (currently Catppuccin Mocha)
- **utils.rs**: Utility functions like color mixing

### `services/`
**Purpose**: Service layer for external integrations. Currently contains stubs with documented interfaces for future implementation.

- **llm.rs**: LLM service for AI-powered explanations
  - `LLMService::explain_word()` - Get word explanations
  - `LLMService::answer_question()` - Answer user questions
  - Planned: Gemini API integration via `rig` crate

- **database.rs**: Database service for persistent storage
  - `DatabaseService::save_card()` - Save flashcards
  - `DatabaseService::load_decks()` - Load deck list
  - `DatabaseService::cache_llm_response()` - Cache LLM responses
  - Planned: `native_db` integration

- **tokenizer.rs**: Tokenizer service for Japanese text processing
  - `TokenizerService::tokenize()` - Tokenize Japanese text
  - `TokenizerService::get_furigana()` - Extract furigana readings
  - Planned: `lindera` integration

### `views/`
**Purpose**: Application views following the Elm architecture pattern (State + Messages + Update + View).

Each view is self-contained with:
- State struct
- Message enum for actions
- `update()` method for state changes
- `view()` method for rendering UI

- **home.rs**: Landing page with text input and deck/text lists
- **practice.rs**: Flashcard practice with SRS algorithm
- **learning.rs**: Interactive text reading with word explanations
- **settings.rs**: Application configuration

### `error.rs`
**Purpose**: Centralized error handling.

- `AppError` enum with variants for different error types
- `AppResult<T>` type alias for convenience
- Automatic conversions from service errors

### `constants.rs`
**Purpose**: Application-wide constants.

- `ui::MAX_CONTENT_WIDTH` - Maximum content width
- `ui::DEFAULT_FONT_SIZE` - Default font size
- `srs::DEFAULT_DAILY_REVIEW_LIMIT` - SRS configuration

## Import Patterns

### For Views
```rust
// Models
use crate::models::{CardType, VocabularyCard, JLPTLevel};

// Components
use crate::components::{jlpt_badge, QuizState};

// UI Styles
use crate::ui::{button_style, section_style};

// Constants
use crate::constants;
```

### For Services
```rust
use crate::models::{WordExplanation, DeckInfo};
use crate::error::{AppResult, AppError};
```

### For Components
```rust
use crate::models::JLPTLevel;
use crate::ui::section_style;
```

## Benefits of This Structure

1. **Separation of Concerns**: Models, views, services, and UI are clearly separated
2. **Code Reusability**: Components can be reused across views
3. **Easy to Test**: Each module has a clear responsibility and can be tested independently
4. **Scalability**: Easy to add new features (e.g., new card types, new services)
5. **Type Safety**: Centralized models prevent duplication and inconsistencies
6. **Maintainability**: Related code is grouped together, making changes easier
7. **Documentation**: Clear module structure makes onboarding easier

## Future Integration Points

When adding the planned dependencies:

1. **LLM Integration** (`rig` + Gemini):
   - Implement `services/llm.rs` methods
   - Add API key management in settings
   - Add caching layer in database service

2. **Database** (`native_db`):
   - Implement `services/database.rs` methods
   - Add migrations if needed
   - Integrate with SRS algorithm

3. **Tokenizer** (`lindera`):
   - Implement `services/tokenizer.rs` methods
   - Add dictionary loading
   - Integrate with learning view

4. **Additional Components**:
   - Add to `components/` directory
   - Export from `components/mod.rs`
   - Import in views as needed

## Migration Notes

- `types.rs` still exists for backward compatibility, re-exporting from `models`
- Old `styles.rs` and `theme.rs` are replaced by `ui/` directory
- All views have been updated to use the new module structure
