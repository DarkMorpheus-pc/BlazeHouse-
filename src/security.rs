use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use std::process::Stdio;
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::Command as AsyncCommand;
use tokio::sync::mpsc;

use crate::capabilities::{has_cmd, run_cmd};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityAudit {
    pub ufw_active: bool,
    pub firewall_state: String,
    pub kptr_restricted: bool,
    pub kptr_state: String,
    pub dmesg_restricted: bool,
    pub dmesg_state: String,
    pub apparmor_active: bool,
    pub apparmor_state: String,
    pub selinux_state: String,
    pub secure_boot_state: String,
    pub lockdown_state: String,
    pub aslr_state: String,
    pub unattended_upgrades: bool,
    pub unattended_state: String,
}

pub fn check_system_security() -> SecurityAudit {
    // 1. UFW Firewall Status
    let (ufw_active, firewall_state) = if has_cmd("ufw") {
        let (ok, out) = run_cmd("ufw", &["status"]);
        if ok && out.contains("Status: active") {
            (true, "ACTIVE".to_string())
        } else {
            (false, "INACTIVE".to_string())
        }
    } else if has_cmd("firewall-cmd") {
        let (ok, out) = run_cmd("firewall-cmd", &["--state"]);
        if ok && out.trim() == "running" {
            (true, "ACTIVE".to_string())
        } else {
            (false, "INACTIVE".to_string())
        }
    } else {
        (false, "NOT AVAILABLE".to_string())
    };

    // 2. Kernel kptr_restrict
    let kptr_val = fs::read_to_string("/proc/sys/kernel/kptr_restrict")
        .unwrap_or_default()
        .trim()
        .to_string();
    let (kptr_restricted, kptr_state) = match kptr_val.as_str() {
        "2" => (true, "ENFORCING (Level 2)".to_string()),
        "1" => (true, "ENFORCING (Level 1)".to_string()),
        "0" => (false, "DISABLED (Level 0)".to_string()),
        _ => (false, "UNKNOWN".to_string()),
    };

    // 3. Kernel dmesg_restrict
    let dmesg_val = fs::read_to_string("/proc/sys/kernel/dmesg_restrict")
        .unwrap_or_default()
        .trim()
        .to_string();
    let (dmesg_restricted, dmesg_state) = match dmesg_val.as_str() {
        "1" => (true, "ENFORCING".to_string()),
        "0" => (false, "DISABLED".to_string()),
        _ => (false, "UNKNOWN".to_string()),
    };

    // 4. AppArmor Status
    let (apparmor_active, apparmor_state) =
        if Path::new("/sys/module/apparmor/parameters/enabled").exists() {
            let enabled = fs::read_to_string("/sys/module/apparmor/parameters/enabled")
                .unwrap_or_default()
                .trim()
                .to_uppercase()
                == "Y";
            if enabled {
                (true, "ACTIVE (Enforcing)".to_string())
            } else {
                (false, "INACTIVE".to_string())
            }
        } else {
            (false, "NOT AVAILABLE".to_string())
        };

    // 5. SELinux Status
    let selinux_state = if has_cmd("getenforce") {
        let (ok, out) = run_cmd("getenforce", &[]);
        if ok {
            out.trim().to_uppercase()
        } else {
            "UNKNOWN".to_string()
        }
    } else if Path::new("/sys/fs/selinux").exists() {
        "ACTIVE".to_string()
    } else {
        "NOT AVAILABLE".to_string()
    };

    // 6. Secure Boot Status
    let secure_boot_state = if has_cmd("mokutil") {
        let (ok, out) = run_cmd("mokutil", &["--sb-state"]);
        if ok && out.contains("SecureBoot enabled") {
            "ACTIVE".to_string()
        } else if ok && out.contains("SecureBoot disabled") {
            "DISABLED".to_string()
        } else {
            "NOT AVAILABLE".to_string()
        }
    } else {
        "NOT AVAILABLE".to_string()
    };

    // 7. Kernel Lockdown Status
    let lockdown_state = if Path::new("/sys/kernel/security/lockdown").exists() {
        let val = fs::read_to_string("/sys/kernel/security/lockdown").unwrap_or_default();
        if val.contains("[integrity]") {
            "ENFORCING (Integrity)".to_string()
        } else if val.contains("[confidentiality]") {
            "ENFORCING (Confidentiality)".to_string()
        } else {
            "DISABLED".to_string()
        }
    } else {
        "NOT AVAILABLE".to_string()
    };

    // 8. ASLR Status (randomize_va_space)
    let aslr_val = fs::read_to_string("/proc/sys/kernel/randomize_va_space")
        .unwrap_or_default()
        .trim()
        .to_string();
    let aslr_state = match aslr_val.as_str() {
        "2" => "FULL (Level 2)".to_string(),
        "1" => "PARTIAL (Level 1)".to_string(),
        "0" => "DISABLED".to_string(),
        _ => "UNKNOWN".to_string(),
    };

    // 9. Unattended Upgrades / Automatic Updates
    let (unattended_upgrades, unattended_state) = if has_cmd("systemctl") && (has_cmd("dnf") || has_cmd("dnf5") || has_cmd("rpm-ostree")) {
        let timers = [
            "dnf-automatic.timer",
            "dnf5-automatic.timer",
            "rpm-ostreed-automatic.timer",
        ];
        let mut active_timer = None;
        for t in timers {
            let (_ok, out) = run_cmd("systemctl", &["is-enabled", t]);
            let st = out.trim();
            if st == "enabled" || st == "static" || st == "indirect" {
                active_timer = Some(t);
                break;
            }
            let (ok_act, out_act) = run_cmd("systemctl", &["is-active", t]);
            if ok_act && out_act.trim() == "active" {
                active_timer = Some(t);
                break;
            }
        }
        if let Some(t) = active_timer {
            (true, format!("ACTIVE ({})", t))
        } else {
            (false, "INACTIVE (dnf-automatic disabled)".to_string())
        }
    } else if has_cmd("dpkg-query") {
        let (ok, out) = run_cmd("dpkg-query", &["-W", "-f=${Status}", "unattended-upgrades"]);
        if ok && out.contains("installed") {
            (true, "INSTALLED & ACTIVE".to_string())
        } else {
            (false, "NOT INSTALLED".to_string())
        }
    } else {
        (false, "NOT AVAILABLE".to_string())
    };

    SecurityAudit {
        ufw_active,
        firewall_state,
        kptr_restricted,
        kptr_state,
        dmesg_restricted,
        dmesg_state,
        apparmor_active,
        apparmor_state,
        selinux_state,
        secure_boot_state,
        lockdown_state,
        aslr_state,
        unattended_upgrades,
        unattended_state,
    }
}

