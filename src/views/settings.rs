use iced::widget::{button, column, container, text};
use iced::{Element, Length, Task};

#[derive(Default)]
pub struct SettingsView {
    // TODO: Add settings state
}

#[derive(Debug, Clone)]
pub enum Message {
    // TODO: Add settings messages
    BackToHome,
}

impl SettingsView {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BackToHome => Task::none(),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let content = column![
            text("Settings").size(32),
            text("Settings coming soon...").size(16),
            button("Back to Home")
                .on_press(Message::BackToHome)
                .padding(10),
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
