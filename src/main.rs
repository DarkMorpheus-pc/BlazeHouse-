use iced::application;

mod app;
mod audio;
mod bluetooth;
mod capabilities;
mod displays;
mod hardware_info;
mod locale_input;
mod network;
mod optimize;
mod power;
mod security;
mod shared_settings;
mod storage;
mod system_info;
mod timedate;
mod ui;
mod updater;
mod users;

use ui::{default_window_settings, BlazeHouseShell};

pub fn main() -> iced::Result {
    application(
        BlazeHouseShell::title,
        BlazeHouseShell::update,
        BlazeHouseShell::view,
    )
    .theme(|s: &BlazeHouseShell| match s.theme.mode {
        crate::ui::theme::ThemeMode::Light => iced::Theme::Light,
        _ => iced::Theme::Dark,
    })
    .window(default_window_settings())
    .run_with(BlazeHouseShell::new)
}
