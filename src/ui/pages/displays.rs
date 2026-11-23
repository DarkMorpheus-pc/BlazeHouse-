use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::display::DisplayService;
use crate::app::error::BlazeError;
use crate::displays::DisplayOutput;
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

#[derive(Debug, Clone)]
pub struct DisplaysData {
    pub displays: Vec<DisplayOutput>,
}

#[derive(Debug, Clone)]
pub enum DisplaysMessage {
    LoadDisplays,
    DisplaysLoaded(Result<Box<DisplaysData>, BlazeError>),
    SetMode(String, String),
    SetScale(String, f32),
    SetEnabled(String, bool),
    ActionFinished(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum DisplaysViewState {
    Loading,
    Loaded(Box<DisplaysData>),
    Error(BlazeError),
}

pub struct DisplaysPage {
    pub state: DisplaysViewState,
    pub is_refreshing: bool,
    pub is_acting: bool,
    pub active_output: Option<String>,
}

impl DisplaysPage {
    pub fn new() -> (Self, Task<DisplaysMessage>) {
        let page = Self {
            state: DisplaysViewState::Loading,
            is_refreshing: true,
            is_acting: false,
            active_output: None,
        };
        let task = Task::perform(
            async move {
                let service = DisplayService::new();
                let displays = service.get_displays()?;
                Ok(Box::new(DisplaysData { displays }))
            },
            DisplaysMessage::DisplaysLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: DisplaysMessage) -> Task<DisplaysMessage> {
        match message {
            DisplaysMessage::LoadDisplays => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = DisplayService::new();
                        let displays = service.get_displays()?;
                        Ok(Box::new(DisplaysData { displays }))
                    },
                    DisplaysMessage::DisplaysLoaded,
                )
            }
            DisplaysMessage::DisplaysLoaded(res) => {
                self.is_refreshing = false;
                self.is_acting = false;
                self.active_output = None;
                match res {
                    Ok(data) => {
                        self.state = DisplaysViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = DisplaysViewState::Error(e);
                    }
                }
                Task::none()
            }
            DisplaysMessage::SetMode(output, mode) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                self.active_output = Some(output.clone());
                let out_target = output.clone();
                let mode_target = mode.clone();
                Task::perform(
                    async move {
                        let service = DisplayService::new();
                        service.set_mode(&out_target, &mode_target)
                    },
                    DisplaysMessage::ActionFinished,
                )
            }
            DisplaysMessage::SetScale(output, scale) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                self.active_output = Some(output.clone());
                let out_target = output.clone();
                Task::perform(
                    async move {
                        let service = DisplayService::new();
                        service.set_scale(&out_target, scale)
                    },
                    DisplaysMessage::ActionFinished,
                )
            }
            DisplaysMessage::SetEnabled(output, enabled) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                self.active_output = Some(output.clone());
                let out_target = output.clone();
                Task::perform(
                    async move {
                        let service = DisplayService::new();
                        service.set_enabled(&out_target, enabled)
                    },
                    DisplaysMessage::ActionFinished,
                )
            }
            DisplaysMessage::ActionFinished(res) => {
                self.is_acting = false;
                self.active_output = None;
                match res {
                    Ok(()) => {
                        // Re-read display outputs to verify actual Linux randr state
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = DisplayService::new();
                                let displays = service.get_displays()?;
                                Ok(Box::new(DisplaysData { displays }))
                            },
                            DisplaysMessage::DisplaysLoaded,
                        )
                    }
                    Err(e) => {
                        self.state = DisplaysViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, DisplaysMessage> {
        match self.state {
            DisplaysViewState::Loading => LoadingState::build(
                "Bağlı ekranlar ve kullanılabilir çözünürlükler taranıyor...",
                theme,
            ),
            DisplaysViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(DisplaysMessage::LoadDisplays),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            DisplaysViewState::Loaded(ref data) => {
                let sub_title =
                    text("Bağlı Ekran Adaptörleri, Çözünürlük ve Yenileme Hızı Yapılandırması")
                        .size(TypographyTokens::CAPTION_SIZE)
                        .color(theme.colors.text_secondary);

                let refresh_btn = PillButton::build(
                    if self.is_refreshing {
                        "⏳ Yenileniyor..."
                    } else {
                        "🔄 Yenile"
                    },
                    ButtonVariant::Tonal,
                    if self.is_refreshing || self.is_acting {
                        None
                    } else {
                        Some(DisplaysMessage::LoadDisplays)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, DisplaysMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                if data.displays.is_empty() {
                    let empty_card = GlassCard::build(
                        "BAĞLI EKRAN BULUNAMADI",
                        InfoRow::build(
                            "Ekran Adaptörü",
                            "Sistemde bağlı aktif ekran çıktısı (Display output) tespit edilemedi",
                            false,
                            theme,
                        ),
                        theme,
                    );
                    page_col = page_col.push(empty_card);
                } else {
                    for disp in data.displays.iter() {
                        let name_clone = disp.name.clone();

                        let current_mode_str = if disp.current.is_empty() {
                            "Bilinmiyor / Belirtilmedi".to_string()
                        } else {
                            disp.current.clone()
                        };

                        let scale_str = format!(
                            "{:.2}x (%{})",
                            disp.scale,
                            (disp.scale * 100.0).round() as u32
                        );
                        let pos_str = format!("X: {}, Y: {}", disp.position.0, disp.position.1);

                        let mut card_col: Column<'a, DisplaysMessage> = column![
                            InfoRow::build(
                                "Ekran Bağlantı Noktası",
                                &disp.name,
                                disp.primary,
                                theme
                            ),
                            Divider::build(theme),
                            InfoRow::build(
                                "Mevcut Çözünürlük & Hız",
                                &current_mode_str,
                                true,
                                theme
                            ),
                            Divider::build(theme),
                            InfoRow::build("Ekran Ölçeği (Scale)", &scale_str, true, theme),
                            Divider::build(theme),
                            InfoRow::build("Konum Koordinatları", &pos_str, false, theme),
                        ]
                        .spacing(SpacingTokens::SM);

                        // Modes presets selection
                        if !disp.modes.is_empty() {
                            card_col = card_col.push(Divider::build(theme));

                            let mut modes_row = row![text("Çözünürlük Modu:")
                                .size(TypographyTokens::BODY_PRIMARY_SIZE)
                                .color(theme.colors.text_primary)]
                            .spacing(SpacingTokens::MD)
                            .align_y(Alignment::Center);

                            for mode_str in disp.modes.iter().take(4) {
                                let is_sel = &disp.current == mode_str;
                                let m_target = mode_str.clone();
                                let out_target = name_clone.clone();

                                let btn = PillButton::build(
                                    mode_str,
                                    if is_sel {
                                        ButtonVariant::Primary
                                    } else {
                                        ButtonVariant::Tonal
                                    },
                                    if self.is_acting || is_sel {
                                        None
                                    } else {
                                        Some(DisplaysMessage::SetMode(out_target, m_target))
                                    },
                                    theme,
                                );
                                modes_row = modes_row.push(btn);
                            }

                            card_col = card_col.push(modes_row);
                        }

                        // Scale selection
                        card_col = card_col.push(Divider::build(theme));
                        let mut scale_row = row![text("Ölçeklendirme:")
                            .size(TypographyTokens::BODY_PRIMARY_SIZE)
                            .color(theme.colors.text_primary)]
                        .spacing(SpacingTokens::MD)
                        .align_y(Alignment::Center);

                        let scales: [(f32, &str); 4] = [
                            (1.0, "%100 (1.0x)"),
                            (1.25, "%125 (1.25x)"),
                            (1.5, "%150 (1.5x)"),
                            (2.0, "%200 (2.0x)"),
                        ];

                        for (sc, label) in scales {
                            let is_sel = (disp.scale - sc).abs() < 0.05;
                            let out_target = name_clone.clone();

                            let btn = PillButton::build(
                                label,
                                if is_sel {
                                    ButtonVariant::Primary
                                } else {
                                    ButtonVariant::Tonal
                                },
                                if self.is_acting || is_sel {
                                    None
                                } else {
                                    Some(DisplaysMessage::SetScale(out_target, sc))
                                },
                                theme,
                            );
                            scale_row = scale_row.push(btn);
                        }

                        card_col = card_col.push(scale_row);

                        page_col =
                            page_col.push(GlassCard::build(&disp.name, card_col.into(), theme));
                    }
                }

                page_col.into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_displays_page_lifecycle() {
        let (mut page, _task) = DisplaysPage::new();
        assert!(matches!(page.state, DisplaysViewState::Loading));

        let service = DisplayService::new();
        let displays = service.get_displays().unwrap_or_default();
        let data = Box::new(DisplaysData { displays });

        let _ = page.update(DisplaysMessage::DisplaysLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, DisplaysViewState::Loaded(_)));
    }
}
