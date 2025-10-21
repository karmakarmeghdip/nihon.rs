use iced::{Element, Task};

use crate::views::{
    home::HomeView, learning::LearningView, practice::PracticeView, settings::SettingsView,
};

#[derive(Debug, Clone)]
pub enum AppMode {
    Home,
    Practice,
    Learning,
    Settings,
}

pub struct App {
    pub mode: AppMode,
    pub home_view: HomeView,
    pub practice_view: PracticeView,
    pub learning_view: LearningView,
    pub settings_view: SettingsView,
    pub theme: iced::Theme,
}

#[derive(Debug, Clone)]
pub enum Message {
    // Navigation
    NavigateTo(AppMode),

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
            theme: iced::Theme::CatppuccinMocha,
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
        self.theme.clone()
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::NavigateTo(mode) => {
                self.mode = mode;
                Task::none()
            }
            Message::Home(msg) => self.home_view.update(msg).map(Message::Home),
            Message::Practice(msg) => self.practice_view.update(msg).map(Message::Practice),
            Message::Learning(msg) => self.learning_view.update(msg).map(Message::Learning),
            Message::Settings(msg) => self.settings_view.update(msg).map(Message::Settings),
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        match self.mode {
            AppMode::Home => self.home_view.view().map(Message::Home),
            AppMode::Practice => self.practice_view.view().map(Message::Practice),
            AppMode::Learning => self.learning_view.view().map(Message::Learning),
            AppMode::Settings => self.settings_view.view().map(Message::Settings),
        }
    }
}
