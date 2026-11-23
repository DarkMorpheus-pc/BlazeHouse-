use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::security::SecurityService;
use crate::security::{CyberTool, SecurityAudit};
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

#[derive(Debug, Clone)]
pub struct SecurityData {
    pub audit: SecurityAudit,
    pub cyber_tools: Vec<CyberTool>,
}

#[derive(Debug, Clone)]
pub enum SecurityMessage {
    LoadSecurity,
    SecurityLoaded(Result<Box<SecurityData>, BlazeError>),
    RepairSetting(String),
    RunDiagnosticTool(String),
    DiagnosticFinished(Result<String, BlazeError>),
    ActionFinished(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum SecurityViewState {
    Loading,
    Loaded(Box<SecurityData>),
    Error(BlazeError),
}

pub struct SecurityPage {
    pub state: SecurityViewState,
    pub is_refreshing: bool,
    pub is_acting: bool,
    pub active_action_key: Option<String>,
    pub status_msg: Option<String>,
}

impl SecurityPage {
    pub fn new() -> (Self, Task<SecurityMessage>) {
        let page = Self {
            state: SecurityViewState::Loading,
            is_refreshing: true,
            is_acting: false,
            active_action_key: None,
            status_msg: None,
        };
        let task = Task::perform(
            async move {
                let service = SecurityService::new();
                let audit = service.run_audit()?;
                let cyber_tools = service.get_cyber_tools()?;
                Ok(Box::new(SecurityData { audit, cyber_tools }))
            },
            SecurityMessage::SecurityLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: SecurityMessage) -> Task<SecurityMessage> {
        match message {
            SecurityMessage::LoadSecurity => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = SecurityService::new();
                        let audit = service.run_audit()?;
                        let cyber_tools = service.get_cyber_tools()?;
                        Ok(Box::new(SecurityData { audit, cyber_tools }))
                    },
                    SecurityMessage::SecurityLoaded,
                )
            }
            SecurityMessage::SecurityLoaded(res) => {
                self.is_refreshing = false;
                self.is_acting = false;
                self.active_action_key = None;
                match res {
                    Ok(data) => {
                        self.state = SecurityViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = SecurityViewState::Error(e);
                    }
                }
                Task::none()
            }
            SecurityMessage::RepairSetting(key) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                self.active_action_key = Some(key.clone());
                let target = key.clone();
                Task::perform(
                    async move {
                        let service = SecurityService::new();
                        service.repair_setting(&target)
                    },
                    SecurityMessage::ActionFinished,
                )
            }
            SecurityMessage::RunDiagnosticTool(cmd) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                self.active_action_key = Some(cmd.clone());
                let target_cmd = cmd.clone();
                let safe_args: Vec<String> = match target_cmd.as_str() {
                    "nmap" => vec!["--version".to_string()],
                    "tcpdump" => vec!["--version".to_string()],
                    "tshark" => vec!["--version".to_string()],
                    "aircrack-ng" => vec!["--help".to_string()],
                    "nikto" => vec!["-Version".to_string()],
                    "sqlmap" => vec!["--version".to_string()],
                    "john" => vec!["--list=build-info".to_string()],
                    "hydra" => vec!["-h".to_string()],
                    "ufw" => vec!["--version".to_string()],
                    "firewall-cmd" => vec!["--version".to_string()],
                    "fail2ban-client" => vec!["--version".to_string()],
                    _ => vec!["--version".to_string()],
                };
                Task::perform(
                    async move {
                        let service = SecurityService::new();
                        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
                        let run_res = service
                            .run_cyber_tool(&target_cmd, &safe_args, false, tx)
                            .await;
                        let mut lines = Vec::new();
                        while let Some(line) = rx.recv().await {
                            if !line.trim().is_empty() {
                                lines.push(line);
                            }
                        }
                        run_res?;
                        let summary = lines
                            .into_iter()
                            .take(6)
                            .collect::<Vec<_>>()
                            .join(" | ");
                        Ok(format!("{}: {}", target_cmd, summary))
                    },
                    SecurityMessage::DiagnosticFinished,
                )
            }
            SecurityMessage::DiagnosticFinished(res) => {
                self.is_acting = false;
                self.active_action_key = None;
                match res {
                    Ok(output) => {
                        self.status_msg = Some(output);
                    }
                    Err(e) => {
                        self.status_msg = Some(format!("Teşhis hatası: {}", e));
                    }
                }
                Task::none()
            }
            SecurityMessage::ActionFinished(res) => {
                self.is_acting = false;
                self.active_action_key = None;
                match res {
                    Ok(()) => {
                        // Re-read security audit state to verify actual Linux security posture
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = SecurityService::new();
                                let audit = service.run_audit()?;
                                let cyber_tools = service.get_cyber_tools()?;
                                Ok(Box::new(SecurityData { audit, cyber_tools }))
                            },
                            SecurityMessage::SecurityLoaded,
                        )
                    }
                    Err(e) => {
                        self.state = SecurityViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, SecurityMessage> {
        match self.state {
            SecurityViewState::Loading => LoadingState::build(
                "Linux güvenlik parametreleri ve denetim mekanizmaları taranıyor...",
                theme,
            ),
            SecurityViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(SecurityMessage::LoadSecurity),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            SecurityViewState::Loaded(ref data) => {
                let sub_title =
                    text("Linux Güvenlik Durumu, Çekirdek Korumaları ve Teşhis Araçları")
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
                        Some(SecurityMessage::LoadSecurity)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, SecurityMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                if let Some(ref msg) = self.status_msg {
                    page_col = page_col.push(GlassCard::build(
                        "SON TEŞHİS ÇIKTISI",
                        text(msg)
                            .size(TypographyTokens::CAPTION_SIZE)
                            .color(theme.colors.text_primary)
                            .into(),
                        theme,
                    ));
                }

                // 1. Core Security Indicators Card
                let audit = &data.audit;

                let ufw_btn = PillButton::build(
                    if self.active_action_key.as_deref() == Some("ufw") {
                        "⏳ İşleniyor..."
                    } else if audit.ufw_active {
                        "🟢 Etkin"
                    } else {
                        "⚙️ Etkinleştir (Polkit)"
                    },
                    if audit.ufw_active {
                        ButtonVariant::Tonal
                    } else {
                        ButtonVariant::Primary
                    },
                    if self.is_acting || audit.ufw_active {
                        None
                    } else {
                        Some(SecurityMessage::RepairSetting("ufw".to_string()))
                    },
                    theme,
                );

                let kptr_btn = PillButton::build(
                    if self.active_action_key.as_deref() == Some("kptr") {
                        "⏳ İşleniyor..."
                    } else if audit.kptr_restricted {
                        "🟢 Kısıtlı"
                    } else {
                        "⚙️ Kısıtla (Polkit)"
                    },
                    if audit.kptr_restricted {
                        ButtonVariant::Tonal
                    } else {
                        ButtonVariant::Primary
                    },
                    if self.is_acting || audit.kptr_restricted {
                        None
                    } else {
                        Some(SecurityMessage::RepairSetting("kptr".to_string()))
                    },
                    theme,
                );

                let dmesg_btn = PillButton::build(
                    if self.active_action_key.as_deref() == Some("dmesg") {
                        "⏳ İşleniyor..."
                    } else if audit.dmesg_restricted {
                        "🟢 Kısıtlı"
                    } else {
                        "⚙️ Kısıtla (Polkit)"
                    },
                    if audit.dmesg_restricted {
                        ButtonVariant::Tonal
                    } else {
                        ButtonVariant::Primary
                    },
                    if self.is_acting || audit.dmesg_restricted {
                        None
                    } else {
                        Some(SecurityMessage::RepairSetting("dmesg".to_string()))
                    },
                    theme,
                );

                let core_sec_col: Column<'a, SecurityMessage> = column![
                    row![
                        InfoRow::build(
                            "Güvenlik Duvarı (UFW / Firewalld)",
                            &audit.firewall_state,
                            audit.ufw_active,
                            theme
                        ),
                        iced::widget::horizontal_space(),
                        ufw_btn
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill),
                    Divider::build(theme),
                    row![
                        InfoRow::build(
                            "Çekirdek Sembol Koruması (kptr_restrict)",
                            &audit.kptr_state,
                            audit.kptr_restricted,
                            theme
                        ),
                        iced::widget::horizontal_space(),
                        kptr_btn
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill),
                    Divider::build(theme),
                    row![
                        InfoRow::build(
                            "Çekirdek Günlük Koruması (dmesg_restrict)",
                            &audit.dmesg_state,
                            audit.dmesg_restricted,
                            theme
                        ),
                        iced::widget::horizontal_space(),
                        dmesg_btn
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill),
                    Divider::build(theme),
                    InfoRow::build(
                        "AppArmor Kısıtlama Modülü",
                        &audit.apparmor_state,
                        audit.apparmor_active,
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build(
                        "SELinux Durumu",
                        &audit.selinux_state,
                        audit.selinux_state.contains("ENFORCING"),
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build(
                        "Güvenli Önyükleme (Secure Boot)",
                        &audit.secure_boot_state,
                        audit.secure_boot_state == "ACTIVE",
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build(
                        "Çekirdek Kilitleme (Kernel Lockdown)",
                        &audit.lockdown_state,
                        audit.lockdown_state.contains("ENFORCING"),
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build(
                        "Adres Alanı Rastgeleleştirme (ASLR)",
                        &audit.aslr_state,
                        audit.aslr_state.contains("FULL"),
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build(
                        "Otomatik Güvenlik Güncellemeleri",
                        &audit.unattended_state,
                        audit.unattended_upgrades,
                        theme
                    ),
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "GÜVENLİK DURUMU VE ÇEKİRDEK KORUMA GÖSTERGELERİ",
                    core_sec_col.into(),
                    theme,
                ));

                // 2. Security Diagnostic Tools Card
                let mut tools_col: Column<'a, SecurityMessage> =
                    column![].spacing(SpacingTokens::SM);

                for (idx, tool) in data.cyber_tools.iter().enumerate() {
                    if idx > 0 {
                        tools_col = tools_col.push(Divider::build(theme));
                    }

                    let is_busy = self.active_action_key.as_deref() == Some(&tool.command);
                    let cmd_clone = tool.command.clone();

                    let tool_btn = PillButton::build(
                        if is_busy {
                            "⏳ Çalıştırılıyor..."
                        } else if tool.installed {
                            "🔬 Teşhis Testi Yap"
                        } else {
                            "❌ Kurulu Değil"
                        },
                        if tool.installed {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_acting || !tool.installed {
                            None
                        } else {
                            Some(SecurityMessage::RunDiagnosticTool(cmd_clone))
                        },
                        theme,
                    );

                    let detail_str = format!(
                        "Komut: {} | Root Gereksinimi: {} | Durum: {}",
                        tool.command,
                        if tool.requires_root {
                            "Evet (Polkit)"
                        } else {
                            "Hayır"
                        },
                        if tool.installed {
                            "Sistemde Mevcut"
                        } else {
                            "Eksik Paket"
                        }
                    );

                    let tool_row = row![
                        InfoRow::build(&tool.name, &detail_str, tool.installed, theme),
                        iced::widget::horizontal_space(),
                        tool_btn
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                    tools_col = tools_col.push(tool_row);
                }

                page_col = page_col.push(GlassCard::build(
                    "GÜVENLİK TEŞHİS VE ANALİZ ARAÇLARI",
                    tools_col.into(),
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
    fn test_security_page_lifecycle() {
        let (mut page, _task) = SecurityPage::new();
        assert!(matches!(page.state, SecurityViewState::Loading));

        let service = SecurityService::new();
        let audit = service.run_audit().unwrap();
        let cyber_tools = service.get_cyber_tools().unwrap();

        let data = Box::new(SecurityData { audit, cyber_tools });

        let _ = page.update(SecurityMessage::SecurityLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, SecurityViewState::Loaded(_)));
    }
}
