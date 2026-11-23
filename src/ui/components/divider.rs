use iced::widget::container;
use iced::{Element, Length};

use crate::ui::theme::Theme;

pub struct Divider;

impl Divider {
    pub fn build<'a, Message: 'a>(theme: &'a Theme) -> Element<'a, Message> {
        let border_color = theme.colors.border;

        container(iced::widget::text(""))
            .width(Length::Fill)
            .height(1.0)
            .style(move |_| container::Style {
                background: Some(border_color.into()),
                text_color: None,
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}
