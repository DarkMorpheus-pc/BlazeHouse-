use iced::widget::{button, text, Button};
use iced::Element;

use crate::ui::theme::Theme;
use crate::ui::tokens::{RadiusTokens, SpacingTokens, TypographyTokens};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ButtonVariant {
    Primary,
    Tonal,
    Danger,
    Ghost,
}

pub struct PillButton;

impl PillButton {
    pub fn build<'a, Message: 'a + Clone>(
        label: &'a str,
        variant: ButtonVariant,
        on_press: Option<Message>,
        theme: &'a Theme,
    ) -> Element<'a, Message> {
        let colors = theme.colors;

        let content = text(label)
            .size(TypographyTokens::BODY_PRIMARY_SIZE)
            .color(match variant {
                ButtonVariant::Primary => colors.accent_on,
                ButtonVariant::Tonal => colors.text_primary,
                ButtonVariant::Danger => colors.accent_on,
                ButtonVariant::Ghost => colors.text_secondary,
            });

        let mut btn: Button<'a, Message> = button(content)
            .padding([SpacingTokens::SM, SpacingTokens::LG])
            .style(move |_, status| {
                let bg = match status {
                    button::Status::Hovered | button::Status::Pressed => match variant {
                        ButtonVariant::Primary => colors.accent_hover,
                        ButtonVariant::Tonal => colors.surface_hover,
                        ButtonVariant::Danger => colors.danger,
                        ButtonVariant::Ghost => colors.surface_hover,
                    },
                    button::Status::Disabled => colors.surface,
                    _ => match variant {
                        ButtonVariant::Primary => colors.accent,
                        ButtonVariant::Tonal => colors.surface_elevated,
                        ButtonVariant::Danger => colors.danger,
                        ButtonVariant::Ghost => iced::Color::TRANSPARENT,
                    },
                };

                let border_color = match variant {
                    ButtonVariant::Ghost => iced::Color::TRANSPARENT,
                    _ => match status {
                        button::Status::Hovered | button::Status::Pressed => colors.border_focus,
                        _ => colors.border,
                    },
                };

                button::Style {
                    background: Some(bg.into()),
                    text_color: colors.text_primary,
                    border: iced::Border {
                        color: border_color,
                        width: 1.0,
                        radius: RadiusTokens::FULL.into(),
                    },
                    shadow: iced::Shadow::default(),
                }
            });

        if let Some(msg) = on_press {
            btn = btn.on_press(msg);
        }

        btn.into()
    }
}
