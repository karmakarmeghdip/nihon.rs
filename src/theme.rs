/// Theme management and utilities
use iced::Theme;

/// Available app themes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppTheme {
    Dark,
    Light,
}

impl AppTheme {
    /// Convert to iced Theme
    pub fn to_iced_theme(self) -> Theme {
        match self {
            Self::Dark => Theme::CatppuccinMocha,
            Self::Light => Theme::CatppuccinLatte,
        }
    }

    /// Toggle between dark and light
    #[allow(dead_code)]
    pub fn toggle(self) -> Self {
        match self {
            Self::Dark => Self::Light,
            Self::Light => Self::Dark,
        }
    }

    /// Check if dark mode
    pub fn is_dark(self) -> bool {
        matches!(self, Self::Dark)
    }
}

impl Default for AppTheme {
    fn default() -> Self {
        Self::Dark
    }
}