pub fn repair_security_setting(setting_key: &str) -> (bool, String) {
    match setting_key {
        "ufw" | "firewall" => {
            if has_cmd("firewall-cmd") {
                run_cmd("pkexec", &["systemctl", "enable", "--now", "firewalld"])
            } else if has_cmd("ufw") {
                run_cmd("pkexec", &["ufw", "enable"])
            } else {
                (false, "firewalld veya ufw bulunamadı".to_string())
            }
        }
        "kptr" => run_cmd("pkexec", &["sysctl", "-w", "kernel.kptr_restrict=2"]),
        "dmesg" => run_cmd("pkexec", &["sysctl", "-w", "kernel.dmesg_restrict=1"]),
        _ => (false, "Bilinmeyen güvenlik ayarı".to_string()),
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CyberTool {
    pub name: String,
    pub command: String,
    pub default_args: Vec<String>,
    pub requires_root: bool,
    pub installed: bool,
}

pub fn list_cyber_tools() -> Vec<CyberTool> {
    let tools_def = vec![
        (
            "Nmap Port Tarayıcı",
            "nmap",
            vec!["-F", "-sV", "127.0.0.1"],
            false,
        ),
        (
            "TCPDump Paket İzleyici",
            "tcpdump",
            vec!["-n", "-c", "10", "-i", "any"],
            true,
        ),
        ("TShark Trafik Analizi", "tshark", vec!["-c", "10"], true),
        (
            "Aircrack-ng Kablosuz Test",
            "aircrack-ng",
            vec!["--help"],
            false,
        ),
        (
            "Nikto Web Güvenlik Taraması",
            "nikto",
            vec!["-h", "localhost"],
            false,
        ),
        (
            "SQLMap SQL Enjeksiyon Testi",
            "sqlmap",
            vec!["--version"],
            false,
        ),
        (
            "John the Ripper Parola Testi",
            "john",
            vec!["--test"],
            false,
        ),
        ("Hydra Kaba Kuvvet Testi", "hydra", vec!["-h"], false),
        (
            "UFW Güvenlik Duvarı Durumu",
            "ufw",
            vec!["status", "verbose"],
            true,
        ),
        ("Fail2ban Koruması", "fail2ban-client", vec!["status"], true),
    ];

    tools_def
        .into_iter()
        .map(|(name, cmd, args, req_root)| CyberTool {
            name: name.to_string(),
            command: cmd.to_string(),
            default_args: args.into_iter().map(String::from).collect(),
            requires_root: req_root,
            installed: has_cmd(cmd),
        })
        .collect()
}

/// Runs a cyber security command asynchronously and streams stdout/stderr lines back via MPSC channel.
pub async fn run_cyber_tool_stream(
    cmd_name: &str,
    args: &[String],
    use_root: bool,
    tx: mpsc::Sender<String>,
) -> Result<(), String> {
    if !has_cmd(cmd_name) && !use_root {
        let _ = tx
            .send(format!("[HATA] Komut bulunamadı: {}", cmd_name))
            .await;
        return Err(format!("Komut bulunamadı: {}", cmd_name));
    }

    let mut child = if use_root {
        let mut full_args = vec![cmd_name.to_string()];
        full_args.extend_from_slice(args);
        AsyncCommand::new("pkexec")
            .args(&full_args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?
    } else {
        AsyncCommand::new(cmd_name)
            .args(args)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .map_err(|e| e.to_string())?
    };

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();

    let tx_stdout = tx.clone();
    if let Some(stdout_stream) = stdout {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stdout_stream).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                if tx_stdout.send(line).await.is_err() {
                    break;
                }
            }
        });
    }

    let tx_stderr = tx.clone();
    if let Some(stderr_stream) = stderr {
        tokio::spawn(async move {
            let mut reader = BufReader::new(stderr_stream).lines();
            while let Ok(Some(line)) = reader.next_line().await {
                if tx_stderr.send(format!("[STDERR] {}", line)).await.is_err() {
                    break;
                }
            }
        });
    }

    let status = child.wait().await.map_err(|e| e.to_string())?;
    let _ = tx
        .send(format!("[BİTTİ] Komut çıkış kodu: {}", status))
        .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_check_system_security() {
        let audit = check_system_security();
        println!("Security Audit: {:?}", audit);
        assert!(!audit.firewall_state.is_empty());
    }

    #[tokio::test]
    async fn test_run_cyber_tool_stream() {
        let (tx, mut rx) = mpsc::channel(100);
        let res = run_cyber_tool_stream("echo", &["test_stream".to_string()], false, tx).await;
        assert!(res.is_ok());

        let mut lines = Vec::new();
        while let Some(line) = rx.recv().await {
            lines.push(line);
        }
        assert!(lines.iter().any(|l| l.contains("test_stream")));
    }
}
