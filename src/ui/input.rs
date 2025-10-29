//! Text input styles using Catppuccin color palette

use iced::widget::text_input;
use iced::{Background, Border};

/// Catppuccin-inspired text input style matching shadcn aesthetics
pub fn text_input_style(theme: &iced::Theme, status: text_input::Status) -> text_input::Style {
    let palette = theme.extended_palette();

    let mut background = Background::Color(palette.background.weak.color);
    let mut border_color = palette.background.strong.color.scale_alpha(0.45);
    let mut value_color = palette.background.base.text;
    let mut placeholder_color = value_color.scale_alpha(0.55);
    let mut icon_color = value_color.scale_alpha(0.8);
    let selection_color = palette.primary.strong.color.scale_alpha(0.25);

    match status {
        text_input::Status::Active => {}
        text_input::Status::Hovered => {
            border_color = palette.primary.weak.color;
        }
        text_input::Status::Focused { is_hovered } => {
            background = Background::Color(palette.background.weaker.color);
            border_color = if is_hovered {
                super::utils::mix_colors(
                    palette.primary.strong.color,
                    palette.primary.base.color,
                    0.3,
                )
            } else {
                palette.primary.strong.color
            };
        }
        text_input::Status::Disabled => {
            background = Background::Color(palette.background.weaker.color);
            value_color = value_color.scale_alpha(0.4);
            placeholder_color = placeholder_color.scale_alpha(0.4);
            icon_color = icon_color.scale_alpha(0.4);
        }
    }

    text_input::Style {
        background,
        border: Border {
            color: border_color,
            width: 1.0,
            radius: iced::border::Radius::from(10.0),
        },
        icon: icon_color,
        placeholder: placeholder_color,
        value: value_color,
        selection: selection_color,
    }
}
