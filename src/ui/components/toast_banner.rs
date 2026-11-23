use iced::widget::{container, row, text};
use iced::{Alignment, Element, Length};

use crate::ui::theme::Theme;
use crate::ui::tokens::{RadiusTokens, SpacingTokens, TypographyTokens};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BannerLevel {
    Info,
    Success,
    Warning,
    Danger,
}

pub struct ToastBanner;

impl ToastBanner {
    pub fn build<'a, Message: 'a>(
        message: &'a str,
        level: BannerLevel,
        theme: &'a Theme,
    ) -> Element<'a, Message> {
        let colors = theme.colors;

        let (bg, fg) = match level {
            BannerLevel::Info => (colors.info, colors.text_primary),
            BannerLevel::Success => (colors.success, colors.accent_on),
            BannerLevel::Warning => (colors.warning, colors.accent_on),
            BannerLevel::Danger => (colors.danger, colors.accent_on),
        };

        let msg_text = text(message)
            .size(TypographyTokens::BODY_PRIMARY_SIZE)
            .color(fg);

        let content = row![msg_text]
            .spacing(SpacingTokens::MD)
            .align_y(Alignment::Center);

        container(content)
            .padding([SpacingTokens::SM, SpacingTokens::LG])
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(bg.into()),
                text_color: Some(fg),
                border: iced::Border {
                    color: colors.border,
                    width: 1.0,
                    radius: RadiusTokens::MD.into(),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}
