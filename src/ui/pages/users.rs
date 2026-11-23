use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};
use crate::users::{self, UserAccount};

#[derive(Debug, Clone)]
pub struct UsersData {
    pub current_user: UserAccount,
    pub system_users: Vec<UserAccount>,
}

#[derive(Debug, Clone)]
pub enum UsersMessage {
    LoadUsers,
    UsersLoaded(Result<Box<UsersData>, BlazeError>),
}

#[derive(Debug, Clone)]
pub enum UsersViewState {
    Loading,
    Loaded(Box<UsersData>),
    Error(BlazeError),
}

pub struct UsersPage {
    pub state: UsersViewState,
    pub is_refreshing: bool,
}

impl UsersPage {
    pub fn new() -> (Self, Task<UsersMessage>) {
        let page = Self {
            state: UsersViewState::Loading,
            is_refreshing: true,
        };
        let task = Task::perform(
            async move {
                let current_user = users::current_user();
                let system_users = users::list_system_human_users();
                Ok(Box::new(UsersData {
                    current_user,
                    system_users,
                }))
            },
            UsersMessage::UsersLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: UsersMessage) -> Task<UsersMessage> {
        match message {
            UsersMessage::LoadUsers => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let current_user = users::current_user();
                        let system_users = users::list_system_human_users();
                        Ok(Box::new(UsersData {
                            current_user,
                            system_users,
                        }))
                    },
                    UsersMessage::UsersLoaded,
                )
            }
            UsersMessage::UsersLoaded(res) => {
                self.is_refreshing = false;
                match res {
                    Ok(data) => {
                        self.state = UsersViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = UsersViewState::Error(e);
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, UsersMessage> {
        match self.state {
            UsersViewState::Loading => LoadingState::build(
                "Aktif kullanıcı ve oturum parametreleri taranıyor...",
                theme,
            ),
            UsersViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(UsersMessage::LoadUsers),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            UsersViewState::Loaded(ref data) => {
                let sub_title = text("Aktif Linux Oturumu, Kullanıcı İzinleri ve Hesap Bilgileri")
                    .size(TypographyTokens::CAPTION_SIZE)
                    .color(theme.colors.text_secondary);

                let refresh_btn = PillButton::build(
                    if self.is_refreshing {
                        "⏳ Yenileniyor..."
                    } else {
                        "🔄 Yenile"
                    },
                    ButtonVariant::Tonal,
                    if self.is_refreshing {
                        None
                    } else {
                        Some(UsersMessage::LoadUsers)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, UsersMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. Current Active User Card
                let curr = &data.current_user;
                let uid_str = format!("UID: {} | GID: {}", curr.uid, curr.gid);
                let groups_str = if curr.supplementary_groups.is_empty() {
                    curr.primary_group.clone()
                } else {
                    curr.supplementary_groups.join(", ")
                };

                let curr_card_content: Column<'a, UsersMessage> = column![
                    InfoRow::build("Kullanıcı Adı (Username)", &curr.username, true, theme),
                    Divider::build(theme),
                    InfoRow::build("Kullanıcı Kimlikleri", &uid_str, false, theme),
                    Divider::build(theme),
                    InfoRow::build("Üye Gruplar", &groups_str, false, theme),
                    Divider::build(theme),
                    InfoRow::build("Ev Dizini (Home)", &curr.home_dir, false, theme),
                    Divider::build(theme),
                    InfoRow::build("Varsayılan Kabuk (Shell)", &curr.shell, false, theme),
                    Divider::build(theme),
                    InfoRow::build(
                        "Yönetici / Sudo İzni",
                        if curr.is_admin {
                            "YÖNETİCİ (ROOT / SUDO YETKİLİ)"
                        } else {
                            "Standart Kullanıcı"
                        },
                        curr.is_admin,
                        theme
                    )
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "AKTİF KULLANICI OTURUMU BİLGİLERİ",
                    curr_card_content.into(),
                    theme,
                ));

                // 2. System Human Users List Card
                let mut users_col: Column<'a, UsersMessage> = column![].spacing(SpacingTokens::SM);

                if data.system_users.is_empty() {
                    users_col = users_col.push(InfoRow::build(
                        "Kullanıcı Listesi",
                        "Sistemde başka insan hesabı tespit edilemedi",
                        false,
                        theme,
                    ));
                } else {
                    for (idx, u) in data.system_users.iter().enumerate() {
                        if idx > 0 {
                            users_col = users_col.push(Divider::build(theme));
                        }
                        let detail_str = format!(
                            "UID: {} | Home: {} | Shell: {} | Status: {}",
                            u.uid,
                            u.home_dir,
                            u.shell,
                            if u.is_current {
                                "Aktif Oturum"
                            } else {
                                "Kayıtlı Hesap"
                            }
                        );
                        users_col = users_col.push(InfoRow::build(
                            &u.username,
                            &detail_str,
                            u.is_current,
                            theme,
                        ));
                    }
                }

                page_col = page_col.push(GlassCard::build(
                    "SİSTEMDE KAYITLI KULLANICI HESAPLARI (/ETC/PASSWD)",
                    users_col.into(),
                    theme,
                ));

                // 3. Security Policy Card
                let sec_text = text("🔒 GÜVENLİK POLİTİKASI: Parola karmaları (hashes) ve /etc/shadow dosyası güvenlik nedeniyle okunmaz veya arayüzde gösterilmez. Yüksek riskli hesap silme/oluşturma işlemleri sistem bütünlüğü için Polkit/CLI üzerinden gerçekleştirilmelidir.")
                    .size(TypographyTokens::CAPTION_SIZE)
                    .color(theme.colors.warning);

                page_col = page_col.push(GlassCard::build(
                    "KULLANICI GÜVENLİK BİLDİRİMİ",
                    sec_text.into(),
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
    fn test_users_page_lifecycle() {
        let (mut page, _task) = UsersPage::new();
        assert!(matches!(page.state, UsersViewState::Loading));

        let current_user = users::current_user();
        let system_users = users::list_system_human_users();
        let data = Box::new(UsersData {
            current_user,
            system_users,
        });

        let _ = page.update(UsersMessage::UsersLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, UsersViewState::Loaded(_)));
    }
}
