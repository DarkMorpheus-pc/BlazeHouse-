use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::audio::AudioService;
use crate::app::error::BlazeError;
use crate::audio::AudioDevice;
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
pub struct SoundData {
    pub server_name: String,
    pub output_volume: Option<u32>,
    pub input_volume: Option<u32>,
    pub muted: bool,
    pub outputs: Vec<AudioDevice>,
    pub inputs: Vec<AudioDevice>,
}

#[derive(Debug, Clone)]
pub enum SoundMessage {
    LoadSound,
    SoundLoaded(Result<Box<SoundData>, BlazeError>),
    SetOutputVolume(u32),
    SetInputVolume(u32),
    ToggleMute(bool),
    SetDefaultOutput(String),
    SetDefaultInput(String),
    ActionFinished(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum SoundViewState {
    Loading,
    Loaded(Box<SoundData>),
    Error(BlazeError),
}

pub struct SoundPage {
    pub state: SoundViewState,
    pub is_refreshing: bool,
    pub is_acting: bool,
}

impl SoundPage {
    pub fn new() -> (Self, Task<SoundMessage>) {
        let page = Self {
            state: SoundViewState::Loading,
            is_refreshing: true,
            is_acting: false,
        };
        let task = Task::perform(
            async move {
                let service = AudioService::new();
                let server_name = service
                    .get_server_name()
                    .unwrap_or_else(|_| "Bilinmiyor".to_string());
                let output_volume = service.get_output_volume().unwrap_or(None);
                let input_volume = service.get_input_volume().unwrap_or(None);
                let muted = service.is_muted().unwrap_or(false);
                let outputs = service.get_outputs().unwrap_or_default();
                let inputs = service.get_inputs().unwrap_or_default();
                Ok(Box::new(SoundData {
                    server_name,
                    output_volume,
                    input_volume,
                    muted,
                    outputs,
                    inputs,
                }))
            },
            SoundMessage::SoundLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: SoundMessage) -> Task<SoundMessage> {
        match message {
            SoundMessage::LoadSound => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = AudioService::new();
                        let server_name = service.get_server_name()?;
                        let output_volume = service.get_output_volume()?;
                        let input_volume = service.get_input_volume()?;
                        let muted = service.is_muted()?;
                        let outputs = service.get_outputs()?;
                        let inputs = service.get_inputs()?;
                        Ok(Box::new(SoundData {
                            server_name,
                            output_volume,
                            input_volume,
                            muted,
                            outputs,
                            inputs,
                        }))
                    },
                    SoundMessage::SoundLoaded,
                )
            }
            SoundMessage::SoundLoaded(res) => {
                self.is_refreshing = false;
                self.is_acting = false;
                match res {
                    Ok(data) => {
                        self.state = SoundViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = SoundViewState::Error(e);
                    }
                }
                Task::none()
            }
            SoundMessage::SetOutputVolume(vol) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let service = AudioService::new();
                        service.set_output_volume(vol)
                    },
                    SoundMessage::ActionFinished,
                )
            }
            SoundMessage::SetInputVolume(vol) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let service = AudioService::new();
                        service.set_input_volume(vol)
                    },
                    SoundMessage::ActionFinished,
                )
            }
            SoundMessage::ToggleMute(mute) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let service = AudioService::new();
                        service.set_muted(mute)
                    },
                    SoundMessage::ActionFinished,
                )
            }
            SoundMessage::SetDefaultOutput(id) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                let target = id.clone();
                Task::perform(
                    async move {
                        let service = AudioService::new();
                        service.set_default_output(&target)
                    },
                    SoundMessage::ActionFinished,
                )
            }
            SoundMessage::SetDefaultInput(id) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                let target = id.clone();
                Task::perform(
                    async move {
                        let service = AudioService::new();
                        service.set_default_input(&target)
                    },
                    SoundMessage::ActionFinished,
                )
            }
            SoundMessage::ActionFinished(res) => {
                self.is_acting = false;
                match res {
                    Ok(()) => {
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = AudioService::new();
                                let server_name = service.get_server_name()?;
                                let output_volume = service.get_output_volume()?;
                                let input_volume = service.get_input_volume()?;
                                let muted = service.is_muted()?;
                                let outputs = service.get_outputs()?;
                                let inputs = service.get_inputs()?;
                                Ok(Box::new(SoundData {
                                    server_name,
                                    output_volume,
                                    input_volume,
                                    muted,
                                    outputs,
                                    inputs,
                                }))
                            },
                            SoundMessage::SoundLoaded,
                        )
                    }
                    Err(e) => {
                        self.state = SoundViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, SoundMessage> {
        match self.state {
            SoundViewState::Loading => {
                LoadingState::build("PipeWire / PulseAudio ses sunucusu sorgulanıyor...", theme)
            }
            SoundViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(SoundMessage::LoadSound),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            SoundViewState::Loaded(ref data) => {
                let sub_title =
                    text("PipeWire / PulseAudio Ses Mimarisi, Seviye Kontrolü ve Aygıtlar")
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
                        Some(SoundMessage::LoadSound)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let out_vol_str = match data.output_volume {
                    Some(0) => "%0",
                    Some(25) => "%25",
                    Some(50) => "%50",
                    Some(75) => "%75",
                    Some(100) => "%100",
                    Some(_) => "%50 (Aktif)",
                    None => "Bilinmiyor",
                };

                let in_vol_str = match data.input_volume {
                    Some(0) => "%0",
                    Some(25) => "%25",
                    Some(50) => "%50",
                    Some(75) => "%75",
                    Some(100) => "%100",
                    Some(_) => "%50 (Aktif)",
                    None => "Bilinmiyor",
                };

                let metric_out = MetricCard::build(
                    "ANA ÇIKIŞ SESİ",
                    out_vol_str,
                    if data.muted {
                        "Sessiz Mode Aktif"
                    } else {
                        "Hoparlör / Kulaklık Çıkışı"
                    },
                    !data.muted,
                    theme,
                );

                let metric_in = MetricCard::build(
                    "MİKROFON GİRİŞİ",
                    in_vol_str,
                    "Aktif Ses Kayıt Seviyesi",
                    false,
                    theme,
                );

                let metric_server = MetricCard::build(
                    "SES SUNUCUSU MİMARİSİ",
                    &data.server_name,
                    "PipeWire / PulseAudio Daemon",
                    false,
                    theme,
                );

                let metric_grid = row![metric_out, metric_in, metric_server]
                    .spacing(SpacingTokens::LG)
                    .width(Length::Fill);

                let mut page_col: Column<'a, SoundMessage> = column![top_bar, metric_grid]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. Master Volume Control Card
                let mute_badge = if data.muted {
                    Badge::build("MUTED", BadgeStyle::Warning, theme)
                } else {
                    Badge::build("UNMUTED", BadgeStyle::Success, theme)
                };

                let mute_btn = PillButton::build(
                    if self.is_acting {
                        "⏳ İşleniyor..."
                    } else if data.muted {
                        "🔊 Sesi Aç"
                    } else {
                        "🔇 Sessize Al"
                    },
                    if data.muted {
                        ButtonVariant::Primary
                    } else {
                        ButtonVariant::Tonal
                    },
                    if self.is_acting {
                        None
                    } else {
                        Some(SoundMessage::ToggleMute(!data.muted))
                    },
                    theme,
                );

                let mut out_preset_row = row![text("Çıkış Seviyesi Preset:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::SM)
                .align_y(Alignment::Center);

                let presets: [(u32, &str); 5] = [
                    (0, "%0"),
                    (25, "%25"),
                    (50, "%50"),
                    (75, "%75"),
                    (100, "%100"),
                ];

                for (vol_val, label) in presets {
                    let is_sel = data.output_volume == Some(vol_val);
                    let btn = PillButton::build(
                        label,
                        if is_sel {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_acting {
                            None
                        } else {
                            Some(SoundMessage::SetOutputVolume(vol_val))
                        },
                        theme,
                    );
                    out_preset_row = out_preset_row.push(btn);
                }

                let mut in_preset_row = row![text("Giriş Seviyesi Preset:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::SM)
                .align_y(Alignment::Center);

                for (vol_val, label) in presets {
                    let is_sel = data.input_volume == Some(vol_val);
                    let btn = PillButton::build(
                        label,
                        if is_sel {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_acting {
                            None
                        } else {
                            Some(SoundMessage::SetInputVolume(vol_val))
                        },
                        theme,
                    );
                    in_preset_row = in_preset_row.push(btn);
                }

                let master_card_content: Column<'a, SoundMessage> = column![
                    row![
                        InfoRow::build("Ses Sunucusu Mimarisi", &data.server_name, true, theme),
                        iced::widget::horizontal_space(),
                        mute_badge,
                        mute_btn
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill),
                    Divider::build(theme),
                    out_preset_row,
                    Divider::build(theme),
                    in_preset_row,
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "ANA SES SEVİYESİ VE SESSİZLİK KONTROLÜ",
                    "Hoparlör / Kulaklık Çıkışı, Mikrofon Girişi ve Mute Ayarları",
                    master_card_content.into(),
                    theme,
                ));

                // 2. Output Devices (Sinks) Section
                let mut out_col: Column<'a, SoundMessage> = column![].spacing(SpacingTokens::SM);

                if data.outputs.is_empty() {
                    out_col = out_col.push(InfoRow::build(
                        "Ses Çıkış Aygıtı",
                        "Sistemde kullanılabilir ses çıkış aygıtı bulunamadı",
                        false,
                        theme,
                    ));
                } else {
                    for (idx, dev) in data.outputs.iter().enumerate() {
                        if idx > 0 {
                            out_col = out_col.push(Divider::build(theme));
                        }

                        let dev_id = dev.id.clone();
                        let is_def = dev.is_default;

                        let select_btn = PillButton::build(
                            if is_def {
                                "⭐ Varsayılan Çıkış"
                            } else {
                                "📌 Varsayılan Yap"
                            },
                            if is_def {
                                ButtonVariant::Primary
                            } else {
                                ButtonVariant::Tonal
                            },
                            if self.is_acting || is_def {
                                None
                            } else {
                                Some(SoundMessage::SetDefaultOutput(dev_id))
                            },
                            theme,
                        );

                        let sink_badge = if is_def {
                            Badge::build("PRIMARY SINK", BadgeStyle::Success, theme)
                        } else {
                            Badge::build("AVAILABLE SINK", BadgeStyle::Neutral, theme)
                        };

                        let detail_str = format!("Aygıt ID: {}", dev.id);

                        let dev_row = row![
                            InfoRow::build(&dev.name, &detail_str, dev.is_default, theme),
                            sink_badge,
                            iced::widget::horizontal_space(),
                            select_btn
                        ]
                        .spacing(SpacingTokens::MD)
                        .align_y(Alignment::Center)
                        .width(Length::Fill);

                        out_col = out_col.push(dev_row);
                    }
                }

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "SES ÇIKIŞ AYGITLARI (SINKS)",
                    "Hoparlör, Kulaklık, HDMI ve USB Ses Çıkış Portları",
                    out_col.into(),
                    theme,
                ));

                // 3. Input Devices (Sources) Section
                let mut in_col: Column<'a, SoundMessage> = column![].spacing(SpacingTokens::SM);

                if data.inputs.is_empty() {
                    in_col = in_col.push(InfoRow::build(
                        "Mikrofon / Ses Girişi",
                        "Sistemde kullanılabilir ses giriş aygıtı bulunamadı",
                        false,
                        theme,
                    ));
                } else {
                    for (idx, dev) in data.inputs.iter().enumerate() {
                        if idx > 0 {
                            in_col = in_col.push(Divider::build(theme));
                        }

                        let dev_id = dev.id.clone();
                        let is_def = dev.is_default;

                        let select_btn = PillButton::build(
                            if is_def {
                                "⭐ Varsayılan Giriş"
                            } else {
                                "📌 Varsayılan Yap"
                            },
                            if is_def {
                                ButtonVariant::Primary
                            } else {
                                ButtonVariant::Tonal
                            },
                            if self.is_acting || is_def {
                                None
                            } else {
                                Some(SoundMessage::SetDefaultInput(dev_id))
                            },
                            theme,
                        );

                        let source_badge = if is_def {
                            Badge::build("PRIMARY SOURCE", BadgeStyle::Success, theme)
                        } else {
                            Badge::build("AVAILABLE SOURCE", BadgeStyle::Neutral, theme)
                        };

                        let detail_str = format!("Aygıt ID: {}", dev.id);

                        let dev_row = row![
                            InfoRow::build(&dev.name, &detail_str, dev.is_default, theme),
                            source_badge,
                            iced::widget::horizontal_space(),
                            select_btn
                        ]
                        .spacing(SpacingTokens::MD)
                        .align_y(Alignment::Center)
                        .width(Length::Fill);

                        in_col = in_col.push(dev_row);
                    }
                }

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "SES GİRİŞ VE MİKROFON AYGITLARI (SOURCES)",
                    "Dahili Mikrofon, Harici Kulaklık Mikrofonu ve Ses Giriş Portları",
                    in_col.into(),
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
    fn test_sound_page_lifecycle() {
        let (mut page, _task) = SoundPage::new();
        assert!(matches!(page.state, SoundViewState::Loading));

        let service = AudioService::new();
        let server_name = service
            .get_server_name()
            .unwrap_or_else(|_| "Bilinmiyor".to_string());
        let output_volume = service.get_output_volume().unwrap_or(None);
        let input_volume = service.get_input_volume().unwrap_or(None);
        let muted = service.is_muted().unwrap_or(false);
        let outputs = service.get_outputs().unwrap_or_default();
        let inputs = service.get_inputs().unwrap_or_default();

        let data = Box::new(SoundData {
            server_name,
            output_volume,
            input_volume,
            muted,
            outputs,
            inputs,
        });

        let _ = page.update(SoundMessage::SoundLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, SoundViewState::Loaded(_)));
    }

    #[test]
    fn test_sound_page_error_and_refresh() {
        let (mut page, _task) = SoundPage::new();
        let err = BlazeError::ExecutionError {
            command: "wpctl".to_string(),
            stdout: "".to_string(),
            stderr: "WirePlumber error".to_string(),
        };
        let _ = page.update(SoundMessage::SoundLoaded(Err(err)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, SoundViewState::Error(_)));

        let _ = page.update(SoundMessage::LoadSound);
        assert!(page.is_refreshing);
    }
}
