use iced::widget::{column, container, scrollable, Column};
use iced::{Element, Length};

use crate::ui::theme::Theme;
use crate::ui::tokens::SpacingTokens;

pub struct PageContainer;

impl PageContainer {
    pub fn build<'a, Message: 'a + Clone>(
        content: Element<'a, Message>,
        theme: &'a Theme,
    ) -> Element<'a, Message> {
        let colors = theme.colors;

        let col: Column<'a, Message> = column![content]
            .spacing(SpacingTokens::XL)
            .padding(SpacingTokens::XXL);

        let scroll = scrollable(col).width(Length::Fill).height(Length::Fill);

        container(scroll)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(colors.background.into()),
                text_color: Some(colors.text_primary),
                border: iced::Border::default(),
                shadow: iced::Shadow::default(),
            })
            .into()
    }
}
