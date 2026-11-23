use serde::{Deserialize, Serialize};
use std::fs;
use sysinfo::{Components, Disks, Networks, System};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LiveTelemetry {
    pub cpu_usage_pct: f32,
    #[serde(rename = "ramUsageGiB")]
    pub ram_usage_gib: f32,
    pub ram_usage_pct: f32,
    #[serde(rename = "ramAvailableGiB")]
    pub ram_available_gib: f32,
    #[serde(rename = "ramCacheGiB")]
    pub ram_cache_gib: f32,
    #[serde(rename = "diskUsageGiB")]
    pub disk_usage_gib: f32,
    pub disk_usage_pct: f32,
    #[serde(rename = "swapUsageGiB")]
    pub swap_usage_gib: f32,
    pub swap_usage_pct: f32,
    pub cpu_temp_c: f32,
    pub gpu_temp_c: f32,
    pub fan_rpm: u32,
    pub core_clocks_mhz: Vec<u32>,
    pub network_down_kbps: u32,
    pub network_up_kbps: u32,
}

fn read_meminfo_bytes(field: &str) -> Option<u64> {
    let content = fs::read_to_string("/proc/meminfo").ok()?;
    let kib = content
        .lines()
        .find(|line| line.starts_with(field))?
        .split_whitespace()
        .nth(1)?
        .parse::<u64>()
        .ok()?;
    Some(kib * 1024)
}

fn bytes_to_gib(value: u64) -> f32 {
    ((value as f64 / 1024.0 / 1024.0 / 1024.0) * 10.0).round() as f32 / 10.0
}

fn read_cpu_temp() -> f32 {
    let components = Components::new_with_refreshed_list();
    for component in &components {
        let label = component.label().to_lowercase();
        if label.contains("cpu")
            || label.contains("core")
            || label.contains("package")
            || label.contains("k10temp")
        {
            if let Some(temp) = component.temperature() {
                if temp > 0.0 {
                    return (temp * 10.0).round() / 10.0;
                }
            }
        }
    }

    // Fallback: search /sys/class/thermal/thermal_zone*/temp
    if let Ok(entries) = fs::read_dir("/sys/class/thermal") {
        for entry in entries.flatten() {
            let path = entry.path().join("temp");
            if let Ok(content) = fs::read_to_string(path) {
                if let Ok(milli_c) = content.trim().parse::<f32>() {
                    let temp = milli_c / 1000.0;
                    if temp > 20.0 && temp < 115.0 {
                        return (temp * 10.0).round() / 10.0;
                    }
                }
            }
        }
    }

    0.0
}

fn read_per_core_clocks(cpu_count: usize, sys_cpus: &[sysinfo::Cpu]) -> Vec<u32> {
    let mut clocks = Vec::new();
    for i in 0..cpu_count {
        let path = format!("/sys/devices/system/cpu/cpu{}/cpufreq/scaling_cur_freq", i);
        if let Ok(content) = fs::read_to_string(path) {
            if let Ok(khz) = content.trim().parse::<u32>() {
                clocks.push(khz / 1000);
                continue;
            }
        }
        // Fallback: sysinfo cpu frequency
        if let Some(cpu) = sys_cpus.get(i) {
            clocks.push(cpu.frequency() as u32);
        } else {
            clocks.push(0);
        }
    }
    clocks
}

fn read_fan_rpm() -> u32 {
    if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
        for entry in entries.flatten() {
            let hwmon_path = entry.path();
            for fan_idx in 1..=5 {
                let fan_file = hwmon_path.join(format!("fan{}_input", fan_idx));
                if let Ok(content) = fs::read_to_string(fan_file) {
                    if let Ok(rpm) = content.trim().parse::<u32>() {
                        if rpm > 0 {
                            return rpm;
                        }
                    }
                }
            }
        }
    }
    0
}

