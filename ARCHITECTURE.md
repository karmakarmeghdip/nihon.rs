# Architecture Diagram

## System Overview

```
┌─────────────────────────────────────────────────────────────────────┐
│                         nihon.rs Application                        │
│                         (iced GUI Framework)                         │
└─────────────────────────────────────────────────────────────────────┘
                                   │
                                   ▼
┌─────────────────────────────────────────────────────────────────────┐
│                            app.rs (Root)                            │
│  ┌───────────────────────────────────────────────────────────────┐ │
│  │  App Struct                                                     │ │
│  │  • Navigation routing between views                            │ │
│  │  • Message delegation                                          │ │
│  │  • Theme management                                            │ │
│  └───────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────────┘
                                   │
        ┌──────────────┬───────────┴──────────┬──────────────┐
        ▼              ▼                      ▼              ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│  Home View   │ │Practice View │ │Learning View │ │Settings View │
│              │ │              │ │              │ │              │
│ • Text input │ │ • Flashcards │ │ • Word click │ │ • Config     │
│ • Deck list  │ │ • Quiz UI    │ │ • LLM Q&A    │ │ • API keys   │
│ • Text list  │ │ • SRS logic  │ │ • Tokenized  │ │ • Appearance │
└──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘
        │              │                      │              │
        └──────────────┴───────────┬──────────┴──────────────┘
                                   ▼
           ┌───────────────────────────────────────────┐
           │         Shared Application Layers         │
           └───────────────────────────────────────────┘
                                   │
        ┌──────────────┬───────────┴──────────┬──────────────┐
        ▼              ▼                      ▼              ▼
┌──────────────┐ ┌──────────────┐ ┌──────────────┐ ┌──────────────┐
│   models/    │ │ components/  │ │     ui/      │ │  services/   │
│              │ │              │ │              │ │              │
│ • CardType   │ │ • jlpt_badge │ │ • button     │ │ • llm        │
│ • WordSeg    │ │ • quiz_state │ │ • input      │ │ • database   │
│ • JLPTLevel  │ │ • examples   │ │ • container  │ │ • tokenizer  │
│ • DeckInfo   │ │              │ │ • slider     │ │              │
└──────────────┘ └──────────────┘ └──────────────┘ └──────────────┘
        │              │                      │              │
        └──────────────┴───────────┬──────────┴──────────────┘
                                   ▼
                    ┌──────────────────────────┐
                    │    constants.rs          │
                    │    error.rs              │
                    └──────────────────────────┘
```

## Data Flow

### Practice Mode Flow
```
User Action (Click Answer)
         │
         ▼
  Message::SelectAnswer
         │
         ▼
  PracticeView::update()
         │
         ├──→ Update QuizState (from components)
         │
         ├──→ Check answer against CardType (from models)
         │
         └──→ Render with UI styles (from ui/)
```

### Learning Mode Flow
```
User Action (Click Word)
         │
         ▼
  Message::SelectWord
         │
         ▼
  LearningView::update()
         │
         ├──→ Select WordSegment (from models)
         │
         ├──→ Call LLMService (from services)
         │
         ├──→ Cache response (DatabaseService)
         │
         └──→ Display with components (jlpt_badge, etc)
```

## Module Dependencies

```
┌─────────┐
│  main   │
└────┬────┘
     │
     ▼
┌─────────┐
│   app   │──────┐
└────┬────┘      │
     │           │
     ├───────────┼──────────────┐
     ▼           ▼              ▼
┌─────────┐ ┌─────────┐   ┌─────────┐
│  views  │ │   ui    │   │constants│
└────┬────┘ └─────────┘   └─────────┘
     │           ▲
     ├───────────┼──────────────┐
     ▼           │              ▼
┌─────────┐ ┌─────────┐   ┌─────────┐
│ models  │ │components   │ services│
└─────────┘ └─────────┘   └─────────┘
     ▲           │              │
     │           ▼              │
     │      ┌─────────┐         │
     └──────│  error  │◄────────┘
            └─────────┘
```

**Legend:**
- `→` : Direct dependency
- `◄` : Uses error types from

## Future Service Integration

```
┌─────────────────────────────────────────────────────────────┐
│                      Learning View                          │
└────────────┬──────────────────────────┬─────────────────────┘
             │                          │
    Message::SelectWord        Message::AskQuestion
             │                          │
             ▼                          ▼
    ┌────────────────┐         ┌────────────────┐
    │ TokenizerSvc   │         │   LLMService   │
    │ (lindera)      │         │   (rig/gemini) │
    └───────┬────────┘         └───────┬────────┘
            │                          │
            │  WordSegments            │  Explanation
            ▼                          ▼
    ┌─────────────────────────────────────────┐
    │           DatabaseService               │
    │           (native_db)                   │
    │                                         │
    │  • Cache LLM responses                  │
    │  • Store flashcards                     │
    │  • Save SRS progress                    │
    └─────────────────────────────────────────┘
```

## Component Reusability

```
┌──────────────────────────────────────────────────────────┐
│                    components/                           │
│                                                          │
│  ┌────────────┐  ┌────────────┐  ┌────────────┐       │
│  │jlpt_badge  │  │quiz_state  │  │example_    │       │
│  │            │  │            │  │  display   │       │
│  └────────────┘  └────────────┘  └────────────┘       │
└──────────────────────────────────────────────────────────┘
         │                │                │
         ├────────────────┼────────────────┤
         ▼                ▼                ▼
    ┌────────┐      ┌────────┐      ┌────────┐
    │Practice│      │Practice│      │Learning│
    │  View  │      │  View  │      │  View  │
    └────────┘      └────────┘      └────────┘

  (Reused across multiple views)
```

## Style Inheritance

```
┌──────────────────────────────────────────┐
│            ui/ (styles)                  │
│                                          │
│  ┌──────────────────────────────────┐   │
│  │  theme.rs                        │   │
│  │  (Catppuccin Mocha/Latte)        │   │
│  └─────────────┬────────────────────┘   │
│                │                         │
│                ├──→ button.rs            │
│                ├──→ input.rs             │
│                ├──→ container.rs         │
│                └──→ slider.rs            │
└──────────────────────────────────────────┘
                 │
                 ▼
        All views use themed styles
```

## Error Handling Flow

```
Service Layer Error
         │
         ▼
    AppError enum
         │
         ├──→ AppError::LLM(...)
         ├──→ AppError::Database(...)
         └──→ AppError::Tokenizer(...)
         │
         ▼
    View catches error
         │
         ▼
  Display to user
  (toast/alert)
```

---

This architecture provides:
- ✅ Clear separation of concerns
- ✅ Reusable components
- ✅ Centralized styling
- ✅ Service layer ready for integration
- ✅ Type-safe error handling
- ✅ Scalable structure
