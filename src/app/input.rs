use crate::app::error::BlazeError;
use crate::capabilities::has_cmd;
use crate::locale_input::{self, InputDeviceInfo};

pub struct InputService;

impl InputService {
    pub fn new() -> Self {
        Self
    }

    fn check_capability(&self) -> Result<(), BlazeError> {
        if !has_cmd("localectl") && !has_cmd("gsettings") {
            return Err(BlazeError::CapabilityMissing {
                command: "localectl / gsettings".to_string(),
                hint: "sudo dnf install -y systemd glib2".to_string(),
            });
        }
        Ok(())
    }

    pub fn get_keyboard_layout(&self) -> Result<(String, String), BlazeError> {
        self.check_capability()?;
        Ok(locale_input::keyboard_layouts())
    }

    pub fn get_available_layouts(&self) -> Vec<&'static str> {
        locale_input::available_layouts()
    }

    pub fn set_keyboard_layout(&self, layout: &str) -> Result<(), BlazeError> {
        self.check_capability()?;
        if locale_input::set_keyboard_layout(layout) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "localectl set-x11-keymap".to_string(),
                stdout: String::new(),
                stderr: format!("Klavye düzeni uygulanamadı: {}", layout),
            })
        }
    }

    pub fn get_mouse_speed(&self) -> Result<Option<f64>, BlazeError> {
        self.check_capability()?;
        Ok(locale_input::mouse_speed())
    }

    pub fn set_mouse_speed(&self, speed: f64) -> Result<(), BlazeError> {
        self.check_capability()?;
        if locale_input::set_mouse_speed(speed) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "gsettings set mouse speed".to_string(),
                stdout: String::new(),
                stderr: "Fare hızı değiştirilemedi".to_string(),
            })
        }
    }

    pub fn get_mouse_natural_scroll(&self) -> Result<bool, BlazeError> {
        self.check_capability()?;
        Ok(locale_input::mouse_natural_scroll())
    }

    pub fn set_mouse_natural_scroll(&self, enabled: bool) -> Result<(), BlazeError> {
        self.check_capability()?;
        if locale_input::set_mouse_natural_scroll(enabled) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "gsettings set mouse natural-scroll".to_string(),
                stdout: String::new(),
                stderr: "Fare doğal kaydırma durumu değiştirilemedi".to_string(),
            })
        }
    }

    pub fn is_touchpad_present(&self) -> Result<bool, BlazeError> {
        Ok(locale_input::touchpad_present())
    }

    pub fn get_touchpad_tap_to_click(&self) -> Result<bool, BlazeError> {
        self.check_capability()?;
        Ok(locale_input::touchpad_tap_to_click())
    }

    pub fn set_touchpad_tap_to_click(&self, enabled: bool) -> Result<(), BlazeError> {
        self.check_capability()?;
        if locale_input::set_touchpad_tap_to_click(enabled) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "gsettings set touchpad tap-to-click".to_string(),
                stdout: String::new(),
                stderr: "Dokunmatik yüzey tıklama durumu değiştirilemedi".to_string(),
            })
        }
    }

    pub fn get_touchpad_natural_scroll(&self) -> Result<bool, BlazeError> {
        self.check_capability()?;
        Ok(locale_input::touchpad_natural_scroll())
    }

    pub fn set_touchpad_natural_scroll(&self, enabled: bool) -> Result<(), BlazeError> {
        self.check_capability()?;
        if locale_input::set_touchpad_natural_scroll(enabled) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "gsettings set touchpad natural-scroll".to_string(),
                stdout: String::new(),
                stderr: "Dokunmatik yüzey doğal kaydırma durumu değiştirilemedi".to_string(),
            })
        }
    }

    pub fn get_input_devices(&self) -> Result<Vec<InputDeviceInfo>, BlazeError> {
        Ok(locale_input::input_devices())
    }
}

impl Default for InputService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_service() {
        let service = InputService::new();
        let _ = service.get_keyboard_layout();
        let _ = service.get_input_devices();
    }
}
