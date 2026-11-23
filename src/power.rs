use crate::capabilities::{has_cmd, run_cmd};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatteryInfo {
    pub name: String,
    pub capacity_percent: u32,
    pub status: String,
    pub technology: String,
    pub health_percent: Option<u32>,
}

pub fn active_power_profile() -> String {
    if has_cmd("powerprofilesctl") {
        let (ok, out) = run_cmd("powerprofilesctl", &["get"]);
        if ok && !out.trim().is_empty() {
            return out.trim().to_string();
        }
    }
    String::from("balanced")
}

pub fn set_power_profile(profile: &str) -> bool {
    let valid_profile = match profile {
        "performance" | "balanced" | "power-saver" => profile,
        _ => "balanced",
    };
    if has_cmd("powerprofilesctl") {
        let (ok, _) = run_cmd("powerprofilesctl", &["set", valid_profile]);
        return ok;
    }
    false
}

pub fn battery_info() -> Option<BatteryInfo> {
    let ps_dir = Path::new("/sys/class/power_supply");
    if !ps_dir.exists() {
        return None;
    }

    if let Ok(entries) = fs::read_dir(ps_dir) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().to_string();
            if name.starts_with("BAT") {
                let bat_path = entry.path();
                let capacity = fs::read_to_string(bat_path.join("capacity"))
                    .unwrap_or_default()
                    .trim()
                    .parse::<u32>()
                    .unwrap_or(0);

                let status = fs::read_to_string(bat_path.join("status"))
                    .unwrap_or_else(|_| String::from("Discharging"))
                    .trim()
                    .to_string();

                let technology = fs::read_to_string(bat_path.join("technology"))
                    .unwrap_or_else(|_| String::from("Li-ion"))
                    .trim()
                    .to_string();

                let energy_full = fs::read_to_string(bat_path.join("energy_full"))
                    .or_else(|_| fs::read_to_string(bat_path.join("charge_full")))
                    .unwrap_or_default()
                    .trim()
                    .parse::<f64>()
                    .unwrap_or(0.0);

                let energy_full_design = fs::read_to_string(bat_path.join("energy_full_design"))
                    .or_else(|_| fs::read_to_string(bat_path.join("charge_full_design")))
                    .unwrap_or_default()
                    .trim()
                    .parse::<f64>()
                    .unwrap_or(0.0);

                let health_percent = if energy_full_design > 0.0 {
                    Some(
                        ((energy_full / energy_full_design) * 100.0)
                            .round()
                            .min(100.0) as u32,
                    )
                } else {
                    None
                };

                return Some(BatteryInfo {
                    name,
                    capacity_percent: capacity,
                    status,
                    technology,
                    health_percent,
                });
            }
        }
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_power_profile() {
        let p = active_power_profile();
        println!("Active Power Profile: {}", p);
    }
}
