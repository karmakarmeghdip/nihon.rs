use crate::styles;
use iced::widget::{
    button, checkbox, column, container, row, scrollable, slider, text, text_input,
};
use iced::{Element, Length, Task, alignment};

#[derive(Debug, Clone)]
pub struct SettingsView {
    font_size: u16,
    dark_mode: bool,
    user_profile: String,
    api_key: String,
    daily_review_limit: String,
    new_cards_per_day: String,
}

impl Default for SettingsView {
    fn default() -> Self {
        Self {
            font_size: 18,
            dark_mode: true,
            user_profile: String::new(),
            api_key: String::new(),
            daily_review_limit: String::from("20"),
            new_cards_per_day: String::from("10"),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Message {
    BackToHome,
    DarkModeChanged(bool),
    FontSizeChanged(u16),
    UserProfileChanged(String),
    ApiKeyChanged(String),
    DailyReviewLimitChanged(String),
    NewCardsPerDayChanged(String),
}

impl SettingsView {
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::BackToHome => Task::none(),
            Message::DarkModeChanged(value) => {
                self.dark_mode = value;
                Task::none()
            }
            Message::FontSizeChanged(size) => {
                self.font_size = size.clamp(12, 32);
                Task::none()
            }
            Message::UserProfileChanged(value) => {
                self.user_profile = value;
                Task::none()
            }
            Message::ApiKeyChanged(value) => {
                self.api_key = value;
                Task::none()
            }
            Message::DailyReviewLimitChanged(value) => {
                self.daily_review_limit = value;
                Task::none()
            }
            Message::NewCardsPerDayChanged(value) => {
                self.new_cards_per_day = value;
                Task::none()
            }
        }
    }

    pub fn set_dark_mode(&mut self, value: bool) {
        self.dark_mode = value;
    }

    pub fn view(&self) -> Element<'_, Message> {
        let appearance_section = container(
            column![
                text("Appearance").size(24),
                checkbox("Use dark theme", self.dark_mode)
                    .on_toggle(Message::DarkModeChanged)
                    .style(styles::checkbox_style),
                column![
                    row![
                        text("Font size:"),
                        text(format!("{} px", self.font_size)).size(14),
                    ]
                    .spacing(8)
                    .align_y(alignment::Vertical::Center),
                    slider(12.0..=32.0, self.font_size as f32, |value| {
                        Message::FontSizeChanged(value.round() as u16)
                    })
                    .style(styles::slider_style),
                ]
                .spacing(10),
            ]
            .spacing(16),
        )
        .padding(20)
        .style(styles::section_style);

        let profile_section = container(
            column![
                text("User Profile").size(24),
                text("Describe your current Japanese level or study goals.").size(14),
                text_input("Beginner learning N5 vocabulary", &self.user_profile)
                    .on_input(Message::UserProfileChanged)
                    .padding(12)
                    .size(16)
                    .width(Length::Fill)
                    .style(styles::text_input_style),
            ]
            .spacing(12),
        )
        .padding(20)
        .style(styles::section_style);

        let llm_section = container(
            column![
                text("LLM Configuration").size(24),
                text("Provide your Gemini API key to enable AI-powered explanations.").size(14),
                text_input("GEMINI_API_KEY", &self.api_key)
                    .on_input(Message::ApiKeyChanged)
                    .padding(12)
                    .size(16)
                    .secure(true)
                    .width(Length::Fill)
                    .style(styles::text_input_style),
            ]
            .spacing(12),
        )
        .padding(20)
        .style(styles::section_style);

        let srs_section = container(
            column![
                text("Spaced Repetition").size(24),
                row![
                    text("Daily review limit"),
                    text_input("20", &self.daily_review_limit)
                        .on_input(Message::DailyReviewLimitChanged)
                        .padding(10)
                        .width(Length::Fixed(100.0))
                        .style(styles::text_input_style),
                ]
                .spacing(12)
                .align_y(alignment::Vertical::Center),
                row![
                    text("New cards per day"),
                    text_input("10", &self.new_cards_per_day)
                        .on_input(Message::NewCardsPerDayChanged)
                        .padding(10)
                        .width(Length::Fixed(100.0))
                        .style(styles::text_input_style),
                ]
                .spacing(12)
                .align_y(alignment::Vertical::Center),
            ]
            .spacing(16),
        )
        .padding(20)
        .style(styles::section_style);

        let content = column![
            text("Settings").size(32),
            text("Customize the app to match your study preferences.").size(16),
            appearance_section,
            profile_section,
            llm_section,
            srs_section,
            button("Back to Home")
                .on_press(Message::BackToHome)
                .padding(12)
                .width(Length::Shrink)
                .style(styles::button_style),
        ]
        .spacing(24)
        .padding(24)
        .max_width(900)
        .align_x(alignment::Horizontal::Center);

        scrollable(
            container(content)
                .width(Length::Fill)
                .center_x(Length::Fill),
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .into()
    }
}
