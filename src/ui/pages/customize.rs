use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::settings::SettingsService;
use crate::shared_settings::SharedSettings;
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

#[derive(Debug, Clone)]
pub struct CustomizeData {
    pub settings: SharedSettings,
    pub config_path: String,
}

#[derive(Debug, Clone)]
pub enum CustomizeMessage {
    LoadCustomize,
    CustomizeLoaded(Result<Box<CustomizeData>, BlazeError>),
    SetThemeMode(String),
    SetAccentColor(String),
    ToggleReduceMotion(bool),
    ActionFinished(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum CustomizeViewState {
    Loading,
    Loaded(Box<CustomizeData>),
    Error(BlazeError),
}

pub struct CustomizePage {
    pub state: CustomizeViewState,
    pub is_refreshing: bool,
    pub is_acting: bool,
}

impl CustomizePage {
    pub fn new() -> (Self, Task<CustomizeMessage>) {
        let page = Self {
            state: CustomizeViewState::Loading,
            is_refreshing: true,
            is_acting: false,
        };
        let task = Task::perform(
            async move {
                let service = SettingsService::new();
                let settings = service.load_settings()?;
                let config_path = SharedSettings::config_path().display().to_string();
                Ok(Box::new(CustomizeData {
                    settings,
                    config_path,
                }))
            },
            CustomizeMessage::CustomizeLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: CustomizeMessage) -> Task<CustomizeMessage> {
        match message {
            CustomizeMessage::LoadCustomize => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = SettingsService::new();
                        let settings = service.load_settings()?;
                        let config_path = SharedSettings::config_path().display().to_string();
                        Ok(Box::new(CustomizeData {
                            settings,
                            config_path,
                        }))
                    },
                    CustomizeMessage::CustomizeLoaded,
                )
            }
            CustomizeMessage::CustomizeLoaded(res) => {
                self.is_refreshing = false;
                self.is_acting = false;
                match res {
                    Ok(data) => {
                        self.state = CustomizeViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = CustomizeViewState::Error(e);
                    }
                }
                Task::none()
            }
            CustomizeMessage::SetThemeMode(mode) => {
                if self.is_acting {
                    return Task::none();
                }
                if let CustomizeViewState::Loaded(ref mut data) = self.state {
                    data.settings.theme_mode = mode.clone();
                    self.is_acting = true;
                    let mut s = data.settings.clone();
                    s.theme_mode = mode;
                    return Task::perform(
                        async move {
                            let service = SettingsService::new();
                            service.save_settings(&s)
                        },
                        CustomizeMessage::ActionFinished,
                    );
                }
                Task::none()
            }
            CustomizeMessage::SetAccentColor(hex) => {
                if self.is_acting {
                    return Task::none();
                }
                if let CustomizeViewState::Loaded(ref mut data) = self.state {
                    data.settings.accent_color = hex.clone();
                    self.is_acting = true;
                    let mut s = data.settings.clone();
                    s.accent_color = hex;
                    return Task::perform(
                        async move {
                            let service = SettingsService::new();
                            service.save_settings(&s)
                        },
                        CustomizeMessage::ActionFinished,
                    );
                }
                Task::none()
            }
            CustomizeMessage::ToggleReduceMotion(enable) => {
                if self.is_acting {
                    return Task::none();
                }
                if let CustomizeViewState::Loaded(ref mut data) = self.state {
                    data.settings.reduce_motion = enable;
                    self.is_acting = true;
                    let mut s = data.settings.clone();
                    s.reduce_motion = enable;
                    return Task::perform(
                        async move {
                            let service = SettingsService::new();
                            service.save_settings(&s)
                        },
                        CustomizeMessage::ActionFinished,
                    );
                }
                Task::none()
            }
            CustomizeMessage::ActionFinished(res) => {
                self.is_acting = false;
                match res {
                    Ok(()) => {
                        // Re-read settings file to verify persistent parity
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = SettingsService::new();
                                let settings = service.load_settings()?;
                                let config_path =
                                    SharedSettings::config_path().display().to_string();
                                Ok(Box::new(CustomizeData {
                                    settings,
                                    config_path,
                                }))
                            },
                            CustomizeMessage::CustomizeLoaded,
                        )
                    }
                    Err(e) => {
                        self.state = CustomizeViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, CustomizeMessage> {
        match self.state {
            CustomizeViewState::Loading => {
                LoadingState::build("Kullanıcı arayüzü ve tema tercihleri yükleniyor...", theme)
            }
            CustomizeViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(CustomizeMessage::LoadCustomize),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            CustomizeViewState::Loaded(ref data) => {
                let sub_title =
                    text("Blaze House Görünüm Teması, Vurgu Renkleri ve Hareket Tercihleri")
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
                        Some(CustomizeMessage::LoadCustomize)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, CustomizeMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. Theme Mode Card
                let mut theme_row = row![text("Tema Seçimi:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center);

                let modes: [(&str, &str); 3] = [
                    ("dark", "🌙 Koyu Tema (Dark)"),
                    ("light", "☀️ Açık Tema (Light)"),
                    ("system", "💻 Sistem Teması"),
                ];

                for (m_code, m_label) in modes {
                    let is_sel = data.settings.theme_mode == m_code;
                    let btn = PillButton::build(
                        m_label,
                        if is_sel {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_acting || is_sel {
                            None
                        } else {
                            Some(CustomizeMessage::SetThemeMode(m_code.to_string()))
                        },
                        theme,
                    );
                    theme_row = theme_row.push(btn);
                }

                let theme_card_content: Column<'a, CustomizeMessage> = column![
                    InfoRow::build("Aktif Tema Modu", &data.settings.theme_mode, true, theme),
                    Divider::build(theme),
                    theme_row
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "GÖRÜNÜM VE TEMA MODU",
                    theme_card_content.into(),
                    theme,
                ));

                // 2. Accent Color Presets Card
                let mut accent_row = row![text("Vurgu Rengi:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center);

                let colors: [(&str, &str); 6] = [
                    ("#8A8F9C", "🩶 Slate"),
                    ("#3B82F6", "🟦 Sapphire"),
                    ("#10B981", "🟩 Emerald"),
                    ("#F59E0B", "🟨 Amber"),
                    ("#EF4444", "🟥 Crimson"),
                    ("#8B5CF6", "🟪 Amethyst"),
                ];

                for (hex, label) in colors {
                    let is_sel = data.settings.accent_color.to_lowercase() == hex.to_lowercase();
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
                            Some(CustomizeMessage::SetAccentColor(hex.to_string()))
                        },
                        theme,
                    );
                    accent_row = accent_row.push(btn);
                }

                let accent_card_content: Column<'a, CustomizeMessage> = column![
                    InfoRow::build(
                        "Seçili Vurgu Rengi Kodu",
                        &data.settings.accent_color,
                        true,
                        theme
                    ),
                    Divider::build(theme),
                    accent_row
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "VURGU RENGİ (ACCENT COLOR)",
                    accent_card_content.into(),
                    theme,
                ));

                // 3. Motion & Persistence Information Card
                let motion_btn = PillButton::build(
                    if self.is_acting {
                        "⏳ İşleniyor..."
                    } else if data.settings.reduce_motion {
                        "✨ Azaltılmış Hareket (Etkin)"
                    } else {
                        "✨ Normal Animasyonlar"
                    },
                    if data.settings.reduce_motion {
                        ButtonVariant::Primary
                    } else {
                        ButtonVariant::Tonal
                    },
                    if self.is_acting {
                        None
                    } else {
                        Some(CustomizeMessage::ToggleReduceMotion(
                            !data.settings.reduce_motion,
                        ))
                    },
                    theme,
                );

                let motion_card_content: Column<'a, CustomizeMessage> = column![
                    row![
                        InfoRow::build(
                            "Azaltılmış Hareket (Reduce Motion)",
                            if data.settings.reduce_motion {
                                "Etkin (Animasyonlar Kapalı)"
                            } else {
                                "Devre Dışı (Standart)"
                            },
                            data.settings.reduce_motion,
                            theme
                        ),
                        iced::widget::horizontal_space(),
                        motion_btn
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill),
                    Divider::build(theme),
                    InfoRow::build("Yapılandırma Dosyası", &data.config_path, false, theme)
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "HAREKET VE KALICI AYARLAR (PERSISTENCE)",
                    motion_card_content.into(),
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
    fn test_customize_page_lifecycle() {
        let (mut page, _task) = CustomizePage::new();
        assert!(matches!(page.state, CustomizeViewState::Loading));

        let service = SettingsService::new();
        let settings = service.load_settings().unwrap_or_default();
        let config_path = SharedSettings::config_path().display().to_string();

        let data = Box::new(CustomizeData {
            settings,
            config_path,
        });

        let _ = page.update(CustomizeMessage::CustomizeLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, CustomizeViewState::Loaded(_)));
    }
}
