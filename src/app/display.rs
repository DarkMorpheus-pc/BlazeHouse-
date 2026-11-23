use crate::app::error::BlazeError;
use crate::capabilities::has_cmd;
use crate::displays::{self, DisplayOutput};

pub struct DisplayService;

impl DisplayService {
    pub fn new() -> Self {
        Self
    }

    fn check_capability(&self) -> Result<(), BlazeError> {
        if !has_cmd("niri") && !has_cmd("wlr-randr") && !has_cmd("xrandr") {
            return Err(BlazeError::CapabilityMissing {
                command: "niri / wlr-randr / xrandr".to_string(),
                hint: "sudo dnf install -y wlr-randr xrandr".to_string(),
            });
        }
        Ok(())
    }

    pub fn get_displays(&self) -> Result<Vec<DisplayOutput>, BlazeError> {
        self.check_capability()?;
        Ok(displays::displays())
    }

    pub fn set_mode(&self, output: &str, mode: &str) -> Result<(), BlazeError> {
        self.check_capability()?;
        if displays::set_mode(output, mode) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "randr set-mode".to_string(),
                stdout: String::new(),
                stderr: format!("Ekran çözünürlüğü uygulanamadı: {} {}", output, mode),
            })
        }
    }

    pub fn set_scale(&self, output: &str, scale: f32) -> Result<(), BlazeError> {
        self.check_capability()?;
        if displays::set_scale(output, scale) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "wlr-randr set-scale".to_string(),
                stdout: String::new(),
                stderr: format!("Ekran ölçeği uygulanamadı: {} {:.2}", output, scale),
            })
        }
    }

    pub fn set_enabled(&self, output: &str, enabled: bool) -> Result<(), BlazeError> {
        self.check_capability()?;
        if displays::set_enabled(output, enabled) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "randr set-enabled".to_string(),
                stdout: String::new(),
                stderr: format!("Ekran durumu değiştirilemedi: {}", output),
            })
        }
    }
}

impl Default for DisplayService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_display_service() {
        let service = DisplayService::new();
        let _ = service.get_displays();
    }
}
