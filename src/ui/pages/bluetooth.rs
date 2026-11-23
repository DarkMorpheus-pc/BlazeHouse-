use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::bluetooth::BluetoothService;
use crate::app::error::BlazeError;
use crate::bluetooth::BluetoothDevice;
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
pub struct BluetoothData {
    pub powered: bool,
    pub discoverable: bool,
    pub devices: Vec<BluetoothDevice>,
}

#[derive(Debug, Clone)]
pub enum BluetoothMessage {
    LoadBluetooth,
    BluetoothLoaded(Result<Box<BluetoothData>, BlazeError>),
    TogglePower(bool),
    ToggleDiscoverable(bool),
    ConnectDevice(String),
    DisconnectDevice(String),
    PairDevice(String),
    RemoveDevice(String),
    ActionFinished(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum BluetoothViewState {
    Loading,
    Loaded(Box<BluetoothData>),
    Error(BlazeError),
}

pub struct BluetoothPage {
    pub state: BluetoothViewState,
    pub is_refreshing: bool,
    pub is_acting: bool,
    pub active_action_mac: Option<String>,
}

impl BluetoothPage {
    pub fn new() -> (Self, Task<BluetoothMessage>) {
        let page = Self {
            state: BluetoothViewState::Loading,
            is_refreshing: true,
            is_acting: false,
            active_action_mac: None,
        };
        let task = Task::perform(
            async move {
                let service = BluetoothService::new();
                let powered = service.is_powered().unwrap_or(false);
                let discoverable = service.is_discoverable().unwrap_or(false);
                let devices = if powered {
                    service.get_devices().unwrap_or_default()
                } else {
                    Vec::new()
                };
                Ok(Box::new(BluetoothData {
                    powered,
                    discoverable,
                    devices,
                }))
            },
            BluetoothMessage::BluetoothLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: BluetoothMessage) -> Task<BluetoothMessage> {
        match message {
            BluetoothMessage::LoadBluetooth => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = BluetoothService::new();
                        let powered = service.is_powered()?;
                        let discoverable = service.is_discoverable()?;
                        let devices = if powered {
                            service.get_devices()?
                        } else {
                            Vec::new()
                        };
                        Ok(Box::new(BluetoothData {
                            powered,
                            discoverable,
                            devices,
                        }))
                    },
                    BluetoothMessage::BluetoothLoaded,
                )
            }
            BluetoothMessage::BluetoothLoaded(res) => {
                self.is_refreshing = false;
                self.is_acting = false;
                self.active_action_mac = None;
                match res {
                    Ok(data) => {
                        self.state = BluetoothViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = BluetoothViewState::Error(e);
                    }
                }
                Task::none()
            }
            BluetoothMessage::TogglePower(enable) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let service = BluetoothService::new();
                        service.set_powered(enable)
                    },
                    BluetoothMessage::ActionFinished,
                )
            }
            BluetoothMessage::ToggleDiscoverable(enable) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let service = BluetoothService::new();
                        service.set_discoverable(enable)
                    },
                    BluetoothMessage::ActionFinished,
                )
            }
            BluetoothMessage::ConnectDevice(mac) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                self.active_action_mac = Some(mac.clone());
                let target = mac.clone();
                Task::perform(
                    async move {
                        let service = BluetoothService::new();
                        service.connect_device(&target)
                    },
                    BluetoothMessage::ActionFinished,
                )
            }
            BluetoothMessage::DisconnectDevice(mac) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                self.active_action_mac = Some(mac.clone());
                let target = mac.clone();
                Task::perform(
                    async move {
                        let service = BluetoothService::new();
                        service.disconnect_device(&target)
                    },
                    BluetoothMessage::ActionFinished,
                )
            }
            BluetoothMessage::PairDevice(mac) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                self.active_action_mac = Some(mac.clone());
                let target = mac.clone();
                Task::perform(
                    async move {
                        let service = BluetoothService::new();
                        service.pair_device(&target)
                    },
                    BluetoothMessage::ActionFinished,
                )
            }
            BluetoothMessage::RemoveDevice(mac) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                self.active_action_mac = Some(mac.clone());
                let target = mac.clone();
                Task::perform(
                    async move {
                        let service = BluetoothService::new();
                        service.remove_device(&target)
                    },
                    BluetoothMessage::ActionFinished,
                )
            }
            BluetoothMessage::ActionFinished(res) => {
                self.is_acting = false;
                self.active_action_mac = None;
                match res {
                    Ok(()) => {
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = BluetoothService::new();
                                let powered = service.is_powered()?;
                                let discoverable = service.is_discoverable()?;
                                let devices = if powered {
                                    service.get_devices()?
                                } else {
                                    Vec::new()
                                };
                                Ok(Box::new(BluetoothData {
                                    powered,
                                    discoverable,
                                    devices,
                                }))
                            },
                            BluetoothMessage::BluetoothLoaded,
                        )
                    }
                    Err(e) => {
                        self.state = BluetoothViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, BluetoothMessage> {
        match self.state {
            BluetoothViewState::Loading => {
                LoadingState::build("Bluetooth adaptörü ve aygıtlar taranıyor...", theme)
            }
            BluetoothViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(BluetoothMessage::LoadBluetooth),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            BluetoothViewState::Loaded(ref data) => {
                let sub_title = text("Bluetooth Adaptör Güç Durumu ve Eşleşen Aygıtlar")
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
                        Some(BluetoothMessage::LoadBluetooth)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, BluetoothMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. Adapter Control Hero Card
                let power_badge = if data.powered {
                    Badge::build("ADAPTER ONLINE", BadgeStyle::Success, theme)
                } else {
                    Badge::build("ADAPTER OFF", BadgeStyle::Neutral, theme)
                };

                let discover_badge = if data.discoverable {
                    Badge::build("DISCOVERABLE", BadgeStyle::Info, theme)
                } else {
                    Badge::build("HIDDEN", BadgeStyle::Neutral, theme)
                };

                let power_btn = PillButton::build(
                    if self.is_acting {
                        "⏳ İşleniyor..."
                    } else if data.powered {
                        "🔵 Adaptörü Kapat"
                    } else {
                        "⚪ Adaptörü Aç"
                    },
                    if data.powered {
                        ButtonVariant::Tonal
                    } else {
                        ButtonVariant::Primary
                    },
                    if self.is_acting {
                        None
                    } else {
                        Some(BluetoothMessage::TogglePower(!data.powered))
                    },
                    theme,
                );

                let discoverable_btn = PillButton::build(
                    if self.is_acting {
                        "⏳ İşleniyor..."
                    } else if data.discoverable {
                        "👀 Gizle"
                    } else {
                        "🙈 Bulunabilir Yap"
                    },
                    if data.discoverable {
                        ButtonVariant::Tonal
                    } else {
                        ButtonVariant::Primary
                    },
                    if self.is_acting || !data.powered {
                        None
                    } else {
                        Some(BluetoothMessage::ToggleDiscoverable(!data.discoverable))
                    },
                    theme,
                );

                let adapter_row = row![
                    column![
                        text(if data.powered {
                            "Bluetooth Radyo Adaptörü Etkin"
                        } else {
                            "Bluetooth Radyo Adaptörü Kapalı"
                        })
                        .size(TypographyTokens::PAGE_TITLE_SIZE)
                        .color(theme.colors.text_primary),
                        text(if data.powered {
                            "Eşleşmiş cihazlar bağlananabilir ve görünür durumdadır."
                        } else {
                            "Cihazları aramak veya bağlanmak için adaptörü açın."
                        })
                        .size(TypographyTokens::CAPTION_SIZE)
                        .color(theme.colors.text_secondary)
                    ]
                    .spacing(SpacingTokens::XS),
                    iced::widget::horizontal_space(),
                    power_badge,
                    discover_badge,
                    discoverable_btn,
                    power_btn
                ]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center)
                .width(Length::Fill);

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "BLUETOOTH ADAPTÖR VE GÜÇ KONTROLÜ",
                    "Radyo Güç Anahtarı ve Cihaz Bulunabilirlik Ayarları",
                    adapter_row.into(),
                    theme,
                ));

                // 2. Devices Section
                let mut dev_col: Column<'a, BluetoothMessage> =
                    column![].spacing(SpacingTokens::SM);

                if !data.powered {
                    dev_col = dev_col.push(InfoRow::build(
                        "Aygıt Listesi",
                        "Bluetooth aygıtlarını görüntülemek için adaptörü açın",
                        false,
                        theme,
                    ));
                } else if data.devices.is_empty() {
                    dev_col = dev_col.push(InfoRow::build(
                        "Kayıtlı Aygıtlar",
                        "Sistemde eşleştirilmiş veya algılanmış Bluetooth aygıtı bulunamadı",
                        false,
                        theme,
                    ));
                } else {
                    for (idx, dev) in data.devices.iter().enumerate() {
                        if idx > 0 {
                            dev_col = dev_col.push(Divider::build(theme));
                        }

                        let is_busy = self.active_action_mac.as_deref() == Some(&dev.mac);
                        let mac_clone = dev.mac.clone();

                        let conn_btn = PillButton::build(
                            if is_busy {
                                "⏳ İşleniyor..."
                            } else if dev.connected {
                                "🔴 Kes"
                            } else {
                                "🔗 Bağlan"
                            },
                            if dev.connected {
                                ButtonVariant::Tonal
                            } else {
                                ButtonVariant::Primary
                            },
                            if self.is_acting {
                                None
                            } else if dev.connected {
                                Some(BluetoothMessage::DisconnectDevice(mac_clone.clone()))
                            } else {
                                Some(BluetoothMessage::ConnectDevice(mac_clone.clone()))
                            },
                            theme,
                        );

                        let remove_btn = PillButton::build(
                            "🗑 Eşleşmeyi Kaldır",
                            ButtonVariant::Danger,
                            if self.is_acting {
                                None
                            } else {
                                Some(BluetoothMessage::RemoveDevice(mac_clone))
                            },
                            theme,
                        );

                        let dev_badge = if dev.connected {
                            Badge::build("CONNECTED", BadgeStyle::Success, theme)
                        } else if dev.paired {
                            Badge::build("PAIRED", BadgeStyle::Info, theme)
                        } else {
                            Badge::build("DISCOVERED", BadgeStyle::Neutral, theme)
                        };

                        let detail_str = format!("MAC: {}", dev.mac);

                        let dev_row = row![
                            InfoRow::build(&dev.name, &detail_str, dev.connected, theme),
                            dev_badge,
                            iced::widget::horizontal_space(),
                            conn_btn,
                            remove_btn
                        ]
                        .spacing(SpacingTokens::MD)
                        .align_y(Alignment::Center)
                        .width(Length::Fill);

                        dev_col = dev_col.push(dev_row);
                    }
                }

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "EŞLEŞEN VE ALGILANAN BLUETOOTH AYGITLARI",
                    "Kulaklıklar, Fare, Klavye ve Eşleşmiş Aygıt Listesi",
                    dev_col.into(),
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
    fn test_bluetooth_page_lifecycle() {
        let (mut page, _task) = BluetoothPage::new();
        assert!(matches!(page.state, BluetoothViewState::Loading));

        let service = BluetoothService::new();
        let powered = service.is_powered().unwrap_or(false);
        let discoverable = service.is_discoverable().unwrap_or(false);
        let devices = if powered {
            service.get_devices().unwrap_or_default()
        } else {
            Vec::new()
        };

        let data = Box::new(BluetoothData {
            powered,
            discoverable,
            devices,
        });

        let _ = page.update(BluetoothMessage::BluetoothLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, BluetoothViewState::Loaded(_)));
    }

    #[test]
    fn test_bluetooth_page_error_and_refresh() {
        let (mut page, _task) = BluetoothPage::new();
        let err = BlazeError::ExecutionError {
            command: "bluetoothctl".to_string(),
            stdout: "".to_string(),
            stderr: "BlueZ error".to_string(),
        };
        let _ = page.update(BluetoothMessage::BluetoothLoaded(Err(err)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, BluetoothViewState::Error(_)));

        let _ = page.update(BluetoothMessage::LoadBluetooth);
        assert!(page.is_refreshing);
    }
}
