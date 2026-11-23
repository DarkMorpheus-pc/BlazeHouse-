use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;
use sysinfo::{Disks, System};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemHardware {
    pub model_name: String,
    pub vendor: String,
    pub cpu_name: String,
    pub cpu_cores: usize,
    pub cpu_threads: usize,
    pub base_clock_ghz: f32,
    pub max_clock_ghz: f32,
    pub gpu_name: String,
    #[serde(rename = "gpuVramGiB")]
    pub gpu_vram_gib: f32,
    #[serde(rename = "totalRamGiB")]
    pub total_ram_gib: f32,
    #[serde(rename = "totalSwapGiB")]
    pub total_swap_gib: f32,
    #[serde(rename = "diskTotalGiB")]
    pub disk_total_gib: f32,
    pub distro_name: String,
    pub distro_id: String,
    pub distro_version: String,
    pub distro_codename: String,
    pub kernel_version: String,
    pub desktop_environment: String,
    pub display_server: String,
    pub uptime: String,
    pub chassis_type: String,
    pub battery_pct: Option<u32>,
    pub battery_status: Option<String>,
}

fn read_file_string(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|s| s.trim().to_string())
}

fn read_meminfo_gib(field: &str) -> Option<f32> {
    let content = fs::read_to_string("/proc/meminfo").ok()?;
    let kib = content
        .lines()
        .find(|line| line.starts_with(field))?
        .split_whitespace()
        .nth(1)?
        .parse::<u64>()
        .ok()?;
    Some(((kib as f64 / 1024.0 / 1024.0) * 10.0).round() as f32 / 10.0)
}

fn detect_distro_info() -> (String, String, String, String) {
    let mut name = "Blaze SolarEvolution".to_string();
    let mut id = "fedora".to_string();
    let mut version = "5".to_string();
    let mut codename = "SolarEvolution".to_string();

    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if line.starts_with("PRETTY_NAME=") {
                name = line
                    .replace("PRETTY_NAME=", "")
                    .trim_matches('"')
                    .to_string();
            } else if line.starts_with("ID=") || line.starts_with("ID_LIKE=") {
                let raw = line
                    .split('=')
                    .nth(1)
                    .unwrap_or("")
                    .trim_matches('"')
                    .to_lowercase();
                if raw.contains("fedora") || raw.contains("rhel") || raw.contains("centos") || raw.contains("blazeos") {
                    id = "fedora".to_string();
                } else if raw.contains("arch")
                    || raw.contains("cachyos")
                    || raw.contains("manjaro")
                    || raw.contains("endeavouros")
                {
                    id = "arch".to_string();
                } else if raw.contains("debian")
                    || raw.contains("ubuntu")
                    || raw.contains("linuxmint")
                    || raw.contains("pop")
                {
                    id = "debian".to_string();
                }
            } else if line.starts_with("VERSION_ID=") {
                version = line
                    .replace("VERSION_ID=", "")
                    .trim_matches('"')
                    .to_string();
            } else if line.starts_with("VERSION_CODENAME=") {
                codename = line
                    .replace("VERSION_CODENAME=", "")
                    .trim_matches('"')
                    .to_string();
            }
        }
    }

    // Additional fallback checks based on package manager presence on disk
    if id == "arch" && !std::path::Path::new("/usr/bin/pacman").exists() {
        if std::path::Path::new("/usr/bin/dnf5").exists()
            || std::path::Path::new("/usr/bin/dnf").exists()
            || std::path::Path::new("/usr/bin/rpm").exists()
        {
            id = "fedora".to_string();
        } else if std::path::Path::new("/usr/bin/apt").exists() {
            id = "debian".to_string();
        }
    }

    (name, id, version, codename)
}

fn detect_chassis_type() -> String {
    if std::path::Path::new("/sys/class/power_supply/BAT0").exists()
        || std::path::Path::new("/sys/class/power_supply/BAT1").exists()
    {
        return "laptop".to_string();
    }
    if let Some(chassis_str) = read_file_string("/sys/class/dmi/id/chassis_type") {
        if let Ok(num) = chassis_str.parse::<u32>() {
            match num {
                8 | 9 | 10 | 11 | 14 | 31 | 32 => return "laptop".to_string(),
                _ => return "desktop".to_string(),
            }
        }
    }
    "desktop".to_string()
}

