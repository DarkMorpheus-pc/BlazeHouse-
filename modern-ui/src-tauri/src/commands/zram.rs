use serde::{Deserialize, Serialize};
use std::fs;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ZramConfig {
    pub enabled: bool,
    #[serde(rename = "allocatedSizeGiB")]
    pub allocated_size_gib: f32,
    pub algorithm: String,
    pub compression_ratio: f32,
    pub swappiness: u32,
    pub priority: u32,
    #[serde(rename = "actualUsedMiB")]
    pub actual_used_mib: u32,
    #[serde(rename = "compressedUsedMiB")]
    pub compressed_used_mib: u32,
}

fn parse_comp_algorithm() -> String {
    if let Ok(content) = fs::read_to_string("/sys/block/zram0/comp_algorithm") {
        for word in content.split_whitespace() {
            if word.starts_with('[') && word.ends_with(']') {
                return word.trim_matches('[').trim_matches(']').to_string();
            }
        }
    }
    "zstd".to_string()
}

fn parse_mm_stat() -> (u32, u32, f32) {
    if let Ok(content) = fs::read_to_string("/sys/block/zram0/mm_stat") {
        let parts: Vec<&str> = content.split_whitespace().collect();
        if parts.len() >= 3 {
            let orig: u64 = parts[0].parse().unwrap_or(0);
            let compr: u64 = parts[1].parse().unwrap_or(1);
            let actual_mib = (orig / (1024 * 1024)) as u32;
            let compressed_mib = (compr / (1024 * 1024)) as u32;
            let ratio = if compr > 0 {
                (orig as f32 / compr as f32 * 100.0).round() / 100.0
            } else {
                1.0
            };
            return (actual_mib, compressed_mib, ratio.max(1.0));
        }
    }
    (0, 0, 1.0)
}

pub fn get_zram_status_blocking() -> Result<ZramConfig, String> {
    let enabled = fs::read_to_string("/proc/swaps")
        .map_err(|e| e.to_string())?
        .lines()
        .skip(1)
        .any(|line| line.split_whitespace().next() == Some("/dev/zram0"));
    let mut allocated_size_gib = 0.0;

    if enabled {
        if let Ok(content) = fs::read_to_string("/sys/block/zram0/disksize") {
            if let Ok(bytes) = content.trim().parse::<f32>() {
                allocated_size_gib = (bytes / (1024.0 * 1024.0 * 1024.0) * 10.0).round() / 10.0;
            }
        }
    }

    let algorithm = parse_comp_algorithm();
    let (actual_used_mib, compressed_used_mib, compression_ratio) = parse_mm_stat();

    let swappiness = fs::read_to_string("/proc/sys/vm/swappiness")
        .ok()
        .and_then(|s| s.trim().parse::<u32>().ok())
        .unwrap_or(60);

    let priority = Command::new("swapon")
        .args(["--show=NAME,PRIO", "--noheadings"])
        .output()
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .and_then(|text| {
            text.lines()
                .find(|line| line.contains("zram0"))
                .and_then(|line| line.split_whitespace().last())
                .and_then(|v| v.parse().ok())
        })
        .unwrap_or(100);

    Ok(ZramConfig {
        enabled,
        allocated_size_gib,
        algorithm,
        compression_ratio,
        swappiness,
        priority,
        actual_used_mib,
        compressed_used_mib,
    })
}

