use iced::widget::{button, row, text, Button};
use iced::{Alignment, Element, Length};

use crate::ui::route::Route;
use crate::ui::theme::Theme;
use crate::ui::tokens::{RadiusTokens, SpacingTokens, TypographyTokens};

pub struct NavItem;

impl NavItem {
    pub fn build<'a, Message: 'a + Clone>(
        route: Route,
        is_selected: bool,
        is_collapsed: bool,
        on_select: Message,
        theme: &'a Theme,
    ) -> Element<'a, Message> {
        let colors = theme.colors;

        let icon_text = text(route.icon_name())
            .size(TypographyTokens::HEADING_SIZE)
            .color(if is_selected {
                colors.accent_on
            } else {
                colors.text_secondary
            });

        let content: Element<'a, Message> = if is_collapsed {
            iced::widget::container(icon_text)
                .width(Length::Fill)
                .align_x(iced::alignment::Horizontal::Center)
                .into()
        } else {
            let label_text = text(route.title())
                .size(TypographyTokens::BODY_PRIMARY_SIZE)
                .color(if is_selected {
                    colors.accent_on
                } else {
                    colors.text_primary
                });

            row![icon_text, label_text]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center)
                .into()
        };

        let btn: Button<'a, Message> = button(content)
            .padding([SpacingTokens::SM, SpacingTokens::MD])
            .width(Length::Fill)
            .on_press(on_select)
            .style(move |_, status| {
                let bg = if is_selected {
                    colors.accent
                } else {
                    match status {
                        button::Status::Hovered | button::Status::Pressed => colors.surface_hover,
                        _ => iced::Color::TRANSPARENT,
                    }
                };

                button::Style {
                    background: Some(bg.into()),
                    text_color: colors.text_primary,
                    border: iced::Border {
                        color: if is_selected {
                            colors.border_focus
                        } else {
                            iced::Color::TRANSPARENT
                        },
                        width: 1.0,
                        radius: RadiusTokens::MD.into(),
                    },
                    shadow: iced::Shadow::default(),
                }
            });

        btn.into()
    }
}
