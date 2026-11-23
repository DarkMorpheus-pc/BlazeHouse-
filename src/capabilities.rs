use std::env;
use std::path::Path;
use std::process::Command;

/// Checks if an executable command exists in PATH or at an absolute path.
pub fn has_cmd(cmd: &str) -> bool {
    if cmd.contains('/') {
        return Path::new(cmd).is_file();
    }
    if let Ok(path_var) = env::var("PATH") {
        for path_dir in env::split_paths(&path_var) {
            let full_path = path_dir.join(cmd);
            if full_path.is_file() {
                return true;
            }
        }
    }
    false
}

/// Helper function to execute a command with args and capture output.
pub fn run_cmd(cmd: &str, args: &[&str]) -> (bool, String) {
    match Command::new(cmd).args(args).output() {
        Ok(output) => {
            let stdout = String::from_utf8_lossy(&output.stdout).to_string();
            let stderr = String::from_utf8_lossy(&output.stderr).to_string();
            if output.status.success() {
                (true, stdout.trim().to_string())
            } else {
                let err_msg = if !stderr.trim().is_empty() {
                    stderr.trim().to_string()
                } else {
                    stdout.trim().to_string()
                };
                (false, err_msg)
            }
        }
        Err(e) => (false, e.to_string()),
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Capability {
    pub name: String,
    pub available: bool,
    pub reason: String,
    pub tooltip: String,
    pub hint: String,
}

impl Capability {
    pub fn ok(name: &str) -> Self {
        Self {
            name: name.to_string(),
            available: true,
            reason: String::new(),
            tooltip: String::new(),
            hint: String::new(),
        }
    }

    pub fn unavailable(name: &str, reason: &str, hint: &str) -> Self {
        Self {
            name: name.to_string(),
            available: false,
            reason: reason.to_string(),
            tooltip: format!("{} ({})", reason, hint),
            hint: hint.to_string(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Capabilities {
    pub audio: Capability,
    pub display: Capability,
    pub network: Capability,
    pub bluetooth: Capability,
    pub datetime: Capability,
    pub locale: Capability,
    pub gsettings: Capability,
    pub polkit: Capability,
    pub firewall: Capability,
    pub power: Capability,
}

impl Capabilities {
    pub fn detect() -> Self {
        let audio = if has_cmd("wpctl") || has_cmd("pactl") {
            Capability::ok("audio")
        } else {
            Capability::unavailable(
                "audio",
                "wpctl veya pactl bulunamadı",
                "sudo dnf install -y wireplumber pulseaudio-utils",
            )
        };

        let display = if has_cmd("niri") || has_cmd("wlr-randr") || has_cmd("xrandr") {
            Capability::ok("display")
        } else {
            Capability::unavailable(
                "display",
                "niri, wlr-randr veya xrandr bulunamadı",
                "sudo dnf install -y wlr-randr xrandr",
            )
        };

        let network = if has_cmd("nmcli") {
            Capability::ok("network")
        } else {
            Capability::unavailable(
                "network",
                "nmcli bulunamadı",
                "sudo dnf install -y NetworkManager",
            )
        };

        let bluetooth = if has_cmd("bluetoothctl") {
            Capability::ok("bluetooth")
        } else {
            Capability::unavailable(
                "bluetooth",
                "bluetoothctl bulunamadı",
                "sudo dnf install -y bluez",
            )
        };

        let datetime = if has_cmd("timedatectl") {
            Capability::ok("datetime")
        } else {
            Capability::unavailable(
                "datetime",
                "timedatectl bulunamadı",
                "systemd-timedated servisini kontrol edin",
            )
        };

        let locale = if has_cmd("localectl") {
            Capability::ok("locale")
        } else {
            Capability::unavailable(
                "locale",
                "localectl bulunamadı",
                "systemd-localed servisini kontrol edin",
            )
        };

        let gsettings = if has_cmd("gsettings") {
            Capability::ok("gsettings")
        } else {
            Capability::unavailable(
                "gsettings",
                "gsettings bulunamadı",
                "sudo dnf install -y glib2",
            )
        };

        let polkit = if has_cmd("pkexec") {
            Capability::ok("polkit")
        } else {
            Capability::unavailable(
                "polkit",
                "pkexec (polkit) bulunamadı",
                "sudo dnf install -y polkit",
            )
        };

        let firewall = if has_cmd("firewall-cmd") || has_cmd("ufw") {
            Capability::ok("firewall")
        } else {
            Capability::unavailable(
                "firewall",
                "firewalld veya ufw bulunamadı",
                "sudo dnf install -y firewalld",
            )
        };

        let power = if has_cmd("powerprofilesctl") || Path::new("/sys/class/power_supply").exists()
        {
            Capability::ok("power")
        } else {
            Capability::unavailable(
                "power",
                "powerprofilesctl veya /sys/class/power_supply bulunamadı",
                "sudo dnf install -y power-profiles-daemon",
            )
        };

        Self {
            audio,
            display,
            network,
            bluetooth,
            datetime,
            locale,
            gsettings,
            polkit,
            firewall,
            power,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_has_cmd() {
        assert!(has_cmd("sh") || has_cmd("bash") || has_cmd("ls"));
        assert!(!has_cmd("non_existent_command_xyz_123"));
    }

    #[test]
    fn test_run_cmd() {
        let (ok, out) = run_cmd("echo", &["hello"]);
        assert!(ok);
        assert_eq!(out, "hello");
    }

    #[test]
    fn test_capabilities_detect() {
        let caps = Capabilities::detect();
        // sh/bash system will have some commands
        println!("Detected caps: {:?}", caps);
    }
}
