use iced::widget::{button, column, container, row, scrollable, text};
use iced::{Alignment, Element, Length, Padding, Task};

use crate::app::App;
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::header::Header;
use crate::ui::components::nav_item::NavItem;
use crate::ui::components::page_container::PageContainer;
use crate::ui::components::toast_banner::{BannerLevel, ToastBanner};
use crate::ui::pages::about::{AboutMessage, AboutPage};
use crate::ui::pages::accessibility::{AccessibilityMessage, AccessibilityPage};
use crate::ui::pages::battery::{BatteryMessage, BatteryPage};
use crate::ui::pages::bluetooth::{BluetoothMessage, BluetoothPage};
use crate::ui::pages::customize::{CustomizeMessage, CustomizePage};
use crate::ui::pages::displays::{DisplaysMessage, DisplaysPage};
use crate::ui::pages::general::{GeneralMessage, GeneralPage};
use crate::ui::pages::hardware::{HardwareMessage, HardwarePage};
use crate::ui::pages::input::{InputMessage, InputPage};
use crate::ui::pages::language_region::{LanguageRegionMessage, LanguageRegionPage};
use crate::ui::pages::network::{NetworkMessage, NetworkPage};
use crate::ui::pages::notifications::{NotificationsMessage, NotificationsPage};
use crate::ui::pages::optimize::{OptimizeMessage, OptimizePage};
use crate::ui::pages::security::{SecurityMessage, SecurityPage};
use crate::ui::pages::sound::{SoundMessage, SoundPage};
use crate::ui::pages::storage::{StorageMessage, StoragePage};
use crate::ui::pages::system_details::{SystemDetailsMessage, SystemDetailsPage};
use crate::ui::pages::users::{UsersMessage, UsersPage};
use crate::ui::pages::zram::{ZramMessage, ZramPage};
use crate::ui::route::{NavCategory, Route};
use crate::ui::theme::{Theme, ThemeMode};
use crate::ui::tokens::{RadiusTokens, SpacingTokens, TypographyTokens};

#[derive(Debug, Clone)]
pub enum ShellMessage {
    NavigateTo(Route),
    ToggleTheme,
    ToggleSidebar,
    ClearBanner,
    General(GeneralMessage),
    SystemDetails(SystemDetailsMessage),
    Hardware(HardwareMessage),
    Storage(StorageMessage),
    Optimize(OptimizeMessage),
    Zram(ZramMessage),
    Battery(BatteryMessage),
    Network(NetworkMessage),
    Bluetooth(BluetoothMessage),
    Sound(SoundMessage),
    Security(SecurityMessage),
    Displays(DisplaysMessage),
    Input(InputMessage),
    Customize(CustomizeMessage),
    LanguageRegion(LanguageRegionMessage),
    Users(UsersMessage),
    Accessibility(AccessibilityMessage),
    Notifications(NotificationsMessage),
    About(AboutMessage),
}

pub struct BlazeHouseShell {
    pub app_api: App,
    pub theme: Theme,
    pub current_route: Route,
    pub sidebar_expanded: bool,
    pub active_banner: Option<(String, BannerLevel)>,
    pub general_page: GeneralPage,
    pub system_details_page: SystemDetailsPage,
    pub hardware_page: HardwarePage,
    pub storage_page: StoragePage,
    pub optimize_page: OptimizePage,
    pub zram_page: ZramPage,
    pub battery_page: BatteryPage,
    pub network_page: NetworkPage,
    pub bluetooth_page: BluetoothPage,
    pub sound_page: SoundPage,
    pub security_page: SecurityPage,
    pub displays_page: DisplaysPage,
    pub input_page: InputPage,
    pub customize_page: CustomizePage,
    pub language_region_page: LanguageRegionPage,
    pub users_page: UsersPage,
    pub accessibility_page: AccessibilityPage,
    pub notifications_page: NotificationsPage,
    pub about_page: AboutPage,
}

