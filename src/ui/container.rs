//! Container styles using Catppuccin color palette

use iced::widget::container;
use iced::{Border, Shadow, Vector};

/// Catppuccin-inspired container section style (for cards)
pub fn section_style(theme: &iced::Theme) -> container::Style {
    let palette = theme.extended_palette();

    let mut style = container::Style::default();
    let card_color = palette.background.weaker.color;
    style.background = Some(card_color.into());
    style.text_color = Some(palette.background.weaker.text);
    style.border = Border {
        color: palette.background.strong.color,
        width: 1.0,
        radius: iced::border::Radius::from(16.0),
    };
    style.shadow = Shadow {
        color: palette.background.strong.color.scale_alpha(0.35),
        offset: Vector::new(0.0, 6.0),
        blur_radius: 18.0,
    };

    style
}