fn detect_battery_info() -> (Option<u32>, Option<String>) {
    let capacity = read_file_string("/sys/class/power_supply/BAT0/capacity")
        .or_else(|| read_file_string("/sys/class/power_supply/BAT1/capacity"))
        .and_then(|s| s.parse::<u32>().ok());

    let status = read_file_string("/sys/class/power_supply/BAT0/status")
        .or_else(|| read_file_string("/sys/class/power_supply/BAT1/status"));

    (capacity, status)
}

fn detect_gpu_name() -> String {
    let mut gpus = Vec::new();
    if let Ok(output) = Command::new("lspci").output() {
        let text = String::from_utf8_lossy(&output.stdout);
        for line in text.lines() {
            if line.contains("VGA compatible controller") || line.contains("3D controller") {
                if let Some(pos) = line.find(": ") {
                    let mut gpu = line[pos + 2..].to_string();
                    if let Some(rev_pos) = gpu.find(" (rev ") {
                        gpu = gpu[..rev_pos].to_string();
                    }
                    gpus.push(gpu);
                }
            }
        }
    }

    if !gpus.is_empty() {
        // If there's a dedicated GPU (NVIDIA / AMD), prioritize it
        if let Some(dedicated) = gpus
            .iter()
            .find(|g| g.contains("NVIDIA") || g.contains("Radeon") || g.contains("AMD"))
        {
            return dedicated.clone();
        }
        return gpus.join(" + ");
    }

    "GPU algılanamadı".to_string()
}

fn detect_cpu_frequencies() -> (f32, f32) {
    let mut base_ghz = 0.0;
    let mut max_ghz = 0.0;

    if let Some(max_khz_str) =
        read_file_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_max_freq")
    {
        if let Ok(khz) = max_khz_str.parse::<f32>() {
            max_ghz = (khz / 1_000_000.0 * 100.0).round() / 100.0;
        }
    }

    if let Some(min_khz_str) =
        read_file_string("/sys/devices/system/cpu/cpu0/cpufreq/cpuinfo_min_freq")
    {
        if let Ok(khz) = min_khz_str.parse::<f32>() {
            base_ghz = (khz / 1_000_000.0 * 100.0).round() / 100.0;
        }
    }

    (base_ghz, max_ghz)
}

fn format_uptime(secs: u64) -> String {
    let days = secs / 86400;
    let hours = (secs % 86400) / 3600;
    let minutes = (secs % 3600) / 60;
    if days > 0 {
        format!("{} gün, {} saat, {} dk", days, hours, minutes)
    } else if hours > 0 {
        format!("{} saat, {} dk", hours, minutes)
    } else {
        format!("{} dk", minutes)
    }
}

