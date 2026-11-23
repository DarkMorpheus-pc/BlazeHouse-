use crate::app::error::BlazeError;
use crate::displays::DisplayOutput;
use crate::hardware_info::{cpu_summary, gather_hardware, HardwareInventory};

#[derive(Debug, Clone)]
pub struct HardwareData {
    pub inventory: HardwareInventory,
    pub displays: Vec<DisplayOutput>,
    pub displays_error: Option<String>,
}

pub struct HardwareService;

impl HardwareService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_hardware_inventory(&self) -> Result<HardwareInventory, BlazeError> {
        let inv = gather_hardware();
        Ok(inv)
    }

    pub fn get_cpu_summary(&self) -> Result<String, BlazeError> {
        let summary = cpu_summary();
        Ok(summary)
    }

    pub fn get_hardware_data(&self) -> Result<HardwareData, BlazeError> {
        let inventory = gather_hardware();
        let display_service = crate::app::display::DisplayService::new();
        let (displays, displays_error) = match display_service.get_displays() {
            Ok(list) => (list, None),
            Err(e) => (Vec::new(), Some(e.to_string())),
        };

        Ok(HardwareData {
            inventory,
            displays,
            displays_error,
        })
    }
}

impl Default for HardwareService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_service() {
        let service = HardwareService::new();
        let inv_res = service.get_hardware_inventory();
        assert!(inv_res.is_ok());
        let cpu_res = service.get_cpu_summary();
        assert!(cpu_res.is_ok());

        let hw_data = service.get_hardware_data();
        assert!(hw_data.is_ok());
        let data = hw_data.unwrap();
        assert!(!data.inventory.cpu.is_empty());
    }
}
