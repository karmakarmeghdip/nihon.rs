//! UI styling and theming
//!
//! This module contains all UI styling functions organized by widget type.

pub mod button;
pub mod input;
pub mod container;
pub mod slider;
pub mod theme;
pub mod utils;

// Re-export commonly used styles
pub use button::button_style;
pub use input::text_input_style;
pub use container::section_style;
pub use slider::slider_style;
pub use theme::get_theme;
pub use utils::mix_colors;