pub fn get_live_telemetry_blocking() -> Result<LiveTelemetry, String> {
    static SYSTEM: std::sync::OnceLock<std::sync::Mutex<System>> = std::sync::OnceLock::new();
    let mut sys = SYSTEM
        .get_or_init(|| std::sync::Mutex::new(System::new()))
        .lock()
        .map_err(|e| e.to_string())?;
    sys.refresh_cpu_all();
    sys.refresh_memory();

    let cpu_usage_pct = (sys.global_cpu_usage() * 10.0).round() / 10.0;

    let total_ram = read_meminfo_bytes("MemTotal:").unwrap_or_else(|| sys.total_memory());
    let available_ram =
        read_meminfo_bytes("MemAvailable:").unwrap_or_else(|| sys.available_memory());
    let used_ram = total_ram.saturating_sub(available_ram);
    let cache_ram = read_meminfo_bytes("Cached:").unwrap_or(0)
        + read_meminfo_bytes("Buffers:").unwrap_or(0)
        + read_meminfo_bytes("SReclaimable:").unwrap_or(0);
    let ram_usage_gib = bytes_to_gib(used_ram);
    let ram_available_gib = bytes_to_gib(available_ram);
    let ram_cache_gib = bytes_to_gib(cache_ram);
    let ram_usage_pct = if total_ram > 0 {
        ((used_ram as f64 / total_ram as f64) * 100.0).round() as f32
    } else {
        0.0
    };

    let total_swap = sys.total_swap();
    let used_swap = sys.used_swap();
    let swap_usage_gib = bytes_to_gib(used_swap);
    let swap_usage_pct = if total_swap > 0 {
        ((used_swap as f64 / total_swap as f64) * 100.0).round() as f32
    } else {
        0.0
    };

    let disks = Disks::new_with_refreshed_list();
    let total_disk_bytes: u64 = disks
        .iter()
        .find(|d| d.mount_point() == std::path::Path::new("/"))
        .map(|d| d.total_space())
        .unwrap_or(0);
    let free_disk_bytes: u64 = disks
        .iter()
        .find(|d| d.mount_point() == std::path::Path::new("/"))
        .map(|d| d.available_space())
        .unwrap_or(0);
    let used_disk_bytes = total_disk_bytes.saturating_sub(free_disk_bytes);

    let disk_usage_gib =
        (used_disk_bytes as f32 / (1024.0 * 1024.0 * 1024.0) * 10.0).round() / 10.0;
    let disk_usage_pct = if total_disk_bytes > 0 {
        ((used_disk_bytes as f32 / total_disk_bytes as f32) * 100.0).round()
    } else {
        0.0
    };

    let cpu_temp_c = read_cpu_temp();
    let gpu_temp_c = 0.0;
    let fan_rpm = read_fan_rpm();
    let core_clocks_mhz = read_per_core_clocks(sys.cpus().len(), sys.cpus());

    let mut networks = Networks::new();
    networks.refresh(true);
    let mut total_received: u64 = 0;
    let mut total_transmitted: u64 = 0;
    for (_interface_name, network) in &networks {
        total_received += network.received();
        total_transmitted += network.transmitted();
    }
    let network_down_kbps = ((total_received as f32 / 1024.0) * 8.0) as u32;
    let network_up_kbps = ((total_transmitted as f32 / 1024.0) * 8.0) as u32;

    Ok(LiveTelemetry {
        cpu_usage_pct,
        ram_usage_gib,
        ram_usage_pct,
        ram_available_gib,
        ram_cache_gib,
        disk_usage_gib,
        disk_usage_pct,
        swap_usage_gib,
        swap_usage_pct,
        cpu_temp_c,
        gpu_temp_c,
        fan_rpm,
        core_clocks_mhz,
        network_down_kbps,
        network_up_kbps,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_real_memory_with_frontend_keys() {
        let telemetry = get_live_telemetry_blocking().expect("telemetry query must succeed");
        assert!(telemetry.ram_available_gib > 0.0);
        let json = serde_json::to_value(telemetry).expect("telemetry must serialize");
        assert!(json.get("ramUsageGiB").is_some());
        assert!(
            json.get("ramAvailableGiB")
                .and_then(|value| value.as_f64())
                .unwrap_or(0.0)
                > 0.0
        );
        assert!(json.get("ramUsageGib").is_none());
    }
}

#[tauri::command]
pub async fn get_live_telemetry() -> Result<LiveTelemetry, String> {
    tauri::async_runtime::spawn_blocking(move || get_live_telemetry_blocking())
        .await
        .map_err(|e| e.to_string())?
}
