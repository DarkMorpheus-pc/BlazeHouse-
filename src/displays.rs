use crate::capabilities::{has_cmd, run_cmd};
use regex::Regex;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayOutput {
    pub name: String,
    pub connected: bool,
    pub enabled: bool,
    pub modes: Vec<String>,
    pub current: String,
    pub scale: f32,
    pub position: (i32, i32),
    pub primary: bool,
}

pub fn displays() -> Vec<DisplayOutput> {
    if has_cmd("niri") {
        let outputs = parse_niri_outputs();
        if !outputs.is_empty() {
            return outputs;
        }
    }
    if has_cmd("wlr-randr") {
        let outputs = parse_wlr_randr();
        if !outputs.is_empty() {
            return outputs;
        }
    }
    if has_cmd("xrandr") {
        return parse_xrandr();
    }
    Vec::new()
}

pub fn set_mode(output: &str, mode: &str) -> bool {
    let (res, rate) = if let Some((r, hz)) = mode.split_once('@') {
        (r, hz)
    } else {
        (mode, "")
    };

    let mode_arg = if rate.is_empty() {
        res.to_string()
    } else {
        format!("{}@{}", res, rate)
    };

    if has_cmd("niri") {
        let (ok, _) = run_cmd("niri", &["msg", "output", output, "mode", &mode_arg]);
        if ok {
            return true;
        }
    }
    if has_cmd("wlr-randr") {
        let (ok, _) = run_cmd("wlr-randr", &["--output", output, "--mode", &mode_arg]);
        if ok {
            return true;
        }
    }
    if has_cmd("xrandr") {
        let mut args = vec!["--output", output, "--mode", res];
        if !rate.is_empty() {
            args.push("--rate");
            args.push(rate);
        }
        let (ok, _) = run_cmd("xrandr", &args);
        return ok;
    }
    false
}

pub fn set_scale(output: &str, scale: f32) -> bool {
    let scale_str = format!("{:.2}", scale);
    if has_cmd("niri") {
        let (ok, _) = run_cmd("niri", &["msg", "output", output, "scale", &scale_str]);
        if ok {
            return true;
        }
    }
    if has_cmd("wlr-randr") {
        let (ok, _) = run_cmd("wlr-randr", &["--output", output, "--scale", &scale_str]);
        if ok {
            return true;
        }
    }
    false
}

pub fn set_enabled(output: &str, enabled: bool) -> bool {
    if has_cmd("niri") {
        let flag = if enabled { "on" } else { "off" };
        let (ok, _) = run_cmd("niri", &["msg", "output", output, flag]);
        if ok {
            return true;
        }
    }
    if has_cmd("wlr-randr") {
        let flag = if enabled { "--on" } else { "--off" };
        let (ok, _) = run_cmd("wlr-randr", &["--output", output, flag]);
        if ok {
            return true;
        }
    }
    if has_cmd("xrandr") {
        let flag = if enabled { "--auto" } else { "--off" };
        let (ok, _) = run_cmd("xrandr", &["--output", output, flag]);
        return ok;
    }
    false
}

fn parse_niri_outputs() -> Vec<DisplayOutput> {
    let (ok, out) = run_cmd("niri", &["msg", "--json", "outputs"]);
    if !ok || out.trim().is_empty() {
        return Vec::new();
    }
    parse_niri_outputs_from_str(&out)
}

