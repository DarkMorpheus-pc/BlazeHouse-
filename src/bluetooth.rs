use crate::capabilities::{has_cmd, run_cmd};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BluetoothDevice {
    pub mac: String,
    pub name: String,
    pub paired: bool,
    pub connected: bool,
}

fn bt_daemon_active() -> bool {
    if !has_cmd("bluetoothctl") {
        return false;
    }
    if has_cmd("systemctl") {
        let (ok, _) = run_cmd("systemctl", &["is-active", "--quiet", "bluetooth"]);
        if !ok {
            return false;
        }
    }
    true
}

pub fn bt_powered() -> bool {
    if bt_daemon_active() {
        let (ok, out) = run_cmd("bluetoothctl", &["--timeout", "2", "show"]);
        if ok {
            for line in out.lines() {
                if line.trim().starts_with("Powered:") {
                    return line.contains("yes");
                }
            }
        }
    }
    false
}

pub fn set_bt_powered(enabled: bool) -> bool {
    if bt_daemon_active() {
        let state = if enabled { "on" } else { "off" };
        let (ok, _) = run_cmd("bluetoothctl", &["--timeout", "2", "power", state]);
        return ok;
    }
    false
}

pub fn bt_discoverable() -> bool {
    if bt_daemon_active() {
        let (ok, out) = run_cmd("bluetoothctl", &["--timeout", "2", "show"]);
        if ok {
            for line in out.lines() {
                if line.trim().starts_with("Discoverable:") {
                    return line.contains("yes");
                }
            }
        }
    }
    false
}

pub fn set_bt_discoverable(enabled: bool) -> bool {
    if bt_daemon_active() {
        let state = if enabled { "on" } else { "off" };
        let (ok, _) = run_cmd("bluetoothctl", &["--timeout", "2", "discoverable", state]);
        return ok;
    }
    false
}

pub fn bt_devices() -> Vec<BluetoothDevice> {
    if !bt_powered() {
        return Vec::new();
    }

    let (ok, out) = run_cmd("bluetoothctl", &["devices"]);
    if !ok || out.trim().is_empty() {
        return Vec::new();
    }

    let mut list = Vec::new();
    for line in out.lines() {
        // "Device 00:11:22:33:44:55 DeviceName"
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 3 && parts[0] == "Device" {
            let mac = parts[1].to_string();
            let name = parts[2..].join(" ");
            let (connected, paired) = check_device_info(&mac);

            list.push(BluetoothDevice {
                mac,
                name,
                paired,
                connected,
            });
        }
    }
    list
}

pub fn bt_connect(mac: &str) -> bool {
    if has_cmd("bluetoothctl") {
        let (ok, _) = run_cmd("bluetoothctl", &["connect", mac]);
        return ok;
    }
    false
}

pub fn bt_disconnect(mac: &str) -> bool {
    if has_cmd("bluetoothctl") {
        let (ok, _) = run_cmd("bluetoothctl", &["disconnect", mac]);
        return ok;
    }
    false
}

pub fn bt_pair(mac: &str) -> bool {
    if has_cmd("bluetoothctl") {
        let (ok, _) = run_cmd("bluetoothctl", &["pair", mac]);
        return ok;
    }
    false
}

pub fn bt_remove(mac: &str) -> bool {
    if has_cmd("bluetoothctl") {
        let (ok, _) = run_cmd("bluetoothctl", &["remove", mac]);
        return ok;
    }
    false
}

fn check_device_info(mac: &str) -> (bool, bool) {
    let (ok, out) = run_cmd("bluetoothctl", &["info", mac]);
    let mut connected = false;
    let mut paired = false;
    if ok {
        for line in out.lines() {
            let l = line.trim();
            if l.starts_with("Connected:") {
                connected = l.contains("yes");
            } else if l.starts_with("Paired:") {
                paired = l.contains("yes");
            }
        }
    }
    (connected, paired)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bluetooth_powered() {
        println!("Bluetooth powered: {}", bt_powered());
    }
}
