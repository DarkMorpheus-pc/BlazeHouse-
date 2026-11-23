use crate::app::error::BlazeError;
use crate::capabilities::has_cmd;
use crate::optimize::{zram_available_algorithms, zram_configure, zram_status, ZramStatus};

pub struct ZramService;

impl ZramService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_status(&self) -> Result<ZramStatus, BlazeError> {
        let st = zram_status();
        Ok(st)
    }

    pub fn get_available_algorithms(&self) -> Result<Vec<String>, BlazeError> {
        Ok(zram_available_algorithms())
    }

    pub fn configure(&self, size_mb: u64, algorithm: &str) -> Result<(), BlazeError> {
        if !has_cmd("pkexec") {
            return Err(BlazeError::CapabilityMissing {
                command: "pkexec".to_string(),
                hint: "ZRAM alanını yapılandırmak için policykit-1 gereklidir".to_string(),
            });
        }

        let (ok, msg) = zram_configure(size_mb, algorithm);
        if ok {
            Ok(())
        } else if msg.contains("dismissed") || msg.contains("cancel") {
            Err(BlazeError::PolkitCancelled)
        } else {
            Err(BlazeError::ExecutionError {
                command: "zram_configure".to_string(),
                stdout: String::new(),
                stderr: msg,
            })
        }
    }
}

impl Default for ZramService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zram_service() {
        let service = ZramService::new();
        let st = service.get_status();
        assert!(st.is_ok());
        let algos = service.get_available_algorithms();
        assert!(algos.is_ok());
    }
}
