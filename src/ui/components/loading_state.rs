use iced::widget::{column, container, text};
use iced::{Alignment, Element, Length};

use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

pub struct LoadingState;

impl LoadingState {
    pub fn build<'a, Message: 'a>(label: &'a str, theme: &'a Theme) -> Element<'a, Message> {
        let colors = theme.colors;

        let brand_badge = text("🔥 BLAZE UNIVERSE")
            .size(TypographyTokens::CAPTION_SIZE)
            .color(colors.accent);

        let main_title = text("Sisteminiz Blaze Evreninde İnceleniyor...")
            .size(TypographyTokens::HEADING_SIZE)
            .color(colors.text_primary);

        let sub_text = text(if label.is_empty() {
            "Donanım telemetrisi ve sistem yapılandırması analiz ediliyor."
        } else {
            label
        })
        .size(TypographyTokens::BODY_MUTED_SIZE)
        .color(colors.text_secondary);

        let col = column![brand_badge, main_title, sub_text]
            .spacing(SpacingTokens::SM)
            .align_x(Alignment::Center);

        container(col)
            .padding(SpacingTokens::XXL)
            .width(Length::Fill)
            .height(Length::Fixed(320.0))
            .align_x(Alignment::Center)
            .align_y(Alignment::Center)
            .style(move |_| container::Style {
                background: Some(colors.background.into()),
                text_color: Some(colors.text_primary),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}
