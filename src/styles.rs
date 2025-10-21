use iced::widget::{button, checkbox, container, slider, text_input};
use iced::{Background, Border, Color, Shadow, Vector};

/// Catppuccin-inspired button style matching shadcn aesthetics
pub fn button_style(theme: &iced::Theme, status: button::Status) -> button::Style {
    let palette = theme.extended_palette();

    let mut base = button::Style::default();
    base.background = Some(Background::Color(palette.primary.strong.color));
    base.text_color = palette.primary.strong.text;
    base.border = Border {
        color: mix_colors(
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
            hovered.background = Some(Background::Color(mix_colors(
                palette.primary.strong.color,
                palette.primary.base.color,
                0.25,
            )));
            hovered.border.color = mix_colors(
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
            pressed.border.color = mix_colors(
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

/// Catppuccin-inspired slider style matching shadcn aesthetics
pub fn slider_style(theme: &iced::Theme, status: slider::Status) -> slider::Style {
    let palette = theme.extended_palette();

    let (active_color, handle_color) = match status {
        slider::Status::Active => (palette.primary.strong.color, palette.background.base.color),
        slider::Status::Hovered => (
            mix_colors(
                palette.primary.strong.color,
                palette.primary.base.color,
                0.2,
            ),
            mix_colors(
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
            border_color: mix_colors(handle_color, palette.primary.strong.color, 0.3),
        },
    }
}

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
                mix_colors(
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

/// Catppuccin-inspired checkbox style matching shadcn aesthetics
pub fn checkbox_style(theme: &iced::Theme, status: checkbox::Status) -> checkbox::Style {
    let palette = theme.extended_palette();

    let mut background_color = palette.background.weak.color;
    let mut icon_color = palette.background.base.text;
    let mut border_color = palette.background.strong.color.scale_alpha(0.5);
    let mut text_color = Some(palette.background.base.text);

    let is_checked = match status {
        checkbox::Status::Active { is_checked }
        | checkbox::Status::Hovered { is_checked }
        | checkbox::Status::Disabled { is_checked } => is_checked,
    };

    match status {
        checkbox::Status::Active { .. } => {
            if is_checked {
                background_color = palette.primary.strong.color;
                border_color = palette.primary.strong.color;
                icon_color = palette.primary.strong.text;
            }
        }
        checkbox::Status::Hovered { .. } => {
            background_color = mix_colors(
                palette.background.weak.color,
                palette.primary.strong.color,
                if is_checked { 0.45 } else { 0.2 },
            );
            border_color = palette.primary.strong.color;
            if is_checked {
                icon_color = palette.primary.strong.text;
            }
        }
        checkbox::Status::Disabled { .. } => {
            text_color = text_color.map(|color| color.scale_alpha(0.5));
            border_color = border_color.scale_alpha(0.3);
            icon_color = icon_color.scale_alpha(0.35);
            background_color = mix_colors(
                palette.background.weak.color,
                palette.background.strong.color,
                if is_checked { 0.3 } else { 0.1 },
            );
        }
    }

    checkbox::Style {
        background: Background::Color(background_color),
        icon_color,
        border: Border {
            color: border_color,
            width: 1.0,
            radius: iced::border::Radius::from(6.0),
        },
        text_color,
    }
}

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

/// Utility function to mix two colors
pub fn mix_colors(a: Color, b: Color, factor: f32) -> Color {
    let t = factor.clamp(0.0, 1.0);

    Color {
        r: a.r + (b.r - a.r) * t,
        g: a.g + (b.g - a.g) * t,
        b: a.b + (b.b - a.b) * t,
        a: a.a + (b.a - a.a) * t,
    }
}
