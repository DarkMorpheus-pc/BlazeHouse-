use crate::app::error::BlazeError;
use crate::bluetooth::{self, BluetoothDevice};
use crate::capabilities::has_cmd;

pub struct BluetoothService;

impl BluetoothService {
    pub fn new() -> Self {
        Self
    }

    fn check_capability(&self) -> Result<(), BlazeError> {
        if !has_cmd("bluetoothctl") {
            return Err(BlazeError::CapabilityMissing {
                command: "bluetoothctl".to_string(),
                hint: "sudo dnf install -y bluez".to_string(),
            });
        }
        Ok(())
    }

    pub fn is_powered(&self) -> Result<bool, BlazeError> {
        self.check_capability()?;
        Ok(bluetooth::bt_powered())
    }

    pub fn set_powered(&self, enabled: bool) -> Result<(), BlazeError> {
        self.check_capability()?;
        if bluetooth::set_bt_powered(enabled) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "bluetoothctl power".to_string(),
                stdout: String::new(),
                stderr: "Bluetooth güç durumu değiştirilemedi".to_string(),
            })
        }
    }

    pub fn is_discoverable(&self) -> Result<bool, BlazeError> {
        self.check_capability()?;
        Ok(bluetooth::bt_discoverable())
    }

    pub fn set_discoverable(&self, enabled: bool) -> Result<(), BlazeError> {
        self.check_capability()?;
        if bluetooth::set_bt_discoverable(enabled) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "bluetoothctl discoverable".to_string(),
                stdout: String::new(),
                stderr: "Bluetooth bulunabilirlik durumu değiştirilemedi".to_string(),
            })
        }
    }

    pub fn get_devices(&self) -> Result<Vec<BluetoothDevice>, BlazeError> {
        self.check_capability()?;
        Ok(bluetooth::bt_devices())
    }

    pub fn connect_device(&self, mac: &str) -> Result<(), BlazeError> {
        self.check_capability()?;
        if bluetooth::bt_connect(mac) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "bluetoothctl connect".to_string(),
                stdout: String::new(),
                stderr: format!("Cihaza bağlanılamadı: {}", mac),
            })
        }
    }

    pub fn disconnect_device(&self, mac: &str) -> Result<(), BlazeError> {
        self.check_capability()?;
        if bluetooth::bt_disconnect(mac) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "bluetoothctl disconnect".to_string(),
                stdout: String::new(),
                stderr: format!("Cihaz bağlantısı kesilemedi: {}", mac),
            })
        }
    }

    pub fn pair_device(&self, mac: &str) -> Result<(), BlazeError> {
        self.check_capability()?;
        if bluetooth::bt_pair(mac) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "bluetoothctl pair".to_string(),
                stdout: String::new(),
                stderr: format!("Cihaz eşleştirilemedi: {}", mac),
            })
        }
    }

    pub fn remove_device(&self, mac: &str) -> Result<(), BlazeError> {
        self.check_capability()?;
        if bluetooth::bt_remove(mac) {
            Ok(())
        } else {
            Err(BlazeError::ExecutionError {
                command: "bluetoothctl remove".to_string(),
                stdout: String::new(),
                stderr: format!("Cihaz kaldırılamadı: {}", mac),
            })
        }
    }
}

impl Default for BluetoothService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluetooth_service() {
        let service = BluetoothService::new();
        let _ = service.is_powered();
        let _ = service.is_discoverable();
    }
}
