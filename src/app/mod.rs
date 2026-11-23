pub mod audio;
pub mod bluetooth;
pub mod display;
pub mod error;
pub mod hardware;
pub mod input;
pub mod network;
pub mod optimization;
pub mod power;
pub mod security;
pub mod settings;
pub mod storage;
pub mod system;
pub mod update;
pub mod zram;

use audio::AudioService;
use bluetooth::BluetoothService;
use display::DisplayService;
use hardware::HardwareService;
use input::InputService;
use network::NetworkService;
use optimization::OptimizationService;
use power::PowerService;
use security::SecurityService;
use settings::SettingsService;
use storage::StorageService;
use system::SystemService;
use update::UpdateService;
use zram::ZramService;

/// Main Application API Facade struct for Blaze House.
/// UI will exclusively communicate through this API layer.
pub struct App {
    pub system: SystemService,
    pub hardware: HardwareService,
    pub audio: AudioService,
    pub display: DisplayService,
    pub input: InputService,
    pub network: NetworkService,
    pub bluetooth: BluetoothService,
    pub power: PowerService,
    pub security: SecurityService,
    pub optimization: OptimizationService,
    pub storage: StorageService,
    pub zram: ZramService,
    pub settings: SettingsService,
    pub update: UpdateService,
}

impl App {
    pub fn new() -> Self {
        Self {
            system: SystemService::new(),
            hardware: HardwareService::new(),
            audio: AudioService::new(),
            display: DisplayService::new(),
            input: InputService::new(),
            network: NetworkService::new(),
            bluetooth: BluetoothService::new(),
            power: PowerService::new(),
            security: SecurityService::new(),
            optimization: OptimizationService::new(),
            storage: StorageService::new(),
            zram: ZramService::new(),
            settings: SettingsService::new(),
            update: UpdateService::new(),
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_app_facade() {
        let app = App::new();
        let sys = app.system.get_system_info();
        assert!(sys.is_ok());
    }
}
