use iced::widget::{column, container, text, Column, Container};
use iced::{Alignment, Length};

use crate::ui::theme::Theme;
use crate::ui::tokens::{RadiusTokens, SpacingTokens, TypographyTokens};

pub struct MetricCard;

impl MetricCard {
    pub fn build<'a, Message: 'a>(
        title: &'a str,
        value: &'a str,
        subtitle: &'a str,
        is_highlighted: bool,
        theme: &'a Theme,
    ) -> Container<'a, Message> {
        let colors = theme.colors;

        let title_text = text(title)
            .size(TypographyTokens::CAPTION_SIZE)
            .color(colors.text_secondary);

        let value_text = text(value)
            .size(TypographyTokens::DISPLAY_SIZE)
            .font(iced::Font::MONOSPACE)
            .color(if is_highlighted {
                colors.accent
            } else {
                colors.text_primary
            });

        let sub_text = text(subtitle)
            .size(TypographyTokens::BODY_MUTED_SIZE)
            .color(colors.text_secondary);

        let col: Column<'a, Message> = column![title_text, value_text, sub_text]
            .spacing(SpacingTokens::XS)
            .align_x(Alignment::Start);

        container(col)
            .padding(SpacingTokens::LG)
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(colors.surface_elevated.into()),
                text_color: Some(colors.text_primary),
                border: iced::Border {
                    color: if is_highlighted {
                        colors.border_focus
                    } else {
                        colors.border
                    },
                    width: 1.0,
                    radius: RadiusTokens::LG.into(),
                },
                shadow: iced::Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.16),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
            })
    }
}
