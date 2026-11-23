use crate::capabilities::{has_cmd, run_cmd};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputDeviceInfo {
    pub name: String,
    pub dev_type: String,
    pub handlers: String,
}

pub fn keyboard_layouts() -> (String, String) {
    if has_cmd("localectl") {
        let (ok, out) = run_cmd("localectl", &["status"]);
        if ok {
            let mut layout = String::from("tr");
            let mut model = String::new();
            for line in out.lines() {
                let trimmed = line.trim();
                if let Some(val) = trimmed.strip_prefix("X11 Layout:") {
                    layout = val.trim().to_string();
                } else if let Some(val) = trimmed.strip_prefix("X11 Model:") {
                    model = val.trim().to_string();
                }
            }
            return (layout, model);
        }
    }
    (String::from("tr"), String::new())
}

pub fn available_layouts() -> Vec<&'static str> {
    vec!["tr", "us", "de", "fr", "es", "ru", "gb", "it"]
}

pub fn update_niri_kdl_layout(content: &str, layout: &str) -> String {
    let re = Regex::new(r#"(\blayout\s+)"[^"]*""#).unwrap();
    if re.is_match(content) {
        re.replace_all(content, format!("${{1}}\"{}\"", layout))
            .to_string()
    } else {
        format!(
            "{}\ninput {{\n    keyboard {{\n        xkb {{\n            layout \"{}\"\n        }}\n    }}\n}}\n",
            content.trim_end(),
            layout
        )
    }
}

pub fn apply_keymap(layout: &str) -> bool {
    set_keyboard_layout(layout)
}

pub fn set_keyboard_layout(layout: &str) -> bool {
    let mut any_ok = false;

    // 1. Update Niri / SolarUI config files if they exist or if running on Niri/SolarUI
    let home = env::var("HOME").unwrap_or_else(|_| String::from("/root"));
    let home_path = PathBuf::from(&home);
    let desktop = env::var("XDG_CURRENT_DESKTOP")
        .unwrap_or_default()
        .to_lowercase();
    let is_niri_or_solar = desktop.contains("niri")
        || desktop.contains("solarui")
        || env::var("NIRI_SOCKET").is_ok();

    for rel in [".config/niri/config.kdl", ".config/solarui/niri.kdl"] {
        let p = home_path.join(rel);
        if p.exists() || (is_niri_or_solar && rel == ".config/niri/config.kdl") {
            if let Some(parent) = p.parent() {
                let _ = fs::create_dir_all(parent);
            }
            let existing = fs::read_to_string(&p).unwrap_or_default();
            let updated = update_niri_kdl_layout(&existing, layout);
            if fs::write(&p, updated).is_ok() {
                any_ok = true;
            }
        }
    }

    // 2. gsettings (GNOME / Cinnamon)
    if has_cmd("gsettings") {
        let sources = format!("[('xkb', '{}')]", layout);
        let (ok, _) = run_cmd(
            "gsettings",
            &["set", "org.gnome.desktop.input-sources", "sources", &sources],
        );
        if ok {
            any_ok = true;
        }
    }

    // 3. X11 setxkbmap if DISPLAY is set
    if env::var("DISPLAY").is_ok() && has_cmd("setxkbmap") {
        let (ok, _) = run_cmd("setxkbmap", &[layout]);
        if ok {
            any_ok = true;
        }
    }

    // 4. System-wide localectl
    if has_cmd("localectl") {
        let (ok, _) = run_cmd("localectl", &["set-x11-keymap", layout]);
        if ok {
            any_ok = true;
        }
    }

    any_ok
}

pub fn system_locale() -> String {
    if has_cmd("localectl") {
        let (ok, out) = run_cmd("localectl", &["status"]);
        if ok {
            for line in out.lines() {
                if let Some(val) = line.trim().strip_prefix("System Locale:") {
                    return val.trim().to_string();
                }
            }
        }
    }
    String::from("LANG=tr_TR.UTF-8")
}

pub fn mouse_speed() -> Option<f64> {
    if has_cmd("gsettings") {
        let (ok, out) = run_cmd(
            "gsettings",
            &["get", "org.gnome.desktop.peripherals.mouse", "speed"],
        );
        if ok {
            if let Ok(val) = out.trim().parse::<f64>() {
                return Some(val);
            }
        }
    }
    None
}

pub fn set_mouse_speed(speed: f64) -> bool {
    let clamped = speed.clamp(-1.0, 1.0);
    if has_cmd("gsettings") {
        let (ok, _) = run_cmd(
            "gsettings",
            &[
                "set",
                "org.gnome.desktop.peripherals.mouse",
                "speed",
                &format!("{:.2}", clamped),
            ],
        );
        return ok;
    }
    false
}

pub fn mouse_natural_scroll() -> bool {
    if has_cmd("gsettings") {
        let (ok, out) = run_cmd(
            "gsettings",
            &[
                "get",
                "org.gnome.desktop.peripherals.mouse",
                "natural-scroll",
            ],
        );
        return ok && out.trim() == "true";
    }
    false
}

pub fn set_mouse_natural_scroll(enabled: bool) -> bool {
    if has_cmd("gsettings") {
        let flag = if enabled { "true" } else { "false" };
        let (ok, _) = run_cmd(
            "gsettings",
            &[
                "set",
                "org.gnome.desktop.peripherals.mouse",
                "natural-scroll",
                flag,
            ],
        );
        return ok;
    }
    false
}

pub fn touchpad_present() -> bool {
    if let Ok(entries) = fs::read_dir("/sys/class/input") {
        for entry in entries.flatten() {
            let name_path = entry.path().join("device/name");
            if let Ok(name) = fs::read_to_string(name_path) {
                let lower = name.to_lowercase();
                if lower.contains("touchpad")
                    || lower.contains("trackpad")
                    || lower.contains("synaptics")
                {
                    return true;
                }
            }
        }
    }
    false
}

pub fn touchpad_tap_to_click() -> bool {
    if has_cmd("gsettings") {
        let (ok, out) = run_cmd(
            "gsettings",
            &[
                "get",
                "org.gnome.desktop.peripherals.touchpad",
                "tap-to-click",
            ],
        );
        return ok && out.trim() == "true";
    }
    false
}

pub fn set_touchpad_tap_to_click(enabled: bool) -> bool {
    if has_cmd("gsettings") {
        let flag = if enabled { "true" } else { "false" };
        let (ok, _) = run_cmd(
            "gsettings",
            &[
                "set",
                "org.gnome.desktop.peripherals.touchpad",
                "tap-to-click",
                flag,
            ],
        );
        return ok;
    }
    false
}

pub fn touchpad_natural_scroll() -> bool {
    if has_cmd("gsettings") {
        let (ok, out) = run_cmd(
            "gsettings",
            &[
                "get",
                "org.gnome.desktop.peripherals.touchpad",
                "natural-scroll",
            ],
        );
        return ok && out.trim() == "true";
    }
    false
}

pub fn set_touchpad_natural_scroll(enabled: bool) -> bool {
    if has_cmd("gsettings") {
        let flag = if enabled { "true" } else { "false" };
        let (ok, _) = run_cmd(
            "gsettings",
            &[
                "set",
                "org.gnome.desktop.peripherals.touchpad",
                "natural-scroll",
                flag,
            ],
        );
        return ok;
    }
    false
}

pub fn input_devices() -> Vec<InputDeviceInfo> {
    if let Ok(content) = fs::read_to_string("/proc/bus/input/devices") {
        let mut list = Vec::new();
        let mut current_name = String::new();
        let mut current_handlers = String::new();

        for line in content.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                if !current_name.is_empty() {
                    let dev_type = classify_input_device(&current_name, &current_handlers);
                    list.push(InputDeviceInfo {
                        name: current_name.clone(),
                        dev_type,
                        handlers: current_handlers.clone(),
                    });
                    current_name.clear();
                    current_handlers.clear();
                }
                continue;
            }

            if let Some(name_val) = trimmed.strip_prefix("N: Name=") {
                current_name = name_val.trim_matches('"').to_string();
            } else if let Some(h_val) = trimmed.strip_prefix("H: Handlers=") {
                current_handlers = h_val.trim().to_string();
            }
        }

        if !current_name.is_empty() {
            let dev_type = classify_input_device(&current_name, &current_handlers);
            list.push(InputDeviceInfo {
                name: current_name,
                dev_type,
                handlers: current_handlers,
            });
        }
        return list;
    }
    Vec::new()
}

fn classify_input_device(name: &str, handlers: &str) -> String {
    let lower = name.to_lowercase();
    let h_lower = handlers.to_lowercase();

    if lower.contains("touchpad") || lower.contains("trackpad") || lower.contains("synaptics") {
        "Dokunmatik Yüzey (Touchpad)".to_string()
    } else if lower.contains("mouse") || h_lower.contains("mouse") {
        "Fare (Mouse)".to_string()
    } else if lower.contains("keyboard") || h_lower.contains("kbd") {
        "Klavye (Keyboard)".to_string()
    } else if lower.contains("joystick") || lower.contains("gamepad") {
        "Oyun Kolu (Gamepad)".to_string()
    } else {
        "Giriş Aygıtı (Input Device)".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keyboard_layouts() {
        let (l, m) = keyboard_layouts();
        println!("Keyboard Layout: {}, Model: {}", l, m);
    }
}
