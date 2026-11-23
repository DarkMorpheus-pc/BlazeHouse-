use crate::app::error::BlazeError;
use crate::capabilities::has_cmd;
use crate::locale_input::{
    available_layouts, keyboard_layouts, mouse_natural_scroll, mouse_speed, set_keyboard_layout,
    set_mouse_natural_scroll, set_mouse_speed, set_touchpad_natural_scroll,
    set_touchpad_tap_to_click, system_locale, touchpad_natural_scroll, touchpad_present,
    touchpad_tap_to_click,
};
use crate::shared_settings::SharedSettings;

pub struct SettingsService;

impl SettingsService {
    pub fn new() -> Self {
        Self
    }

    pub fn load_settings(&self) -> Result<SharedSettings, BlazeError> {
        Ok(SharedSettings::load())
    }

    pub fn save_settings(&self, settings: &SharedSettings) -> Result<(), BlazeError> {
        settings.save().map_err(BlazeError::IoError)
    }

    pub fn get_keyboard_layouts(&self) -> Result<(String, String), BlazeError> {
        if !has_cmd("localectl") {
            return Err(BlazeError::CapabilityMissing {
                command: "localectl".to_string(),
                hint: "systemd-localed servisini kontrol edin".to_string(),
            });
        }
        Ok(keyboard_layouts())
    }

    pub fn get_available_keyboard_layouts(&self) -> Vec<&'static str> {
        available_layouts()
    }

    pub fn set_keyboard_layout(&self, layout: &str) -> Result<(), BlazeError> {
        if !has_cmd("localectl") {
            return Err(BlazeError::CapabilityMissing {
                command: "localectl".to_string(),
                hint: "systemd-localed servisini kontrol edin".to_string(),
            });
        }
        if set_keyboard_layout(layout) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "localectl set-x11-keymap".to_string(),
                stdout: String::new(),
                stderr: format!("Klavye düzeni değiştirilemedi: {}", layout),
            })
        }
    }

    pub fn get_system_locale(&self) -> Result<String, BlazeError> {
        Ok(system_locale())
    }

    pub fn get_mouse_speed(&self) -> Result<Option<f64>, BlazeError> {
        Ok(mouse_speed())
    }

    pub fn set_mouse_speed(&self, speed: f64) -> Result<(), BlazeError> {
        if set_mouse_speed(speed) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "gsettings set mouse speed".to_string(),
                stdout: String::new(),
                stderr: format!("Fare hızı değiştirilemedi: {:.2}", speed),
            })
        }
    }

    pub fn get_mouse_natural_scroll(&self) -> Result<bool, BlazeError> {
        Ok(mouse_natural_scroll())
    }

    pub fn set_mouse_natural_scroll(&self, enabled: bool) -> Result<(), BlazeError> {
        if set_mouse_natural_scroll(enabled) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "gsettings set mouse natural-scroll".to_string(),
                stdout: String::new(),
                stderr: "Doğal kaydırma ayarı değiştirilemedi".to_string(),
            })
        }
    }

    pub fn is_touchpad_present(&self) -> Result<bool, BlazeError> {
        Ok(touchpad_present())
    }

    pub fn get_touchpad_tap_to_click(&self) -> Result<bool, BlazeError> {
        Ok(touchpad_tap_to_click())
    }

    pub fn set_touchpad_tap_to_click(&self, enabled: bool) -> Result<(), BlazeError> {
        if set_touchpad_tap_to_click(enabled) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "gsettings set touchpad tap-to-click".to_string(),
                stdout: String::new(),
                stderr: "Dokunarak tıklama değiştirilemedi".to_string(),
            })
        }
    }

    pub fn get_touchpad_natural_scroll(&self) -> Result<bool, BlazeError> {
        Ok(touchpad_natural_scroll())
    }

    pub fn set_touchpad_natural_scroll(&self, enabled: bool) -> Result<(), BlazeError> {
        if set_touchpad_natural_scroll(enabled) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "gsettings set touchpad natural-scroll".to_string(),
                stdout: String::new(),
                stderr: "Dokunmatik yüzey doğal kaydırma değiştirilemedi".to_string(),
            })
        }
    }
}

impl Default for SettingsService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_settings_service() {
        let service = SettingsService::new();
        let s = service.load_settings();
        assert!(s.is_ok());
    }
}
