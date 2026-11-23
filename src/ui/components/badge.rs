use iced::widget::{container, text, Container};
use iced::{Color, Element};

use crate::ui::theme::Theme;
use crate::ui::tokens::{RadiusTokens, SpacingTokens, TypographyTokens};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BadgeStyle {
    Success,
    Warning,
    Danger,
    Info,
    Neutral,
    Accent,
}

pub struct Badge;

impl Badge {
    pub fn build<'a, Message: 'a>(
        label: &'a str,
        style_type: BadgeStyle,
        theme: &'a Theme,
    ) -> Element<'a, Message> {
        let colors = theme.colors;

        let (bg, fg) = match style_type {
            BadgeStyle::Success => (colors.success, colors.background),
            BadgeStyle::Warning => (colors.warning, colors.background),
            BadgeStyle::Danger => (colors.danger, colors.accent_on),
            BadgeStyle::Info => (colors.info, colors.background),
            BadgeStyle::Neutral => (colors.surface_elevated, colors.text_secondary),
            BadgeStyle::Accent => (colors.accent, colors.accent_on),
        };

        let txt = text(label).size(TypographyTokens::CAPTION_SIZE).color(fg);

        let cont: Container<'a, Message> = container(txt)
            .padding([SpacingTokens::XS, SpacingTokens::SM])
            .style(move |_| container::Style {
                background: Some(bg.into()),
                text_color: Some(fg),
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: RadiusTokens::FULL.into(),
                },
                shadow: iced::Shadow::default(),
            });

        cont.into()
    }
}