impl BlazeHouseShell {
    pub fn new() -> (Self, Task<ShellMessage>) {
        let (mut general_page, gen_task) = GeneralPage::new();
        if let Ok(initial_overview) = crate::app::system::SystemService::new().get_overview_data() {
            let _ = general_page.update(GeneralMessage::OverviewLoaded(Ok(Box::new(
                initial_overview,
            ))));
        }
        let (system_details_page, dt_task) = SystemDetailsPage::new();
        let (hardware_page, hw_task) = HardwarePage::new();
        let (storage_page, st_task) = StoragePage::new();
        let (optimize_page, opt_task) = OptimizePage::new();
        let (zram_page, zr_task) = ZramPage::new();
        let (battery_page, bat_task) = BatteryPage::new();
        let (network_page, net_task) = NetworkPage::new();
        let (bluetooth_page, bt_task) = BluetoothPage::new();
        let (sound_page, snd_task) = SoundPage::new();
        let (security_page, sec_task) = SecurityPage::new();
        let (displays_page, disp_task) = DisplaysPage::new();
        let (input_page, inp_task) = InputPage::new();
        let (customize_page, cust_task) = CustomizePage::new();
        let (language_region_page, lang_task) = LanguageRegionPage::new();
        let (users_page, usr_task) = UsersPage::new();
        let (accessibility_page, a11y_task) = AccessibilityPage::new();
        let (notifications_page, notif_task) = NotificationsPage::new();
        let (about_page, abt_task) = AboutPage::new();

        let shell = Self {
            app_api: App::new(),
            theme: Theme::dark(),
            current_route: Route::General,
            sidebar_expanded: true,
            active_banner: None,
            general_page,
            system_details_page,
            hardware_page,
            storage_page,
            optimize_page,
            zram_page,
            battery_page,
            network_page,
            bluetooth_page,
            sound_page,
            security_page,
            displays_page,
            input_page,
            customize_page,
            language_region_page,
            users_page,
            accessibility_page,
            notifications_page,
            about_page,
        };

        let init_task = Task::batch([
            gen_task.map(ShellMessage::General),
            dt_task.map(ShellMessage::SystemDetails),
            hw_task.map(ShellMessage::Hardware),
            st_task.map(ShellMessage::Storage),
            opt_task.map(ShellMessage::Optimize),
            zr_task.map(ShellMessage::Zram),
            bat_task.map(ShellMessage::Battery),
            net_task.map(ShellMessage::Network),
            bt_task.map(ShellMessage::Bluetooth),
            snd_task.map(ShellMessage::Sound),
            sec_task.map(ShellMessage::Security),
            disp_task.map(ShellMessage::Displays),
            inp_task.map(ShellMessage::Input),
            cust_task.map(ShellMessage::Customize),
            lang_task.map(ShellMessage::LanguageRegion),
            usr_task.map(ShellMessage::Users),
            a11y_task.map(ShellMessage::Accessibility),
            notif_task.map(ShellMessage::Notifications),
            abt_task.map(ShellMessage::About),
        ]);

        (shell, init_task)
    }

    pub fn title(&self) -> String {
        format!("Blaze House 2.0 — {}", self.current_route.title())
    }

