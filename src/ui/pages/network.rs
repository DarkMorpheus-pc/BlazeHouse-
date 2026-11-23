use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::network::NetworkService;
use crate::network::{ActiveConnection, NetworkInterfaceInfo, WifiNetwork};
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
pub struct NetworkData {
    pub wifi_enabled: bool,
    pub active_connections: Vec<ActiveConnection>,
    pub interfaces: Vec<NetworkInterfaceInfo>,
    pub wifi_networks: Vec<WifiNetwork>,
}

#[derive(Debug, Clone)]
pub enum NetworkMessage {
    LoadNetwork,
    NetworkLoaded(Result<Box<NetworkData>, BlazeError>),
    ToggleWifi(bool),
    WifiToggled(Result<(), BlazeError>),
    ConnectWifi(String),
    DisconnectWifi(String),
    ActionFinished(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum NetworkViewState {
    Loading,
    Loaded(Box<NetworkData>),
    Error(BlazeError),
}

pub struct NetworkPage {
    pub state: NetworkViewState,
    pub is_refreshing: bool,
    pub is_acting: bool,
    pub active_action_ssid: Option<String>,
}

impl NetworkPage {
    pub fn new() -> (Self, Task<NetworkMessage>) {
        let page = Self {
            state: NetworkViewState::Loading,
            is_refreshing: true,
            is_acting: false,
            active_action_ssid: None,
        };
        let task = Task::perform(
            async move {
                let service = NetworkService::new();
                let wifi_enabled = service.is_wifi_enabled().unwrap_or(false);
                let active_connections = service.get_active_connections().unwrap_or_default();
                let interfaces = service.get_interfaces().unwrap_or_default();
                let wifi_networks = if wifi_enabled {
                    service.get_wifi_networks().unwrap_or_default()
                } else {
                    Vec::new()
                };
                Ok(Box::new(NetworkData {
                    wifi_enabled,
                    active_connections,
                    interfaces,
                    wifi_networks,
                }))
            },
            NetworkMessage::NetworkLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: NetworkMessage) -> Task<NetworkMessage> {
        match message {
            NetworkMessage::LoadNetwork => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = NetworkService::new();
                        let wifi_enabled = service.is_wifi_enabled()?;
                        let active_connections = service.get_active_connections()?;
                        let interfaces = service.get_interfaces()?;
                        let wifi_networks = if wifi_enabled {
                            service.get_wifi_networks()?
                        } else {
                            Vec::new()
                        };
                        Ok(Box::new(NetworkData {
                            wifi_enabled,
                            active_connections,
                            interfaces,
                            wifi_networks,
                        }))
                    },
                    NetworkMessage::NetworkLoaded,
                )
            }
            NetworkMessage::NetworkLoaded(res) => {
                self.is_refreshing = false;
                self.is_acting = false;
                self.active_action_ssid = None;
                match res {
                    Ok(data) => {
                        self.state = NetworkViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = NetworkViewState::Error(e);
                    }
                }
                Task::none()
            }
            NetworkMessage::ToggleWifi(enable) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let service = NetworkService::new();
                        service.set_wifi_enabled(enable)
                    },
                    NetworkMessage::WifiToggled,
                )
            }
            NetworkMessage::WifiToggled(res) => {
                self.is_acting = false;
                match res {
                    Ok(()) => {
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = NetworkService::new();
                                let wifi_enabled = service.is_wifi_enabled()?;
                                let active_connections = service.get_active_connections()?;
                                let interfaces = service.get_interfaces()?;
                                let wifi_networks = if wifi_enabled {
                                    service.get_wifi_networks()?
                                } else {
                                    Vec::new()
                                };
                                Ok(Box::new(NetworkData {
                                    wifi_enabled,
                                    active_connections,
                                    interfaces,
                                    wifi_networks,
                                }))
                            },
                            NetworkMessage::NetworkLoaded,
                        )
                    }
                    Err(e) => {
                        self.state = NetworkViewState::Error(e);
                        Task::none()
                    }
                }
            }
            NetworkMessage::ConnectWifi(ssid) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                self.active_action_ssid = Some(ssid.clone());
                let target = ssid.clone();
                Task::perform(
                    async move {
                        let service = NetworkService::new();
                        service.connect_wifi(&target)
                    },
                    NetworkMessage::ActionFinished,
                )
            }
            NetworkMessage::DisconnectWifi(ssid) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                self.active_action_ssid = Some(ssid.clone());
                let target = ssid.clone();
                Task::perform(
                    async move {
                        let service = NetworkService::new();
                        service.disconnect_wifi(&target)
                    },
                    NetworkMessage::ActionFinished,
                )
            }
            NetworkMessage::ActionFinished(res) => {
                self.is_acting = false;
                self.active_action_ssid = None;
                match res {
                    Ok(()) => {
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = NetworkService::new();
                                let wifi_enabled = service.is_wifi_enabled()?;
                                let active_connections = service.get_active_connections()?;
                                let interfaces = service.get_interfaces()?;
                                let wifi_networks = if wifi_enabled {
                                    service.get_wifi_networks()?
                                } else {
                                    Vec::new()
                                };
                                Ok(Box::new(NetworkData {
                                    wifi_enabled,
                                    active_connections,
                                    interfaces,
                                    wifi_networks,
                                }))
                            },
                            NetworkMessage::NetworkLoaded,
                        )
                    }
                    Err(e) => {
                        self.state = NetworkViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, NetworkMessage> {
        match self.state {
            NetworkViewState::Loading => {
                LoadingState::build("Ağ arayüzleri ve Wi-Fi kapsama alanı taranıyor...", theme)
            }
            NetworkViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(NetworkMessage::LoadNetwork),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            NetworkViewState::Loaded(ref data) => {
                let sub_title = text("Wi-Fi Adaptörü, Aktif Bağlantılar ve Donanım Arayüzleri")
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
                        Some(NetworkMessage::LoadNetwork)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, NetworkMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. Hero Wi-Fi Power & Status Card
                let status_badge = if !data.wifi_enabled {
                    Badge::build("WI-FI OFF", BadgeStyle::Neutral, theme)
                } else if !data.active_connections.is_empty() {
                    Badge::build("CONNECTED", BadgeStyle::Success, theme)
                } else {
                    Badge::build("DISCONNECTED", BadgeStyle::Warning, theme)
                };

                let wifi_toggle_btn = PillButton::build(
                    if self.is_acting {
                        "⏳ İşleniyor..."
                    } else if data.wifi_enabled {
                        "📶 Wi-Fi Kapat"
                    } else {
                        "📡 Wi-Fi Etkinleştir"
                    },
                    if data.wifi_enabled {
                        ButtonVariant::Tonal
                    } else {
                        ButtonVariant::Primary
                    },
                    if self.is_acting {
                        None
                    } else {
                        Some(NetworkMessage::ToggleWifi(!data.wifi_enabled))
                    },
                    theme,
                );

                let hero_header = row![
                    column![
                        text(if data.wifi_enabled {
                            "Wi-Fi Kablosuz Ağ Adaptörü Etkin"
                        } else {
                            "Wi-Fi Kablosuz Ağ Adaptörü Kapalı"
                        })
                        .size(TypographyTokens::PAGE_TITLE_SIZE)
                        .color(theme.colors.text_primary),
                        text(if data.wifi_enabled {
                            "Aktif sinyal taraması ve bağlantı yönetimi çalışıyor."
                        } else {
                            "Kablosuz ağları taramak için adaptörü açın."
                        })
                        .size(TypographyTokens::CAPTION_SIZE)
                        .color(theme.colors.text_secondary)
                    ]
                    .spacing(SpacingTokens::XS),
                    iced::widget::horizontal_space(),
                    status_badge,
                    wifi_toggle_btn
                ]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center)
                .width(Length::Fill);

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "KABLOSUZ AĞ (WI-FI) KONTROL VE ADAPTÖR DURUMU",
                    "Ağ Adaptörü Güç Yönetimi ve Bağlantı Statüsü",
                    hero_header.into(),
                    theme,
                ));

                // 2. Active Connections & Interfaces Section
                let mut if_col: Column<'a, NetworkMessage> = column![].spacing(SpacingTokens::SM);

                if !data.active_connections.is_empty() {
                    for (idx, conn) in data.active_connections.iter().enumerate() {
                        if idx > 0 {
                            if_col = if_col.push(Divider::build(theme));
                        }
                        let conn_badge = Badge::build("ACTIVE", BadgeStyle::Success, theme);
                        let conn_row = row![
                            InfoRow::build(
                                &format!("Bağlı Ağ: {}", conn.name),
                                &format!("Tür: {}", conn.conn_type),
                                true,
                                theme
                            ),
                            iced::widget::horizontal_space(),
                            conn_badge
                        ]
                        .align_y(Alignment::Center)
                        .width(Length::Fill);

                        if_col = if_col.push(conn_row);
                    }
                    if_col = if_col.push(Divider::build(theme));
                }

                if data.interfaces.is_empty() {
                    if_col = if_col.push(InfoRow::build(
                        "Ağ Arayüzü",
                        "Sistemde tespit edilen aktif ağ adaptörü bulunamadı",
                        false,
                        theme,
                    ));
                } else {
                    for (idx, iface) in data.interfaces.iter().enumerate() {
                        if idx > 0 {
                            if_col = if_col.push(Divider::build(theme));
                        }
                        let info_str = format!(
                            "Tür: {} | Durum: {} | IP: {} | GW: {}",
                            iface.if_type, iface.state, iface.ip_address, iface.gateway
                        );
                        if_col = if_col.push(InfoRow::build(&iface.name, &info_str, true, theme));
                    }
                }

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "DONANIM AĞ ARAYÜZLERİ VE IP ADRESLERİ",
                    "Ethernet, Wi-Fi Arayüz Parametreleri ve IP Yönlendirmeleri",
                    if_col.into(),
                    theme,
                ));

                // 3. Wi-Fi Networks Scanner Section
                let mut wifi_col: Column<'a, NetworkMessage> = column![].spacing(SpacingTokens::SM);

                if !data.wifi_enabled {
                    wifi_col = wifi_col.push(InfoRow::build(
                        "Wi-Fi Taraması",
                        "Kablosuz ağ taraması yapabilmek için Wi-Fi adaptörünü açın",
                        false,
                        theme,
                    ));
                } else if data.wifi_networks.is_empty() {
                    wifi_col = wifi_col.push(InfoRow::build(
                        "Kapsama Alanı",
                        "Yakındaki kullanılabilir Wi-Fi ağı bulunamadı",
                        false,
                        theme,
                    ));
                } else {
                    for (idx, net) in data.wifi_networks.iter().enumerate() {
                        if idx > 0 {
                            wifi_col = wifi_col.push(Divider::build(theme));
                        }

                        let is_busy = self.active_action_ssid.as_deref() == Some(&net.ssid);
                        let ssid_clone = net.ssid.clone();

                        let act_btn = PillButton::build(
                            if is_busy {
                                "⏳ Bağlanılıyor..."
                            } else if net.active {
                                "🔴 Bağlantıyı Kes"
                            } else {
                                "📶 Bağlan"
                            },
                            if net.active {
                                ButtonVariant::Tonal
                            } else {
                                ButtonVariant::Primary
                            },
                            if self.is_acting {
                                None
                            } else if net.active {
                                Some(NetworkMessage::DisconnectWifi(ssid_clone))
                            } else {
                                Some(NetworkMessage::ConnectWifi(ssid_clone))
                            },
                            theme,
                        );

                        let sig_text = match net.signal {
                            s if s >= 75 => "%75+ STRONG",
                            s if s >= 40 => "%40+ GOOD",
                            _ => "%0+ WEAK",
                        };
                        let signal_badge = if net.signal >= 75 {
                            Badge::build(sig_text, BadgeStyle::Success, theme)
                        } else if net.signal >= 40 {
                            Badge::build(sig_text, BadgeStyle::Info, theme)
                        } else {
                            Badge::build(sig_text, BadgeStyle::Warning, theme)
                        };

                        let detail_str = format!("Güvenlik: {}", net.security);

                        let wifi_item_row = row![
                            InfoRow::build(&net.ssid, &detail_str, net.active, theme),
                            signal_badge,
                            iced::widget::horizontal_space(),
                            act_btn
                        ]
                        .spacing(SpacingTokens::MD)
                        .align_y(Alignment::Center)
                        .width(Length::Fill);

                        wifi_col = wifi_col.push(wifi_item_row);
                    }
                }

                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "KULLANILABİLİR WI-FI AĞLARI",
                    "Kapsama Alanındaki Kablosuz Ağlar ve Sinyal Seviyeleri",
                    wifi_col.into(),
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
    fn test_network_page_lifecycle() {
        let (mut page, _task) = NetworkPage::new();
        assert!(matches!(page.state, NetworkViewState::Loading));

        let service = NetworkService::new();
        let wifi_enabled = service.is_wifi_enabled().unwrap_or(false);
        let active_connections = service.get_active_connections().unwrap_or_default();
        let interfaces = service.get_interfaces().unwrap_or_default();
        let wifi_networks = if wifi_enabled {
            service.get_wifi_networks().unwrap_or_default()
        } else {
            Vec::new()
        };

        let data = Box::new(NetworkData {
            wifi_enabled,
            active_connections,
            interfaces,
            wifi_networks,
        });

        let _ = page.update(NetworkMessage::NetworkLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, NetworkViewState::Loaded(_)));
    }

    #[test]
    fn test_network_page_error_and_refresh() {
        let (mut page, _task) = NetworkPage::new();
        let err = BlazeError::ExecutionError {
            command: "nmcli".to_string(),
            stdout: "".to_string(),
            stderr: "NetworkManager error".to_string(),
        };
        let _ = page.update(NetworkMessage::NetworkLoaded(Err(err)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, NetworkViewState::Error(_)));

        let _ = page.update(NetworkMessage::LoadNetwork);
        assert!(page.is_refreshing);
    }
}
