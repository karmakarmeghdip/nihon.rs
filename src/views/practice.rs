//! Practice view - Flashcard-based spaced repetition practice
//!
//! This view implements:
//! - Flashcard display with vocabulary/grammar questions
//! - SRS (Spaced Repetition System) algorithm
//! - Multiple choice quiz interface
//! - Furigana display and romaji toggle
//! - Example sentences and JLPT level badges

use crate::constants::ui;
use crate::styles;
use iced::widget::{button, column, container, row, scrollable, text, Space};
use iced::{Alignment, Border, Color, Element, Fill, Length, Shadow, Task, Vector};

/// Represents a single furigana span
#[derive(Debug, Clone)]
pub struct FuriganaSpan {
    pub text: String,
    pub reading: Option<String>,
}

/// JLPT difficulty levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JLPTLevel {
    N5, // Beginner
    N4,
    N3, // Intermediate
    N2,
    N1, // Advanced
    Unknown,
}

impl JLPTLevel {
    pub fn as_str(&self) -> &str {
        match self {
            JLPTLevel::N5 => "N5",
            JLPTLevel::N4 => "N4",
            JLPTLevel::N3 => "N3",
            JLPTLevel::N2 => "N2",
            JLPTLevel::N1 => "N1",
            JLPTLevel::Unknown => "?",
        }
    }

    pub fn color(&self) -> Color {
        match self {
            JLPTLevel::N5 => Color::from_rgb(0.4, 0.8, 0.4), // Green
            JLPTLevel::N4 => Color::from_rgb(0.6, 0.8, 0.4), // Light green
            JLPTLevel::N3 => Color::from_rgb(0.9, 0.8, 0.3), // Yellow
            JLPTLevel::N2 => Color::from_rgb(0.9, 0.6, 0.3), // Orange
            JLPTLevel::N1 => Color::from_rgb(0.9, 0.3, 0.3), // Red
            JLPTLevel::Unknown => Color::from_rgb(0.6, 0.6, 0.6), // Gray
        }
    }
}

/// Example sentence with Japanese and English
#[derive(Debug, Clone)]
pub struct ExampleSentence {
    pub japanese: String,
    pub english: String,
}

/// Type of flashcard
#[derive(Debug, Clone)]
pub enum CardType {
    Vocabulary(VocabularyCard),
    Grammar(GrammarCard),
}

/// Vocabulary flashcard
#[derive(Debug, Clone)]
pub struct VocabularyCard {
    pub kanji: String,
    pub hiragana: String,
    pub romaji: String,
    pub meaning: String,
    pub wrong_answers: Vec<String>,
    pub example_sentences: Vec<ExampleSentence>,
    pub jlpt_level: JLPTLevel,
}

/// Grammar flashcard
#[derive(Debug, Clone)]
pub struct GrammarCard {
    pub pattern: String,
    pub pattern_reading: String,
    pub explanation: String,
    pub wrong_answers: Vec<String>,
    pub example_sentences: Vec<ExampleSentence>,
    pub jlpt_level: JLPTLevel,
}

/// State of the current quiz
#[derive(Debug, Clone, PartialEq)]
enum QuizState {
    Question,
    AnswerCorrect,
    AnswerIncorrect { selected: usize, correct: usize },
}

pub struct PracticeView {
    cards: Vec<CardType>,
    current_index: usize,
    show_romaji: bool,
    show_examples: bool,
    quiz_state: QuizState,
    score: usize,
    total_answered: usize,
}

