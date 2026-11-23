use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::optimization::OptimizationService;
use crate::optimize::{CleanupAction, Tweak};
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
pub struct OptimizeData {
    pub tweaks: Vec<(Tweak, Option<bool>)>,
    pub cleanup_actions: Vec<CleanupAction>,
}

#[derive(Debug, Clone)]
pub enum OptimizeMessage {
    LoadOptimize,
    OptimizeLoaded(Result<Box<OptimizeData>, BlazeError>),
    RunCleanupAction(String),
    CleanupFinished(String, Result<(), BlazeError>),
    ToggleTweak(String, bool),
    TweakApplied(String, Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum OptimizeViewState {
    Loading,
    Loaded(Box<OptimizeData>),
    Error(BlazeError),
}

pub struct OptimizePage {
    pub state: OptimizeViewState,
    pub is_refreshing: bool,
    pub active_action_key: Option<String>,
    pub last_status_message: Option<(String, bool)>, // (message, is_success)
}

impl OptimizePage {
    pub fn new() -> (Self, Task<OptimizeMessage>) {
        let page = Self {
            state: OptimizeViewState::Loading,
            is_refreshing: true,
            active_action_key: None,
            last_status_message: None,
        };
        let task = Task::perform(
            async move {
                let service = OptimizationService::new();
                let tweaks_catalog = service.get_tweaks_catalog()?;
                let mut tweaks = Vec::new();
                for t in tweaks_catalog {
                    let st = service.get_tweak_state(&t).unwrap_or(None);
                    tweaks.push((t, st));
                }
                let cleanup_actions = service.get_cleanup_actions()?;
                Ok(Box::new(OptimizeData {
                    tweaks,
                    cleanup_actions,
                }))
            },
            OptimizeMessage::OptimizeLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: OptimizeMessage) -> Task<OptimizeMessage> {
        match message {
            OptimizeMessage::LoadOptimize => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = OptimizationService::new();
                        let tweaks_catalog = service.get_tweaks_catalog()?;
                        let mut tweaks = Vec::new();
                        for t in tweaks_catalog {
                            let st = service.get_tweak_state(&t).unwrap_or(None);
                            tweaks.push((t, st));
                        }
                        let cleanup_actions = service.get_cleanup_actions()?;
                        Ok(Box::new(OptimizeData {
                            tweaks,
                            cleanup_actions,
                        }))
                    },
                    OptimizeMessage::OptimizeLoaded,
                )
            }
            OptimizeMessage::OptimizeLoaded(res) => {
                self.is_refreshing = false;
                self.active_action_key = None;
                match res {
                    Ok(data) => {
                        self.state = OptimizeViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = OptimizeViewState::Error(e);
                    }
                }
                Task::none()
            }
            OptimizeMessage::RunCleanupAction(key) => {
                if self.active_action_key.is_some() {
                    return Task::none();
                }
                self.active_action_key = Some(key.clone());
                self.last_status_message = None;
                let k = key.clone();
                Task::perform(
                    async move {
                        let service = OptimizationService::new();
                        service.run_cleanup_action(&k)
                    },
                    move |res| OptimizeMessage::CleanupFinished(key.clone(), res),
                )
            }
            OptimizeMessage::CleanupFinished(key, res) => {
                self.active_action_key = None;
                match res {
                    Ok(()) => {
                        self.last_status_message = Some((
                            format!("Temizlik işlemi ({}) başarıyla tamamlandı.", key),
                            true,
                        ));
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = OptimizationService::new();
                                let tweaks_catalog = service.get_tweaks_catalog()?;
                                let mut tweaks = Vec::new();
                                for t in tweaks_catalog {
                                    let st = service.get_tweak_state(&t).unwrap_or(None);
                                    tweaks.push((t, st));
                                }
                                let cleanup_actions = service.get_cleanup_actions()?;
                                Ok(Box::new(OptimizeData {
                                    tweaks,
                                    cleanup_actions,
                                }))
                            },
                            OptimizeMessage::OptimizeLoaded,
                        )
                    }
                    Err(e) => {
                        self.last_status_message =
                            Some((format!("İşlem hatası ({}): {}", key, e), false));
                        self.state = OptimizeViewState::Error(e);
                        Task::none()
                    }
                }
            }
            OptimizeMessage::ToggleTweak(key, enable) => {
                if self.active_action_key.is_some() {
                    return Task::none();
                }
                self.active_action_key = Some(key.clone());
                self.last_status_message = None;
                let k = key.clone();
                Task::perform(
                    async move {
                        let service = OptimizationService::new();
                        let tweaks = service.get_tweaks_catalog()?;
                        if let Some(target) = tweaks.iter().find(|t| t.key == k) {
                            service.apply_tweak(target, enable)
                        } else {
                            Err(BlazeError::NotFound(format!("Tweak({})", k)))
                        }
                    },
                    move |res| OptimizeMessage::TweakApplied(key.clone(), res),
                )
            }
            OptimizeMessage::TweakApplied(key, res) => {
                self.active_action_key = None;
                match res {
                    Ok(()) => {
                        self.last_status_message =
                            Some((format!("Ayar ({}) başarıyla güncellendi.", key), true));
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = OptimizationService::new();
                                let tweaks_catalog = service.get_tweaks_catalog()?;
                                let mut tweaks = Vec::new();
                                for t in tweaks_catalog {
                                    let st = service.get_tweak_state(&t).unwrap_or(None);
                                    tweaks.push((t, st));
                                }
                                let cleanup_actions = service.get_cleanup_actions()?;
                                Ok(Box::new(OptimizeData {
                                    tweaks,
                                    cleanup_actions,
                                }))
                            },
                            OptimizeMessage::OptimizeLoaded,
                        )
                    }
                    Err(e) => {
                        self.last_status_message =
                            Some((format!("Ayar güncelleme hatası ({}): {}", key, e), false));
                        self.state = OptimizeViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, OptimizeMessage> {
        match self.state {
            OptimizeViewState::Loading => {
                LoadingState::build("Optimizasyon ve temizlik kataloğu taranıyor...", theme)
            }
            OptimizeViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(OptimizeMessage::LoadOptimize),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            OptimizeViewState::Loaded(ref data) => {
                let sub_title =
                    text("Sistem Optimizasyonu, Önbellek Temizliği ve Gizlilik Ayarları")
                        .size(TypographyTokens::CAPTION_SIZE)
                        .color(theme.colors.text_secondary);

                let refresh_btn = PillButton::build(
                    if self.is_refreshing {
                        "⏳ Yenileniyor..."
                    } else {
                        "🔄 Yenile"
                    },
                    ButtonVariant::Tonal,
                    if self.is_refreshing || self.active_action_key.is_some() {
                        None
                    } else {
                        Some(OptimizeMessage::LoadOptimize)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                // Hero Status Card
                let (status_badge, status_desc) = if self.active_action_key.is_some() {
                    (
                        Badge::build("RUNNING", BadgeStyle::Info, theme),
                        "Optimizasyon işlemi yürütülüyor...",
                    )
                } else if let Some((ref msg, is_success)) = self.last_status_message {
                    if is_success {
                        (
                            Badge::build("COMPLETED", BadgeStyle::Success, theme),
                            msg.as_str(),
                        )
                    } else {
                        (
                            Badge::build("FAILED", BadgeStyle::Warning, theme),
                            msg.as_str(),
                        )
                    }
                } else {
                    (
                        Badge::build("READY", BadgeStyle::Success, theme),
                        "Sistem optimizasyon ve temizlik işlemleri için hazır",
                    )
                };

                let hero_card = GlassCard::build_with_subtitle(
                    "SYSTEM OPTIMIZATION",
                    status_desc,
                    row![
                        column![
                            text("Sistem Temizliği ve Gizlilik Tercihleri")
                                .size(TypographyTokens::PAGE_TITLE_SIZE)
                                .color(theme.colors.text_primary),
                            text("Gereksiz dosyaları temizleyin ve sistem gizlilik seçeneklerini yönetin")
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

                // Telemetry summary grid
                let clean_count_str = match data.cleanup_actions.len() {
                    0 => "0",
                    1 => "1",
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    _ => "4+",
                };
                let tweaks_count_str = match data.tweaks.len() {
                    0 => "0",
                    1 => "1",
                    2 => "2",
                    3 => "3",
                    4 => "4",
                    _ => "4+",
                };

                let metric1 = MetricCard::build(
                    "TEMİZLİK AKSİYONLARI",
                    clean_count_str,
                    "Güvenli Ön Önbellek ve Log Temizleyicileri",
                    false,
                    theme,
                );
                let metric2 = MetricCard::build(
                    "GİZLİLİK VE TWEAKLER",
                    tweaks_count_str,
                    "Kullanıcı Tercihi ve Davranış Kontrolleri",
                    false,
                    theme,
                );
                let metric_grid = row![metric1, metric2]
                    .spacing(SpacingTokens::LG)
                    .width(Length::Fill);

                let mut page_col: Column<'a, OptimizeMessage> =
                    column![top_bar, hero_card, metric_grid]
                        .spacing(SpacingTokens::XL)
                        .width(Length::Fill);

                // 1. Safe Cleanup Actions Section
                let mut cleanup_col: Column<'a, OptimizeMessage> =
                    column![].spacing(SpacingTokens::SM);
                for (idx, action) in data.cleanup_actions.iter().enumerate() {
                    if idx > 0 {
                        cleanup_col = cleanup_col.push(Divider::build(theme));
                    }

                    let is_active = self.active_action_key.as_deref() == Some(&action.key);
                    let k = action.key.clone();

                    let act_btn = PillButton::build(
                        if is_active {
                            "⏳ Temizleniyor..."
                        } else {
                            "🧹 Temizle"
                        },
                        ButtonVariant::Primary,
                        if self.active_action_key.is_some() {
                            None
                        } else {
                            Some(OptimizeMessage::RunCleanupAction(k))
                        },
                        theme,
                    );

                    let title_label = match action.key.as_str() {
                        "pkg-cache" => "Paket Önbelleği (DNF / APT)",
                        "orphans" => "Gereksiz Bağımlılıklar (Autoremove)",
                        "journal" => "Sistem Günlükleri (Journald)",
                        "thumbnails" => "Küçük Resim Önbelleği",
                        "user-cache" => "Uygulama Önbelleği (~/.cache)",
                        "trash" => "Çöp Kutusu",
                        _ => &action.title,
                    };

                    let desc_label = match action.key.as_str() {
                        "pkg-cache" => "İndirilmiş paket arşivlerini ve önbellek dosyalarını siler",
                        "orphans" => "Sistemde kullanılmayan yetim bağımlılık paketlerini kaldırır",
                        "journal" => "3 günden eski systemd log kayıtlarını temizler",
                        "thumbnails" => "Önizleme görsel önbellek verilerini sıfırlar",
                        "user-cache" => "Aktif oturum soketlerine dokunmadan uygulama önbelleğini temizler",
                        "trash" => "Kullanıcı çöp kutusundaki silinmiş dosyaları kalıcı olarak boşaltır",
                        _ => &action.desc,
                    };

                    let action_badge = if is_active {
                        Badge::build("RUNNING", BadgeStyle::Info, theme)
                    } else {
                        Badge::build("SAFE CLEANUP", BadgeStyle::Neutral, theme)
                    };

                    let action_row = row![
                        InfoRow::build(title_label, desc_label, is_active, theme),
                        action_badge,
                        iced::widget::horizontal_space(),
                        act_btn
                    ]
                    .spacing(SpacingTokens::MD)
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                    cleanup_col = cleanup_col.push(action_row);
                }

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "SAFE CLEANUP — GÜVENLİ TEMİZLİK AKSİYONLARI",
                    "Gereksiz paket arşivleri, yetim bağımlılıklar ve log önbelleği temizliği",
                    cleanup_col.into(),
                    theme,
                ));

                // 2. System Tweaks (Privacy & Behavior) Section
                let mut tweaks_col: Column<'a, OptimizeMessage> =
                    column![].spacing(SpacingTokens::SM);
                for (idx, (tweak, st)) in data.tweaks.iter().enumerate() {
                    if idx > 0 {
                        tweaks_col = tweaks_col.push(Divider::build(theme));
                    }

                    let is_active = self.active_action_key.as_deref() == Some(&tweak.key);
                    let current_enabled = st.unwrap_or(false);
                    let k = tweak.key.clone();

                    let toggle_btn = PillButton::build(
                        if is_active {
                            "⏳ İşleniyor..."
                        } else if current_enabled {
                            "🟢 Etkin"
                        } else {
                            "⚪ Kapalı"
                        },
                        if current_enabled {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.active_action_key.is_some() {
                            None
                        } else {
                            Some(OptimizeMessage::ToggleTweak(k, !current_enabled))
                        },
                        theme,
                    );

                    let title_label = match tweak.key.as_str() {
                        "recent-files" => "Son Kullanılan Dosya Geçmişi",
                        "app-usage" => "Yazılım Kullanım İstatistikleri",
                        "location" => "Konum Servisleri Takibi",
                        _ => &tweak.title,
                    };

                    let desc_label = match tweak.key.as_str() {
                        "recent-files" => "Gizlilik için dosya erişim kaydını kısıtlar",
                        "app-usage" => "Telemetri ve yazılım kullanım veri gönderimini kapatır",
                        "location" => "Arka plan konum takip servislerini engeller",
                        _ => &tweak.desc,
                    };

                    let state_badge = if current_enabled {
                        Badge::build("ENABLED", BadgeStyle::Success, theme)
                    } else {
                        Badge::build("DISABLED", BadgeStyle::Neutral, theme)
                    };

                    let tweak_row = row![
                        InfoRow::build(title_label, desc_label, current_enabled, theme),
                        state_badge,
                        iced::widget::horizontal_space(),
                        toggle_btn
                    ]
                    .spacing(SpacingTokens::MD)
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                    tweaks_col = tweaks_col.push(tweak_row);
                }

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "SYSTEM PREFERENCES — GİZLİLİK VE DAVRANIŞ TWEAKLERİ",
                    "Kullanıcı geçmişi kaydı, telemetri ve konum izleme tercihleri",
                    tweaks_col.into(),
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
    fn test_optimize_page_lifecycle() {
        let (mut page, _task) = OptimizePage::new();
        assert!(matches!(page.state, OptimizeViewState::Loading));

        let service = OptimizationService::new();
        let tweaks_catalog = service.get_tweaks_catalog().unwrap();
        let mut tweaks = Vec::new();
        for t in tweaks_catalog {
            let st = service.get_tweak_state(&t).unwrap_or(None);
            tweaks.push((t, st));
        }
        let cleanup_actions = service.get_cleanup_actions().unwrap();
        let data = Box::new(OptimizeData {
            tweaks,
            cleanup_actions,
        });

        let _ = page.update(OptimizeMessage::OptimizeLoaded(Ok(data.clone())));
        assert!(!page.is_refreshing);
        if let OptimizeViewState::Loaded(ref d) = page.state {
            assert!(!d.cleanup_actions.is_empty());
        } else {
            panic!("Expected OptimizeViewState::Loaded");
        }
    }
}
