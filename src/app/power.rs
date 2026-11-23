use crate::app::error::BlazeError;
use crate::power::{self, BatteryInfo};

pub struct PowerService;

impl PowerService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_profile(&self) -> Result<String, BlazeError> {
        Ok(power::active_power_profile())
    }

    pub fn set_profile(&self, profile: &str) -> Result<(), BlazeError> {
        if power::set_power_profile(profile) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "powerprofilesctl set".to_string(),
                stdout: String::new(),
                stderr: format!("Güç profili değiştirilemedi: {}", profile),
            })
        }
    }

    pub fn get_battery_info(&self) -> Result<Option<BatteryInfo>, BlazeError> {
        Ok(power::battery_info())
    }
}

impl Default for PowerService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_service() {
        let service = PowerService::new();
        let p = service.get_profile();
        assert!(p.is_ok());
    }
}