pub fn apply_zram_config_blocking(
    enabled: bool,
    allocated_size_gib: f32,
    algorithm: String,
    swappiness: u32,
    priority: u32,
) -> Result<Vec<String>, String> {
    if enabled && !(0.5..=256.0).contains(&allocated_size_gib) {
        return Err("ZRAM boyutu 0.5–256 GiB aralığında olmalıdır".to_string());
    }
    if !matches!(algorithm.as_str(), "zstd" | "lz4" | "lzo" | "deflate") {
        return Err("Çekirdek tarafından desteklenmeyen ZRAM algoritması".to_string());
    }
    if !(0..=200).contains(&swappiness) || priority > 32767 {
        return Err("Geçersiz swappiness veya swap önceliği".to_string());
    }
    if enabled {
        if let Ok(available) = fs::read_to_string("/sys/block/zram0/comp_algorithm") {
            if !available
                .split_whitespace()
                .any(|item| item.trim_matches(['[', ']']) == algorithm)
            {
                return Err("Algoritma bu çekirdek tarafından desteklenmiyor".into());
            }
        }
    }
    let size_mb = (allocated_size_gib * 1024.0) as u32;
    let config_content = if enabled {
        format!(
            "[zram0]\nzram-size = {}\ncompression-algorithm = {}\nswap-priority = {}\n",
            size_mb, algorithm, priority
        )
    } else {
        "[zram0]\nzram-size = 0\n".into()
    };

    let script = format!(
        "set -eu; if grep -q '^/dev/zram0 ' /proc/swaps; then swapoff /dev/zram0; fi; if [ -e /sys/block/zram0 ]; then systemctl stop systemd-zram-setup@zram0.service; fi; \
         mkdir -p /etc/systemd/zram-generator.conf.d; \
         printf '%s' '{config_content}' > /etc/systemd/zram-generator.conf.d/99-blaze-house.conf; \
         systemctl daemon-reload; {}",
        if enabled {
            format!("systemctl start dev-zram0.swap; sysctl vm.swappiness={swappiness}; printf '%s\\n' 'vm.swappiness={swappiness}' > /etc/sysctl.d/99-blaze-house-zram.conf")
        } else {
            String::new()
        }
    );

    let output = Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(&script)
        .output()
        .map_err(|e| format!("Yetkilendirme veya çalıştırma hatası: {}", e))?;

    if !output.status.success() {
        return Err(format!(
            "İşlem tamamlanamadı; bazı adımlar uygulanmış olabilir. Durumu yenileyin: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    let actual = get_zram_status_blocking()?;
    if actual.enabled != enabled
        || (enabled
            && ((actual.allocated_size_gib - allocated_size_gib).abs() > 0.11
                || actual.algorithm != algorithm
                || actual.priority != priority
                || actual.swappiness != swappiness))
    {
        return Err(
            "Yapılandırma sonrası çekirdek değerleri istekle uyuşmuyor. Durumu yenileyin.".into(),
        );
    }
    if !enabled {
        return Ok(vec![
            "ZRAM devre dışı; /proc/swaps üzerinden doğrulandı.".into()
        ]);
    }

    Ok(vec![
        format!("ZRAM boyutu {} MB olarak ayarlandı", size_mb),
        format!("Algoritma '{}' olarak yapılandırıldı", algorithm),
        format!("vm.swappiness = {} uygulandı", swappiness),
        "systemd-zram-setup@zram0.service servisi güncellendi".to_string(),
        "BAŞARILI: ZRAM sıkıştırılmış takas alanı aktif!".to_string(),
    ])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serializes_gib_and_mib_keys_for_frontend() {
        let json =
            serde_json::to_value(get_zram_status_blocking().expect("zram query must succeed"))
                .expect("zram status must serialize");
        assert!(json.get("allocatedSizeGiB").is_some());
        assert!(json.get("actualUsedMiB").is_some());
        assert!(json.get("compressedUsedMiB").is_some());
    }
}

#[tauri::command]
pub async fn get_zram_status() -> Result<ZramConfig, String> {
    tauri::async_runtime::spawn_blocking(move || get_zram_status_blocking())
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn apply_zram_config(
    enabled: bool,
    allocated_size_gib: f32,
    algorithm: String,
    swappiness: u32,
    priority: u32,
) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        apply_zram_config_blocking(enabled, allocated_size_gib, algorithm, swappiness, priority)
    })
    .await
    .map_err(|e| e.to_string())?
}
