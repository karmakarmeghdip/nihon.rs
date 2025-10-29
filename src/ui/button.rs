//! Button styles using Catppuccin color palette

use iced::widget::button;
use iced::{Background, Border, Shadow, Vector};

/// Catppuccin-inspired button style matching shadcn aesthetics
pub fn button_style(theme: &iced::Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    let mut base = button::Style::default();
    base.background = Some(Background::Color(palette.primary.strong.color));
    base.text_color = palette.primary.strong.text;
    base.border = Border {
        color: super::utils::mix_colors(
            palette.primary.strong.color,
            palette.background.base.color,
            0.45,
        ),
        width: 1.0,
        radius: iced::border::Radius::from(10.0),
    };
    base.shadow = Shadow {
        color: palette.background.strong.color.scale_alpha(0.25),
        offset: Vector::new(0.0, 2.0),
        blur_radius: 14.0,
    };
    base.snap = false;

    match status {
        button::Status::Active => base,
        button::Status::Hovered => {
            let mut hovered = base;
            hovered.background = Some(Background::Color(super::utils::mix_colors(
                palette.primary.strong.color,
                palette.primary.base.color,
                0.25,
            )));
            hovered.border.color = super::utils::mix_colors(
                palette.primary.strong.color,
                palette.primary.base.color,
                0.35,
            );
            hovered.shadow = Shadow {
                offset: Vector::new(0.0, 4.0),
                blur_radius: 18.0,
                color: base.shadow.color,
            };
            hovered
        }
        button::Status::Pressed => {
            let mut pressed = base;
            pressed.background = Some(Background::Color(palette.primary.base.color));
            pressed.border.color = super::utils::mix_colors(
                palette.primary.base.color,
                palette.background.base.color,
                0.3,
            );
            pressed.shadow = Shadow {
                offset: Vector::new(0.0, 1.0),
                blur_radius: 10.0,
                color: base.shadow.color.scale_alpha(0.7),
            };
            pressed
        }
        button::Status::Disabled => {
            let mut disabled = base;
            disabled.background = disabled
                .background
                .map(|background| background.scale_alpha(0.4));
            disabled.text_color = disabled.text_color.scale_alpha(0.45);
            disabled.border.color = disabled.border.color.scale_alpha(0.3);
            disabled.shadow = Shadow::default();
            disabled
        }
    }
}
