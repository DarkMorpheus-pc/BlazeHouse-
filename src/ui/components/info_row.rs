use iced::widget::{row, text};
use iced::{Alignment, Element, Length};

use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

pub struct InfoRow;

impl InfoRow {
    pub fn build<'a, Message: 'a>(
        label: &str,
        value: &str,
        is_monospace: bool,
        theme: &'a Theme,
    ) -> Element<'a, Message> {
        let label_text = text(label.to_string())
            .size(TypographyTokens::BODY_PRIMARY_SIZE)
            .color(theme.colors.text_primary);

        let mut val_text = text(value.to_string())
            .size(if is_monospace {
                TypographyTokens::TECH_VAL_SIZE
            } else {
                TypographyTokens::BODY_PRIMARY_SIZE
            })
            .color(theme.colors.text_secondary);

        if is_monospace {
            val_text = val_text.font(iced::Font::MONOSPACE);
        }

        let r = row![label_text, iced::widget::horizontal_space(), val_text]
            .spacing(SpacingTokens::MD)
            .align_y(Alignment::Center)
            .width(Length::Fill);

        r.into()
    }
}
