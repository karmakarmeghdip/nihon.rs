//! Practice view - Flashcard-based spaced repetition practice
//!
//! This view will implement:
//! - Flashcard display with vocabulary/grammar questions
//! - SRS (Spaced Repetition System) algorithm
//! - Answer feedback and grading

use crate::styles;
use iced::widget::{button, column, container, text};
use iced::{Element, Length, Task};

#[derive(Default)]
pub struct PracticeView {
    // TODO: Add practice mode state
}

#[derive(Debug, Clone)]
pub enum Message {
    // TODO: Add practice mode messages
    BackToHome,
}

impl PracticeView {
    #[allow(dead_code)]
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BackToHome => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = column![
            text("Practice Mode").size(32),
            text("Practice mode coming soon...").size(16),
            button("Back to Home")
                .on_press(Message::BackToHome)
                .padding(12)
                .style(styles::button_style),
        ]
        .spacing(20)
        .padding(20);

        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .into()
    }
}
