use tokio::sync::mpsc;

use crate::app::error::BlazeError;
use crate::capabilities::has_cmd;
use crate::security::{
    check_system_security, list_cyber_tools, repair_security_setting, run_cyber_tool_stream,
    CyberTool, SecurityAudit,
};

pub struct SecurityService;

impl SecurityService {
    pub fn new() -> Self {
        Self
    }

    pub fn run_audit(&self) -> Result<SecurityAudit, BlazeError> {
        let audit = check_system_security();
        Ok(audit)
    }

    pub fn repair_setting(&self, setting_key: &str) -> Result<(), BlazeError> {
        if !has_cmd("pkexec") {
            return Err(BlazeError::CapabilityMissing {
                command: "pkexec".to_string(),
                hint: "sudo dnf install -y polkit".to_string(),
            });
        }

        let (ok, err_msg) = repair_security_setting(setting_key);
        if ok {
            Ok(())
        } else if err_msg.contains("dismissed") || err_msg.contains("cancel") {
            Err(BlazeError::PolkitCancelled)
        } else {
            Err(BlazeError::PermissionDenied {
                action: format!("repair_security_setting({})", setting_key),
                details: err_msg,
            })
        }
    }

    pub fn get_cyber_tools(&self) -> Result<Vec<CyberTool>, BlazeError> {
        Ok(list_cyber_tools())
    }

    pub async fn run_cyber_tool(
        &self,
        tool_command: &str,
        args: &[String],
        use_root: bool,
        tx: mpsc::Sender<String>,
    ) -> Result<(), BlazeError> {
        if use_root && !has_cmd("pkexec") {
            return Err(BlazeError::CapabilityMissing {
                command: "pkexec".to_string(),
                hint: "Root ile çalıştırmak için policykit-1 gereklidir".to_string(),
            });
        }

        match run_cyber_tool_stream(tool_command, args, use_root, tx).await {
            Ok(()) => Ok(()),
            Err(e) => Err(BlazeError::ExecutionError {
                command: tool_command.to_string(),
                stdout: String::new(),
                stderr: e,
            }),
        }
    }
}

impl Default for SecurityService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_security_service_audit() {
        let service = SecurityService::new();
        let res = service.run_audit();
        assert!(res.is_ok());
    }

    #[tokio::test]
    async fn test_security_service_cyber_tools() {
        let service = SecurityService::new();
        let tools = service.get_cyber_tools();
        assert!(tools.is_ok());

        let (tx, mut rx) = mpsc::channel(50);
        let res = service
            .run_cyber_tool("echo", &["test_cyber".to_string()], false, tx)
            .await;
        assert!(res.is_ok());
        let mut lines = Vec::new();
        while let Some(l) = rx.recv().await {
            lines.push(l);
        }
        assert!(lines.iter().any(|line| line.contains("test_cyber")));
    }
}
