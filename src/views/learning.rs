//! Learning view - Interactive text reading with AI assistance
//!
//! This view implements:
//! - Tokenized Japanese text display
//! - Clickable words for AI-powered explanations
//! - Context-aware grammar and vocabulary help
//! - On-demand LLM processing with loading states
//! - Add words to flashcards functionality

use crate::constants::ui;
use crate::models::{ExampleSentence, WordExplanation, WordSegment};
use crate::ui::{button_style, section_style, text_input_style};
use iced::widget::{
    button, column, container, row, scrollable, text, text_input, Space,
};
use iced::{Alignment, Color, Element, Fill, Length, Task};

/// Loading state for LLM explanation
#[derive(Debug, Clone, PartialEq)]
enum LoadingState {
    Idle,
    Loading,
    Loaded,
    Error(String),
}

pub struct LearningView {
    // Current text being studied
    original_text: String,
    
    // Parsed word segments (simulated tokenization)
    word_segments: Vec<WordSegment>,
    
    // Currently selected word index
    selected_word_index: Option<usize>,
    
    // Loading state for LLM
    loading_state: LoadingState,
    
    // User question input
    question_input: String,
    
    // LLM responses to user questions
    qa_history: Vec<(String, String)>, // (question, answer)
}

impl Default for LearningView {
    fn default() -> Self {
        // Create sample parsed text for demonstration
        let sample_text = "今日は日本語を勉強します。";
        let sample_segments = vec![
            WordSegment {
                surface: "今日".to_string(),
                reading: "きょう".to_string(),
                base_form: "今日".to_string(),
                explanation: None,
                is_selected: false,
            },
            WordSegment {
                surface: "は".to_string(),
                reading: "は".to_string(),
                base_form: "は".to_string(),
                explanation: None,
                is_selected: false,
            },
            WordSegment {
                surface: "日本語".to_string(),
                reading: "にほんご".to_string(),
                base_form: "日本語".to_string(),
                explanation: None,
                is_selected: false,
            },
            WordSegment {
                surface: "を".to_string(),
                reading: "を".to_string(),
                base_form: "を".to_string(),
                explanation: None,
                is_selected: false,
            },
            WordSegment {
                surface: "勉強".to_string(),
                reading: "べんきょう".to_string(),
                base_form: "勉強".to_string(),
                explanation: None,
                is_selected: false,
            },
            WordSegment {
                surface: "します".to_string(),
                reading: "します".to_string(),
                base_form: "する".to_string(),
                explanation: None,
                is_selected: false,
            },
            WordSegment {
                surface: "。".to_string(),
                reading: "。".to_string(),
                base_form: "。".to_string(),
                explanation: None,
                is_selected: false,
            },
        ];

        Self {
            original_text: sample_text.to_string(),
            word_segments: sample_segments,
            selected_word_index: None,
            loading_state: LoadingState::Idle,
            question_input: String::new(),
            qa_history: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackToHome,
    SelectWord(usize),
    RequestExplanation,
    ExplanationReceived(WordExplanation),
    ExplanationError(String),
    AddToVocabularyFlashcards,
    AddToGrammarFlashcards,
    QuestionInputChanged(String),
    AskQuestion,
    QuestionAnswered(String),
    NextWord,
    PreviousWord,
}

impl LearningView {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BackToHome => Task::none(),
            
            Message::SelectWord(index) => {
                // Deselect all words
                for segment in &mut self.word_segments {
                    segment.is_selected = false;
                }
                
                // Select the clicked word
                if let Some(segment) = self.word_segments.get_mut(index) {
                    segment.is_selected = true;
                    self.selected_word_index = Some(index);
                    
                    // If no explanation exists, request one
                    if segment.explanation.is_none() {
                        self.loading_state = LoadingState::Loading;
                        // TODO: In real implementation, spawn async task to call LLM
                        // For now, simulate with sample data
                        let surface = segment.surface.clone();
                        return Task::done(Message::ExplanationReceived(
                            self.generate_sample_explanation(&surface),
                        ));
                    } else {
                        self.loading_state = LoadingState::Loaded;
                    }
                }
                Task::none()
            }
            
            Message::RequestExplanation => {
                if let Some(index) = self.selected_word_index {
                    self.loading_state = LoadingState::Loading;
                    if let Some(segment) = self.word_segments.get(index) {
                        // TODO: Real LLM call here
                        return Task::done(Message::ExplanationReceived(
                            self.generate_sample_explanation(&segment.surface),
                        ));
                    }
                }
                Task::none()
            }
            
            Message::ExplanationReceived(explanation) => {
                if let Some(index) = self.selected_word_index {
                    if let Some(segment) = self.word_segments.get_mut(index) {
                        segment.explanation = Some(explanation);
                        self.loading_state = LoadingState::Loaded;
                    }
                }
                Task::none()
            }
            
            Message::ExplanationError(error) => {
                self.loading_state = LoadingState::Error(error);
                Task::none()
            }
            
            Message::AddToVocabularyFlashcards => {
                if let Some(index) = self.selected_word_index {
                    if let Some(segment) = self.word_segments.get(index) {
                        println!("Adding to vocabulary flashcards: {}", segment.surface);
                        // TODO: Create flashcard and save to database
                    }
                }
                Task::none()
            }
            
            Message::AddToGrammarFlashcards => {
                if let Some(index) = self.selected_word_index {
                    if let Some(segment) = self.word_segments.get(index) {
                        println!("Adding to grammar flashcards: {}", segment.surface);
                        // TODO: Create grammar flashcard and save to database
                    }
                }
                Task::none()
            }
            
            Message::QuestionInputChanged(input) => {
                self.question_input = input;
                Task::none()
            }
            
            Message::AskQuestion => {
                if !self.question_input.trim().is_empty() {
                    let question = self.question_input.clone();
                    self.question_input.clear();
                    // TODO: Real LLM call here
                    let answer = format!("This is a simulated answer to: '{}'", question);
                    return Task::done(Message::QuestionAnswered(answer));
                }
                Task::none()
            }
            
            Message::QuestionAnswered(answer) => {
                if let Some((_question, _)) = self.qa_history.last() {
                    // Update the last Q&A pair
                    if let Some(last) = self.qa_history.last_mut() {
                        last.1 = answer;
                    }
                }
                Task::none()
            }
            
            Message::NextWord => {
                if let Some(current_index) = self.selected_word_index {
                    if current_index < self.word_segments.len() - 1 {
                        return Task::done(Message::SelectWord(current_index + 1));
                    }
                }
                Task::none()
            }
            
            Message::PreviousWord => {
                if let Some(current_index) = self.selected_word_index {
                    if current_index > 0 {
                        return Task::done(Message::SelectWord(current_index - 1));
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        if self.word_segments.is_empty() {
            return self.empty_state();
        }

        let content = column![
            self.header(),
            Space::new().height(20),
            self.word_segments_display(),
            Space::new().height(20),
            self.explanation_panel(),
            Space::new().height(20),
            self.question_section(),
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
            text("Learning Mode").size(32),
            text("No text to learn yet.").size(16),
            text("Go back to Home and paste Japanese text to start learning!").size(14),
            Space::new().height(20),
            button("Back to Home")
                .on_press(Message::BackToHome)
                .padding(12)
                .style(button_style),
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
        let title = text("Learning Mode").size(32);

        let subtitle = text("Click on words to see explanations").size(16);

        let back_button = button("← Back to Home")
            .on_press(Message::BackToHome)
            .padding(10)
            .style(button_style);

        row![
            column![title, subtitle].spacing(5).width(Length::Fill),
            back_button
        ]
        .align_y(Alignment::Center)
        .spacing(10)
        .into()
    }

    fn word_segments_display(&self) -> Element<'_, Message> {
        let title = text("Japanese Text").size(20);

        let word_buttons: Vec<Element<'_, Message>> = self.word_segments
            .iter()
            .enumerate()
            .map(|(idx, segment)| {
                let is_selected = segment.is_selected;
                let has_explanation = segment.explanation.is_some();

                button(
                    column![
                        text(&segment.surface).size(24),
                        text(&segment.reading).size(12),
                    ]
                    .spacing(2)
                    .align_x(Alignment::Center),
                )
                .on_press(Message::SelectWord(idx))
                .padding(10)
                .style(move |theme: &iced::Theme, status| {
                    let mut style = button_style(theme, status);
                    if is_selected {
                        style.background = Some(Color::from_rgb(0.3, 0.5, 0.8).into());
                    } else if has_explanation {
                        style.background = Some(Color::from_rgb(0.4, 0.7, 0.4).into());
                    }
                    style
                })
                .into()
            })
            .collect();

        let words_row = row(word_buttons).spacing(5).wrap();

        let legend = row![
            container(text("● Selected").size(12))
                .padding([5, 10])
                .style(|theme: &iced::Theme| {
                    let mut style = section_style(theme);
                    style.background = Some(Color::from_rgb(0.3, 0.5, 0.8).into());
                    style
                }),
            container(text("● Has Explanation").size(12))
                .padding([5, 10])
                .style(|theme: &iced::Theme| {
                    let mut style = section_style(theme);
                    style.background = Some(Color::from_rgb(0.4, 0.7, 0.4).into());
                    style
                }),
        ]
        .spacing(10);

        container(
            column![title, words_row, Space::new().height(10), legend].spacing(15),
        )
        .padding(20)
        .width(Length::Fill)
        .style(section_style)
        .into()
    }

    fn explanation_panel(&self) -> Element<'_, Message> {
        if let Some(index) = self.selected_word_index {
            if let Some(segment) = self.word_segments.get(index) {
                return match &self.loading_state {
                    LoadingState::Idle => {
                        container(text("Click 'Request Explanation' to learn more").size(14))
                            .padding(20)
                            .width(Length::Fill)
                            .center_x(Fill)
                            .into()
                    }
                    LoadingState::Loading => {
                        container(
                            column![
                                text("Loading explanation...").size(16),
                                text("(Querying AI tutor)").size(12),
                            ]
                            .spacing(5)
                            .align_x(Alignment::Center),
                        )
                        .padding(30)
                        .width(Length::Fill)
                        .center_x(Fill)
                        .style(section_style)
                        .into()
                    }
                    LoadingState::Loaded => {
                        if let Some(explanation) = &segment.explanation {
                            self.display_explanation(segment, explanation)
                        } else {
                            container(text("No explanation available").size(14))
                                .padding(20)
                                .width(Length::Fill)
                                .center_x(Fill)
                                .into()
                        }
                    }
                    LoadingState::Error(error) => {
                        container(
                            column![
                                text("Error loading explanation").size(16),
                                text(error).size(12),
                            ]
                            .spacing(5),
                        )
                        .padding(20)
                        .width(Length::Fill)
                        .style(|theme: &iced::Theme| {
                            let mut style = section_style(theme);
                            style.background = Some(Color::from_rgb(0.8, 0.3, 0.3).into());
                            style
                        })
                        .into()
                    }
                };
            }
        }

        container(text("Select a word to see its explanation").size(14))
            .padding(20)
            .width(Length::Fill)
            .center_x(Fill)
            .into()
    }

    fn display_explanation<'a>(
        &'a self,
        segment: &'a WordSegment,
        explanation: &'a WordExplanation,
    ) -> Element<'a, Message> {
        let word_display: Element<'_, Message> = column![
            text(&segment.surface).size(32),
            text(&segment.reading).size(18),
            text(format!("Dictionary form: {}", segment.base_form)).size(14),
        ]
        .spacing(5)
        .align_x(Alignment::Center)
        .into();

        let jlpt_badge: Element<'_, Message> = container(text(&explanation.jlpt_level).size(12))
            .padding([4, 12])
            .style(|theme: &iced::Theme| {
                let mut style = section_style(theme);
                style.background = Some(Color::from_rgb(0.4, 0.6, 0.8).into());
                style.text_color = Some(Color::WHITE);
                style
            })
            .into();

        let meaning: Element<'_, Message> = column![
            text("Meaning:").size(16),
            text(&explanation.meaning).size(14),
        ]
        .spacing(5)
        .into();

