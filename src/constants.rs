//! Application-wide constants and configuration values

/// UI Constants
pub mod ui {
    /// Maximum width for main content containers
    pub const MAX_CONTENT_WIDTH: f32 = 1200.0;

    /// Maximum width for settings panels
    pub const MAX_SETTINGS_WIDTH: f32 = 900.0;

    /// Default spacing between elements
    #[allow(dead_code)]
    pub const DEFAULT_SPACING: f32 = 10.0;

    /// Default padding for containers
    #[allow(dead_code)]
    pub const DEFAULT_PADDING: f32 = 20.0;

    /// Font size range
    pub const MIN_FONT_SIZE: u16 = 12;
    pub const MAX_FONT_SIZE: u16 = 32;
    pub const DEFAULT_FONT_SIZE: u16 = 18;
}

/// SRS (Spaced Repetition System) Constants
pub mod srs {
    /// Default daily review limit
    pub const DEFAULT_DAILY_REVIEW_LIMIT: usize = 20;

    /// Default new cards per day
    pub const DEFAULT_NEW_CARDS_PER_DAY: usize = 10;
}

/// Application metadata
pub mod app {
    /// Application name
    #[allow(dead_code)]
    pub const NAME: &str = "NihonRS";

    /// Version (from Cargo.toml)
    #[allow(dead_code)]
    pub const VERSION: &str = env!("CARGO_PKG_VERSION");
}
