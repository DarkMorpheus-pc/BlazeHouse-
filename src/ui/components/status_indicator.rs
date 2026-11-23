use iced::widget::{container, row, text};
use iced::{Alignment, Color, Element};

use crate::ui::theme::Theme;
use crate::ui::tokens::{RadiusTokens, SpacingTokens, TypographyTokens};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StatusType {
    Success,
    Warning,
    Danger,
    Info,
    Neutral,
}

pub struct StatusIndicator;

impl StatusIndicator {
    pub fn build<'a, Message: 'a>(
        label: &'a str,
        status: StatusType,
        theme: &'a Theme,
    ) -> Element<'a, Message> {
        let colors = theme.colors;

        let color = match status {
            StatusType::Success => colors.success,
            StatusType::Warning => colors.warning,
            StatusType::Danger => colors.danger,
            StatusType::Info => colors.info,
            StatusType::Neutral => colors.text_secondary,
        };

        let dot = container(text(""))
            .width(8.0)
            .height(8.0)
            .style(move |_| container::Style {
                background: Some(color.into()),
                text_color: None,
                border: iced::Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: RadiusTokens::FULL.into(),
                },
                shadow: iced::Shadow::default(),
            });

        let label_text = text(label)
            .size(TypographyTokens::BODY_MUTED_SIZE)
            .color(colors.text_primary);

        row![dot, label_text]
            .spacing(SpacingTokens::SM)
            .align_y(Alignment::Center)
            .into()
    }
}
