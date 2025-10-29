//! JLPT level badge component

use crate::models::JLPTLevel;
use crate::ui::section_style;
use iced::widget::{container, text};
use iced::{Border, Color, Element, Shadow};

/// Create a JLPT level badge widget
pub fn jlpt_badge<'a, Message: 'a>(level: JLPTLevel) -> Element<'a, Message> {
    let level_str = level.as_str().to_string();
    let level_color = level.color();

    container(text(level_str).size(12))
        .padding([4, 12])
        .style(move |theme: &iced::Theme| {
            let mut style = section_style(theme);
            style.background = Some(level_color.into());
            style.text_color = Some(Color::WHITE);
            style.border = Border {
                radius: iced::border::Radius::from(12.0),
                ..Default::default()
            };
            style.shadow = Shadow::default();
            style
        })
        .into()
}
