use crate::styles;
use iced::widget::{button, column, container, text};
use iced::{Element, Length, Task};

#[derive(Default)]
pub struct LearningView {
    // TODO: Add learning mode state
}

#[derive(Debug, Clone)]
pub enum Message {
    // TODO: Add learning mode messages
    BackToHome,
}

impl LearningView {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BackToHome => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = column![
            text("Learning Mode").size(32),
            text("Learning mode coming soon...").size(16),
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
