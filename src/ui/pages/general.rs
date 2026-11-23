use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::system::{OverviewData, SystemService};
use crate::ui::components::badge::{Badge, BadgeStyle};
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::components::metric_card::MetricCard;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

#[derive(Debug, Clone)]
pub enum GeneralMessage {
    LoadOverview,
    OverviewLoaded(Result<Box<OverviewData>, BlazeError>),
}

#[derive(Debug, Clone)]
pub enum GeneralViewState {
    Loading,
    Loaded(Box<OverviewData>),
    Error(BlazeError),
}

pub struct GeneralPage {
    pub state: GeneralViewState,
    pub is_refreshing: bool,
}

impl GeneralPage {
    pub fn new() -> (Self, Task<GeneralMessage>) {
        let page = Self {
            state: GeneralViewState::Loading,
            is_refreshing: true,
        };
        let task = Task::perform(
            async move {
                let service = SystemService::new();
                service.get_overview_data().map(Box::new)
            },
            GeneralMessage::OverviewLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: GeneralMessage) -> Task<GeneralMessage> {
        match message {
            GeneralMessage::LoadOverview => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = SystemService::new();
                        service.get_overview_data().map(Box::new)
                    },
                    GeneralMessage::OverviewLoaded,
                )
            }
            GeneralMessage::OverviewLoaded(res) => {
                self.is_refreshing = false;
                match res {
                    Ok(data) => {
                        self.state = GeneralViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = GeneralViewState::Error(e);
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, GeneralMessage> {
        match self.state {
            GeneralViewState::Loading => {
                LoadingState::build("Sistem genel durumu okunuyor...", theme)
            }
            GeneralViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(GeneralMessage::LoadOverview),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            GeneralViewState::Loaded(ref data) => {
                let sub_title = text("BlazeOS Kontrol Merkezi Telemetri ve Sistem Durumu")
                    .size(TypographyTokens::CAPTION_SIZE)
                    .color(theme.colors.text_secondary);

                let refresh_btn = PillButton::build(
                    if self.is_refreshing {
                        "⏳ Yenileniyor..."
                    } else {
                        "🔄 Yenile"
                    },
                    ButtonVariant::Tonal,
                    if self.is_refreshing {
                        None
                    } else {
                        Some(GeneralMessage::LoadOverview)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                // 1. Hero System Identity Card
                let os_title_text = text(&data.os_name)
                    .size(TypographyTokens::DISPLAY_SIZE)
                    .color(theme.colors.accent);

                let edition_text = text(&data.edition_line)
                    .size(TypographyTokens::PAGE_TITLE_SIZE)
                    .color(theme.colors.text_primary);

                let sub_meta = text(format!(
                    "Kernel: {}  |  Arch: {}  |  Host: {}",
                    data.kernel, data.arch, data.hostname
                ))
                .size(TypographyTokens::CAPTION_SIZE)
                .font(iced::Font::MONOSPACE)
                .color(theme.colors.text_secondary);

                let status_badge = Badge::build("SYSTEM HEALTHY", BadgeStyle::Success, theme);

                let hero_header = row![
                    column![os_title_text, edition_text].spacing(SpacingTokens::XS),
                    iced::widget::horizontal_space(),
                    status_badge
                ]
                .align_y(Alignment::Center)
                .width(Length::Fill);

                let hero_col = column![hero_header, Divider::build(theme), sub_meta]
                    .spacing(SpacingTokens::MD);

                let hero_card =
                    GlassCard::build("SİSTEM BİLGİSİ VE KİMLİK", hero_col.into(), theme);

                // 2. Resource Telemetry Grid (RAM, Uptime, Storage)
                let metric_ram = MetricCard::build(
                    "SİSTEM BELLEĞİ (RAM)",
                    &data.ram_summary,
                    "Aktif Bellek Durumu",
                    true,
                    theme,
                );

                let metric_uptime = MetricCard::build(
                    "ÇALIŞMA SÜRESİ (UPTIME)",
                    &data.uptime,
                    "Kesintisiz Sistem Oturumu",
                    false,
                    theme,
                );

                let metric_storage = MetricCard::build(
                    "KÖK DEPOLAMA (/)",
                    &data.storage_summary,
                    "Boş / Toplam Kapasite",
                    false,
                    theme,
                );

                let metric_grid = row![metric_ram, metric_uptime, metric_storage]
                    .spacing(SpacingTokens::LG)
                    .width(Length::Fill);

                // 3. Hardware & Architecture Detailed Snapshot
                let hw_card_content: Column<'a, GeneralMessage> = column![
                    InfoRow::build("İşlemci (CPU)", &data.cpu_model, false, theme),
                    Divider::build(theme),
                    InfoRow::build("İşlemci Çekirdekleri", &data.cpu_cores, true, theme),
                    Divider::build(theme),
                    InfoRow::build("Grafik Kartı (GPU)", &data.gpu_summary, false, theme),
                    Divider::build(theme),
                    InfoRow::build("Sistem Mimarisi", &data.arch, true, theme),
                ]
                .spacing(SpacingTokens::SM);

                let hw_card =
                    GlassCard::build("DONANIM VE MİMARİ DETAYLARI", hw_card_content.into(), theme);

                let page_col: Column<'a, GeneralMessage> =
                    column![top_bar, hero_card, metric_grid, hw_card]
                        .spacing(SpacingTokens::XL)
                        .width(Length::Fill);

                page_col.into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_general_page_lifecycle() {
        let (mut page, _task) = GeneralPage::new();
        assert!(matches!(page.state, GeneralViewState::Loading));
        assert!(page.is_refreshing);

        let service = SystemService::new();
        let data = service.get_overview_data().unwrap();
        let _ = page.update(GeneralMessage::OverviewLoaded(Ok(Box::new(data.clone()))));

        assert!(!page.is_refreshing);
        if let GeneralViewState::Loaded(ref d) = page.state {
            assert_eq!(d.kernel, data.kernel);
        } else {
            panic!("Expected GeneralViewState::Loaded");
        }
    }

    #[test]
    fn test_general_page_error_and_refresh() {
        let (mut page, _task) = GeneralPage::new();
        let err = BlazeError::ExecutionError {
            command: "test".to_string(),
            stdout: "".to_string(),
            stderr: "Failed".to_string(),
        };
        let _ = page.update(GeneralMessage::OverviewLoaded(Err(err)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, GeneralViewState::Error(_)));

        let _ = page.update(GeneralMessage::LoadOverview);
        assert!(page.is_refreshing);
    }
}
