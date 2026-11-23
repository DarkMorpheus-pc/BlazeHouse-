use iced::widget::{column, container, text};
use iced::{Alignment, Element, Length};

use crate::app::error::BlazeError;
use crate::ui::theme::Theme;
use crate::ui::tokens::{RadiusTokens, SpacingTokens, TypographyTokens};

pub struct ErrorState;

impl ErrorState {
    pub fn build<'a, Message: 'a>(err: &'a BlazeError, theme: &'a Theme) -> Element<'a, Message> {
        let colors = theme.colors;

        let icon_text = text("⚠️")
            .size(TypographyTokens::DISPLAY_SIZE)
            .color(colors.danger);

        let err_msg = text(err.to_string())
            .size(TypographyTokens::BODY_PRIMARY_SIZE)
            .color(colors.danger);

        let hint_text = match err {
            BlazeError::CapabilityMissing { hint, .. } => text(format!("Çözüm: {}", hint))
                .size(TypographyTokens::BODY_MUTED_SIZE)
                .color(colors.text_secondary),
            _ => text("İşlem gerçekleştirilemedi. Sistem durumunu ve izinleri kontrol edin.")
                .size(TypographyTokens::BODY_MUTED_SIZE)
                .color(colors.text_secondary),
        };

        let col = column![icon_text, err_msg, hint_text]
            .spacing(SpacingTokens::MD)
            .align_x(Alignment::Center);

        container(col)
            .padding(SpacingTokens::XL)
            .width(Length::Fill)
            .style(move |_| container::Style {
                background: Some(colors.surface.into()),
                text_color: Some(colors.text_primary),
                border: iced::Border {
                    color: colors.danger,
                    width: 1.0,
                    radius: RadiusTokens::LG.into(),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}
