//! Slider styles using Catppuccin color palette

use iced::widget::slider;
use iced::{Background, Border};

/// Catppuccin-inspired slider style matching shadcn aesthetics
pub fn slider_style(theme: &iced::Theme, status: slider::Status) -> slider::Style {
    let palette = theme.extended_palette();

    let (active_color, handle_color) = match status {
        slider::Status::Active => (palette.primary.strong.color, palette.background.base.color),
        slider::Status::Hovered => (
            super::utils::mix_colors(
                palette.primary.strong.color,
                palette.primary.base.color,
                0.2,
            ),
            super::utils::mix_colors(
                palette.primary.strong.color,
                palette.background.base.color,
                0.2,
            ),
        ),
        slider::Status::Dragged => (palette.primary.base.color, palette.primary.strong.color),
    };

    slider::Style {
        rail: slider::Rail {
            backgrounds: (
                Background::Color(active_color),
                Background::Color(palette.background.stronger.color),
            ),
            width: 6.0,
            border: Border {
                radius: iced::border::Radius::from(999.0),
                width: 1.0,
                color: palette.background.strong.color.scale_alpha(0.35),
            },
        },
        handle: slider::Handle {
            shape: slider::HandleShape::Circle { radius: 9.0 },
            background: Background::Color(handle_color),
            border_width: 2.0,
            border_color: super::utils::mix_colors(handle_color, palette.primary.strong.color, 0.3),
        },
    }
}
