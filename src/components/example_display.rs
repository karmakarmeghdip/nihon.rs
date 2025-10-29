//! Example sentence display component

use crate::models::ExampleSentence;
use crate::ui::section_style;
use iced::widget::{column, container, text};
use iced::{Element, Length};

/// Display a list of example sentences in a styled container
pub fn example_sentences<'a, Message: 'a>(
    examples: &'a [ExampleSentence],
) -> Element<'a, Message> {
    if examples.is_empty() {
        return text("No examples available").size(12).into();
    }

    let examples_list = examples.iter().fold(
        column![].spacing(15),
        |col, example| {
            col.push(
                container(
                    column![
                        text(&example.japanese).size(14),
                        text(&example.english).size(12),
                    ]
                    .spacing(5),
                )
                .padding(10)
                .width(Length::Fill)
                .style(|theme: &iced::Theme| {
                    let palette = theme.extended_palette();
                    let mut style = section_style(theme);
                    style.background = Some(palette.background.weak.color.into());
                    style
                }),
            )
        },
    );

    examples_list.into()
}
