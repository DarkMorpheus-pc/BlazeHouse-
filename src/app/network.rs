use crate::app::error::BlazeError;
use crate::capabilities::has_cmd;
use crate::network::{self, ActiveConnection, NetworkInterfaceInfo, WifiNetwork};

pub struct NetworkService;

impl NetworkService {
    pub fn new() -> Self {
        Self
    }

    fn check_capability(&self) -> Result<(), BlazeError> {
        if !has_cmd("nmcli") {
            return Err(BlazeError::CapabilityMissing {
                command: "nmcli".to_string(),
                hint: "sudo dnf install -y NetworkManager".to_string(),
            });
        }
        Ok(())
    }

    pub fn is_wifi_enabled(&self) -> Result<bool, BlazeError> {
        self.check_capability()?;
        Ok(network::wifi_enabled())
    }

    pub fn set_wifi_enabled(&self, enabled: bool) -> Result<(), BlazeError> {
        self.check_capability()?;
        if network::set_wifi_enabled(enabled) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "nmcli radio wifi".to_string(),
                stdout: String::new(),
                stderr: "Wi-Fi durumu değiştirilemedi".to_string(),
            })
        }
    }

    pub fn get_wifi_networks(&self) -> Result<Vec<WifiNetwork>, BlazeError> {
        self.check_capability()?;
        Ok(network::wifi_networks())
    }

    pub fn connect_wifi(&self, ssid: &str) -> Result<(), BlazeError> {
        self.check_capability()?;
        if network::connect_wifi(ssid) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "nmcli device wifi connect".to_string(),
                stdout: String::new(),
                stderr: format!("Ağa bağlanılamadı: {}", ssid),
            })
        }
    }

    pub fn disconnect_wifi(&self, ssid: &str) -> Result<(), BlazeError> {
        self.check_capability()?;
        if network::disconnect_wifi(ssid) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "nmcli connection down".to_string(),
                stdout: String::new(),
                stderr: format!("Ağ bağlantısı kesilemedi: {}", ssid),
            })
        }
    }

    pub fn get_active_connections(&self) -> Result<Vec<ActiveConnection>, BlazeError> {
        self.check_capability()?;
        Ok(network::active_connections())
    }

    pub fn get_interfaces(&self) -> Result<Vec<NetworkInterfaceInfo>, BlazeError> {
        self.check_capability()?;
        Ok(network::network_interfaces())
    }
}

impl Default for NetworkService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_service() {
        let service = NetworkService::new();
        let _ = service.is_wifi_enabled();
        let _ = service.get_active_connections();
        let _ = service.get_interfaces();
    }
}
