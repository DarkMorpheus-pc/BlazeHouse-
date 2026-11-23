use iced::window::Settings;
use iced::Size;

pub fn default_window_settings() -> Settings {
    Settings {
        size: Size::new(1100.0, 750.0),
        min_size: Some(Size::new(850.0, 600.0)),
        resizable: true,
        decorations: true,
        transparent: false,
        ..Default::default()
    }
}
