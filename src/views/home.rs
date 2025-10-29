//! Home view - Main landing page with text input and deck/text lists
//!
//! This view allows users to:
//! - Input Japanese text for practice or learning
//! - View and select existing decks
//! - View and continue saved texts
//! - Navigate to settings

use crate::constants::ui;
use crate::styles;
use crate::types::{DeckInfo, TextInfo};
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Fill, Length, Task};

#[derive(Default)]
pub struct HomeView {
    input_text: String,
    decks: Vec<DeckInfo>,
    saved_texts: Vec<TextInfo>,
}

impl HomeView {
    /// Check if text input is valid (not empty)
    fn has_valid_input(&self) -> bool {
        !self.input_text.trim().is_empty()
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    SubmitForPractice,
    SubmitForLearning,
    SelectDeck(String),
    SelectText(String),
    NavigateToSettings,
}

impl HomeView {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::InputChanged(text) => {
                self.input_text = text;
                Task::none()
            }
            Message::SubmitForPractice => {
                // TODO: Process text and navigate to practice mode
                println!("Submit for practice: {}", self.input_text);
                Task::none()
            }
            Message::SubmitForLearning => {
                // TODO: Process text and navigate to learning mode
                println!("Submit for learning: {}", self.input_text);
                Task::none()
            }
            Message::SelectDeck(id) => {
                println!("Selected deck: {}", id);
                Task::none()
            }
            Message::SelectText(id) => {
                println!("Selected text: {}", id);
                Task::none()
            }
            Message::NavigateToSettings => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let title = text("nihon.rs - Japanese Learning Tool")
            .size(32)
            .width(Length::Fill);

        let subtitle = text("Paste Japanese text below to start learning")
            .size(16)
            .width(Length::Fill);

        // Text input area
        let input = text_input(
            "貼り付けてください... (Paste Japanese text here)",
            &self.input_text,
        )
        .on_input(Message::InputChanged)
        .padding(15)
        .size(16)
        .style(styles::text_input_style);

        // Action buttons
        let practice_button = button(
            text("Practice Mode")
                .size(18)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center),
        )
        .padding(15)
        .width(Length::Fill)
        .style(styles::button_style);

        let practice_button = if self.has_valid_input() {
            practice_button.on_press(Message::SubmitForPractice)
        } else {
            practice_button
        };

        let learning_button = button(
            text("Learning Mode")
                .size(18)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center),
        )
        .padding(15)
        .width(Length::Fill)
        .style(styles::button_style);

        let learning_button = if self.has_valid_input() {
            learning_button.on_press(Message::SubmitForLearning)
        } else {
            learning_button
        };

        let buttons = row![practice_button, learning_button]
            .spacing(10)
            .width(Length::Fill);

        // Decks section
        let decks_title = text("Your Decks").size(24).width(Length::Fill);

        let decks_list = if self.decks.is_empty() {
            column![text("No decks yet. Create one by practicing some text!").size(14)].spacing(5)
        } else {
            self.decks.iter().fold(column![].spacing(10), |col, deck| {
                col.push(self.deck_card(deck))
            })
        };

        // Saved texts section
        let texts_title = text("Saved Texts").size(24).width(Length::Fill);

        let texts_list = if self.saved_texts.is_empty() {
            column![text("No saved texts yet. Start learning mode to save texts!").size(14)]
                .spacing(5)
        } else {
            self.saved_texts
                .iter()
                .fold(column![].spacing(10), |col, text_info| {
                    col.push(self.text_card(text_info))
                })
        };

        // Settings button
        let settings_button = button("Settings")
            .on_press(Message::NavigateToSettings)
            .padding(10)
            .style(styles::button_style);

        // Main layout
        let content = column![
            // Header
            row![title, settings_button]
                .align_y(Alignment::Center)
                .spacing(10),
            subtitle,
            // Input section
            text("Input Text").size(20),
            input,
            buttons,
            // Content sections
            decks_title,
            decks_list,
            texts_title,
            texts_list,
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

    fn deck_card<'a>(&self, deck: &'a DeckInfo) -> Element<'a, Message> {
        let name = text(&deck.name).size(18);

        let stats = text(format!(
            "Total: {} | Due: {} | New: {}",
            deck.total_cards, deck.due_cards, deck.new_cards
        ))
        .size(14);

        let open_button = button(text("Open"))
            .on_press(Message::SelectDeck(deck.id.clone()))
            .padding(8)
            .style(styles::button_style);

        container(
            row![
                column![name, stats].spacing(5).width(Length::Fill),
                open_button
            ]
            .align_y(Alignment::Center)
            .spacing(10)
            .padding(15),
        )
        .width(Length::Fill)
        .style(styles::section_style)
        .into()
    }

    fn text_card<'a>(&self, text_info: &'a TextInfo) -> Element<'a, Message> {
        let title = text(&text_info.title).size(16);

        let preview = text(&text_info.preview).size(14);

        let date = text(&text_info.created_at).size(12);

        let open_button = button(text("Continue"))
            .on_press(Message::SelectText(text_info.id.clone()))
            .padding(8)
            .style(styles::button_style);

        container(
            row![
                column![title, preview, date].spacing(5).width(Length::Fill),
                open_button
            ]
            .align_y(Alignment::Center)
            .spacing(10)
            .padding(15),
        )
        .width(Length::Fill)
        .style(styles::section_style)
        .into()
    }
}