pub fn parse_niri_outputs_from_str(out: &str) -> Vec<DisplayOutput> {
    let Ok(val) = serde_json::from_str::<serde_json::Value>(out) else {
        return Vec::new();
    };
    let Some(map) = val.as_object() else {
        return Vec::new();
    };

    let mut outputs = Vec::new();
    for (idx, (key, obj)) in map.iter().enumerate() {
        let name = obj
            .get("name")
            .and_then(|v| v.as_str())
            .unwrap_or(key)
            .to_string();
        let current_idx = obj.get("current_mode").and_then(|v| v.as_u64());
        let mut modes = Vec::new();
        let mut current = String::new();

        if let Some(modes_arr) = obj.get("modes").and_then(|v| v.as_array()) {
            for (m_idx, m) in modes_arr.iter().enumerate() {
                let w = m.get("width").and_then(|v| v.as_u64()).unwrap_or(0);
                let h = m.get("height").and_then(|v| v.as_u64()).unwrap_or(0);
                let raw_rr = m
                    .get("refresh_rate")
                    .and_then(|v| v.as_f64())
                    .unwrap_or(60000.0);
                let hz = if raw_rr > 1000.0 {
                    raw_rr / 1000.0
                } else {
                    raw_rr
                };
                if w > 0 && h > 0 {
                    let mode_str = format!("{}x{}@{:.2}", w, h, hz);
                    if current_idx == Some(m_idx as u64) {
                        current = mode_str.clone();
                    }
                    if !modes.contains(&mode_str) {
                        modes.push(mode_str);
                    }
                }
            }
        }

        let logical = obj.get("logical");
        let enabled = logical.map(|v| !v.is_null()).unwrap_or(true);
        let scale = logical
            .and_then(|l| l.get("scale"))
            .and_then(|v| v.as_f64())
            .unwrap_or(1.0) as f32;
        let x = logical
            .and_then(|l| l.get("x"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;
        let y = logical
            .and_then(|l| l.get("y"))
            .and_then(|v| v.as_i64())
            .unwrap_or(0) as i32;

        outputs.push(DisplayOutput {
            name,
            connected: true,
            enabled,
            modes,
            current,
            scale,
            position: (x, y),
            primary: idx == 0,
        });
    }
    outputs
}

fn parse_wlr_randr() -> Vec<DisplayOutput> {
    let (ok, out) = run_cmd("wlr-randr", &[]);
    if !ok || out.trim().is_empty() {
        return Vec::new();
    }

    let mut outputs = Vec::new();
    let mut current_output: Option<DisplayOutput> = None;
    let mode_re = Regex::new(r"^(\d+x\d+) px,\s*([0-9.]+) Hz(.*)$").unwrap();

    for line in out.lines() {
        if !line.starts_with(' ') && !line.trim().is_empty() {
            if let Some(o) = current_output.take() {
                outputs.push(o);
            }
            let name = line
                .split_whitespace()
                .next()
                .unwrap_or("Display")
                .to_string();
            current_output = Some(DisplayOutput {
                name,
                connected: true,
                enabled: true,
                modes: Vec::new(),
                current: String::new(),
                scale: 1.0,
                position: (0, 0),
                primary: false,
            });
            continue;
        }

        if let Some(ref mut o) = current_output {
            let s = line.trim();
            if let Some(val) = s.strip_prefix("Enabled:") {
                o.enabled = val.trim().to_lowercase() == "yes";
            } else if let Some(val) = s.strip_prefix("Scale:") {
                if let Ok(sc) = val.trim().parse::<f32>() {
                    o.scale = sc;
                }
            } else if let Some(val) = s.strip_prefix("Position:") {
                let parts: Vec<&str> = val.trim().split(',').collect();
                if parts.len() == 2 {
                    if let (Ok(x), Ok(y)) = (
                        parts[0].trim().parse::<i32>(),
                        parts[1].trim().parse::<i32>(),
                    ) {
                        o.position = (x, y);
                    }
                }
            } else if let Some(caps) = mode_re.captures(s) {
                let res = caps.get(1).unwrap().as_str();
                let hz: f64 = caps.get(2).unwrap().as_str().parse().unwrap_or(60.0);
                let extra = caps.get(3).unwrap().as_str();
                let mode_str = format!("{}@{:.2}", res, hz);
                o.modes.push(mode_str.clone());
                if extra.contains("current") {
                    o.current = mode_str;
                }
            }
        }
    }

    if let Some(o) = current_output {
        outputs.push(o);
    }
    outputs
}

fn parse_xrandr() -> Vec<DisplayOutput> {
    let (ok, out) = run_cmd("xrandr", &["--query"]);
    if !ok || out.trim().is_empty() {
        return Vec::new();
    }

    let mut outputs = Vec::new();
    let mut current_output: Option<DisplayOutput> = None;
    let header_re = Regex::new(r"^(\S+)\s+(connected|disconnected)").unwrap();
    let mode_re = Regex::new(r"^\s+(\d+x\d+)\s+(.*)$").unwrap();

    for line in out.lines() {
        if let Some(caps) = header_re.captures(line) {
            if let Some(o) = current_output.take() {
                outputs.push(o);
            }
            let name = caps.get(1).unwrap().as_str().to_string();
            let connected = caps.get(2).unwrap().as_str() == "connected";
            let primary = line.contains("primary");

            current_output = Some(DisplayOutput {
                name,
                connected,
                enabled: connected,
                modes: Vec::new(),
                current: String::new(),
                scale: 1.0,
                position: (0, 0),
                primary,
            });
            continue;
        }

        if let Some(ref mut o) = current_output {
            if let Some(caps) = mode_re.captures(line) {
                let res = caps.get(1).unwrap().as_str();
                let rest = caps.get(2).unwrap().as_str();
                for part in rest.split_whitespace() {
                    let is_current = part.contains('*');
                    let clean_rate = part.replace(['*', '+'], "");
                    if let Ok(hz) = clean_rate.parse::<f64>() {
                        let mode_str = format!("{}@{:.2}", res, hz);
                        o.modes.push(mode_str.clone());
                        if is_current {
                            o.current = mode_str;
                        }
                    }
                }
            }
        }
    }

    if let Some(o) = current_output {
        outputs.push(o);
    }
    outputs
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_displays() {
        let list = displays();
        println!("Detected Displays: {:?}", list);
    }
}
