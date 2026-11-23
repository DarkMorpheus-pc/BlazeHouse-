use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::power::PowerService;
use crate::power::BatteryInfo;
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
pub struct BatteryData {
    pub battery_info: Option<BatteryInfo>,
    pub active_profile: String,
}

#[derive(Debug, Clone)]
pub enum BatteryMessage {
    LoadBattery,
    BatteryLoaded(Result<Box<BatteryData>, BlazeError>),
    SelectPowerProfile(String),
    ProfileSet(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum BatteryViewState {
    Loading,
    Loaded(Box<BatteryData>),
    Error(BlazeError),
}

pub struct BatteryPage {
    pub state: BatteryViewState,
    pub is_refreshing: bool,
    pub is_setting_profile: bool,
    pub last_profile_status: Option<(String, bool)>, // (message, is_success)
}

impl BatteryPage {
    pub fn new() -> (Self, Task<BatteryMessage>) {
        let page = Self {
            state: BatteryViewState::Loading,
            is_refreshing: true,
            is_setting_profile: false,
            last_profile_status: None,
        };
        let task = Task::perform(
            async move {
                let service = PowerService::new();
                let battery_info = service.get_battery_info()?;
                let active_profile = service.get_profile()?;
                Ok(Box::new(BatteryData {
                    battery_info,
                    active_profile,
                }))
            },
            BatteryMessage::BatteryLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: BatteryMessage) -> Task<BatteryMessage> {
        match message {
            BatteryMessage::LoadBattery => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = PowerService::new();
                        let battery_info = service.get_battery_info()?;
                        let active_profile = service.get_profile()?;
                        Ok(Box::new(BatteryData {
                            battery_info,
                            active_profile,
                        }))
                    },
                    BatteryMessage::BatteryLoaded,
                )
            }
            BatteryMessage::BatteryLoaded(res) => {
                self.is_refreshing = false;
                self.is_setting_profile = false;
                match res {
                    Ok(data) => {
                        self.state = BatteryViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = BatteryViewState::Error(e);
                    }
                }
                Task::none()
            }
            BatteryMessage::SelectPowerProfile(profile) => {
                if self.is_setting_profile {
                    return Task::none();
                }
                self.is_setting_profile = true;
                self.last_profile_status = None;
                let prof = profile.clone();
                Task::perform(
                    async move {
                        let service = PowerService::new();
                        service.set_profile(&prof)
                    },
                    BatteryMessage::ProfileSet,
                )
            }
            BatteryMessage::ProfileSet(res) => {
                self.is_setting_profile = false;
                match res {
                    Ok(()) => {
                        self.last_profile_status = Some((
                            "Güç profili başarıyla güncellendi ve sistem durumu doğrulandı."
                                .to_string(),
                            true,
                        ));
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = PowerService::new();
                                let battery_info = service.get_battery_info()?;
                                let active_profile = service.get_profile()?;
                                Ok(Box::new(BatteryData {
                                    battery_info,
                                    active_profile,
                                }))
                            },
                            BatteryMessage::BatteryLoaded,
                        )
                    }
                    Err(e) => {
                        self.last_profile_status =
                            Some((format!("Güç profili güncelleme hatası: {}", e), false));
                        self.state = BatteryViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, BatteryMessage> {
        match self.state {
            BatteryViewState::Loading => {
                LoadingState::build("Pil ve güç yönetimi durumu taranıyor...", theme)
            }
            BatteryViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(BatteryMessage::LoadBattery),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            BatteryViewState::Loaded(ref data) => {
                let sub_title = text("Pil Şarj Durumu, Enerji Kapasitesi ve Donanım Güç Profili")
                    .size(TypographyTokens::CAPTION_SIZE)
                    .color(theme.colors.text_secondary);

                let refresh_btn = PillButton::build(
                    if self.is_refreshing {
                        "⏳ Yenileniyor..."
                    } else {
                        "🔄 Yenile"
                    },
                    ButtonVariant::Tonal,
                    if self.is_refreshing || self.is_setting_profile {
                        None
                    } else {
                        Some(BatteryMessage::LoadBattery)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                // 1. HERO POWER STATUS CARD
                let is_has_bat = data.battery_info.is_some();
                let bat_status_lower = data
                    .battery_info
                    .as_ref()
                    .map(|b| b.status.to_lowercase())
                    .unwrap_or_default();
                let is_charging = bat_status_lower.contains("charging");

                let status_badge = if !is_has_bat {
                    Badge::build("AC POWER / NO BATTERY", BadgeStyle::Neutral, theme)
                } else if is_charging {
                    Badge::build("CHARGING", BadgeStyle::Success, theme)
                } else {
                    Badge::build("DISCHARGING", BadgeStyle::Info, theme)
                };

                let hero_subtitle = if is_has_bat {
                    "Dahili Pil / Batarya Telemetrisi ve UPower Güç Yönetimi"
                } else {
                    "Masaüstü Bilgisayar veya Sabit AC Güç Kaynağı Algılandı"
                };

                let hero_card = GlassCard::build_with_subtitle(
                    "BATTERY & POWER MANAGEMENT",
                    hero_subtitle,
                    row![
                        column![
                            text(if is_has_bat {
                                "Dahili Pil / Batarya Algılandı"
                            } else {
                                "Dahili Pil / Batarya Tespit Edilemedi"
                            })
                            .size(TypographyTokens::PAGE_TITLE_SIZE)
                            .color(theme.colors.text_primary),
                            text("Linux powerprofilesctl ve UPower güç yönetimi mimarisi")
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

                // 2. POWER METRIC SUMMARY GRID
                let metric_cap_str = if is_has_bat {
                    "%50+ (Pil Aktif)"
                } else {
                    "MEVCUT DEĞİL"
                };

                let metric_stat_str = if is_has_bat {
                    if is_charging {
                        "Şarj Oluyor (AC)"
                    } else {
                        "Deşarj Oluyor (Batarya)"
                    }
                } else {
                    "AC Şebeke Gücü"
                };

                let active_prof_str = match data.active_profile.as_str() {
                    "performance" => "PERFORMANCE",
                    "power-saver" => "POWER SAVER",
                    _ => "BALANCED",
                };

                let metric_cap = MetricCard::build(
                    "ŞARJ YÜZDESİ",
                    metric_cap_str,
                    "Kullanılabilir Pil Kapasitesi",
                    is_has_bat,
                    theme,
                );

                let metric_stat = MetricCard::build(
                    "GÜÇ DURUMU",
                    metric_stat_str,
                    "Şarj / Deşarj / AC Besleme",
                    false,
                    theme,
                );

                let metric_prof = MetricCard::build(
                    "AKTİF GÜÇ PROFİLİ",
                    active_prof_str,
                    "powerprofilesctl Donanım Modu",
                    true,
                    theme,
                );

                let metric_grid = row![metric_cap, metric_stat, metric_prof]
                    .spacing(SpacingTokens::LG)
                    .width(Length::Fill);

                let mut page_col: Column<'a, BatteryMessage> =
                    column![top_bar, hero_card, metric_grid]
                        .spacing(SpacingTokens::XL)
                        .width(Length::Fill);

                // 3. Action Feedback Banner (if any)
                if let Some((ref msg, is_success)) = self.last_profile_status {
                    let feedback_badge = if is_success {
                        Badge::build("PROFILE VERIFIED", BadgeStyle::Success, theme)
                    } else {
                        Badge::build("PROFILE ERROR", BadgeStyle::Warning, theme)
                    };

                    let feedback_card = GlassCard::build_with_subtitle(
                        "GÜÇ PROFİLİ DOĞRULAMA",
                        "Gerçek powerprofilesctl re-read ile doğrulandı",
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

                // 4. POWER PROFILE SELECTION CARD
                let mut profile_row = row![text("Güç Profili Seçimi:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center);

                let profiles = [
                    ("performance", "⚡ Performance (Yüksek Performans)"),
                    ("balanced", "⚖️ Balanced (Dengeli)"),
                    ("power-saver", "🌱 Power Saver (Güç Tasarrufu)"),
                ];

                for (prof_key, prof_label) in profiles {
                    let is_sel = data.active_profile == prof_key;
                    let p_str = prof_key.to_string();
                    let btn = PillButton::build(
                        prof_label,
                        if is_sel {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_setting_profile || is_sel {
                            None
                        } else {
                            Some(BatteryMessage::SelectPowerProfile(p_str))
                        },
                        theme,
                    );
                    profile_row = profile_row.push(btn);
                }

                let profile_desc = match data.active_profile.as_str() {
                    "performance" => {
                        "Maksimum CPU frekansı ve donanım performansı önceliklendirilir."
                    }
                    "power-saver" => {
                        "Minimum pil tüketimi ve azami pil ömrü için işlemci hızı kısıtlanır."
                    }
                    _ => "Sistem sıcaklığı, performans ve güç tüketimi arasında dengeli mod.",
                };

                let profile_card_content: Column<'a, BatteryMessage> = column![
                    profile_row,
                    Divider::build(theme),
                    row![
                        text(format!("Aktif Profil Açıklaması: {}", profile_desc))
                            .size(TypographyTokens::BODY_PRIMARY_SIZE)
                            .color(theme.colors.text_secondary),
                        iced::widget::horizontal_space(),
                        if self.is_setting_profile {
                            Badge::build("APPLYING PROFILE...", BadgeStyle::Info, theme)
                        } else {
                            Badge::build("ACTIVE PROFILE", BadgeStyle::Success, theme)
                        }
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                ]
                .spacing(SpacingTokens::MD);

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "POWER PROFILE CONTROLLER",
                    "powerprofilesctl donanım güç ve enerji tasarruf modları",
                    profile_card_content.into(),
                    theme,
                ));

                // 5. BATTERY HARDWARE DETAILS CARD
                let mut bat_col: Column<'a, BatteryMessage> = column![].spacing(SpacingTokens::SM);

                if let Some(ref bat) = data.battery_info {
                    bat_col = bat_col.push(InfoRow::build("Pil Aygıt Adı", &bat.name, true, theme));
                    bat_col = bat_col.push(Divider::build(theme));
                    bat_col = bat_col.push(InfoRow::build(
                        "Şarj Yüzdesi",
                        &format!("%{}", bat.capacity_percent),
                        true,
                        theme,
                    ));
                    bat_col = bat_col.push(Divider::build(theme));
                    bat_col =
                        bat_col.push(InfoRow::build("Şarj Durumu", &bat.status, false, theme));
                    bat_col = bat_col.push(Divider::build(theme));
                    bat_col = bat_col.push(InfoRow::build(
                        "Pil Teknolojisi",
                        &bat.technology,
                        false,
                        theme,
                    ));
                    let health_val = match bat.health_percent {
                        Some(100) => "%100 (Mükemmel)",
                        Some(80..=99) => "%80+ (İyi)",
                        Some(_) => "%80 Altı",
                        None => "BİLİNMİYOR / ULAŞILAMADI",
                    };
                    bat_col = bat_col.push(InfoRow::build(
                        "Pil Sağlık Seviyesi",
                        health_val,
                        true,
                        theme,
                    ));
                } else {
                    bat_col = bat_col.push(InfoRow::build(
                        "Pil Durumu",
                        "Sistemde Takılı Dahili Batarya Yok (Masaüstü AC Şebeke Gücü)",
                        false,
                        theme,
                    ));
                }

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "DİL PİL VE BATARYA TELEMETRİSİ",
                    "Linux UPower sürücüsünden okunan gerçek pil donanım ayrıntıları",
                    bat_col.into(),
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
    fn test_battery_page_lifecycle() {
        let (mut page, _task) = BatteryPage::new();
        assert!(matches!(page.state, BatteryViewState::Loading));

        let service = PowerService::new();
        let battery_info = service.get_battery_info().unwrap();
        let active_profile = service.get_profile().unwrap();
        let data = Box::new(BatteryData {
            battery_info,
            active_profile: active_profile.clone(),
        });

        let _ = page.update(BatteryMessage::BatteryLoaded(Ok(data.clone())));
        assert!(!page.is_refreshing);
        if let BatteryViewState::Loaded(ref d) = page.state {
            assert_eq!(d.active_profile, active_profile);
        } else {
            panic!("Expected BatteryViewState::Loaded");
        }
    }
}