    pub fn update(&mut self, message: ShellMessage) -> Task<ShellMessage> {
        match message {
            ShellMessage::NavigateTo(route) => {
                self.current_route = route;
                Task::none()
            }
            ShellMessage::ToggleTheme => {
                self.theme.toggle_mode();
                let mode_str = match self.theme.mode {
                    ThemeMode::Light => "light",
                    ThemeMode::System => "system",
                    ThemeMode::Dark => "dark",
                };
                self.customize_page
                    .update(CustomizeMessage::SetThemeMode(mode_str.to_string()))
                    .map(ShellMessage::Customize)
            }
            ShellMessage::ToggleSidebar => {
                self.sidebar_expanded = !self.sidebar_expanded;
                Task::none()
            }
            ShellMessage::ClearBanner => {
                self.active_banner = None;
                Task::none()
            }
            ShellMessage::General(msg) => self.general_page.update(msg).map(ShellMessage::General),
            ShellMessage::SystemDetails(msg) => self
                .system_details_page
                .update(msg)
                .map(ShellMessage::SystemDetails),
            ShellMessage::Hardware(msg) => {
                self.hardware_page.update(msg).map(ShellMessage::Hardware)
            }
            ShellMessage::Storage(msg) => self.storage_page.update(msg).map(ShellMessage::Storage),
            ShellMessage::Optimize(msg) => {
                self.optimize_page.update(msg).map(ShellMessage::Optimize)
            }
            ShellMessage::Zram(msg) => self.zram_page.update(msg).map(ShellMessage::Zram),
            ShellMessage::Battery(msg) => self.battery_page.update(msg).map(ShellMessage::Battery),
            ShellMessage::Network(msg) => self.network_page.update(msg).map(ShellMessage::Network),
            ShellMessage::Bluetooth(msg) => {
                self.bluetooth_page.update(msg).map(ShellMessage::Bluetooth)
            }
            ShellMessage::Sound(msg) => self.sound_page.update(msg).map(ShellMessage::Sound),
            ShellMessage::Security(msg) => {
                self.security_page.update(msg).map(ShellMessage::Security)
            }
            ShellMessage::Displays(msg) => {
                self.displays_page.update(msg).map(ShellMessage::Displays)
            }
            ShellMessage::Input(msg) => self.input_page.update(msg).map(ShellMessage::Input),
            ShellMessage::Customize(msg) => {
                match &msg {
                    CustomizeMessage::CustomizeLoaded(Ok(data)) => {
                        let m = match data.settings.theme_mode.as_str() {
                            "light" => ThemeMode::Light,
                            "system" | "auto" => ThemeMode::System,
                            _ => ThemeMode::Dark,
                        };
                        self.theme.set_mode(m);
                        self.theme.set_accent_hex(&data.settings.accent_color);
                        self.theme.reduce_motion = data.settings.reduce_motion;
                    }
                    CustomizeMessage::SetThemeMode(mode) => {
                        let m = match mode.as_str() {
                            "light" => ThemeMode::Light,
                            "system" | "auto" => ThemeMode::System,
                            _ => ThemeMode::Dark,
                        };
                        self.theme.set_mode(m);
                    }
                    CustomizeMessage::SetAccentColor(hex) => {
                        self.theme.set_accent_hex(hex);
                    }
                    CustomizeMessage::ToggleReduceMotion(reduce) => {
                        self.theme.reduce_motion = *reduce;
                    }
                    _ => {}
                }
                self.customize_page.update(msg).map(ShellMessage::Customize)
            }
            ShellMessage::LanguageRegion(msg) => self
                .language_region_page
                .update(msg)
                .map(ShellMessage::LanguageRegion),
            ShellMessage::Users(msg) => self.users_page.update(msg).map(ShellMessage::Users),
            ShellMessage::Accessibility(msg) => self
                .accessibility_page
                .update(msg)
                .map(ShellMessage::Accessibility),
            ShellMessage::Notifications(msg) => self
                .notifications_page
                .update(msg)
                .map(ShellMessage::Notifications),
            ShellMessage::About(msg) => self.about_page.update(msg).map(ShellMessage::About),
        }
    }

    pub fn view(&self) -> Element<'_, ShellMessage> {
        let sidebar = self.render_sidebar();
        let header_elem: Element<'_, ShellMessage> =
            Header::build(self.current_route, &self.theme).into();

        let content_shell = self.render_route_shell();
        let page_view = PageContainer::build(content_shell, &self.theme);

        let mut right_col = column![header_elem];

        if let Some((ref msg, level)) = self.active_banner {
            let banner = ToastBanner::build(msg, level, &self.theme);
            right_col = right_col.push(banner);
        }

        right_col = right_col.push(page_view);

        let main_layout = row![sidebar, right_col.width(Length::Fill).height(Length::Fill)]
            .width(Length::Fill)
            .height(Length::Fill);

