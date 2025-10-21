//! Main application structure and message routing
//!
//! This module implements the root App following the Elm architecture pattern.
//! It manages navigation between different modes (Home, Practice, Learning, Settings)
//! and routes messages to the appropriate view handlers.

use iced::{Element, Task};

use crate::theme::AppTheme;
use crate::views::{
    home::HomeView, learning::LearningView, practice::PracticeView, settings::SettingsView,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Home,
    Practice,
    Learning,
    Settings,
}

pub struct App {
    mode: AppMode,
    home_view: HomeView,
    practice_view: PracticeView,
    learning_view: LearningView,
    settings_view: SettingsView,
    theme: AppTheme,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Home messages
    Home(crate::views::home::Message),

    // Practice messages
    Practice(crate::views::practice::Message),

    // Learning messages
    Learning(crate::views::learning::Message),

    // Settings messages
    Settings(crate::views::settings::Message),
}

impl Default for App {
    fn default() -> Self {
        Self {
            mode: AppMode::Home,
            home_view: HomeView::default(),
            practice_view: PracticeView::default(),
            learning_view: LearningView::default(),
            settings_view: SettingsView::default(),
            theme: AppTheme::default(),
        }
    }
}

impl App {
    pub fn new() -> (Self, Task<Message>) {
        (Self::default(), Task::none())
    }

    pub fn title(&self) -> String {
        match self.mode {
            AppMode::Home => "NihonRS - Home".to_string(),
            AppMode::Practice => "NihonRS - Practice".to_string(),
            AppMode::Learning => "NihonRS - Learning".to_string(),
            AppMode::Settings => "NihonRS - Settings".to_string(),
        }
    }

    pub fn theme(&self) -> iced::Theme {
        self.theme.to_iced_theme()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Home(msg) => self.handle_home_message(msg),
            Message::Practice(msg) => self.handle_practice_message(msg),
            Message::Learning(msg) => self.handle_learning_message(msg),
            Message::Settings(msg) => self.handle_settings_message(msg),
        }
    }

    fn navigate_to(&mut self, mode: AppMode) {
        // Sync theme state when navigating to settings
        if mode == AppMode::Settings {
            self.settings_view.set_dark_mode(self.theme.is_dark());
        }
        self.mode = mode;
    }

    fn handle_home_message(&mut self, msg: crate::views::home::Message) -> Task<Message> {
        use crate::views::home::Message as HomeMessage;

        match msg {
            HomeMessage::NavigateToSettings => {
                self.navigate_to(AppMode::Settings);
                Task::none()
            }
            HomeMessage::SubmitForPractice => {
                // TODO: Process text and navigate to practice
                self.navigate_to(AppMode::Practice);
                Task::none()
            }
            HomeMessage::SubmitForLearning => {
                // TODO: Process text and navigate to learning
                self.navigate_to(AppMode::Learning);
                Task::none()
            }
            _ => self.home_view.update(msg).map(Message::Home),
        }
    }

    fn handle_practice_message(&mut self, msg: crate::views::practice::Message) -> Task<Message> {
        use crate::views::practice::Message as PracticeMessage;

        match msg {
            PracticeMessage::BackToHome => {
                self.navigate_to(AppMode::Home);
                Task::none()
            }
        }
    }

    fn handle_learning_message(&mut self, msg: crate::views::learning::Message) -> Task<Message> {
        use crate::views::learning::Message as LearningMessage;

        match msg {
            LearningMessage::BackToHome => {
                self.navigate_to(AppMode::Home);
                Task::none()
            }
        }
    }

    fn handle_settings_message(&mut self, msg: crate::views::settings::Message) -> Task<Message> {
        use crate::views::settings::Message as SettingsMessage;

        let task = self
            .settings_view
            .update(msg.clone())
            .map(Message::Settings);

        match msg {
            SettingsMessage::BackToHome => {
                self.navigate_to(AppMode::Home);
                task
            }
            SettingsMessage::DarkModeChanged(enabled) => {
                self.apply_theme(enabled);
                task
            }
            _ => task,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        use iced::widget::container;
        use iced::{Fill, Length};
        let content = match self.mode {
            AppMode::Home => self.home_view.view().map(Message::Home),
            AppMode::Practice => self.practice_view.view().map(Message::Practice),
            AppMode::Learning => self.learning_view.view().map(Message::Learning),
            AppMode::Settings => self.settings_view.view().map(Message::Settings),
        };
        container(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .center_x(Fill)
            .into()
    }

    fn apply_theme(&mut self, dark_mode: bool) {
        self.theme = if dark_mode {
            AppTheme::Dark
        } else {
            AppTheme::Light
        };
        self.settings_view.set_dark_mode(dark_mode);
    }
}
