use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::settings::SettingsService;
use crate::capabilities::{has_cmd, run_cmd};
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

#[derive(Debug, Clone)]
pub struct AccessibilityData {
    pub reduce_motion: bool,
    pub high_contrast: bool,
    pub text_scaling: f32,
    pub screen_reader_available: bool,
    pub screen_reader_active: bool,
}

#[derive(Debug, Clone)]
pub enum AccessibilityMessage {
    LoadAccessibility,
    AccessibilityLoaded(Result<Box<AccessibilityData>, BlazeError>),
    ToggleReduceMotion(bool),
    ToggleHighContrast(bool),
    SetTextScaling(f32),
    ActionFinished(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum AccessibilityViewState {
    Loading,
    Loaded(Box<AccessibilityData>),
    Error(BlazeError),
}

pub struct AccessibilityPage {
    pub state: AccessibilityViewState,
    pub is_refreshing: bool,
    pub is_acting: bool,
}

impl AccessibilityPage {
    pub fn new() -> (Self, Task<AccessibilityMessage>) {
        let page = Self {
            state: AccessibilityViewState::Loading,
            is_refreshing: true,
            is_acting: false,
        };
        let task = Task::perform(
            async move {
                let service = SettingsService::new();
                let settings = service.load_settings().unwrap_or_default();
                let reduce_motion = settings.reduce_motion;

                let mut high_contrast = false;
                let mut text_scaling = 1.0f32;
                let mut screen_reader_active = false;
                let screen_reader_available = has_cmd("orca");

                if has_cmd("gsettings") {
                    let (ok_hc, out_hc) = run_cmd(
                        "gsettings",
                        &["get", "org.gnome.desktop.interface", "high-contrast"],
                    );
                    if ok_hc && out_hc.trim() == "true" {
                        high_contrast = true;
                    }

                    let (ok_ts, out_ts) = run_cmd(
                        "gsettings",
                        &["get", "org.gnome.desktop.interface", "text-scaling-factor"],
                    );
                    if ok_ts {
                        if let Ok(ts) = out_ts.trim().parse::<f32>() {
                            text_scaling = ts;
                        }
                    }

                    let (ok_sr, out_sr) = run_cmd(
                        "gsettings",
                        &[
                            "get",
                            "org.gnome.desktop.a11y.applications",
                            "screen-reader-enabled",
                        ],
                    );
                    if ok_sr && out_sr.trim() == "true" {
                        screen_reader_active = true;
                    }
                }

                Ok(Box::new(AccessibilityData {
                    reduce_motion,
                    high_contrast,
                    text_scaling,
                    screen_reader_available,
                    screen_reader_active,
                }))
            },
            AccessibilityMessage::AccessibilityLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: AccessibilityMessage) -> Task<AccessibilityMessage> {
        match message {
            AccessibilityMessage::LoadAccessibility => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = SettingsService::new();
                        let settings = service.load_settings()?;
                        let reduce_motion = settings.reduce_motion;

                        let mut high_contrast = false;
                        let mut text_scaling = 1.0f32;
                        let mut screen_reader_active = false;
                        let screen_reader_available = has_cmd("orca");

                        if has_cmd("gsettings") {
                            let (ok_hc, out_hc) = run_cmd(
                                "gsettings",
                                &["get", "org.gnome.desktop.interface", "high-contrast"],
                            );
                            if ok_hc && out_hc.trim() == "true" {
                                high_contrast = true;
                            }

                            let (ok_ts, out_ts) = run_cmd(
                                "gsettings",
                                &["get", "org.gnome.desktop.interface", "text-scaling-factor"],
                            );
                            if ok_ts {
                                if let Ok(ts) = out_ts.trim().parse::<f32>() {
                                    text_scaling = ts;
                                }
                            }

                            let (ok_sr, out_sr) = run_cmd(
                                "gsettings",
                                &[
                                    "get",
                                    "org.gnome.desktop.a11y.applications",
                                    "screen-reader-enabled",
                                ],
                            );
                            if ok_sr && out_sr.trim() == "true" {
                                screen_reader_active = true;
                            }
                        }

                        Ok(Box::new(AccessibilityData {
                            reduce_motion,
                            high_contrast,
                            text_scaling,
                            screen_reader_available,
                            screen_reader_active,
                        }))
                    },
                    AccessibilityMessage::AccessibilityLoaded,
                )
            }
            AccessibilityMessage::AccessibilityLoaded(res) => {
                self.is_refreshing = false;
                self.is_acting = false;
                match res {
                    Ok(data) => {
                        self.state = AccessibilityViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = AccessibilityViewState::Error(e);
                    }
                }
                Task::none()
            }
            AccessibilityMessage::ToggleReduceMotion(enable) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let service = SettingsService::new();
                        let mut settings = service.load_settings().unwrap_or_default();
                        settings.reduce_motion = enable;
                        service.save_settings(&settings)
                    },
                    AccessibilityMessage::ActionFinished,
                )
            }
            AccessibilityMessage::ToggleHighContrast(enable) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        if has_cmd("gsettings") {
                            let flag = if enable { "true" } else { "false" };
                            let (ok, _) = run_cmd(
                                "gsettings",
                                &["set", "org.gnome.desktop.interface", "high-contrast", flag],
                            );
                            if ok {
                                return Ok(());
                            }
                        }
                        Err(BlazeError::ExecutionError {
                            command: "gsettings set high-contrast".to_string(),
                            stdout: String::new(),
                            stderr: "Yüksek karşıtlık ayarı uygulanamadı".to_string(),
                        })
                    },
                    AccessibilityMessage::ActionFinished,
                )
            }
            AccessibilityMessage::SetTextScaling(scale) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        if has_cmd("gsettings") {
                            let (ok, _) = run_cmd(
                                "gsettings",
                                &[
                                    "set",
                                    "org.gnome.desktop.interface",
                                    "text-scaling-factor",
                                    &format!("{:.2}", scale),
                                ],
                            );
                            if ok {
                                return Ok(());
                            }
                        }
                        Err(BlazeError::ExecutionError {
                            command: "gsettings set text-scaling-factor".to_string(),
                            stdout: String::new(),
                            stderr: "Metin ölçeklendirme ayarı uygulanamadı".to_string(),
                        })
                    },
                    AccessibilityMessage::ActionFinished,
                )
            }
            AccessibilityMessage::ActionFinished(res) => {
                self.is_acting = false;
                match res {
                    Ok(()) => {
                        // Re-read accessibility status
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = SettingsService::new();
                                let settings = service.load_settings()?;
                                let reduce_motion = settings.reduce_motion;

                                let mut high_contrast = false;
                                let mut text_scaling = 1.0f32;
                                let mut screen_reader_active = false;
                                let screen_reader_available = has_cmd("orca");

                                if has_cmd("gsettings") {
                                    let (ok_hc, out_hc) = run_cmd(
                                        "gsettings",
                                        &["get", "org.gnome.desktop.interface", "high-contrast"],
                                    );
                                    if ok_hc && out_hc.trim() == "true" {
                                        high_contrast = true;
                                    }

                                    let (ok_ts, out_ts) = run_cmd(
                                        "gsettings",
                                        &[
                                            "get",
                                            "org.gnome.desktop.interface",
                                            "text-scaling-factor",
                                        ],
                                    );
                                    if ok_ts {
                                        if let Ok(ts) = out_ts.trim().parse::<f32>() {
                                            text_scaling = ts;
                                        }
                                    }

                                    let (ok_sr, out_sr) = run_cmd(
                                        "gsettings",
                                        &[
                                            "get",
                                            "org.gnome.desktop.a11y.applications",
                                            "screen-reader-enabled",
                                        ],
                                    );
                                    if ok_sr && out_sr.trim() == "true" {
                                        screen_reader_active = true;
                                    }
                                }

                                Ok(Box::new(AccessibilityData {
                                    reduce_motion,
                                    high_contrast,
                                    text_scaling,
                                    screen_reader_available,
                                    screen_reader_active,
                                }))
                            },
                            AccessibilityMessage::AccessibilityLoaded,
                        )
                    }
                    Err(e) => {
                        self.state = AccessibilityViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, AccessibilityMessage> {
        match self.state {
            AccessibilityViewState::Loading => LoadingState::build(
                "Erişilebilirlik ve masaüstü yardım özellikleri taranıyor...",
                theme,
            ),
            AccessibilityViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(AccessibilityMessage::LoadAccessibility),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            AccessibilityViewState::Loaded(ref data) => {
                let sub_title = text(
                    "Masaüstü Erişilebilirlik, Metin Boyutu, Karşıtlık ve Ekran Okuyucu Ayarları",
                )
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
                        Some(AccessibilityMessage::LoadAccessibility)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, AccessibilityMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. Motion & Display Accessibility Card
                let motion_btn = PillButton::build(
                    if self.is_acting {
                        "⏳ İşleniyor..."
                    } else if data.reduce_motion {
                        "✨ Azaltılmış Hareket (Etkin)"
                    } else {
                        "✨ Normal Animasyonlar"
                    },
                    if data.reduce_motion {
                        ButtonVariant::Primary
                    } else {
                        ButtonVariant::Tonal
                    },
                    if self.is_acting {
                        None
                    } else {
                        Some(AccessibilityMessage::ToggleReduceMotion(
                            !data.reduce_motion,
                        ))
                    },
                    theme,
                );

                let hc_btn = PillButton::build(
                    if self.is_acting {
                        "⏳ İşleniyor..."
                    } else if data.high_contrast {
                        "👁️ Yüksek Karşıtlık (Etkin)"
                    } else {
                        "👁️ Standart Karşıtlık"
                    },
                    if data.high_contrast {
                        ButtonVariant::Primary
                    } else {
                        ButtonVariant::Tonal
                    },
                    if self.is_acting {
                        None
                    } else {
                        Some(AccessibilityMessage::ToggleHighContrast(
                            !data.high_contrast,
                        ))
                    },
                    theme,
                );

                let mut ts_row = row![text("Metin Ölçeği:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center);

                let scales: [(f32, &str); 3] = [
                    (1.0, "Normal (%100)"),
                    (1.25, "Büyük (%125)"),
                    (1.5, "Çok Büyük (%150)"),
                ];

                for (sc, label) in scales {
                    let is_sel = (data.text_scaling - sc).abs() < 0.05;
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
                            Some(AccessibilityMessage::SetTextScaling(sc))
                        },
                        theme,
                    );
                    ts_row = ts_row.push(btn);
                }

                let a11y_card_content: Column<'a, AccessibilityMessage> = column![
                    row![
                        InfoRow::build(
                            "Azaltılmış Hareket (Reduce Motion)",
                            if data.reduce_motion {
                                "Etkin"
                            } else {
                                "Devre Dışı"
                            },
                            data.reduce_motion,
                            theme
                        ),
                        iced::widget::horizontal_space(),
                        motion_btn
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill),
                    Divider::build(theme),
                    row![
                        InfoRow::build(
                            "Yüksek Karşıtlık (High Contrast)",
                            if data.high_contrast {
                                "Etkin"
                            } else {
                                "Devre Dışı"
                            },
                            data.high_contrast,
                            theme
                        ),
                        iced::widget::horizontal_space(),
                        hc_btn
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill),
                    Divider::build(theme),
                    ts_row
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "GÖRSEL VE HAREKET ERİŞİLEBİLİRLİĞİ",
                    a11y_card_content.into(),
                    theme,
                ));

                // 2. Screen Reader Card
                let sr_status = if data.screen_reader_active {
                    "Aktif (Çalışıyor)"
                } else if data.screen_reader_available {
                    "Kurulu (Orca Mevcut)"
                } else {
                    "Kurulu Değil (NOT AVAILABLE)"
                };

                let sr_card_content: Column<'a, AccessibilityMessage> = column![
                    InfoRow::build(
                        "Ekran Okuyucu (Orca Screen Reader)",
                        sr_status,
                        data.screen_reader_active,
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build(
                        "Klavye Odaklanma (Keyboard Focus)",
                        "Tab / Shift+Tab ile Görsel Odak Çerçevesi Desteklenir",
                        true,
                        theme
                    )
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "EKRAN OKUYUCU VE KLAVYE ERİŞİLEBİLİRLİĞİ",
                    sr_card_content.into(),
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
    fn test_accessibility_page_lifecycle() {
        let (mut page, _task) = AccessibilityPage::new();
        assert!(matches!(page.state, AccessibilityViewState::Loading));

        let data = Box::new(AccessibilityData {
            reduce_motion: false,
            high_contrast: false,
            text_scaling: 1.0,
            screen_reader_available: true,
            screen_reader_active: false,
        });

        let _ = page.update(AccessibilityMessage::AccessibilityLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, AccessibilityViewState::Loaded(_)));
    }
}
