use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};
use std::collections::HashMap;

use crate::app::error::BlazeError;
use crate::app::settings::SettingsService;
use crate::timedate;
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

#[derive(Debug, Clone)]
pub struct LanguageRegionData {
    pub system_locale: String,
    pub keyboard_layout: String,
    pub keyboard_model: String,
    pub timezone: String,
    pub ntp_enabled: bool,
    pub ntp_synchronized: bool,
    pub available_layouts: Vec<&'static str>,
}

#[derive(Debug, Clone)]
pub enum LanguageRegionMessage {
    LoadLanguageRegion,
    LanguageRegionLoaded(Result<Box<LanguageRegionData>, BlazeError>),
    SetKeyboardLayout(String),
    SetTimezone(String),
    SetNtp(bool),
    ActionFinished(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum LanguageRegionViewState {
    Loading,
    Loaded(Box<LanguageRegionData>),
    Error(BlazeError),
}

pub struct LanguageRegionPage {
    pub state: LanguageRegionViewState,
    pub is_refreshing: bool,
    pub is_acting: bool,
}

impl LanguageRegionPage {
    pub fn new() -> (Self, Task<LanguageRegionMessage>) {
        let page = Self {
            state: LanguageRegionViewState::Loading,
            is_refreshing: true,
            is_acting: false,
        };
        let task = Task::perform(
            async move {
                let service = SettingsService::new();
                let system_locale = service
                    .get_system_locale()
                    .unwrap_or_else(|_| "LANG=tr_TR.UTF-8".to_string());
                let (keyboard_layout, keyboard_model) = service
                    .get_keyboard_layouts()
                    .unwrap_or(("tr".to_string(), "pc105".to_string()));
                let available_layouts = service.get_available_keyboard_layouts();

                let td_map: HashMap<String, String> = timedate::timedate_status();
                let timezone = td_map
                    .get("Timezone")
                    .cloned()
                    .unwrap_or_else(|| "Europe/Istanbul".to_string());
                let ntp_enabled = td_map.get("NTP").map(|s| s == "yes").unwrap_or(false);
                let ntp_synchronized = td_map
                    .get("NTPSynchronized")
                    .map(|s| s == "yes")
                    .unwrap_or(false);

                Ok(Box::new(LanguageRegionData {
                    system_locale,
                    keyboard_layout,
                    keyboard_model,
                    timezone,
                    ntp_enabled,
                    ntp_synchronized,
                    available_layouts,
                }))
            },
            LanguageRegionMessage::LanguageRegionLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: LanguageRegionMessage) -> Task<LanguageRegionMessage> {
        match message {
            LanguageRegionMessage::LoadLanguageRegion => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = SettingsService::new();
                        let system_locale = service.get_system_locale()?;
                        let (keyboard_layout, keyboard_model) = service.get_keyboard_layouts()?;
                        let available_layouts = service.get_available_keyboard_layouts();

                        let td_map = timedate::timedate_status();
                        let timezone = td_map
                            .get("Timezone")
                            .cloned()
                            .unwrap_or_else(|| "Europe/Istanbul".to_string());
                        let ntp_enabled = td_map.get("NTP").map(|s| s == "yes").unwrap_or(false);
                        let ntp_synchronized = td_map
                            .get("NTPSynchronized")
                            .map(|s| s == "yes")
                            .unwrap_or(false);

                        Ok(Box::new(LanguageRegionData {
                            system_locale,
                            keyboard_layout,
                            keyboard_model,
                            timezone,
                            ntp_enabled,
                            ntp_synchronized,
                            available_layouts,
                        }))
                    },
                    LanguageRegionMessage::LanguageRegionLoaded,
                )
            }
            LanguageRegionMessage::LanguageRegionLoaded(res) => {
                self.is_refreshing = false;
                self.is_acting = false;
                match res {
                    Ok(data) => {
                        self.state = LanguageRegionViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = LanguageRegionViewState::Error(e);
                    }
                }
                Task::none()
            }
            LanguageRegionMessage::SetKeyboardLayout(layout) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                let target = layout.clone();
                Task::perform(
                    async move {
                        let service = SettingsService::new();
                        service.set_keyboard_layout(&target)
                    },
                    LanguageRegionMessage::ActionFinished,
                )
            }
            LanguageRegionMessage::SetTimezone(tz) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                let target = tz.clone();
                Task::perform(
                    async move {
                        if timedate::set_timezone(&target) {
                            Ok(())
                        } else {
                            Err(BlazeError::ExecutionError {
                                command: "timedatectl set-timezone".to_string(),
                                stdout: String::new(),
                                stderr: format!("Zaman dilimi değiştirilemedi: {}", target),
                            })
                        }
                    },
                    LanguageRegionMessage::ActionFinished,
                )
            }
            LanguageRegionMessage::SetNtp(enable) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        if timedate::set_ntp(enable) {
                            Ok(())
                        } else {
                            Err(BlazeError::ExecutionError {
                                command: "timedatectl set-ntp".to_string(),
                                stdout: String::new(),
                                stderr: "NTP saat senkronizasyonu değiştirilemedi".to_string(),
                            })
                        }
                    },
                    LanguageRegionMessage::ActionFinished,
                )
            }
            LanguageRegionMessage::ActionFinished(res) => {
                self.is_acting = false;
                match res {
                    Ok(()) => {
                        // Re-read system locale and timedatectl status to verify parity
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = SettingsService::new();
                                let system_locale = service.get_system_locale()?;
                                let (keyboard_layout, keyboard_model) =
                                    service.get_keyboard_layouts()?;
                                let available_layouts = service.get_available_keyboard_layouts();

                                let td_map = timedate::timedate_status();
                                let timezone = td_map
                                    .get("Timezone")
                                    .cloned()
                                    .unwrap_or_else(|| "Europe/Istanbul".to_string());
                                let ntp_enabled =
                                    td_map.get("NTP").map(|s| s == "yes").unwrap_or(false);
                                let ntp_synchronized = td_map
                                    .get("NTPSynchronized")
                                    .map(|s| s == "yes")
                                    .unwrap_or(false);

                                Ok(Box::new(LanguageRegionData {
                                    system_locale,
                                    keyboard_layout,
                                    keyboard_model,
                                    timezone,
                                    ntp_enabled,
                                    ntp_synchronized,
                                    available_layouts,
                                }))
                            },
                            LanguageRegionMessage::LanguageRegionLoaded,
                        )
                    }
                    Err(e) => {
                        self.state = LanguageRegionViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, LanguageRegionMessage> {
        match self.state {
            LanguageRegionViewState::Loading => LoadingState::build(
                "Sistem dili, bölge ve zaman dilimi parametreleri okunuyor...",
                theme,
            ),
            LanguageRegionViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(LanguageRegionMessage::LoadLanguageRegion),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            LanguageRegionViewState::Loaded(ref data) => {
                let sub_title =
                    text("Sistem Yerel Ayarları (Locale), Zaman Dilimi ve Klavye Yapılandırması")
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
                        Some(LanguageRegionMessage::LoadLanguageRegion)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, LanguageRegionMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. Locale Card
                let locale_card_content: Column<'a, LanguageRegionMessage> = column![
                    InfoRow::build(
                        "Sistem Yerel Tanımı (System Locale)",
                        &data.system_locale,
                        true,
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build("Varsayılan Dil (Language)", "Türkçe (tr_TR)", false, theme)
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "SİSTEM DİLİ VE YEREL AYARLAR (LOCALE)",
                    locale_card_content.into(),
                    theme,
                ));

                // 2. Timezone & NTP Sync Card
                let ntp_btn = PillButton::build(
                    if self.is_acting {
                        "⏳ İşleniyor..."
                    } else if data.ntp_enabled {
                        "⏰ NTP Senkronizasyonu (Etkin)"
                    } else {
                        "⏰ NTP Senkronizasyonu (Kapalı)"
                    },
                    if data.ntp_enabled {
                        ButtonVariant::Primary
                    } else {
                        ButtonVariant::Tonal
                    },
                    if self.is_acting {
                        None
                    } else {
                        Some(LanguageRegionMessage::SetNtp(!data.ntp_enabled))
                    },
                    theme,
                );

                let mut tz_row = row![text("Zaman Dilimi:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center);

                let tzones: [(&str, &str); 4] = [
                    ("Europe/Istanbul", "🇹🇷 Europe/Istanbul"),
                    ("UTC", "🌐 UTC"),
                    ("Europe/Berlin", "🇩🇪 Europe/Berlin"),
                    ("America/New_York", "🇺🇸 America/New_York"),
                ];

                for (tz_code, tz_label) in tzones {
                    let is_sel = data.timezone == tz_code;
                    let btn = PillButton::build(
                        tz_label,
                        if is_sel {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_acting || is_sel {
                            None
                        } else {
                            Some(LanguageRegionMessage::SetTimezone(tz_code.to_string()))
                        },
                        theme,
                    );
                    tz_row = tz_row.push(btn);
                }

                let tz_card_content: Column<'a, LanguageRegionMessage> = column![
                    InfoRow::build("Aktif Zaman Dilimi", &data.timezone, true, theme),
                    Divider::build(theme),
                    tz_row,
                    Divider::build(theme),
                    row![
                        InfoRow::build(
                            "Otomatik Saat Senkronizasyonu (NTP)",
                            if data.ntp_synchronized {
                                "Senkronize Edildi (Aktif)"
                            } else if data.ntp_enabled {
                                "Etkinleştirildi"
                            } else {
                                "Devre Dışı"
                            },
                            data.ntp_enabled,
                            theme
                        ),
                        iced::widget::horizontal_space(),
                        ntp_btn
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "ZAMAN DİLİMİ VE SAAT SENKRONİZASYONU (TIMEDATECTL)",
                    tz_card_content.into(),
                    theme,
                ));

                // 3. Keyboard Layout Card
                let mut kbd_row = row![text("Klavye Düzeni:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center);

                let layout_labels: [(&str, &str); 6] = [
                    ("tr", "tr (Türkçe Q)"),
                    ("us", "us (English US)"),
                    ("de", "de (Deutsch)"),
                    ("fr", "fr (Français)"),
                    ("es", "es (Español)"),
                    ("gb", "gb (English UK)"),
                ];

                for (l_code, l_label) in layout_labels {
                    let is_sel = data.keyboard_layout == l_code;
                    let is_avail = data.available_layouts.contains(&l_code);
                    let btn = PillButton::build(
                        l_label,
                        if is_sel {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_acting || is_sel || !is_avail {
                            None
                        } else {
                            Some(LanguageRegionMessage::SetKeyboardLayout(l_code.to_string()))
                        },
                        theme,
                    );
                    kbd_row = kbd_row.push(btn);
                }

                let kbd_card_content: Column<'a, LanguageRegionMessage> = column![
                    InfoRow::build("Aktif Klavye Haritası", &data.keyboard_layout, true, theme),
                    Divider::build(theme),
                    kbd_row
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "KLAVYE HARİTASI VE GİRİŞ METODU (LOCALECTL)",
                    kbd_card_content.into(),
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
    fn test_language_region_page_lifecycle() {
        let (mut page, _task) = LanguageRegionPage::new();
        assert!(matches!(page.state, LanguageRegionViewState::Loading));

        let service = SettingsService::new();
        let system_locale = service
            .get_system_locale()
            .unwrap_or_else(|_| "LANG=tr_TR.UTF-8".to_string());
        let (keyboard_layout, keyboard_model) = service
            .get_keyboard_layouts()
            .unwrap_or(("tr".to_string(), "pc105".to_string()));
        let available_layouts = service.get_available_keyboard_layouts();

        let data = Box::new(LanguageRegionData {
            system_locale,
            keyboard_layout,
            keyboard_model,
            timezone: "Europe/Istanbul".to_string(),
            ntp_enabled: true,
            ntp_synchronized: true,
            available_layouts,
        });

        let _ = page.update(LanguageRegionMessage::LanguageRegionLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, LanguageRegionViewState::Loaded(_)));
    }
}
