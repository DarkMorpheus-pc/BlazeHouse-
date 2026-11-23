use serde::{Deserialize, Serialize};
use std::fs;
use sysinfo::System;

use crate::capabilities::run_cmd;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentInfo {
    pub label: String,
    pub value: String,
}

impl ComponentInfo {
    pub fn new(label: &str, value: &str) -> Self {
        Self {
            label: label.to_string(),
            value: value.to_string(),
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct HardwareInventory {
    pub cpu: Vec<ComponentInfo>,
    pub memory: Vec<ComponentInfo>,
    pub storage: Vec<ComponentInfo>,
    pub graphics: Vec<ComponentInfo>,
    pub network: Vec<ComponentInfo>,
    pub board: Vec<ComponentInfo>,
}

pub fn cpu_summary() -> String {
    if let Ok(cpuinfo) = fs::read_to_string("/proc/cpuinfo") {
        for line in cpuinfo.lines() {
            if line.starts_with("model name") {
                if let Some((_, val)) = line.split_once(':') {
                    return val.trim().to_string();
                }
            }
        }
    }
    String::from("Bilinmeyen İşlemci")
}

pub fn dmi_product() -> (String, String) {
    let vendor_path = "/sys/class/dmi/id/sys_vendor";
    let product_path = "/sys/class/dmi/id/product_name";

    let vendor = fs::read_to_string(vendor_path)
        .unwrap_or_default()
        .trim()
        .to_string();
    let product = fs::read_to_string(product_path)
        .unwrap_or_default()
        .trim()
        .to_string();

    (vendor, product)
}

pub fn gather_hardware() -> HardwareInventory {
    let mut inv = HardwareInventory::default();
    let mut sys = System::new_all();
    sys.refresh_all();

    // 1. CPU
    let cpu_model = cpu_summary();
    let cpu_count = sys.cpus().len();
    let physical_core_count = sys.physical_core_count().unwrap_or(cpu_count);
    inv.cpu.push(ComponentInfo::new("Model", &cpu_model));
    inv.cpu.push(ComponentInfo::new(
        "Çekirdek Sayısı",
        &format!("{} Fiziksel / {} Mantıksal", physical_core_count, cpu_count),
    ));
    if let Some(first_cpu) = sys.cpus().first() {
        inv.cpu.push(ComponentInfo::new(
            "Frekans",
            &format!("{:.2} GHz", first_cpu.frequency() as f64 / 1000.0),
        ));
    }

    // 2. Memory
    let total_mem_gib = sys.total_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    let avail_mem_gib = sys.available_memory() as f64 / (1024.0 * 1024.0 * 1024.0);
    inv.memory.push(ComponentInfo::new(
        "Toplam RAM",
        &format!("{:.2} GiB", total_mem_gib),
    ));
    inv.memory.push(ComponentInfo::new(
        "Kullanılabilir RAM",
        &format!("{:.2} GiB", avail_mem_gib),
    ));

    // 3. Storage
    let (ok_lsblk, out_lsblk) = run_cmd("lsblk", &["-d", "-n", "-o", "NAME,SIZE,TYPE,MODEL"]);
    if ok_lsblk && !out_lsblk.is_empty() {
        for line in out_lsblk.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 2 {
                let name = parts[0];
                let size = parts[1];
                let model = if parts.len() >= 4 {
                    parts[3..].join(" ")
                } else {
                    String::from("Disk")
                };
                inv.storage.push(ComponentInfo::new(
                    &format!("/dev/{}", name),
                    &format!("{} ({})", size, model),
                ));
            }
        }
    } else {
        inv.storage
            .push(ComponentInfo::new("Disk", "lsblk çıktısı alınamadı"));
    }

    // 4. Graphics
    let (ok_lspci, out_lspci) = run_cmd("lspci", &["-nn"]);
    if ok_lspci && !out_lspci.is_empty() {
        for line in out_lspci.lines() {
            if line.contains("VGA compatible controller")
                || line.contains("3D controller")
                || line.contains("Display controller")
            {
                if let Some((_, dev)) = line.split_once(": ") {
                    inv.graphics
                        .push(ComponentInfo::new("Ekran Kartı", dev.trim()));
                }
            }
        }
    }
    if inv.graphics.is_empty() {
        inv.graphics.push(ComponentInfo::new(
            "Ekran Kartı",
            "Standart Grafik Bağdaştırıcısı",
        ));
    }

    // 5. Network
    if let Ok(entries) = fs::read_dir("/sys/class/net") {
        for entry in entries.flatten() {
            let iface = entry.file_name().to_string_lossy().to_string();
            if iface != "lo" {
                let operstate = fs::read_to_string(entry.path().join("operstate"))
                    .unwrap_or_else(|_| "unknown".to_string())
                    .trim()
                    .to_string();
                let address = fs::read_to_string(entry.path().join("address"))
                    .unwrap_or_default()
                    .trim()
                    .to_string();
                inv.network.push(ComponentInfo::new(
                    &iface,
                    &format!("Durum: {}, MAC: {}", operstate, address),
                ));
            }
        }
    }

    // 6. Board / DMI
    let (vendor, product) = dmi_product();
    let bios_ver = fs::read_to_string("/sys/class/dmi/id/bios_version")
        .unwrap_or_default()
        .trim()
        .to_string();
    inv.board.push(ComponentInfo::new(
        "Üretici",
        if vendor.is_empty() {
            "Bilinmiyor"
        } else {
            &vendor
        },
    ));
    inv.board.push(ComponentInfo::new(
        "Model",
        if product.is_empty() {
            "Bilinmiyor"
        } else {
            &product
        },
    ));
    if !bios_ver.is_empty() {
        inv.board.push(ComponentInfo::new("BIOS Sürümü", &bios_ver));
    }

    inv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gather_hardware() {
        let inv = gather_hardware();
        assert!(!inv.cpu.is_empty());
        assert!(!inv.memory.is_empty());
        println!("Hardware Inventory: {:?}", inv);
    }
}