impl Default for PracticeView {
    fn default() -> Self {
        // Create sample cards for demonstration
        let sample_cards = vec![
            CardType::Vocabulary(VocabularyCard {
                kanji: "食べる".to_string(),
                hiragana: "たべる".to_string(),
                romaji: "taberu".to_string(),
                meaning: "to eat".to_string(),
                wrong_answers: vec![
                    "to drink".to_string(),
                    "to cook".to_string(),
                    "to buy".to_string(),
                ],
                example_sentences: vec![
                    ExampleSentence {
                        japanese: "朝ごはんを食べます。".to_string(),
                        english: "I eat breakfast.".to_string(),
                    },
                    ExampleSentence {
                        japanese: "寿司を食べたいです。".to_string(),
                        english: "I want to eat sushi.".to_string(),
                    },
                ],
                jlpt_level: JLPTLevel::N5,
            }),
            CardType::Grammar(GrammarCard {
                pattern: "〜てもいい".to_string(),
                pattern_reading: "てもいい".to_string(),
                explanation: "Permission: 'it's okay to...'".to_string(),
                wrong_answers: vec![
                    "Obligation: 'must do'".to_string(),
                    "Prohibition: 'must not'".to_string(),
                    "Suggestion: 'how about'".to_string(),
                ],
                example_sentences: vec![
                    ExampleSentence {
                        japanese: "ここで写真を撮ってもいいですか。".to_string(),
                        english: "Is it okay to take pictures here?".to_string(),
                    },
                    ExampleSentence {
                        japanese: "窓を開けてもいいですよ。".to_string(),
                        english: "It's okay to open the window.".to_string(),
                    },
                ],
                jlpt_level: JLPTLevel::N4,
            }),
            CardType::Vocabulary(VocabularyCard {
                kanji: "勉強".to_string(),
                hiragana: "べんきょう".to_string(),
                romaji: "benkyou".to_string(),
                meaning: "study".to_string(),
                wrong_answers: vec![
                    "work".to_string(),
                    "homework".to_string(),
                    "school".to_string(),
                ],
                example_sentences: vec![
                    ExampleSentence {
                        japanese: "毎日日本語を勉強しています。".to_string(),
                        english: "I study Japanese every day.".to_string(),
                    },
                ],
                jlpt_level: JLPTLevel::N5,
            }),
        ];

        Self {
            cards: sample_cards,
            current_index: 0,
            show_romaji: false,
            show_examples: false,
            quiz_state: QuizState::Question,
            score: 0,
            total_answered: 0,
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackToHome,
    ToggleRomaji,
    ToggleExamples,
    SelectAnswer(usize),
    NextCard,
    PreviousCard,
}

impl PracticeView {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BackToHome => Task::none(),
            Message::ToggleRomaji => {
                self.show_romaji = !self.show_romaji;
                Task::none()
            }
            Message::ToggleExamples => {
                self.show_examples = !self.show_examples;
                Task::none()
            }
            Message::SelectAnswer(selected) => {
                if self.quiz_state == QuizState::Question {
                    // Answer 0 is always the correct answer
                    if selected == 0 {
                        self.quiz_state = QuizState::AnswerCorrect;
                        self.score += 1;
                    } else {
                        self.quiz_state = QuizState::AnswerIncorrect {
                            selected,
                            correct: 0,
                        };
                    }
                    self.total_answered += 1;
                }
                Task::none()
            }
            Message::NextCard => {
                if self.current_index < self.cards.len() - 1 {
                    self.current_index += 1;
                    self.quiz_state = QuizState::Question;
                    self.show_examples = false;
                }
                Task::none()
            }
            Message::PreviousCard => {
                if self.current_index > 0 {
                    self.current_index -= 1;
                    self.quiz_state = QuizState::Question;
                    self.show_examples = false;
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if self.cards.is_empty() {
            return self.empty_state();
        }

        let current_card = &self.cards[self.current_index];

        let content = column![
            self.header(),
            Space::with_height(20),
            self.progress_bar(),
            Space::with_height(20),
            self.card_display(current_card),
            Space::with_height(20),
            self.quiz_section(current_card),
            Space::with_height(20),
            self.navigation_controls(),
        ]
        .spacing(10)
        .padding(20)
        .width(Length::Fill)
        .max_width(ui::MAX_CONTENT_WIDTH)
        .align_x(Alignment::Center);

        scrollable(container(content).width(Length::Fill).center_x(Fill))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()
    }

    fn empty_state(&self) -> Element<'_, Message> {
        let content = column![
            text("Practice Mode").size(32),
            text("No cards to practice yet.").size(16),
            text("Go back to Home and create some flashcards from Japanese text!").size(14),
            Space::with_height(20),
            button("Back to Home")
                .on_press(Message::BackToHome)
                .padding(12)
                .style(styles::button_style),
        ]
        .spacing(20)
        .padding(20)
        .align_x(Alignment::Center);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Fill)
            .center_y(Fill)
            .into()
    }

    fn header(&self) -> Element<'_, Message> {
        let title = text("Practice Mode").size(32);

        let stats = text(format!(
            "Score: {}/{} ({}%)",
            self.score,
            self.total_answered,
            if self.total_answered > 0 {
                (self.score * 100) / self.total_answered
            } else {
                0
            }
        ))
        .size(16);

        let back_button = button("← Back to Home")
            .on_press(Message::BackToHome)
            .padding(10)
            .style(styles::button_style);

        row![
            column![title, stats].spacing(5).width(Length::Fill),
            back_button
        ]
        .align_y(Alignment::Center)
        .spacing(10)
        .into()
    }

