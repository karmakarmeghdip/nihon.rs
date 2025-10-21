# 🇯🇵 nihon.rs

[![Rust](https://img.shields.io/badge/rust-2024-orange.svg)](https://www.rust-lang.org/)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Hacktoberfest](https://img.shields.io/badge/hacktoberfest-welcome-ff69b4.svg)](https://hacktoberfest.com/)
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](CONTRIBUTING.md)

> **⚠️ Early Development Notice**: This project is in active development and not yet functional. We're building it from the ground up and would love your help! Perfect for Hacktoberfest contributors.

A Japanese language learning application built with **Rust** and **iced**, combining AI-powered tutoring with spaced repetition flashcards. Learn vocabulary and grammar by pasting any Japanese text and let the AI guide your journey.

## 🎯 What We're Building

nihon.rs will help learners:
- 📝 **Paste any Japanese text** and get instant word breakdowns
- 🤖 **AI-powered explanations** tailored to your skill level (using Gemini)
- 🎴 **Smart flashcards** with spaced repetition (SRS algorithm)
- 📊 **Track progress** by JLPT difficulty levels (N5-N1)
- 💡 **Interactive learning** with context-aware grammar notes

### Current Status: MVP Phase 1

✅ **What's Done:**
- Basic UI skeleton with home, practice, learning, and settings views
- Navigation system between different modes
- Text input and theme switching
- Project architecture using iced's Elm pattern

🚧 **What We're Working On:**
- Japanese text tokenization (integrating `lindera`)
- LLM integration for explanations (using `rig` + Gemini)
- Flashcard generation and SRS system
- Database persistence with `native_db`
- Furigana display and romaji conversion

## 🚀 Quick Start

### Prerequisites

- **Rust** 1.80+ ([Install Rust](https://www.rust-lang.org/tools/install))
- **Git** ([Install Git](https://git-scm.com/downloads))

### Installation

```bash
# Clone the repository
git clone https://github.com/karmakarmeghdip/nihon.rs.git
cd nihon.rs

# Build and run
cargo run
```

That's it! The app should launch with the home screen.

## 🤝 Contributing

**We especially welcome first-time contributors!** Whether you're new to Rust or open source, we have tasks for all skill levels.

### 🌟 Good First Issues

Perfect for beginners:
- 🎨 **UI improvements**: Add tooltips, improve layouts, create icons
- 📝 **Documentation**: Improve code comments, write tutorials
- 🐛 **Bug fixes**: Fix typos, improve error messages
- ✅ **Testing**: Write unit tests for existing functions
- 🎭 **Styling**: Enhance theme colors, improve spacing

Check out our [issues labeled `good first issue`](https://github.com/karmakarmeghdip/nihon.rs/labels/good%20first%20issue)!

### 💪 More Challenging Tasks

For experienced contributors:
- 🔧 **Feature implementation**: Flashcard UI, LLM integration, database setup
- 🏗️ **Architecture**: Design state management patterns
- 🌐 **Japanese processing**: Integrate tokenization libraries
- ⚡ **Performance**: Optimize rendering and async operations

### How to Contribute

1. **Fork** the repository
2. **Create a branch**: `git checkout -b feature/your-feature-name`
3. **Make your changes** and commit: `git commit -m 'Add some feature'`
4. **Push** to your fork: `git push origin feature/your-feature-name`
5. **Open a Pull Request** and describe your changes

See our [Contributing Guide](CONTRIBUTING.md) for detailed instructions.

## 📚 Tech Stack

| Purpose | Technology | Why? |
|---------|-----------|------|
| **UI Framework** | [iced](https://github.com/iced-rs/iced) | Modern, type-safe GUI with Elm architecture |
| **LLM Integration** | [rig](https://github.com/0xPlaygrounds/rig) | Rust-native LLM framework |
| **Tokenization** | [lindera](https://github.com/lindera-morphology/lindera) | Japanese morphological analysis |
| **Furigana** | [furigana](https://crates.io/crates/furigana) | Map readings to kanji |
| **Database** | [native_db](https://github.com/vincent-herlemont/native_db) | Embedded local storage |

## 🏗️ Project Structure

```
nihon.rs/
├── src/
│   ├── app.rs          # Main app state & routing
│   ├── main.rs         # Entry point
│   ├── styles.rs       # UI theming
│   └── views/          # UI components
│       ├── home.rs     # Home screen
│       ├── practice.rs # Flashcard mode
│       ├── learning.rs # AI tutor mode
│       └── settings.rs # User preferences
├── Cargo.toml          # Dependencies (package name: nihonrs)
├── SPEC.md             # Detailed specification
└── README.md           # You are here!
```

## 🎓 Learning Resources

New to Rust or iced? These resources will help:

### Rust Basics
- [The Rust Book](https://doc.rust-lang.org/book/) - Official Rust guide
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/) - Learn by doing
- [Rustlings](https://github.com/rust-lang/rustlings) - Interactive exercises

### iced Framework
- [iced Book](https://book.iced.rs/) - Official framework guide
- [iced Examples](https://github.com/iced-rs/iced/tree/master/examples) - Sample applications
- Our [Copilot Instructions](.github/copilot-instructions.md) - Project-specific patterns

### Japanese Processing
- Check out `SPEC.md` for our planned text processing pipeline
- Documentation in code (run `cargo doc --open`)

## 💬 Development Tips

### Building the Project

```bash
# Fast compilation check (no binary)
cargo check

# Run in development mode
cargo run

# Run with optimizations (slower build, faster runtime)
cargo run --release

# Generate and view documentation
cargo doc --open
```

### Code Style

We follow standard Rust conventions:
- Use `cargo fmt` before committing
- Run `cargo clippy` to catch common mistakes
- Write descriptive commit messages

### Getting Help

- 💬 Open a [discussion](https://github.com/karmakarmeghdip/nihon.rs/discussions) for questions
- 🐛 Report bugs via [issues](https://github.com/karmakarmeghdip/nihon.rs/issues)
- 📧 Reach out to maintainers directly (see `Cargo.toml` for contacts)

## 🎯 Roadmap

### Phase 1: Foundation (Current)
- [x] Project setup and architecture
- [x] Basic UI skeleton
- [ ] Japanese text tokenization
- [ ] LLM integration setup

### Phase 2: Core Features
- [ ] Flashcard generation with AI
- [ ] Spaced repetition algorithm (SM-2)
- [ ] Database persistence
- [ ] Furigana display

### Phase 3: Learning Mode
- [ ] Interactive AI tutor
- [ ] Context-aware explanations
- [ ] Example sentence generation
- [ ] User progress tracking

### Phase 4: Polish
- [ ] Settings persistence
- [ ] Theme customization
- [ ] Export/import decks
- [ ] Statistics dashboard

## 📖 Documentation

- **[SPEC.md](SPEC.md)** - Comprehensive project specification with architecture details
- **[.github/copilot-instructions.md](.github/copilot-instructions.md)** - AI agent guidelines (useful for understanding project patterns)
- **Inline docs** - Run `cargo doc --open` to browse all code documentation

## 🙏 Acknowledgments

This project is built on amazing open-source work:
- [iced](https://github.com/iced-rs/iced) for the beautiful UI framework
- [lindera](https://github.com/lindera-morphology/lindera) for Japanese text processing
- [rig](https://github.com/0xPlaygrounds/rig) for LLM integration
- The Rust community for excellent tooling and support

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🌟 Contributors

Thanks to everyone who contributes to making Japanese learning accessible!

<!-- ALL-CONTRIBUTORS-LIST:START -->
<!-- Add your name here when you contribute! -->
<!-- ALL-CONTRIBUTORS-LIST:END -->

---

**Made with ❤️ for Japanese learners worldwide**

*Questions? Ideas? We'd love to hear from you! Open an issue or start a discussion.*
