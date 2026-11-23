use crate::ui::tokens::ColorTokens;
use iced::Color;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThemeMode {
    Dark,
    Light,
    System,
}

#[derive(Debug, Clone, Copy)]
pub struct Theme {
    pub mode: ThemeMode,
    pub colors: ColorTokens,
    pub reduce_motion: bool,
    pub custom_accent: Option<Color>,
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            mode: ThemeMode::Dark,
            colors: ColorTokens::dark(),
            reduce_motion: false,
            custom_accent: None,
        }
    }

    pub fn light() -> Self {
        Self {
            mode: ThemeMode::Light,
            colors: ColorTokens::light(),
            reduce_motion: false,
            custom_accent: None,
        }
    }

    pub fn set_mode(&mut self, mode: ThemeMode) {
        self.mode = mode;
        match mode {
            ThemeMode::Dark | ThemeMode::System => self.colors = ColorTokens::dark(),
            ThemeMode::Light => self.colors = ColorTokens::light(),
        }
        if let Some(acc) = self.custom_accent {
            self.colors.accent = acc;
            self.colors.border_focus = acc;
        }
    }

    pub fn set_accent_hex(&mut self, hex: &str) {
        let clean = hex.trim().trim_start_matches('#');
        if clean.len() == 6 {
            if let (Ok(r), Ok(g), Ok(b)) = (
                u8::from_str_radix(&clean[0..2], 16),
                u8::from_str_radix(&clean[2..4], 16),
                u8::from_str_radix(&clean[4..6], 16),
            ) {
                let c = Color::from_rgb8(r, g, b);
                self.custom_accent = Some(c);
                self.colors.accent = c;
                self.colors.border_focus = c;
            }
        }
    }

    pub fn toggle_mode(&mut self) {
        match self.mode {
            ThemeMode::Dark | ThemeMode::System => self.set_mode(ThemeMode::Light),
            ThemeMode::Light => self.set_mode(ThemeMode::Dark),
        }
    }
}

impl Default for Theme {
    fn default() -> Self {
        Self::dark()
    }
}