    fn progress_bar(&self) -> Element<'_, Message> {
        let progress_text = text(format!(
            "Card {} of {}",
            self.current_index + 1,
            self.cards.len()
        ))
        .size(14);

        container(progress_text)
            .padding(10)
            .width(Length::Fill)
            .center_x(Fill)
            .into()
    }

    fn card_display(&self, card: &CardType) -> Element<'_, Message> {
        match card {
            CardType::Vocabulary(vocab) => self.vocabulary_card(vocab),
            CardType::Grammar(grammar) => self.grammar_card(grammar),
        }
    }

    fn vocabulary_card(&self, card: &VocabularyCard) -> Element<'_, Message> {
        let kanji_text = text(&card.kanji).size(48);

        let hiragana_text = text(&card.hiragana).size(20);

        let romaji_section = if self.show_romaji {
            column![text(&card.romaji).size(16)]
        } else {
            column![
                button("Show Romaji")
                    .on_press(Message::ToggleRomaji)
                    .padding(8)
                    .style(styles::button_style)
            ]
        };

        let jlpt_badge = self.jlpt_badge(card.jlpt_level);

        let card_content = column![
            jlpt_badge,
            Space::with_height(10),
            kanji_text,
            hiragana_text,
            romaji_section,
        ]
        .spacing(10)
        .align_x(Alignment::Center)
        .width(Length::Fill);

        container(card_content)
            .padding(30)
            .width(Length::Fill)
            .style(styles::section_style)
            .into()
    }

    fn grammar_card(&self, card: &GrammarCard) -> Element<'_, Message> {
        let pattern_text = text(&card.pattern).size(48);

        let reading_text = text(&card.pattern_reading).size(20);

        let jlpt_badge = self.jlpt_badge(card.jlpt_level);

        let card_content = column![
            jlpt_badge,
            Space::with_height(10),
            text("Grammar Pattern").size(14),
            pattern_text,
            reading_text,
        ]
        .spacing(10)
        .align_x(Alignment::Center)
        .width(Length::Fill);

        container(card_content)
            .padding(30)
            .width(Length::Fill)
            .style(styles::section_style)
            .into()
    }

    fn jlpt_badge(&self, level: JLPTLevel) -> Element<'_, Message> {
        let badge_text = text(level.as_str()).size(12);

        container(badge_text)
            .padding([4, 12])
            .style(move |theme: &iced::Theme| {
                let mut style = styles::section_style(theme);
                style.background = Some(level.color().into());
                style.text_color = Some(Color::WHITE);
                style.border = Border {
                    radius: iced::border::Radius::from(12.0),
                    ..Default::default()
                };
                style.shadow = Shadow::default();
                style
            })
            .into()
    }

    fn quiz_section(&self, card: &CardType) -> Element<'_, Message> {
        let question = text("What does this mean?").size(18);

        let (correct_answer, wrong_answers) = match card {
            CardType::Vocabulary(vocab) => (&vocab.meaning, &vocab.wrong_answers),
            CardType::Grammar(grammar) => (&grammar.explanation, &grammar.wrong_answers),
        };

        // Shuffle answers (in real implementation, this would be done when card is shown)
        let mut all_answers = vec![correct_answer.clone()];
        all_answers.extend(wrong_answers.iter().cloned());

        let answer_buttons = all_answers
            .iter()
            .enumerate()
            .fold(column![].spacing(10), |col, (idx, answer)| {
                let button_style = match &self.quiz_state {
                    QuizState::Question => styles::button_style,
                    QuizState::AnswerCorrect if idx == 0 => {
                        |theme: &iced::Theme, status| {
                            let mut style = styles::button_style(theme, status);
                            style.background = Some(Color::from_rgb(0.2, 0.8, 0.2).into());
                            style
                        }
                    }
                    QuizState::AnswerIncorrect { selected, correct } => {
                        if idx == *selected {
                            |theme: &iced::Theme, status| {
                                let mut style = styles::button_style(theme, status);
                                style.background = Some(Color::from_rgb(0.8, 0.2, 0.2).into());
                                style
                            }
                        } else if idx == *correct {
                            |theme: &iced::Theme, status| {
                                let mut style = styles::button_style(theme, status);
                                style.background = Some(Color::from_rgb(0.2, 0.8, 0.2).into());
                                style
                            }
                        } else {
                            styles::button_style
                        }
                    }
                    _ => styles::button_style,
                };

                let btn = button(
                    text(answer)
                        .size(16)
                        .width(Length::Fill)
                        .align_x(iced::alignment::Horizontal::Center),
                )
                .padding(15)
                .width(Length::Fill)
                .style(button_style);

                let btn = if self.quiz_state == QuizState::Question {
                    btn.on_press(Message::SelectAnswer(idx))
                } else {
                    btn
                };

                col.push(btn)
            });

        let examples_section = if self.quiz_state != QuizState::Question {
            let examples = match card {
                CardType::Vocabulary(vocab) => &vocab.example_sentences,
                CardType::Grammar(grammar) => &grammar.example_sentences,
            };

            if self.show_examples {
                let examples_list = examples.iter().fold(
                    column![].spacing(15),
                    |col, example| {
                        col.push(column![
                            text(&example.japanese).size(16),
                            text(&example.english).size(14),
                        ].spacing(5))
                    },
                );

                column![
                    Space::with_height(20),
                    button("Hide Examples")
                        .on_press(Message::ToggleExamples)
                        .padding(10)
                        .style(styles::button_style),
                    container(examples_list)
                        .padding(15)
                        .width(Length::Fill)
                        .style(styles::section_style),
                ]
            } else {
                column![
                    Space::with_height(20),
                    button("Show Example Sentences")
                        .on_press(Message::ToggleExamples)
                        .padding(10)
                        .style(styles::button_style),
                ]
            }
        } else {
            column![]
        };

        column![question, answer_buttons, examples_section]
            .spacing(15)
            .into()
    }

    fn navigation_controls(&self) -> Element<'_, Message> {
        let prev_button = button("← Previous")
            .padding(12)
            .style(styles::button_style);

        let prev_button = if self.current_index > 0 {
            prev_button.on_press(Message::PreviousCard)
        } else {
            prev_button
        };

        let next_button = button("Next →")
            .padding(12)
            .style(styles::button_style);

        let next_button = if self.current_index < self.cards.len() - 1
            && self.quiz_state != QuizState::Question
        {
            next_button.on_press(Message::NextCard)
        } else {
            next_button
        };

        row![prev_button, Space::with_width(Fill), next_button]
            .spacing(10)
            .width(Length::Fill)
            .into()
    }
}
