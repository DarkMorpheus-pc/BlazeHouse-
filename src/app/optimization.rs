use crate::app::error::BlazeError;
use crate::capabilities::has_cmd;
use crate::optimize::{
    apply_tweak, build_catalog, cleanup_actions, query_tweak_state, run_cleanup, CleanupAction,
    Tweak,
};

pub struct OptimizationService;

impl OptimizationService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_tweaks_catalog(&self) -> Result<Vec<Tweak>, BlazeError> {
        Ok(build_catalog())
    }

    pub fn get_tweak_state(&self, tweak: &Tweak) -> Result<Option<bool>, BlazeError> {
        Ok(query_tweak_state(tweak))
    }

    pub fn apply_tweak(&self, tweak: &Tweak, enable: bool) -> Result<(), BlazeError> {
        if tweak.needs_root && !has_cmd("pkexec") {
            return Err(BlazeError::CapabilityMissing {
                command: "pkexec".to_string(),
                hint: "Sistem ayarını değiştirmek için policykit-1 gereklidir".to_string(),
            });
        }

        let (ok, msg) = apply_tweak(tweak, enable);
        if ok {
            Ok(())
        } else if msg.contains("dismissed") || msg.contains("cancel") {
            Err(BlazeError::PolkitCancelled)
        } else {
            Err(BlazeError::ExecutionError {
                command: tweak.key.clone(),
                stdout: String::new(),
                stderr: msg,
            })
        }
    }

    pub fn get_cleanup_actions(&self) -> Result<Vec<CleanupAction>, BlazeError> {
        Ok(cleanup_actions())
    }

    pub fn run_cleanup_action(&self, action_key: &str) -> Result<(), BlazeError> {
        let catalog = cleanup_actions();
        let action = catalog.iter().find(|a| a.key == action_key);

        if let Some(act) = action {
            if act.needs_root && !has_cmd("pkexec") {
                return Err(BlazeError::CapabilityMissing {
                    command: "pkexec".to_string(),
                    hint: "Temizlik işlemi için policykit-1 gereklidir".to_string(),
                });
            }
        }

        let (ok, msg) = run_cleanup(action_key);
        if ok {
            Ok(())
        } else if msg.contains("dismissed") || msg.contains("cancel") {
            Err(BlazeError::PolkitCancelled)
        } else {
            Err(BlazeError::ExecutionError {
                command: format!("cleanup({})", action_key),
                stdout: String::new(),
                stderr: msg,
            })
        }
    }
}

impl Default for OptimizationService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimization_service() {
        let service = OptimizationService::new();
        let tweaks = service.get_tweaks_catalog();
        assert!(tweaks.is_ok());
        let actions = service.get_cleanup_actions();
        assert!(actions.is_ok());
    }
}
