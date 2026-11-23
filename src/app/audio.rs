use crate::app::error::BlazeError;
use crate::audio::{self, AudioDevice};
use crate::capabilities::has_cmd;

pub struct AudioService;

impl AudioService {
    pub fn new() -> Self {
        Self
    }

    fn check_capability(&self) -> Result<(), BlazeError> {
        if !has_cmd("wpctl") && !has_cmd("pactl") {
            return Err(BlazeError::CapabilityMissing {
                command: "wpctl / pactl".to_string(),
                hint: "sudo dnf install -y wireplumber pulseaudio-utils".to_string(),
            });
        }
        Ok(())
    }

    pub fn get_outputs(&self) -> Result<Vec<AudioDevice>, BlazeError> {
        self.check_capability()?;
        Ok(audio::outputs())
    }

    pub fn get_inputs(&self) -> Result<Vec<AudioDevice>, BlazeError> {
        self.check_capability()?;
        Ok(audio::inputs())
    }

    pub fn get_output_volume(&self) -> Result<Option<u32>, BlazeError> {
        self.check_capability()?;
        Ok(audio::output_volume())
    }

    pub fn get_input_volume(&self) -> Result<Option<u32>, BlazeError> {
        self.check_capability()?;
        Ok(audio::input_volume())
    }

    pub fn is_muted(&self) -> Result<bool, BlazeError> {
        self.check_capability()?;
        Ok(audio::muted())
    }

    pub fn set_output_volume(&self, vol: u32) -> Result<(), BlazeError> {
        self.check_capability()?;
        if audio::set_output_volume(vol) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "wpctl/pactl set-volume".to_string(),
                stdout: String::new(),
                stderr: "Ses seviyesi ayarlanamadı".to_string(),
            })
        }
    }

    pub fn set_input_volume(&self, vol: u32) -> Result<(), BlazeError> {
        self.check_capability()?;
        if audio::set_input_volume(vol) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "wpctl/pactl set-input-volume".to_string(),
                stdout: String::new(),
                stderr: "Mikrofon ses seviyesi ayarlanamadı".to_string(),
            })
        }
    }

    pub fn set_muted(&self, mute: bool) -> Result<(), BlazeError> {
        self.check_capability()?;
        if audio::set_muted(mute) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "wpctl/pactl set-mute".to_string(),
                stdout: String::new(),
                stderr: "Sessize alma durumu değiştirilemedi".to_string(),
            })
        }
    }

    pub fn get_server_name(&self) -> Result<String, BlazeError> {
        Ok(audio::server_name())
    }

    pub fn set_default_output(&self, id_or_name: &str) -> Result<(), BlazeError> {
        self.check_capability()?;
        if audio::set_default_output(id_or_name) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "wpctl/pactl set-default".to_string(),
                stdout: String::new(),
                stderr: format!("Varsayılan çıkış aygıtı değiştirilemedi: {}", id_or_name),
            })
        }
    }

    pub fn set_default_input(&self, id_or_name: &str) -> Result<(), BlazeError> {
        self.check_capability()?;
        if audio::set_default_input(id_or_name) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "wpctl/pactl set-default".to_string(),
                stdout: String::new(),
                stderr: format!("Varsayılan giriş aygıtı değiştirilemedi: {}", id_or_name),
            })
        }
    }
}

impl Default for AudioService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_service() {
        let service = AudioService::new();
        let _ = service.get_outputs();
        let _ = service.get_server_name();
        let _ = service.get_output_volume();
        let _ = service.is_muted();
    }
}
