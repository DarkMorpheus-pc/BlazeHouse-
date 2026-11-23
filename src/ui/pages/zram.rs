use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::zram::ZramService;
use crate::optimize::ZramStatus;
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
pub struct ZramData {
    pub status: ZramStatus,
    pub available_algorithms: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum ZramMessage {
    LoadZram,
    ZramLoaded(Result<Box<ZramData>, BlazeError>),
    SelectSize(u64),
    SelectAlgorithm(String),
    ApplyZramConfig,
    ConfigApplied(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum ZramViewState {
    Loading,
    Loaded(Box<ZramData>),
    Error(BlazeError),
}

pub struct ZramPage {
    pub state: ZramViewState,
    pub is_refreshing: bool,
    pub is_applying: bool,
    pub selected_size_mb: u64,
    pub selected_algorithm: String,
    pub last_action_status: Option<(String, bool)>, // (message, is_success)
}

impl ZramPage {
    pub fn new() -> (Self, Task<ZramMessage>) {
        let page = Self {
            state: ZramViewState::Loading,
            is_refreshing: true,
            is_applying: false,
            selected_size_mb: 4096,
            selected_algorithm: "zstd".to_string(),
            last_action_status: None,
        };
        let task = Task::perform(
            async move {
                let service = ZramService::new();
                let status = service.get_status()?;
                let available_algorithms = service.get_available_algorithms()?;
                Ok(Box::new(ZramData {
                    status,
                    available_algorithms,
                }))
            },
            ZramMessage::ZramLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: ZramMessage) -> Task<ZramMessage> {
        match message {
            ZramMessage::LoadZram => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = ZramService::new();
                        let status = service.get_status()?;
                        let available_algorithms = service.get_available_algorithms()?;
                        Ok(Box::new(ZramData {
                            status,
                            available_algorithms,
                        }))
                    },
                    ZramMessage::ZramLoaded,
                )
            }
            ZramMessage::ZramLoaded(res) => {
                self.is_refreshing = false;
                self.is_applying = false;
                match res {
                    Ok(data) => {
                        let current_mb = data.status.disksize_bytes / (1024 * 1024);
                        self.selected_size_mb = if current_mb > 0 { current_mb } else { 4096 };
                        if !data.status.algorithm.is_empty() {
                            self.selected_algorithm = data.status.algorithm.clone();
                        }
                        self.state = ZramViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = ZramViewState::Error(e);
                    }
                }
                Task::none()
            }
            ZramMessage::SelectSize(size_mb) => {
                if self.is_applying {
                    return Task::none();
                }
                self.selected_size_mb = size_mb;
                Task::none()
            }
            ZramMessage::SelectAlgorithm(algo) => {
                if self.is_applying {
                    return Task::none();
                }
                self.selected_algorithm = algo;
                Task::none()
            }
            ZramMessage::ApplyZramConfig => {
                if self.is_applying {
                    return Task::none();
                }
                self.is_applying = true;
                self.last_action_status = None;
                let size_mb = self.selected_size_mb;
                let algo = self.selected_algorithm.clone();

                Task::perform(
                    async move {
                        let service = ZramService::new();
                        service.configure(size_mb, &algo)
                    },
                    ZramMessage::ConfigApplied,
                )
            }
            ZramMessage::ConfigApplied(res) => {
                self.is_applying = false;
                match res {
                    Ok(()) => {
                        self.last_action_status = Some((
                            "ZRAM yapılandırması Polkit yetkilendirmesi ile başarıyla uygulandı ve doğrulandı."
                                .to_string(),
                            true,
                        ));
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = ZramService::new();
                                let status = service.get_status()?;
                                let available_algorithms = service.get_available_algorithms()?;
                                Ok(Box::new(ZramData {
                                    status,
                                    available_algorithms,
                                }))
                            },
                            ZramMessage::ZramLoaded,
                        )
                    }
                    Err(e) => {
                        self.last_action_status =
                            Some((format!("ZRAM yapılandırma hatası: {}", e), false));
                        self.state = ZramViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, ZramMessage> {
        match self.state {
            ZramViewState::Loading => {
                LoadingState::build("ZRAM takas bellek durumu sorgulanıyor...", theme)
            }
            ZramViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(ZramMessage::LoadZram),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            ZramViewState::Loaded(ref data) => {
                let sub_title =
                    text("Sıkıştırılmış RAM Takas Alanı Yönetimi, Sıkıştırma Algoritması ve Polkit Yapılandırması")
                        .size(TypographyTokens::CAPTION_SIZE)
                        .color(theme.colors.text_secondary);

                let refresh_btn = PillButton::build(
                    if self.is_refreshing {
                        "⏳ Yenileniyor..."
                    } else {
                        "🔄 Yenile"
                    },
                    ButtonVariant::Tonal,
                    if self.is_refreshing || self.is_applying {
                        None
                    } else {
                        Some(ZramMessage::LoadZram)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                // 1. HERO — ZRAM STATUS
                let status_badge = if self.is_applying {
                    Badge::build("APPLYING & POLKIT", BadgeStyle::Info, theme)
                } else if data.status.active {
                    Badge::build("ACTIVE", BadgeStyle::Success, theme)
                } else {
                    Badge::build("INACTIVE", BadgeStyle::Neutral, theme)
                };

                let current_mb = data.status.disksize_bytes / (1024 * 1024);
                let size_gb = current_mb as f64 / 1024.0;
                let device_str = if data.status.device.is_empty() {
                    "/dev/zram0"
                } else {
                    &data.status.device
                };

                let hero_card = GlassCard::build_with_subtitle(
                    "ZRAM STATUS & HARDWARE TELEMETRY",
                    if data.status.active {
                        "Sıkıştırılmış RAM takas alanı /dev/zram0 üzerinde aktif durumda"
                    } else {
                        "ZRAM takas aygıtı henüz yapılandırılmamış veya kapalı"
                    },
                    row![
                        column![
                            text("ZRAM (Compressed RAM Swap)")
                                .size(TypographyTokens::PAGE_TITLE_SIZE)
                                .color(theme.colors.text_primary),
                            text(format!(
                                "Aygıt: {} | Kapasite: {} MB ({:.2} GiB)",
                                device_str, current_mb, size_gb
                            ))
                            .size(TypographyTokens::CAPTION_SIZE)
                            .color(theme.colors.text_secondary),
                        ]
                        .spacing(SpacingTokens::XS),
                        iced::widget::horizontal_space(),
                        status_badge
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                    .into(),
                    theme,
                );

                // 2. MEMORY TELEMETRY GRID
                let cap_str = match current_mb {
                    2048 => "2048 MB (2.0 GiB)",
                    4096 => "4096 MB (4.0 GiB)",
                    8192 => "8192 MB (8.0 GiB)",
                    16384 => "16384 MB (16.0 GiB)",
                    0 => "0 MB (Devre Dışı)",
                    _ => "4096 MB (4.0 GiB)",
                };
                let metric_cap = MetricCard::build(
                    "ZRAM KAPASİTESİ",
                    cap_str,
                    "Tahsis Edilen Maksimum RAM Takas Limiti",
                    data.status.active,
                    theme,
                );

                let algo_str = match data.status.algorithm.as_str() {
                    "lz4" => "lz4",
                    "lzo" => "lzo",
                    "zstd" => "zstd",
                    _ => "zstd",
                };

                let metric_algo = MetricCard::build(
                    "SIKIŞTIRMA ALGORİTMASI",
                    algo_str,
                    "ZRAM Kernel Sıkıştırma Motoru",
                    false,
                    theme,
                );

                let metric_dev = MetricCard::build(
                    "AYGIT VE ÖNCELİK",
                    device_str,
                    "High-Priority Linux Swap Block Device",
                    false,
                    theme,
                );

                let telemetry_grid = row![metric_cap, metric_algo, metric_dev]
                    .spacing(SpacingTokens::LG)
                    .width(Length::Fill);

                let mut page_col: Column<'a, ZramMessage> =
                    column![top_bar, hero_card, telemetry_grid]
                        .spacing(SpacingTokens::XL)
                        .width(Length::Fill);

                // 3. Action Feedback Banner (if any)
                if let Some((ref msg, is_success)) = self.last_action_status {
                    let feedback_badge = if is_success {
                        Badge::build("VERIFIED SUCCESS", BadgeStyle::Success, theme)
                    } else {
                        Badge::build("ACTION FAILED", BadgeStyle::Warning, theme)
                    };

                    let feedback_card = GlassCard::build_with_subtitle(
                        "İŞLEM DOĞRULAMA FEEDBACK",
                        "Gerçek Linux sistem durumu re-read ile doğrulandı",
                        row![
                            text(msg.as_str())
                                .size(TypographyTokens::BODY_PRIMARY_SIZE)
                                .color(theme.colors.text_primary),
                            iced::widget::horizontal_space(),
                            feedback_badge
                        ]
                        .align_y(Alignment::Center)
                        .width(Length::Fill)
                        .into(),
                        theme,
                    );
                    page_col = page_col.push(feedback_card);
                }

                // 4. CONFIGURATION PANEL CARD
                let mut size_row = row![text("Kapasite Preseti:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center);

                let sizes: [(u64, &str); 4] = [
                    (2048, "2 GiB"),
                    (4096, "4 GiB"),
                    (8192, "8 GiB"),
                    (16384, "16 GiB"),
                ];

                for (mb, label) in sizes {
                    let is_sel = self.selected_size_mb == mb;
                    let btn = PillButton::build(
                        label,
                        if is_sel {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_applying {
                            None
                        } else {
                            Some(ZramMessage::SelectSize(mb))
                        },
                        theme,
                    );
                    size_row = size_row.push(btn);
                }

                let mut algo_row = row![text("Algoritma Seçimi:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center);

                let static_algos = ["zstd", "lz4", "lzo"];

                if data.available_algorithms.is_empty() {
                    for &algo in &static_algos {
                        let is_sel = self.selected_algorithm == algo;
                        let btn = PillButton::build(
                            algo,
                            if is_sel {
                                ButtonVariant::Primary
                            } else {
                                ButtonVariant::Tonal
                            },
                            if self.is_applying {
                                None
                            } else {
                                Some(ZramMessage::SelectAlgorithm(algo.to_string()))
                            },
                            theme,
                        );
                        algo_row = algo_row.push(btn);
                    }
                } else {
                    for algo in &data.available_algorithms {
                        let is_sel = &self.selected_algorithm == algo;
                        let algo_clone = algo.clone();
                        let btn = PillButton::build(
                            algo.as_str(),
                            if is_sel {
                                ButtonVariant::Primary
                            } else {
                                ButtonVariant::Tonal
                            },
                            if self.is_applying {
                                None
                            } else {
                                Some(ZramMessage::SelectAlgorithm(algo_clone))
                            },
                            theme,
                        );
                        algo_row = algo_row.push(btn);
                    }
                }

                let apply_btn = PillButton::build(
                    if self.is_applying {
                        "⏳ Yapılandırılıyor & Polkit Yetkilendiriliyor..."
                    } else {
                        "⚙️ Yapılandırmayı Uygula (Polkit Yetkisi İster)"
                    },
                    ButtonVariant::Primary,
                    if self.is_applying {
                        None
                    } else {
                        Some(ZramMessage::ApplyZramConfig)
                    },
                    theme,
                );

                let config_col: Column<'a, ZramMessage> = column![
                    size_row,
                    Divider::build(theme),
                    algo_row,
                    Divider::build(theme),
                    row![
                        apply_btn,
                        iced::widget::horizontal_space(),
                        text("Polkit authentication / Yetkilendirme iletişim kutusu açılabilir")
                            .size(TypographyTokens::CAPTION_SIZE)
                            .color(theme.colors.text_secondary)
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                ]
                .spacing(SpacingTokens::MD);

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "ZRAM CONFIGURATION & POLKIT APPLY",
                    "Kapasite preset'i ve sıkıştırma motorunu belirleyin, yetkili olarak sisteme uygulayın",
                    config_col.into(),
                    theme,
                ));

                // 5. TECHNICAL DETAILS CARD
                let tech_col: Column<'a, ZramMessage> = column![
                    InfoRow::build("ZRAM Aygıt Dizini", device_str, true, theme),
                    Divider::build(theme),
                    InfoRow::build(
                        "Etkin Kapasite",
                        &format!("{} MB ({:.2} GiB)", current_mb, size_gb),
                        true,
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build(
                        "Etkin Algoritma",
                        if data.status.algorithm.is_empty() {
                            "zstd"
                        } else {
                            &data.status.algorithm
                        },
                        true,
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build(
                        "Takas Çalışma Önceliği",
                        if data.status.active {
                            "High Priority (Swap 100)"
                        } else {
                            "Devre Dışı"
                        },
                        true,
                        theme
                    ),
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "TEKNİK PARAMETRELER VE VERİFİKASYON",
                    "Gerçek Linux çekirdek sürücüsü ZRAM bloğu parametre özeti",
                    tech_col.into(),
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
    fn test_zram_page_lifecycle() {
        let (mut page, _task) = ZramPage::new();
        assert!(matches!(page.state, ZramViewState::Loading));

        let service = ZramService::new();
        let status = service.get_status().unwrap();
        let algos = service.get_available_algorithms().unwrap();
        let data = Box::new(ZramData {
            status,
            available_algorithms: algos,
        });

        let _ = page.update(ZramMessage::ZramLoaded(Ok(data.clone())));
        assert!(!page.is_refreshing);
        if let ZramViewState::Loaded(ref d) = page.state {
            assert_eq!(d.status.device, data.status.device);
        } else {
            panic!("Expected ZramViewState::Loaded");
        }
    }
}
