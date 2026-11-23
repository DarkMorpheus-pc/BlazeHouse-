use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::update::UpdateService;
use crate::system_info::SystemInfo;
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};
use crate::updater::UpdateInfo;

#[derive(Debug, Clone)]
pub struct AboutData {
    pub app_version: String,
    pub os_name: String,
    pub version_id: String,
    pub edition_line: String,
    pub kernel: String,
    pub arch: String,
    pub hostname: String,
    pub update_info: Option<UpdateInfo>,
}

#[derive(Debug, Clone)]
pub enum AboutMessage {
    LoadAbout,
    AboutLoaded(Result<Box<AboutData>, BlazeError>),
    CheckUpdates,
    UpdatesChecked(Result<UpdateInfo, BlazeError>),
}

#[derive(Debug, Clone)]
pub enum AboutViewState {
    Loading,
    Loaded(Box<AboutData>),
    Error(BlazeError),
}

pub struct AboutPage {
    pub state: AboutViewState,
    pub is_refreshing: bool,
    pub is_checking_updates: bool,
}

impl AboutPage {
    pub fn new() -> (Self, Task<AboutMessage>) {
        let page = Self {
            state: AboutViewState::Loading,
            is_refreshing: true,
            is_checking_updates: false,
        };
        let task = Task::perform(
            async move {
                let sys = SystemInfo::gather();
                let app_version = env!("CARGO_PKG_VERSION").to_string();

                Ok(Box::new(AboutData {
                    app_version,
                    os_name: sys.os_name,
                    version_id: sys.version_id,
                    edition_line: sys.edition_line,
                    kernel: sys.kernel,
                    arch: sys.arch,
                    hostname: sys.hostname,
                    update_info: None,
                }))
            },
            AboutMessage::AboutLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: AboutMessage) -> Task<AboutMessage> {
        match message {
            AboutMessage::LoadAbout => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let sys = SystemInfo::gather();
                        let app_version = env!("CARGO_PKG_VERSION").to_string();

                        Ok(Box::new(AboutData {
                            app_version,
                            os_name: sys.os_name,
                            version_id: sys.version_id,
                            edition_line: sys.edition_line,
                            kernel: sys.kernel,
                            arch: sys.arch,
                            hostname: sys.hostname,
                            update_info: None,
                        }))
                    },
                    AboutMessage::AboutLoaded,
                )
            }
            AboutMessage::AboutLoaded(res) => {
                self.is_refreshing = false;
                self.is_checking_updates = false;
                match res {
                    Ok(data) => {
                        self.state = AboutViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = AboutViewState::Error(e);
                    }
                }
                Task::none()
            }
            AboutMessage::CheckUpdates => {
                if self.is_checking_updates {
                    return Task::none();
                }
                self.is_checking_updates = true;
                let current_ver = env!("CARGO_PKG_VERSION").to_string();
                Task::perform(
                    async move {
                        let service = UpdateService::new();
                        service.check_for_updates(&current_ver).await
                    },
                    AboutMessage::UpdatesChecked,
                )
            }
            AboutMessage::UpdatesChecked(res) => {
                self.is_checking_updates = false;
                match res {
                    Ok(info) => {
                        if let AboutViewState::Loaded(ref mut data) = self.state {
                            data.update_info = Some(info);
                        }
                    }
                    Err(e) => {
                        self.state = AboutViewState::Error(e);
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, AboutMessage> {
        match self.state {
            AboutViewState::Loading => LoadingState::build(
                "Blaze House proje bilgileri ve sistem sürümü yükleniyor...",
                theme,
            ),
            AboutViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(AboutMessage::LoadAbout),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            AboutViewState::Loaded(ref data) => {
                let sub_title =
                    text("Blaze House 2.0 Sistem Kontrol Merkezi Sürüm Bilgileri ve Güncellemeler")
                        .size(TypographyTokens::CAPTION_SIZE)
                        .color(theme.colors.text_secondary);

                let refresh_btn = PillButton::build(
                    if self.is_refreshing {
                        "⏳ Yenileniyor..."
                    } else {
                        "🔄 Yenile"
                    },
                    ButtonVariant::Tonal,
                    if self.is_refreshing || self.is_checking_updates {
                        None
                    } else {
                        Some(AboutMessage::LoadAbout)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, AboutMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. Application & OS About Card
                let about_card_content: Column<'a, AboutMessage> = column![
                    InfoRow::build(
                        "Uygulama Adı",
                        "Blaze House 2.0 Control Center",
                        true,
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build(
                        "Uygulama Sürümü (Version)",
                        &format!("v{}", data.app_version),
                        true,
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build("İşletim Sistemi (OS)", &data.os_name, true, theme),
                    Divider::build(theme),
                    InfoRow::build("Dağıtım Sürümü (Release)", &data.version_id, false, theme),
                    Divider::build(theme),
                    InfoRow::build("Sistem Çekirdeği (Kernel)", &data.kernel, false, theme),
                    Divider::build(theme),
                    InfoRow::build("İşlemci Mimarisi (Arch)", &data.arch, false, theme),
                    Divider::build(theme),
                    InfoRow::build("Makine Adı (Hostname)", &data.hostname, false, theme),
                    Divider::build(theme),
                    InfoRow::build("Sürüm Tipi (Edition)", &data.edition_line, false, theme)
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "🔥 BLAZE HOUSE 2.0 UYGULAMA VE SİSTEM BİLGİLERİ",
                    about_card_content.into(),
                    theme,
                ));

                // 2. Update Check Card
                let update_btn = PillButton::build(
                    if self.is_checking_updates {
                        "⏳ Güncellemeler Kontrol Ediliyor..."
                    } else {
                        "🚀 Güncellemeleri Kontrol Et"
                    },
                    ButtonVariant::Primary,
                    if self.is_checking_updates {
                        None
                    } else {
                        Some(AboutMessage::CheckUpdates)
                    },
                    theme,
                );

                let update_status_str = if let Some(ref up) = data.update_info {
                    if up.update_available {
                        format!(
                            "Sürüm v{} mevcut! (İndirme: {})",
                            up.latest_version, up.download_url
                        )
                    } else {
                        format!("Sisteminiz güncel! (Mevcut sürüm: v{})", data.app_version)
                    }
                } else {
                    "Güncelleme kontrolü henüz yapılmadı".to_string()
                };

                let is_up_avail = data
                    .update_info
                    .as_ref()
                    .map(|u| u.update_available)
                    .unwrap_or(false);

                let update_card_content: Column<'a, AboutMessage> = column![row![
                    InfoRow::build("Güncelleme Durumu", &update_status_str, is_up_avail, theme),
                    iced::widget::horizontal_space(),
                    update_btn
                ]
                .align_y(Alignment::Center)
                .width(Length::Fill)]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "SÜRÜM VE GÜNCELLEME DENETLEYİCİ",
                    update_card_content.into(),
                    theme,
                ));

                // 3. License & Legal Card
                let legal_card_content: Column<'a, AboutMessage> = column![
                    InfoRow::build("Lisans (License)", "MIT Open Source License", false, theme),
                    Divider::build(theme),
                    InfoRow::build(
                        "Geliştirici Ekip",
                        "BlazeOS Core Engineering Team / Antigravity",
                        false,
                        theme
                    )
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "LİSANS VE TELİF HAKLARI",
                    legal_card_content.into(),
                    theme,
                ));

                page_col.into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_about_page_lifecycle() {
        let (mut page, _task) = AboutPage::new();
        assert!(matches!(page.state, AboutViewState::Loading));

        let sys = SystemInfo::gather();
        let app_version = env!("CARGO_PKG_VERSION").to_string();

        let data = Box::new(AboutData {
            app_version,
            os_name: sys.os_name,
            version_id: sys.version_id,
            edition_line: sys.edition_line,
            kernel: sys.kernel,
            arch: sys.arch,
            hostname: sys.hostname,
            update_info: None,
        });

        let _ = page.update(AboutMessage::AboutLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, AboutViewState::Loaded(_)));
    }
}
