# Quick Reference: New Code Structure

## 📁 Where to Find Things

| What you need | Where it is | Import path |
|--------------|-------------|-------------|
| **Flashcard types** | `src/models/flashcard.rs` | `crate::models::{CardType, VocabularyCard, GrammarCard}` |
| **Word/language types** | `src/models/word.rs` | `crate::models::{WordSegment, WordExplanation, JLPTLevel}` |
| **Deck metadata** | `src/models/deck.rs` | `crate::models::{DeckInfo, TextInfo}` |
| **JLPT badge** | `src/components/jlpt_badge.rs` | `crate::components::jlpt_badge` |
| **Quiz state** | `src/components/quiz_state.rs` | `crate::components::QuizState` |
| **Button styles** | `src/ui/button.rs` | `crate::ui::button_style` |
| **Input styles** | `src/ui/input.rs` | `crate::ui::text_input_style` |
| **Container styles** | `src/ui/container.rs` | `crate::ui::section_style` |
| **Constants** | `src/constants.rs` | `crate::constants::ui::MAX_CONTENT_WIDTH` |
| **Error types** | `src/error.rs` | `crate::error::{AppError, AppResult}` |
| **LLM service** | `src/services/llm.rs` | `crate::services::LLMService` |
| **Database** | `src/services/database.rs` | `crate::services::DatabaseService` |
| **Tokenizer** | `src/services/tokenizer.rs` | `crate::services::TokenizerService` |

## 🎨 Common Import Patterns

### For a new view:
```rust
use crate::constants;
use crate::models::{CardType, DeckInfo};
use crate::components::{jlpt_badge, QuizState};
use crate::ui::{button_style, section_style, text_input_style};
use iced::widget::{button, column, container, row, text};
use iced::{Alignment, Element, Length, Task};
```

### For a new component:
```rust
use crate::models::JLPTLevel;
use crate::ui::section_style;
use iced::widget::{container, text};
use iced::Element;
```

### For a new service:
```rust
use crate::models::{WordExplanation, CardType};
use crate::error::{AppError, AppResult};
```

## 🏗️ Adding New Features

### Adding a New Flashcard Type
1. Add variant to `CardType` enum in `src/models/flashcard.rs`
2. Create struct in same file
3. Update helper methods on `CardType`
4. Add display logic in `src/views/practice.rs`

### Adding a New UI Component
1. Create file in `src/components/`
2. Export from `src/components/mod.rs`
3. Use in views via `crate::components::{your_component}`

### Adding a New Style
1. Create or update file in `src/ui/`
2. Export from `src/ui/mod.rs`
3. Use in views via `crate::ui::{your_style}`

### Implementing a Service
1. Navigate to `src/services/{llm,database,tokenizer}.rs`
2. Implement the stubbed methods
3. Add dependencies to `Cargo.toml`
4. Update error types if needed

## 🔍 Common Tasks

### Change button style globally
→ Edit `src/ui/button.rs`

### Add a new JLPT level color
→ Edit `src/models/word.rs` → `JLPTLevel::color()`

### Change max content width
→ Edit `src/constants.rs` → `ui::MAX_CONTENT_WIDTH`

### Add new error type
→ Edit `src/error.rs` → Add variant to `AppError`

### Update theme
→ Edit `src/ui/theme.rs` → `get_theme()`

## 📝 Style Guide

### Naming Conventions
- **Modules**: lowercase_with_underscores
- **Types**: PascalCase
- **Functions**: snake_case
- **Constants**: SCREAMING_SNAKE_CASE

### File Organization
- One main type per file (with related helpers)
- Group related functionality in modules
- Export only what's needed from modules

### Documentation
- Use `//!` for module docs
- Use `///` for public API docs
- Include examples for non-obvious functions

## 🚀 Building & Running

```bash
# Quick check
cargo check

# Run in debug mode
cargo run

# Build release
cargo build --release

# Check for warnings
cargo clippy

# Format code
cargo fmt
```

## 🐛 Troubleshooting

**Import not found?**
→ Check if it's exported from the module's `mod.rs`

**Old style references?**
→ Replace `styles::` with `crate::ui::`

**Type not found?**
→ Check if it's in `models/` now

**Unused warnings?**
→ Normal for MVP phase, ignore or prefix with `_`

## 📚 Documentation Files

- `CODE_ORGANIZATION.md` - Detailed structure guide
- `REORGANIZATION_SUMMARY.md` - Change summary
- `.github/copilot-instructions.md` - AI agent guidelines
- This file - Quick reference

---

Last updated: 2025-10-29