        main_layout.into()
    }

    fn render_sidebar(&self) -> Element<'_, ShellMessage> {
        let colors = self.theme.colors;

        let brand_text = text(if self.sidebar_expanded {
            "🔥 BLAZE HOUSE"
        } else {
            "🔥"
        })
        .size(TypographyTokens::HEADING_SIZE)
        .color(colors.accent);

        let toggle_side_btn = button(text(if self.sidebar_expanded { "◀" } else { "▶" }))
            .padding(SpacingTokens::XS)
            .on_press(ShellMessage::ToggleSidebar)
            .style(move |_, _| button::Style {
                background: Some(colors.surface_elevated.into()),
                text_color: colors.text_primary,
                border: iced::Border {
                    color: colors.border,
                    width: 1.0,
                    radius: RadiusTokens::SM.into(),
                },
                shadow: iced::Shadow::default(),
            });

        let top_brand_row = row![
            brand_text,
            iced::widget::horizontal_space(),
            toggle_side_btn
        ]
        .align_y(Alignment::Center)
        .padding(SpacingTokens::MD);

        let mut nav_col = column![top_brand_row].spacing(SpacingTokens::XS);

        let mut current_cat: Option<NavCategory> = None;

        for &route in Route::all_routes() {
            let cat = route.category();
            if current_cat != Some(cat) {
                current_cat = Some(cat);
                if self.sidebar_expanded {
                    let cat_label = text(cat.title())
                        .size(TypographyTokens::CAPTION_SIZE)
                        .color(colors.text_disabled);
                    nav_col = nav_col.push(
                        container(cat_label).padding(
                            Padding::ZERO
                                .top(SpacingTokens::SM)
                                .right(SpacingTokens::MD)
                                .left(SpacingTokens::MD),
                        ),
                    );
                }
            }

            let is_selected = self.current_route == route;
            let nav_item = NavItem::build(
                route,
                is_selected,
                !self.sidebar_expanded,
                ShellMessage::NavigateTo(route),
                &self.theme,
            );
            nav_col = nav_col.push(nav_item);
        }

        let theme_label = if self.sidebar_expanded {
            match self.theme.mode {
                ThemeMode::Dark => "🌙 Koyu Tema",
                ThemeMode::Light => "☀️ Açık Tema",
                ThemeMode::System => "💻 Sistem Teması",
            }
        } else {
            match self.theme.mode {
                ThemeMode::Dark => "🌙",
                ThemeMode::Light => "☀️",
                ThemeMode::System => "💻",
            }
        };

        let theme_btn = PillButton::build(
            theme_label,
            ButtonVariant::Tonal,
            Some(ShellMessage::ToggleTheme),
            &self.theme,
        );

        let footer = container(theme_btn)
            .padding(SpacingTokens::MD)
            .width(Length::Fill);

        let sidebar_width = if self.sidebar_expanded { 240.0 } else { 80.0 };

        let scroll_nav = scrollable(nav_col).height(Length::Fill);

        let full_sidebar = column![scroll_nav, footer];

        container(full_sidebar)
            .width(Length::Fixed(sidebar_width))
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(colors.surface.into()),
                text_color: Some(colors.text_primary),
                border: iced::Border {
                    color: colors.border,
                    width: 1.0,
                    radius: 0.0.into(),
                },
                shadow: iced::Shadow::default(),
            })
            .into()
    }

    fn render_route_shell<'a>(&'a self) -> Element<'a, ShellMessage> {
        match self.current_route {
            Route::General => self
                .general_page
                .view(&self.theme)
                .map(ShellMessage::General),
            Route::SystemDetails => {
                let sys_view = self
                    .system_details_page
                    .view(&self.theme)
                    .map(ShellMessage::SystemDetails);
                let hw_view = self
                    .hardware_page
                    .view(&self.theme)
                    .map(ShellMessage::Hardware);
                iced::widget::column![sys_view, hw_view]
                    .spacing(crate::ui::tokens::SpacingTokens::XL)
                    .into()
            }
            Route::Displays => self
                .displays_page
                .view(&self.theme)
                .map(ShellMessage::Displays),
            Route::Storage => self
                .storage_page
                .view(&self.theme)
                .map(ShellMessage::Storage),
            Route::Optimize => self
                .optimize_page
                .view(&self.theme)
                .map(ShellMessage::Optimize),
            Route::Zram => self.zram_page.view(&self.theme).map(ShellMessage::Zram),
            Route::Battery => self
                .battery_page
                .view(&self.theme)
                .map(ShellMessage::Battery),
            Route::Network => self
                .network_page
                .view(&self.theme)
                .map(ShellMessage::Network),
            Route::Bluetooth => self
                .bluetooth_page
                .view(&self.theme)
                .map(ShellMessage::Bluetooth),
            Route::Sound => self.sound_page.view(&self.theme).map(ShellMessage::Sound),
            Route::Security => self
                .security_page
                .view(&self.theme)
                .map(ShellMessage::Security),
            Route::Input => self.input_page.view(&self.theme).map(ShellMessage::Input),
            Route::Customize => self
                .customize_page
                .view(&self.theme)
                .map(ShellMessage::Customize),
            Route::LanguageRegion => self
                .language_region_page
                .view(&self.theme)
                .map(ShellMessage::LanguageRegion),
            Route::Users => self.users_page.view(&self.theme).map(ShellMessage::Users),
            Route::Accessibility => self
                .accessibility_page
                .view(&self.theme)
                .map(ShellMessage::Accessibility),
            Route::Notifications => self
                .notifications_page
                .view(&self.theme)
                .map(ShellMessage::Notifications),
            Route::About => self.about_page.view(&self.theme).map(ShellMessage::About),
        }
    }
}
