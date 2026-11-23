use iced::widget::{container, row, text, Container};
use iced::{Alignment, Length};

use crate::ui::components::badge::{Badge, BadgeStyle};
use crate::ui::route::Route;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

pub struct Header;

impl Header {
    pub fn build<'a, Message: 'a + Clone>(
        current_route: Route,
        theme: &'a Theme,
    ) -> Container<'a, Message> {
        let colors = theme.colors;

        let title = text(current_route.title())
            .size(TypographyTokens::PAGE_TITLE_SIZE)
            .color(colors.text_primary);

        let category_sub = text(current_route.category().title())
            .size(TypographyTokens::CAPTION_SIZE)
            .color(colors.text_secondary);

        let left_col = iced::widget::column![title, category_sub].spacing(SpacingTokens::XS);

        let system_badge = Badge::build("SYSTEM ACTIVE", BadgeStyle::Success, theme);

        let app_brand = text("BLAZE HOUSE 2.0")
            .size(TypographyTokens::CAPTION_SIZE)
            .color(colors.accent);

        let right_row = row![system_badge, app_brand]
            .spacing(SpacingTokens::MD)
            .align_y(Alignment::Center);

        let r = row![left_col, iced::widget::horizontal_space(), right_row]
            .align_y(Alignment::Center)
            .width(Length::Fill);

        container(r)
            .padding([SpacingTokens::MD, SpacingTokens::XL])
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(colors.surface.into()),
                text_color: Some(colors.text_primary),
                border: iced::Border {
                    color: colors.border,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                shadow: iced::Shadow {
                    color: iced::Color::from_rgba(0.0, 0.0, 0.0, 0.12),
                    offset: iced::Vector::new(0.0, 2.0),
                    blur_radius: 8.0,
                },
            })
    }
}