        let mut grammar_elements: Vec<Element<'_, Message>> = Vec::new();
        if let Some(grammar) = &explanation.grammar_notes {
            grammar_elements.push(Space::new().height(10).into());
            grammar_elements.push(text("Grammar Notes:").size(16).into());
            grammar_elements.push(text(grammar).size(14).into());
        }
        let grammar_section: Element<'_, Message> = column(grammar_elements).spacing(5).into();

        let mut examples_elements: Vec<Element<'_, Message>> = Vec::new();
        if !explanation.examples.is_empty() {
            examples_elements.push(Space::new().height(10).into());
            examples_elements.push(text("Example Sentences:").size(16).into());
            
            for example in &explanation.examples {
                let example_container = container(
                    column![
                        text(&example.japanese).size(14),
                        text(&example.english).size(12),
                    ]
                    .spacing(5),
                )
                .padding(10)
                .width(Length::Fill)
                .style(|theme: &iced::Theme| {
                    let palette = theme.extended_palette();
                    let mut style = section_style(theme);
                    style.background = Some(palette.background.weak.color.into());
                    style
                });
                examples_elements.push(example_container.into());
            }
        }
        let examples_section: Element<'_, Message> = column(examples_elements).spacing(10).into();

        let action_buttons: Element<'_, Message> = row![
            button("Add to Vocabulary")
                .on_press(Message::AddToVocabularyFlashcards)
                .padding(10)
                .style(button_style),
            button("Add to Grammar")
                .on_press(Message::AddToGrammarFlashcards)
                .padding(10)
                .style(button_style),
        ]
        .spacing(10)
        .into();

        let navigation: Element<'_, Message> = row![
            button("← Previous")
                .padding(10)
                .style(button_style)
                .on_press(Message::PreviousWord),
            Space::new().width(Fill),
            button("Next →")
                .padding(10)
                .style(button_style)
                .on_press(Message::NextWord),
        ]
        .width(Length::Fill)
        .into();

        container(
            column(vec![
                jlpt_badge,
                word_display,
                Space::new().height(15).into(),
                meaning,
                grammar_section,
                examples_section,
                Space::new().height(20).into(),
                action_buttons,
                Space::new().height(10).into(),
                navigation,
            ])
            .spacing(10),
        )
        .padding(20)
        .width(Length::Fill)
        .style(section_style)
        .into()
    }

    fn question_section(&self) -> Element<'_, Message> {
        let title = text("Ask a Question").size(20);

        let input = text_input(
            "Ask about grammar, usage, or anything else...",
            &self.question_input,
        )
        .on_input(Message::QuestionInputChanged)
        .padding(12)
        .size(14)
        .style(text_input_style);

        let ask_button = button("Ask")
            .padding(12)
            .style(button_style);

        let ask_button = if !self.question_input.trim().is_empty() {
            ask_button.on_press(Message::AskQuestion)
        } else {
            ask_button
        };

        let qa_history = if !self.qa_history.is_empty() {
            let history_list =
                self.qa_history
                    .iter()
                    .fold(column![].spacing(15), |col, (q, a)| {
                        col.push(
                            container(
                                column![
                                    text(format!("Q: {}", q)).size(14),
                                    text(format!("A: {}", a)).size(12),
                                ]
                                .spacing(5),
                            )
                            .padding(15)
                            .width(Length::Fill)
                            .style(|theme: &iced::Theme| {
                                let palette = theme.extended_palette();
                                let mut style = section_style(theme);
                                style.background =
                                    Some(palette.background.weak.color.into());
                                style
                            }),
                        )
                    });

            column![Space::new().height(15), history_list].spacing(10)
        } else {
            column![]
        };

        container(
            column![
                title,
                row![input, ask_button].spacing(10),
                qa_history,
            ]
            .spacing(15),
        )
        .padding(20)
        .width(Length::Fill)
        .style(section_style)
        .into()
    }

    // Helper function to generate sample explanations (will be replaced with real LLM)
    fn generate_sample_explanation(&self, word: &str) -> WordExplanation {
        match word {
            "今日" => WordExplanation {
                meaning: "today".to_string(),
                grammar_notes: None,
                examples: vec![
                    ExampleSentence {
                        japanese: "今日は晴れです。".to_string(),
                        english: "Today is sunny.".to_string(),
                    },
                ],
                jlpt_level: "N5".to_string(),
            },
            "日本語" => WordExplanation {
                meaning: "Japanese language".to_string(),
                grammar_notes: Some("Compound of 日本 (Japan) + 語 (language)".to_string()),
                examples: vec![
                    ExampleSentence {
                        japanese: "日本語を話せますか。".to_string(),
                        english: "Can you speak Japanese?".to_string(),
                    },
                ],
                jlpt_level: "N5".to_string(),
            },
            "勉強" => WordExplanation {
                meaning: "study".to_string(),
                grammar_notes: Some("Noun that can be used with する to make a verb (勉強する = to study)".to_string()),
                examples: vec![
                    ExampleSentence {
                        japanese: "毎日勉強します。".to_string(),
                        english: "I study every day.".to_string(),
                    },
                ],
                jlpt_level: "N5".to_string(),
            },
            _ => WordExplanation {
                meaning: format!("Meaning of '{}' (simulated)", word),
                grammar_notes: Some("This is a simulated explanation. In the real app, this will come from the AI tutor.".to_string()),
                examples: vec![],
                jlpt_level: "N5".to_string(),
            },
        }
    }
}
