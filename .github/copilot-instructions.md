# nihon.rs - AI Agent Instructions

## Project Overview
nihon.rs is a Japanese language learning application built with **Rust** and **iced-rs** (GUI framework). It combines AI-powered tutoring (via LLM) with spaced repetition flashcards for vocabulary and grammar practice.

## Architecture Pattern: The Elm Architecture

This project uses **iced-rs** which follows the Elm architecture pattern. All UI components follow this structure:

```rust
struct ViewName {
    // State fields
}

enum Message {
    // Actions that can occur
}

impl ViewName {
    fn update(&mut self, message: Message) -> Task<Message> {
        // Handle state changes, return async tasks
    }
    
    fn view(&self) -> Element<'_, Message> {
        // Pure function: state → UI
    }
}
```

**Critical**: Messages flow **up** through `Message` enums (wrapped via `.map()`), state flows **down** through view references.

### App Structure
- `src/app.rs`: Root App with routing between modes (Home/Practice/Learning/Settings)
- `src/views/*`: Each view is self-contained with own state, messages, update, and view logic
- Navigation: `Message::NavigateTo(AppMode)` changes `app.mode`, which changes which view renders
- **Navigation Pattern**: All non-home views should include a "Back to Home" button that sends `Message::BackToHome` (mapped to parent's `Message::NavigateTo(AppMode::Home)`)

## Key Code Patterns

### 1. Message Wrapping Pattern
When a view's message needs to propagate to parent:
```rust
// In app.rs
pub enum Message {
    Home(crate::views::home::Message),
    Practice(crate::views::practice::Message),
    // ...
}

fn view(&self) -> Element<'_, Message> {
    self.home_view.view().map(Message::Home)  // Wrap child messages
}
```

### 2. Task-Based Async
iced uses `Task<Message>` instead of traditional async/await in update logic:
```rust
fn update(&mut self, message: Message) -> Task<Message> {
    match message {
        Message::FetchData => Task::perform(
            async_fetch_operation(),
            |result| Message::DataFetched(result)
        ),
        // Synchronous changes return Task::none()
        Message::UpdateText(s) => {
            self.text = s;
            Task::none()
        }
    }
}
```

### 3. View Composition
Build UI with `column![]`, `row![]`, `container()`, `scrollable()`:
```rust
let content = column![
    text("Title").size(32),
    button("Click").on_press(Message::Clicked),
]
.spacing(10)
.padding(20);

container(content).center_x(Fill).into()
```

## Japanese Text Processing Stack (Per SPEC.md)

### Planned Dependencies (not yet implemented):
- **lindera**: Tokenization/morphological analysis (fast, local)
- **furigana**: Map readings to kanji positions
- **japanese**: Hiragana/Katakana/Romaji conversion
- **rig**: LLM framework for Gemini API integration

### Processing Pipeline Design:
1. **Learning Mode**: On-demand processing
   - Tokenize with lindera immediately (show word list)
   - Generate LLM explanations **only when user clicks a word** (show spinner)
   - Cache LLM responses in database
   
2. **Practice Mode**: Flashcard generation
   - Tokenize → extract words → call LLM for definitions + wrong answers + JLPT level
   - Generate `VocabularyCard` or `GrammarCard` structs
   - Store in `native_db` with SRS metadata

## Development Workflow

### Build & Run
```powershell
cargo check          # Fast compilation check
cargo run            # Run the app
cargo build --release  # Optimized build
```

### Adding New UI Features
1. Define new `Message` variants in the relevant view's `enum Message`
2. Handle in `update()` method
3. Add UI elements in `view()` with `.on_press(Message::Variant)`
4. If needs parent interaction, add to parent's `Message` enum and map it
5. **Input Validation**: Disable buttons when input is invalid (e.g., empty text fields) - use conditional `.on_press()` or omit it entirely

### Adding New Views
1. Create `src/views/newview.rs` with struct, Message enum, update(), view()
2. Add `pub mod newview;` to `src/views/mod.rs`
3. Add field to `App` struct in `app.rs`
4. Add new `AppMode` variant
5. Wire up in App's update/view match statements with `.map(Message::NewView)`

## Critical Implementation Notes

### Theme
Currently uses `iced::Theme::CatppuccinMocha` (dark theme). Can toggle via settings.

### Edition
Uses **Rust 2024 edition** (see Cargo.toml). Be aware of edition-specific features.

### iced Version
Uses git version of iced (`git="https://github.com/iced-rs/iced"`), not crates.io. This means:
- May have bleeding-edge API changes
- Check iced GitHub docs, not crates.io docs
- `Task<Message>` is current API (older docs may show `Command<Message>`)

### Documentation Access
- **Local docs**: Run `cargo doc --open` to browse all dependency documentation locally
- **AI assistance**: Use context7 MCP tools to fetch up-to-date library documentation (especially for iced, rig, lindera)

### Future LLM Integration
Per SPEC.md, will use **Gemini 2.5 Pro** via `rig` crate:
- API key stored in settings
- Async calls wrapped in `Task::perform()`
- Responses cached in `native_db` to minimize API costs
- User context (skill level, learning history) injected into prompts
- **Error Handling**: Auto-retry with exponential backoff (limit: 3-5 attempts), then show toast/alert notification on failure

### Data Models (Planned)
- **FlashCard**: `Vocabulary` vs `Grammar` card types
- **SRSProgress**: SM-2 algorithm data (ease_factor, interval, repetitions)
- **LearningText**: Cached processed texts with LLM explanations
- All stored in `native_db` (embedded database)

## Common Gotchas

1. **Length types**: Use `Length::Fill` for flexible sizing, not `Length::Fill` (the enum variant)
2. **Message mapping**: Forget to `.map()` child messages → compiler error "expected Message, found view::Message"
3. **Task returns**: Every `update()` must return `Task<Message>`, even if just `Task::none()`
4. **Element lifetime**: `view()` returns `Element<'_, Message>` - the lifetime is tied to self

## Testing
Currently no tests implemented. When adding:
- Unit tests: Use `#[cfg(test)]` attribute on test modules within source files
- Integration tests: `tests/` directory
- Run: `cargo test`

## Documentation Organization

**All documentation must be placed in the `docs/` folder:**
- Technical guides: `docs/GUIDE_NAME.md`
- API documentation: `docs/API_NAME.md`
- Integration guides: `docs/INTEGRATION_NAME.md`
- Architecture docs: `docs/ARCHITECTURE.md`
- Any other documentation files

**Root-level docs are reserved for:**
- `README.md` - Project overview
- `SPEC.md` - Project specification
- `CONTRIBUTING.md` - Contribution guidelines
- `Cargo.toml`, `Cargo.lock` - Package manifests

When creating new documentation (guides, tutorials, references), always place them in `docs/` and reference them from README.md if needed.

## Project Status
**MVP Phase 1**: Basic UI skeleton is complete
- Home view with text input and deck/text lists ✓
- Placeholder views for Practice/Learning/Settings ✓
- Navigation structure in place ✓

**Next Steps** (per SPEC.md):
- Integrate `lindera` for tokenization
- Set up LLM integration with `rig` + Gemini
- Implement flashcard UI and SRS algorithm
- Add `native_db` persistence layer ✓

## Design Decisions
- **Navigation**: All non-home views include "Back to Home" button (desktop app, no browser navigation)
- **Empty Input**: Buttons disabled when text input is empty (prevent invalid submissions)
- **LLM Errors**: Auto-retry with exponential backoff (3-5 attempts), then show toast/alert on failure