pub fn get_system_hardware_blocking() -> Result<SystemHardware, String> {
    let mut sys = System::new();
    sys.refresh_cpu_all();
    sys.refresh_memory();

    let (distro_name, distro_id, distro_version, distro_codename) = detect_distro_info();
    let (base_clock_ghz, max_clock_ghz) = detect_cpu_frequencies();
    let chassis_type = detect_chassis_type();
    let (battery_pct, battery_status) = detect_battery_info();

    let model_name = read_file_string("/sys/class/dmi/id/product_name")
        .or_else(|| read_file_string("/sys/class/dmi/id/board_name"))
        .unwrap_or_else(|| "Standard PC / Workstation".to_string());

    let vendor = read_file_string("/sys/class/dmi/id/sys_vendor")
        .unwrap_or_else(|| "Linux System".to_string());

    let cpus = sys.cpus();
    let cpu_name = cpus
        .first()
        .map(|c| c.brand().trim().to_string())
        .unwrap_or_else(|| "x86_64 Processor".to_string());

    let cpu_threads = cpus.len();
    let cpu_cores = sys.physical_core_count().unwrap_or(cpu_threads);

    let total_ram_gib = read_meminfo_gib("MemTotal:").unwrap_or_else(|| {
        ((sys.total_memory() as f64 / 1024.0 / 1024.0 / 1024.0) * 10.0).round() as f32 / 10.0
    });
    let total_swap_gib = read_meminfo_gib("SwapTotal:").unwrap_or_else(|| {
        ((sys.total_swap() as f64 / 1024.0 / 1024.0 / 1024.0) * 10.0).round() as f32 / 10.0
    });

    let disks = Disks::new_with_refreshed_list();
    let total_disk_bytes: u64 = disks
        .iter()
        .find(|d| d.mount_point() == std::path::Path::new("/"))
        .map(|d| d.total_space())
        .unwrap_or(0);
    let disk_total_gib =
        (total_disk_bytes as f32 / (1024.0 * 1024.0 * 1024.0) * 10.0).round() / 10.0;

    let kernel_version = System::kernel_version().unwrap_or_else(|| "Linux Kernel".to_string());

    let desktop_environment = {
        let raw = std::env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| std::env::var("DESKTOP_SESSION"))
            .unwrap_or_else(|_| "SolarUI".to_string());
        let lower = raw.to_lowercase();
        if lower.contains("niri") || lower.contains("solar") || lower.contains("noctalia") || lower.contains("caelestia") {
            "SolarUI".to_string()
        } else {
            raw
        }
    };

    let display_server = std::env::var("XDG_SESSION_TYPE")
        .map(|s| {
            if s.eq_ignore_ascii_case("wayland") {
                "Wayland (Native Compositor)".to_string()
            } else {
                "X11 Window System".to_string()
            }
        })
        .unwrap_or_else(|_| "Wayland".to_string());

    let uptime = format_uptime(System::uptime());
    let gpu_name = detect_gpu_name();

    Ok(SystemHardware {
        model_name,
        vendor,
        cpu_name,
        cpu_cores,
        cpu_threads,
        base_clock_ghz,
        max_clock_ghz,
        gpu_name,
        gpu_vram_gib: 0.0,
        total_ram_gib,
        total_swap_gib,
        disk_total_gib,
        distro_name,
        distro_id,
        distro_version,
        distro_codename,
        kernel_version,
        desktop_environment,
        display_server,
        uptime,
        chassis_type,
        battery_pct,
        battery_status,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reports_physical_memory_with_frontend_key() {
        let hardware = get_system_hardware_blocking().expect("hardware query must succeed");
        assert!(hardware.total_ram_gib > 0.0);
        let json = serde_json::to_value(hardware).expect("hardware must serialize");
        assert!(
            json.get("totalRamGiB")
                .and_then(|value| value.as_f64())
                .unwrap_or(0.0)
                > 0.0
        );
        assert!(json.get("totalRamGib").is_none());
    }
}

static HARDWARE_CACHE: std::sync::RwLock<Option<SystemHardware>> = std::sync::RwLock::new(None);

#[tauri::command]
pub async fn get_system_hardware() -> Result<SystemHardware, String> {
    if let Ok(guard) = HARDWARE_CACHE.read() {
        if let Some(ref hw) = *guard {
            return Ok(hw.clone());
        }
    }

    let hw = tauri::async_runtime::spawn_blocking(move || get_system_hardware_blocking())
        .await
        .map_err(|e| e.to_string())??;

    if let Ok(mut guard) = HARDWARE_CACHE.write() {
        *guard = Some(hw.clone());
    }

    Ok(hw)
}

#[tauri::command]
pub async fn refresh_system_hardware() -> Result<SystemHardware, String> {
    let hw = tauri::async_runtime::spawn_blocking(move || get_system_hardware_blocking())
        .await
        .map_err(|e| e.to_string())??;

    if let Ok(mut guard) = HARDWARE_CACHE.write() {
        *guard = Some(hw.clone());
    }

    Ok(hw)
}
