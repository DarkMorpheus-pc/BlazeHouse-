use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::system::{SystemDetailsData, SystemService};
use crate::ui::components::badge::{Badge, BadgeStyle};
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

#[derive(Debug, Clone)]
pub enum SystemDetailsMessage {
    LoadDetails,
    DetailsLoaded(Result<Box<SystemDetailsData>, BlazeError>),
}

#[derive(Debug, Clone)]
pub enum SystemDetailsViewState {
    Loading,
    Loaded(Box<SystemDetailsData>),
    Error(BlazeError),
}

pub struct SystemDetailsPage {
    pub state: SystemDetailsViewState,
    pub is_refreshing: bool,
}

impl SystemDetailsPage {
    pub fn new() -> (Self, Task<SystemDetailsMessage>) {
        let page = Self {
            state: SystemDetailsViewState::Loading,
            is_refreshing: true,
        };
        let task = Task::perform(
            async move {
                let service = SystemService::new();
                service.get_system_details_data().map(Box::new)
            },
            SystemDetailsMessage::DetailsLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: SystemDetailsMessage) -> Task<SystemDetailsMessage> {
        match message {
            SystemDetailsMessage::LoadDetails => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = SystemService::new();
                        service.get_system_details_data().map(Box::new)
                    },
                    SystemDetailsMessage::DetailsLoaded,
                )
            }
            SystemDetailsMessage::DetailsLoaded(res) => {
                self.is_refreshing = false;
                match res {
                    Ok(data) => {
                        self.state = SystemDetailsViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = SystemDetailsViewState::Error(e);
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, SystemDetailsMessage> {
        match self.state {
            SystemDetailsViewState::Loading => {
                LoadingState::build("Sistem detayları ve çekirdek verileri taranıyor...", theme)
            }
            SystemDetailsViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(SystemDetailsMessage::LoadDetails),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            SystemDetailsViewState::Loaded(ref data) => {
                let sub_title =
                    text("Teknik İşletim Sistemi Parametreleri, Çekirdek ve Oturum Envanteri")
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
                        Some(SystemDetailsMessage::LoadDetails)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                // Section 1: Operating System
                let os_badge = Badge::build("OFFICIAL RELEASE", BadgeStyle::Accent, theme);
                let os_card_header = row![
                    InfoRow::build("İşletim Sistemi Adı", &data.os_name, false, theme),
                    iced::widget::horizontal_space(),
                    os_badge
                ]
                .align_y(Alignment::Center)
                .width(Length::Fill);

                let card1_content: Column<'a, SystemDetailsMessage> = column![
                    os_card_header,
                    Divider::build(theme),
                    InfoRow::build("Sürüm / Edisyon Tanımı", &data.edition_line, false, theme),
                    Divider::build(theme),
                    InfoRow::build(
                        "Versiyon Kimliği (VERSION_ID)",
                        &data.version_id,
                        true,
                        theme
                    ),
                ]
                .spacing(SpacingTokens::SM);

                let card1 = GlassCard::build_with_subtitle(
                    "İŞLETİM SİSTEMİ VE DAĞITIM",
                    "Resmi Dağıtım Tanımı, Edisyon ve Sürüm Kimlikleri",
                    card1_content.into(),
                    theme,
                );

                // Section 2: Kernel & Architecture
                let card2_content: Column<'a, SystemDetailsMessage> = column![
                    InfoRow::build("Çekirdek (Kernel) Sürümü", &data.kernel, true, theme),
                    Divider::build(theme),
                    InfoRow::build("İşlemci / Donanım Mimarisi", &data.arch, true, theme),
                    Divider::build(theme),
                    InfoRow::build("Çalışma Süresi (Uptime)", &data.uptime, false, theme),
                ]
                .spacing(SpacingTokens::SM);

                let card2 = GlassCard::build_with_subtitle(
                    "ÇEKİRDEK VE MİMARİ TELEMETRİSİ",
                    "Linux Kernel Derlemesi, CPU Mimarisi ve Oturum Süresi",
                    card2_content.into(),
                    theme,
                );

                // Section 3: Session & Desktop Environment
                let card3_content: Column<'a, SystemDetailsMessage> = column![
                    InfoRow::build("Makine Adı (Hostname)", &data.hostname, true, theme),
                    Divider::build(theme),
                    InfoRow::build("Masaüstü Ortamı (DE)", &data.desktop_env, false, theme),
                    Divider::build(theme),
                    InfoRow::build(
                        "Görüntü Sunucusu / Oturum Türü",
                        &data.session_type,
                        true,
                        theme
                    ),
                ]
                .spacing(SpacingTokens::SM);

                let card3 = GlassCard::build_with_subtitle(
                    "OTURUM VE MASAÜSTÜ ÇEVRESİ",
                    "Ağ Makine Tanımlayıcısı, Pencere Yöneticisi ve Görüntü Sunucusu",
                    card3_content.into(),
                    theme,
                );

                let page_col: Column<'a, SystemDetailsMessage> =
                    column![top_bar, card1, card2, card3]
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
    fn test_system_details_page_lifecycle() {
        let (mut page, _task) = SystemDetailsPage::new();
        assert!(matches!(page.state, SystemDetailsViewState::Loading));

        let service = SystemService::new();
        let data = service.get_system_details_data().unwrap();
        let _ = page.update(SystemDetailsMessage::DetailsLoaded(Ok(Box::new(
            data.clone(),
        ))));

        assert!(!page.is_refreshing);
        if let SystemDetailsViewState::Loaded(ref d) = page.state {
            assert_eq!(d.kernel, data.kernel);
            assert_eq!(d.hostname, data.hostname);
        } else {
            panic!("Expected SystemDetailsViewState::Loaded");
        }
    }

    #[test]
    fn test_system_details_page_error_handling() {
        let (mut page, _task) = SystemDetailsPage::new();
        let err = BlazeError::ExecutionError {
            command: "test".to_string(),
            stdout: "".to_string(),
            stderr: "Failed".to_string(),
        };
        let _ = page.update(SystemDetailsMessage::DetailsLoaded(Err(err)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, SystemDetailsViewState::Error(_)));
    }
}
