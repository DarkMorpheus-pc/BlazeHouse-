use iced::widget::{column, container, text, Column, Container};
use iced::{Element, Length};

use crate::ui::theme::Theme;
use crate::ui::tokens::{RadiusTokens, SpacingTokens, TypographyTokens};

pub struct GlassCard;

impl GlassCard {
    pub fn build<'a, Message: 'a>(
        title: &'a str,
        content: Element<'a, Message>,
        theme: &'a Theme,
    ) -> Container<'a, Message> {
        let colors = theme.colors;

        let title_text = text(title)
            .size(TypographyTokens::HEADING_SIZE)
            .color(colors.text_primary);

        let col: Column<'a, Message> = column![title_text, content].spacing(SpacingTokens::MD);

        container(col)
            .padding(SpacingTokens::LG)
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(colors.surface.into()),
                text_color: Some(colors.text_primary),
                border: iced::Border {
                    color: colors.border,
                    width: 1.0,
                    radius: RadiusTokens::LG.into(),
                },
                shadow: iced::Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.20),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
            })
    }

    pub fn build_with_subtitle<'a, Message: 'a>(
        title: &'a str,
        subtitle: &'a str,
        content: Element<'a, Message>,
        theme: &'a Theme,
    ) -> Container<'a, Message> {
        let colors = theme.colors;

        let title_text = text(title)
            .size(TypographyTokens::HEADING_SIZE)
            .color(colors.text_primary);

        let sub_text = text(subtitle)
            .size(TypographyTokens::CAPTION_SIZE)
            .color(colors.text_secondary);

        let header_col = column![title_text, sub_text].spacing(SpacingTokens::XS);

        let col: Column<'a, Message> = column![header_col, content].spacing(SpacingTokens::MD);

        container(col)
            .padding(SpacingTokens::LG)
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(colors.surface.into()),
                text_color: Some(colors.text_primary),
                border: iced::Border {
                    color: colors.border,
                    width: 1.0,
                    radius: RadiusTokens::LG.into(),
                },
                shadow: iced::Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.20),
                    offset: iced::Vector::new(0.0, 4.0),
                    blur_radius: 16.0,
                },
            })
    }
}
