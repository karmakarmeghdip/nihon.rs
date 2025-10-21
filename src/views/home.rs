use crate::styles;
use iced::widget::{button, column, container, row, scrollable, text, text_input};
use iced::{Alignment, Element, Fill, Length, Task};

#[derive(Default)]
pub struct HomeView {
    input_text: String,
    decks: Vec<DeckInfo>,
    saved_texts: Vec<TextInfo>,
}

#[derive(Debug, Clone)]
pub struct DeckInfo {
    pub id: String,
    pub name: String,
    pub total_cards: usize,
    pub due_cards: usize,
    pub new_cards: usize,
}

#[derive(Debug, Clone)]
pub struct TextInfo {
    pub id: String,
    pub title: String,
    pub preview: String,
    pub created_at: String,
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
        let title = text("NihonRS - Japanese Learning Tool")
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
        let buttons = row![
            button(
                text("Practice Mode")
                    .size(18)
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center)
            )
            .on_press(Message::SubmitForPractice)
            .padding(15)
            .width(Length::Fill)
            .style(styles::button_style),
            button(
                text("Learning Mode")
                    .size(18)
                    .width(Length::Fill)
                    .align_x(iced::alignment::Horizontal::Center)
            )
            .on_press(Message::SubmitForLearning)
            .padding(15)
            .width(Length::Fill)
            .style(styles::button_style),
        ]
        .spacing(10)
        .width(Length::Fill);

        // Decks section
        let decks_title = text("Your Decks").size(24).width(Length::Fill);

        let decks_list = if self.decks.is_empty() {
            column![text("No decks yet. Create one by practicing some text!").size(14)].spacing(5)
        } else {
            let deck_items = self.decks.iter().fold(column![].spacing(10), |col, deck| {
                col.push(self.deck_card(deck))
            });
            deck_items
        };

        // Saved texts section
        let texts_title = text("Saved Texts").size(24).width(Length::Fill);

        let texts_list = if self.saved_texts.is_empty() {
            column![text("No saved texts yet. Start learning mode to save texts!").size(14)]
                .spacing(5)
        } else {
            let text_items = self
                .saved_texts
                .iter()
                .fold(column![].spacing(10), |col, text_info| {
                    col.push(self.text_card(text_info))
                });
            text_items
        };

        // Settings button
        let settings_button = button(text("⚙ Settings"))
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
        .max_width(1200)
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
