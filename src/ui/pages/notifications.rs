use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
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
pub struct NotificationsData {
    pub daemon_name: String,
    pub daemon_status: String,
    pub dnd_active: bool,
    pub control_supported: bool,
}

#[derive(Debug, Clone)]
pub enum NotificationsMessage {
    LoadNotifications,
    NotificationsLoaded(Result<Box<NotificationsData>, BlazeError>),
    ToggleDnd(bool),
    ActionFinished(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum NotificationsViewState {
    Loading,
    Loaded(Box<NotificationsData>),
    Error(BlazeError),
}

pub struct NotificationsPage {
    pub state: NotificationsViewState,
    pub is_refreshing: bool,
    pub is_acting: bool,
}

fn is_proc_running(proc_name: &str) -> bool {
    if has_cmd("pgrep") {
        let (ok, _) = run_cmd("pgrep", &["-x", proc_name]);
        if ok {
            return true;
        }
        let (ok_f, _) = run_cmd("pgrep", &["-f", proc_name]);
        if ok_f {
            return true;
        }
    }
    false
}

fn detect_notifications_state() -> NotificationsData {
    let desktop = std::env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase();

    // 1. swaync
    if is_proc_running("swaync") && has_cmd("swaync-client") {
        let (ok, out) = run_cmd("swaync-client", &["-D"]);
        return NotificationsData {
            daemon_name: "Sway Notification Center (swaync)".to_string(),
            daemon_status: "AKTİF".to_string(),
            dnd_active: ok && out.trim() == "true",
            control_supported: true,
        };
    }

    // 2. noctalia / quickshell (SolarUI / Niri)
    if is_proc_running("quickshell")
        || is_proc_running("noctalia")
        || desktop.contains("solarui")
        || desktop.contains("niri")
    {
        let home = std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
        let noctalia_settings = std::path::PathBuf::from(home).join(".config/noctalia/settings.json");
        let mut dnd = false;
        if let Ok(content) = std::fs::read_to_string(&noctalia_settings) {
            if let Ok(val) = serde_json::from_str::<serde_json::Value>(&content) {
                dnd = val
                    .get("doNotDisturb")
                    .and_then(|v| v.as_bool())
                    .unwrap_or(false);
            }
        }
        return NotificationsData {
            daemon_name: "SolarUI / Noctalia Notification Daemon (quickshell)".to_string(),
            daemon_status: "AKTİF".to_string(),
            dnd_active: dnd,
            control_supported: true,
        };
    }

    // 3. mako
    if is_proc_running("mako") && has_cmd("makoctl") {
        let (ok, out) = run_cmd("makoctl", &["mode"]);
        return NotificationsData {
            daemon_name: "Mako Notification Daemon".to_string(),
            daemon_status: "AKTİF".to_string(),
            dnd_active: ok && out.contains("do-not-disturb"),
            control_supported: true,
        };
    }

    // 4. dunst
    if is_proc_running("dunst") && has_cmd("dunstctl") {
        let (ok, out) = run_cmd("dunstctl", &["is-paused"]);
        return NotificationsData {
            daemon_name: "Dunst Notification Daemon".to_string(),
            daemon_status: "AKTİF".to_string(),
            dnd_active: ok && out.trim() == "true",
            control_supported: true,
        };
    }

    // 5. KDE Plasma (plasmashell)
    if is_proc_running("plasmashell") || desktop.contains("kde") || desktop.contains("plasma") {
        return NotificationsData {
            daemon_name: "KDE Plasma Notifications (plasmashell)".to_string(),
            daemon_status: "AKTİF".to_string(),
            dnd_active: false,
            control_supported: false,
        };
    }

    // 6. Cinnamon
    if (is_proc_running("cinnamon") || desktop.contains("cinnamon")) && has_cmd("gsettings") {
        let (ok_c, out_c) = run_cmd(
            "gsettings",
            &[
                "get",
                "org.cinnamon.desktop.notifications",
                "display-notifications",
            ],
        );
        if ok_c {
            return NotificationsData {
                daemon_name: "Cinnamon Desktop Notifications".to_string(),
                daemon_status: "AKTİF".to_string(),
                dnd_active: out_c.trim() == "false",
                control_supported: true,
            };
        }
    }

    // 7. GNOME Shell
    if (is_proc_running("gnome-shell") || desktop.contains("gnome")) && has_cmd("gsettings") {
        let (ok_g, out_g) = run_cmd(
            "gsettings",
            &["get", "org.gnome.desktop.notifications", "show-banners"],
        );
        if ok_g {
            return NotificationsData {
                daemon_name: "GNOME Desktop Notifications".to_string(),
                daemon_status: "AKTİF".to_string(),
                dnd_active: out_g.trim() == "false",
                control_supported: true,
            };
        }
    }

    // Fallbacks if CLI tools exist
    if has_cmd("swaync-client") {
        let (ok, out) = run_cmd("swaync-client", &["-D"]);
        if ok {
            return NotificationsData {
                daemon_name: "Sway Notification Center (swaync)".to_string(),
                daemon_status: "AKTİF".to_string(),
                dnd_active: out.trim() == "true",
                control_supported: true,
            };
        }
    }
    if has_cmd("dunstctl") {
        let (ok, out) = run_cmd("dunstctl", &["is-paused"]);
        if ok {
            return NotificationsData {
                daemon_name: "Dunst Notification Daemon".to_string(),
                daemon_status: "AKTİF".to_string(),
                dnd_active: out.trim() == "true",
                control_supported: true,
            };
        }
    }
    if has_cmd("notify-send") {
        NotificationsData {
            daemon_name: "Desktop Notification Subsystem (notify-send)".to_string(),
            daemon_status: "MEVCUT (Sadece Gönderim Desteklenir)".to_string(),
            dnd_active: false,
            control_supported: false,
        }
    } else {
        NotificationsData {
            daemon_name: "Bildirim Servisi".to_string(),
            daemon_status: "NOT AVAILABLE / KONTROL EDİLEMEZ".to_string(),
            dnd_active: false,
            control_supported: false,
        }
    }
}

impl NotificationsPage {
    pub fn new() -> (Self, Task<NotificationsMessage>) {
        let page = Self {
            state: NotificationsViewState::Loading,
            is_refreshing: true,
            is_acting: false,
        };
        let task = Task::perform(
            async move { Ok(Box::new(detect_notifications_state())) },
            NotificationsMessage::NotificationsLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: NotificationsMessage) -> Task<NotificationsMessage> {
        match message {
            NotificationsMessage::LoadNotifications => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move { Ok(Box::new(detect_notifications_state())) },
                    NotificationsMessage::NotificationsLoaded,
                )
            }
            NotificationsMessage::NotificationsLoaded(res) => {
                self.is_refreshing = false;
                self.is_acting = false;
                match res {
                    Ok(data) => {
                        self.state = NotificationsViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = NotificationsViewState::Error(e);
                    }
                }
                Task::none()
            }
            NotificationsMessage::ToggleDnd(enable_dnd) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let desktop = std::env::var("XDG_CURRENT_DESKTOP")
                            .unwrap_or_default()
                            .to_lowercase();

                        if is_proc_running("swaync") && has_cmd("swaync-client") {
                            let (ok, _) = run_cmd("swaync-client", &["-d"]);
                            if ok {
                                return Ok(());
                            }
                        } else if is_proc_running("quickshell")
                            || is_proc_running("noctalia")
                            || desktop.contains("solarui")
                            || desktop.contains("niri")
                        {
                            let home =
                                std::env::var("HOME").unwrap_or_else(|_| "/root".to_string());
                            let noctalia_path = std::path::PathBuf::from(home)
                                .join(".config/noctalia/settings.json");
                            if let Some(parent) = noctalia_path.parent() {
                                let _ = std::fs::create_dir_all(parent);
                            }
                            let mut json_val: serde_json::Value = if noctalia_path.exists() {
                                std::fs::read_to_string(&noctalia_path)
                                    .ok()
                                    .and_then(|s| serde_json::from_str(&s).ok())
                                    .unwrap_or_else(|| serde_json::json!({}))
                            } else {
                                serde_json::json!({})
                            };
                            if let Some(obj) = json_val.as_object_mut() {
                                obj.insert(
                                    "doNotDisturb".to_string(),
                                    serde_json::Value::Bool(enable_dnd),
                                );
                                if let Ok(s) = serde_json::to_string_pretty(&json_val) {
                                    if std::fs::write(&noctalia_path, s).is_ok() {
                                        return Ok(());
                                    }
                                }
                            }
                        } else if is_proc_running("mako") && has_cmd("makoctl") {
                            let flag = if enable_dnd { "-a" } else { "-r" };
                            let (ok, _) = run_cmd("makoctl", &["mode", flag, "do-not-disturb"]);
                            if ok {
                                return Ok(());
                            }
                        } else if is_proc_running("dunst") && has_cmd("dunstctl") {
                            let flag = if enable_dnd { "true" } else { "false" };
                            let (ok, _) = run_cmd("dunstctl", &["set-paused", flag]);
                            if ok {
                                return Ok(());
                            }
                        } else if (is_proc_running("cinnamon") || desktop.contains("cinnamon"))
                            && has_cmd("gsettings")
                        {
                            let val = if enable_dnd { "false" } else { "true" };
                            let (ok, _) = run_cmd(
                                "gsettings",
                                &[
                                    "set",
                                    "org.cinnamon.desktop.notifications",
                                    "display-notifications",
                                    val,
                                ],
                            );
                            if ok {
                                return Ok(());
                            }
                        } else if has_cmd("gsettings") {
                            let show_banners = if enable_dnd { "false" } else { "true" };
                            let (ok, _) = run_cmd(
                                "gsettings",
                                &[
                                    "set",
                                    "org.gnome.desktop.notifications",
                                    "show-banners",
                                    show_banners,
                                ],
                            );
                            if ok {
                                return Ok(());
                            }
                        }

                        Err(BlazeError::ExecutionError {
                            command: "notification dnd set".to_string(),
                            stdout: String::new(),
                            stderr: "Rahatsız Etmeyin modu bu masaüstü ortamında değiştirilemedi"
                                .to_string(),
                        })
                    },
                    NotificationsMessage::ActionFinished,
                )
            }
            NotificationsMessage::ActionFinished(res) => {
                self.is_acting = false;
                match res {
                    Ok(()) => {
                        // Re-read notification status
                        self.is_refreshing = true;
                        Task::perform(
                            async move { Ok(Box::new(detect_notifications_state())) },
                            NotificationsMessage::NotificationsLoaded,
                        )
                    }
                    Err(e) => {
                        self.state = NotificationsViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, NotificationsMessage> {
        match self.state {
            NotificationsViewState::Loading => LoadingState::build(
                "Masaüstü bildirim sunucusu ve parametreleri taranıyor...",
                theme,
            ),
            NotificationsViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(NotificationsMessage::LoadNotifications),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            NotificationsViewState::Loaded(ref data) => {
                let sub_title = text(
                    "Masaüstü Bildirim Sunucusu, Rahatsız Etmeyin (DND) Modu ve Servis Durumu",
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
                        Some(NotificationsMessage::LoadNotifications)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, NotificationsMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. Notification Daemon Status Card
                let notif_card_content: Column<'a, NotificationsMessage> = column![
                    InfoRow::build(
                        "Bildirim Sunucusu (Notification Daemon)",
                        &data.daemon_name,
                        true,
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build(
                        "Servis Durumu",
                        &data.daemon_status,
                        data.control_supported,
                        theme
                    )
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "MASAÜSTÜ BİLDİRİM SERVİSİ (NOTIFICATION DAEMON)",
                    notif_card_content.into(),
                    theme,
                ));

                // 2. Do Not Disturb (DND) Card
                if data.control_supported {
                    let dnd_btn = PillButton::build(
                        if self.is_acting {
                            "⏳ İşleniyor..."
                        } else if data.dnd_active {
                            "🔕 Rahatsız Etmeyin (Etkin)"
                        } else {
                            "🔔 Bildirimler Açık (Normal)"
                        },
                        if data.dnd_active {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_acting {
                            None
                        } else {
                            Some(NotificationsMessage::ToggleDnd(!data.dnd_active))
                        },
                        theme,
                    );

                    let dnd_card_content: Column<'a, NotificationsMessage> = column![row![
                        InfoRow::build(
                            "Rahatsız Etmeyin (Do Not Disturb)",
                            if data.dnd_active {
                                "Etkin (Bildirim Sesleri ve Afişler Sessizde)"
                            } else {
                                "Devre Dışı (Standart)"
                            },
                            data.dnd_active,
                            theme
                        ),
                        iced::widget::horizontal_space(),
                        dnd_btn
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill)]
                    .spacing(SpacingTokens::SM);

                    page_col = page_col.push(GlassCard::build(
                        "RAHATSIZ ETMEYİN VE SESSİZ MOD",
                        dnd_card_content.into(),
                        theme,
                    ));
                } else {
                    let not_avail_text = text("⚠️ NOT AVAILABLE / UNSUPPORTED: Mevcut masaüstü ortamınız veya pencere yöneticiniz bildirimi sessize alma API'si sağlamamaktadır. Sahte kontrol eklenmemiştir.")
                        .size(TypographyTokens::CAPTION_SIZE)
                        .color(theme.colors.warning);

                    page_col = page_col.push(GlassCard::build(
                        "BİLDİRİM KONTROL KAPASİTESİ",
                        not_avail_text.into(),
                        theme,
                    ));
                }

                page_col.into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_notifications_page_lifecycle() {
        let (mut page, _task) = NotificationsPage::new();
        assert!(matches!(page.state, NotificationsViewState::Loading));

        let data = Box::new(NotificationsData {
            daemon_name: "Test Notification Daemon".to_string(),
            daemon_status: "AKTİF".to_string(),
            dnd_active: false,
            control_supported: true,
        });

        let _ = page.update(NotificationsMessage::NotificationsLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, NotificationsViewState::Loaded(_)));
    }
}
