# Contributing to nihon.rs 🎉

First off, **thank you** for considering contributing to nihon.rs! Whether you're fixing a typo or implementing a major feature, every contribution helps make Japanese learning more accessible.

## 🌟 Welcome!

This project is perfect for:
- 🌱 **First-time contributors** - We have beginner-friendly issues!
- 🦀 **Rust learners** - Great project to practice Rust skills
- 🇯🇵 **Japanese enthusiasts** - Combine coding with language learning
- 🎨 **UI/UX designers** - Help make the app beautiful and intuitive
- 📚 **Technical writers** - Improve documentation and tutorials

## 📋 Code of Conduct

Be kind, respectful, and constructive. We're all here to learn and build something cool together!

## 🚀 Getting Started

### 1. Set Up Your Development Environment

```bash
# Fork the repository on GitHub, then clone your fork
git clone https://github.com/YOUR_USERNAME/nihon.rs.git
cd nihon.rs

# Add upstream remote to stay synced
git remote add upstream https://github.com/karmakarmeghdip/nihon.rs.git

# Install dependencies and run
cargo run
```

### 2. Find Something to Work On

- **Browse issues**: Check [open issues](https://github.com/karmakarmeghdip/nihon.rs/issues)
- **Good first issues**: Look for the [`good first issue`](https://github.com/karmakarmeghdip/nihon.rs/labels/good%20first%20issue) label
- **Ask questions**: Comment on an issue to clarify requirements
- **Propose new features**: Open a discussion first to get feedback

### 3. Create Your Feature Branch

```bash
# Update your fork with latest changes
git checkout main
git pull upstream main

# Create a new branch for your work
git checkout -b feature/your-feature-name
```

Use descriptive branch names:
- `feature/add-furigana-tooltips`
- `fix/button-alignment-home-screen`
- `docs/improve-setup-instructions`

## 💻 Development Workflow

### Making Changes

1. **Write code** following our style guide (see below)
2. **Test locally**: `cargo run` and manually test your changes
3. **Check for errors**: `cargo check`
4. **Format code**: `cargo fmt`
5. **Run linter**: `cargo clippy`

### Commit Messages

Write clear, descriptive commit messages:

✅ **Good:**
```
Add furigana tooltip component to learning view

- Created FuriganaTooltip widget
- Integrated with text display
- Added hover interaction
```

❌ **Avoid:**
```
fixed stuff
updates
WIP
```

### Code Style Guidelines

#### General Rust Conventions
- Follow [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- Run `cargo fmt` before committing
- Address `cargo clippy` warnings
- Use meaningful variable names

#### iced-Specific Patterns
This project uses the **Elm Architecture** pattern. All UI components follow:

```rust
struct ViewName {
    // State fields
}

enum Message {
    // Actions that can occur
}

impl ViewName {
    fn update(&mut self, message: Message) -> Task<Message> {
        // Handle state changes
    }
    
    fn view(&self) -> Element<'_, Message> {
        // Pure function: state → UI
    }
}
```

**Key Rules:**
- Messages flow **up** (wrapped via `.map()`)
- State flows **down** (through view references)
- `update()` must return `Task<Message>`
- `view()` should be a pure function

#### Documentation
- Add doc comments (`///`) for public functions and types
- Explain *why*, not just *what*
- Include examples for complex functions

```rust
/// Generates furigana spans for a Japanese word.
///
/// # Arguments
/// * `word` - The kanji/kana word (e.g., "食べる")
/// * `reading` - The full hiragana reading (e.g., "たべる")
///
/// # Returns
/// Vector of furigana spans with text and optional ruby text
///
/// # Example
/// ```
/// let spans = generate_furigana("食べる", "たべる");
/// ```
pub fn generate_furigana(word: &str, reading: &str) -> Vec<FuriganaSpan> {
    // Implementation
}
```

## 🎯 Types of Contributions

### 🐛 Bug Fixes
1. Check if the bug is already reported
2. If not, open an issue first with reproduction steps
3. Reference the issue in your PR: "Fixes #123"

### ✨ New Features
1. **Discuss first**: Open an issue or discussion to propose the feature
2. Wait for maintainer feedback before implementing
3. Follow the architecture patterns in `SPEC.md`
4. Update documentation as needed

### 📝 Documentation
- Improve README or CONTRIBUTING guides
- Add inline code comments
- Write tutorials or examples
- Fix typos or clarify confusing sections

### 🎨 UI/UX Improvements
- Enhance layouts and spacing
- Improve color schemes
- Add icons or visual feedback
- Make the app more accessible

### 🧪 Testing
- Write unit tests for functions
- Add integration tests
- Test edge cases
- Improve test coverage

## 📤 Submitting a Pull Request

### Before Submitting

- [ ] Code compiles: `cargo check`
- [ ] Formatted: `cargo fmt`
- [ ] No clippy warnings: `cargo clippy`
- [ ] Tested manually: `cargo run`
- [ ] Documentation updated (if needed)
- [ ] Commits are clean and descriptive

### Creating the PR

1. **Push your branch**:
   ```bash
   git push origin feature/your-feature-name
   ```

2. **Open a Pull Request** on GitHub

3. **Fill out the PR template** with:
   - What changed and why
   - How to test it
   - Screenshots (for UI changes)
   - Related issues (e.g., "Closes #42")

4. **Respond to feedback** from reviewers
   - Make requested changes in new commits
   - Push updates to your branch
   - Be patient and respectful

### PR Review Process

- Maintainers will review within a few days
- Automated checks must pass (formatting, clippy)
- At least one maintainer approval required
- Merge conflicts should be resolved by rebasing

## 🏗️ Understanding the Codebase

### Key Files

| File | Purpose |
|------|---------|
| `src/app.rs` | Root app with routing and state |
| `src/views/home.rs` | Home screen UI |
| `src/views/practice.rs` | Flashcard mode |
| `src/views/learning.rs` | AI tutor mode |
| `src/views/settings.rs` | Settings panel |
| `src/styles.rs` | UI theme and styling |
| `SPEC.md` | Detailed architecture specification |

### Navigation Flow

```
App (app.rs)
├── AppMode::Home → HomeView
├── AppMode::Practice → PracticeView
├── AppMode::Learning → LearningView
└── AppMode::Settings → SettingsView
```

Messages navigate via: `Message::NavigateTo(AppMode::XYZ)`

### Adding a New View

1. Create `src/views/newview.rs`:
   ```rust
   pub struct NewView { /* state */ }
   pub enum Message { /* actions */ }
   impl NewView {
       pub fn update(&mut self, msg: Message) -> Task<Message> { /* ... */ }
       pub fn view(&self) -> Element<'_, Message> { /* ... */ }
   }
   ```

2. Add to `src/views/mod.rs`:
   ```rust
   pub mod newview;
   ```

3. Wire up in `src/app.rs`:
   - Add field to `App` struct
   - Add variant to `AppMode` enum
   - Add variant to `Message` enum
   - Handle in `update()` and `view()` with `.map()`

## 🎓 Learning Resources

### Rust
- [The Rust Book](https://doc.rust-lang.org/book/) - Comprehensive guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises

### iced Framework
- [iced Book](https://book.iced.rs/) - Official guide
- [iced Examples](https://github.com/iced-rs/iced/tree/master/examples) - Sample code
- [The Elm Architecture](https://guide.elm-lang.org/architecture/) - Pattern explanation

### Project-Specific
- Run `cargo doc --open` to browse inline documentation
- Read `.github/copilot-instructions.md` for architecture patterns
- Check `SPEC.md` for full feature specifications

## ❓ Getting Help

Stuck? Here's how to get unstuck:

1. **Read the docs**: Check README, SPEC.md, and inline comments
2. **Search issues**: Someone might have had the same question
3. **Ask in discussions**: Open a [discussion thread](https://github.com/karmakarmeghdip/nihon.rs/discussions)
4. **Comment on issues**: Ask for clarification on specific tasks
5. **Join the conversation**: We're friendly and want to help!

## 🏷️ Issue Labels

- `good first issue` - Perfect for newcomers
- `help wanted` - We'd love contributions here
- `bug` - Something's broken
- `enhancement` - New feature or improvement
- `documentation` - Docs need work
- `question` - Needs discussion

## 🎉 Recognition

Contributors will be:
- Listed in README.md
- Credited in release notes
- Thanked publicly (if you want!)

Every contribution matters, no matter how small. Thank you for making nihon.rs better! 💜

---

**Ready to contribute?** Pick an issue and let's build something amazing together! 🚀
