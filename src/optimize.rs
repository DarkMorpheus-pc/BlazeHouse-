use crate::capabilities::{has_cmd, run_cmd};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TweakKind {
    Gsettings,
    Sysctl,
    Service,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tweak {
    pub key: String,
    pub title: String,
    pub desc: String,
    pub category: String,
    pub kind: TweakKind,
    pub schema: String,
    pub gkey: String,
    pub on_value: String,
    pub off_value: String,
    pub needs_root: bool,
}

pub fn build_catalog() -> Vec<Tweak> {
    vec![
        // Privacy
        Tweak {
            key: "recent-files".to_string(),
            title: "opt.tw.recent_files.title".to_string(),
            desc: "opt.tw.recent_files.desc".to_string(),
            category: "opt.cat.privacy".to_string(),
            kind: TweakKind::Gsettings,
            schema: "org.gnome.desktop.privacy".to_string(),
            gkey: "remember-recent-files".to_string(),
            on_value: "false".to_string(),
            off_value: "true".to_string(),
            needs_root: false,
        },
        Tweak {
            key: "app-usage".to_string(),
            title: "opt.tw.app_usage.title".to_string(),
            desc: "opt.tw.app_usage.desc".to_string(),
            category: "opt.cat.privacy".to_string(),
            kind: TweakKind::Gsettings,
            schema: "org.gnome.desktop.privacy".to_string(),
            gkey: "send-software-usage-stats".to_string(),
            on_value: "false".to_string(),
            off_value: "true".to_string(),
            needs_root: false,
        },
        Tweak {
            key: "location".to_string(),
            title: "opt.tw.location.title".to_string(),
            desc: "opt.tw.location.desc".to_string(),
            category: "opt.cat.privacy".to_string(),
            kind: TweakKind::Gsettings,
            schema: "org.gnome.system.location".to_string(),
            gkey: "enabled".to_string(),
            on_value: "false".to_string(),
            off_value: "true".to_string(),
            needs_root: false,
        },
        // Performance
        Tweak {
            key: "animations".to_string(),
            title: "opt.tw.animations.title".to_string(),
            desc: "opt.tw.animations.desc".to_string(),
            category: "opt.cat.performance".to_string(),
            kind: TweakKind::Gsettings,
            schema: "org.gnome.desktop.interface".to_string(),
            gkey: "enable-animations".to_string(),
            on_value: "false".to_string(),
            off_value: "true".to_string(),
            needs_root: false,
        },
        Tweak {
            key: "swappiness".to_string(),
            title: "opt.tw.swappiness.title".to_string(),
            desc: "opt.tw.swappiness.desc".to_string(),
            category: "opt.cat.performance".to_string(),
            kind: TweakKind::Sysctl,
            schema: String::new(),
            gkey: String::new(),
            on_value: "10".to_string(),
            off_value: "60".to_string(),
            needs_root: true,
        },
        Tweak {
            key: "vfs-cache".to_string(),
            title: "opt.tw.vfs_cache.title".to_string(),
            desc: "opt.tw.vfs_cache.desc".to_string(),
            category: "opt.cat.performance".to_string(),
            kind: TweakKind::Sysctl,
            schema: String::new(),
            gkey: String::new(),
            on_value: "50".to_string(),
            off_value: "100".to_string(),
            needs_root: true,
        },
        // Services
        Tweak {
            key: "svc-bluetooth".to_string(),
            title: "opt.tw.svc_bluetooth.title".to_string(),
            desc: "opt.tw.svc_bluetooth.desc".to_string(),
            category: "opt.cat.services".to_string(),
            kind: TweakKind::Service,
            schema: String::new(),
            gkey: String::new(),
            on_value: "disabled".to_string(),
            off_value: "enabled".to_string(),
            needs_root: true,
        },
        Tweak {
            key: "svc-cups".to_string(),
            title: "opt.tw.svc_cups.title".to_string(),
            desc: "opt.tw.svc_cups.desc".to_string(),
            category: "opt.cat.services".to_string(),
            kind: TweakKind::Service,
            schema: String::new(),
            gkey: String::new(),
            on_value: "disabled".to_string(),
            off_value: "enabled".to_string(),
            needs_root: true,
        },
        Tweak {
            key: "svc-avahi".to_string(),
            title: "opt.tw.svc_avahi.title".to_string(),
            desc: "opt.tw.svc_avahi.desc".to_string(),
            category: "opt.cat.services".to_string(),
            kind: TweakKind::Service,
            schema: String::new(),
            gkey: String::new(),
            on_value: "disabled".to_string(),
            off_value: "enabled".to_string(),
            needs_root: true,
        },
    ]
}

pub fn query_tweak_state(tw: &Tweak) -> Option<bool> {
    match tw.kind {
        TweakKind::Gsettings => {
            if has_cmd("gsettings") {
                let (ok, out) = run_cmd("gsettings", &["get", &tw.schema, &tw.gkey]);
                if ok {
                    return Some(out.trim() == tw.on_value);
                }
            }
            None
        }
        TweakKind::Sysctl => {
            let sysctl_name = match tw.key.as_str() {
                "swappiness" => "vm/swappiness",
                "vfs-cache" => "vm/vfs_cache_pressure",
                _ => return None,
            };
            if let Ok(content) = fs::read_to_string(format!("/proc/sys/{}", sysctl_name)) {
                return Some(content.trim() == tw.on_value);
            }
            None
        }
        TweakKind::Service => {
            let svc_name = match tw.key.as_str() {
                "svc-bluetooth" => "bluetooth.service",
                "svc-cups" => "cups.service",
                "svc-avahi" => "avahi-daemon.service",
                _ => return None,
            };
            if has_cmd("systemctl") {
                let (_ok, out) = run_cmd("systemctl", &["is-enabled", svc_name]);
                let state = out.trim();
                if state == "disabled" || state == "masked" {
                    return Some(true);
                } else if state == "enabled" || state == "static" || state == "indirect" {
                    return Some(false);
                }
            }
            None
        }
    }
}

pub fn update_sysctl_conf_content(existing: &str, key: &str, val: &str) -> String {
    let mut lines: Vec<String> = Vec::new();
    let mut found = false;
    let target_line = format!("{} = {}", key, val);

    for line in existing.lines() {
        let trimmed = line.trim();
        if !trimmed.starts_with('#') {
            if let Some((k, _)) = trimmed.split_once('=') {
                if k.trim() == key {
                    lines.push(target_line.clone());
                    found = true;
                    continue;
                }
            }
        }
        lines.push(line.to_string());
    }

    if !found {
        lines.push(target_line);
    }

    let mut out = lines.join("\n");
    if !out.ends_with('\n') {
        out.push('\n');
    }
    out
}

pub fn apply_tweak(tw: &Tweak, enable: bool) -> (bool, String) {
    let target_val = if enable { &tw.on_value } else { &tw.off_value };

    match tw.kind {
        TweakKind::Gsettings => {
            if has_cmd("gsettings") {
                let (ok, out) = run_cmd("gsettings", &["set", &tw.schema, &tw.gkey, target_val]);
                return (ok, out);
            }
            (false, "gsettings bulunamadı".to_string())
        }
        TweakKind::Sysctl => {
            let sysctl_key = match tw.key.as_str() {
                "swappiness" => "vm.swappiness",
                "vfs-cache" => "vm.vfs_cache_pressure",
                _ => return (false, "Bilinmeyen sysctl anahtarı".to_string()),
            };
            let conf_path = "/etc/sysctl.d/99-blaze-house.conf";
            let existing = fs::read_to_string(conf_path).unwrap_or_default();
            let updated_conf = update_sysctl_conf_content(&existing, sysctl_key, target_val);
            let script = format!(
                "mkdir -p /etc/sysctl.d && cat > /etc/sysctl.d/99-blaze-house.conf << 'EOF'\n{}EOF\nsysctl -w {}={}",
                updated_conf, sysctl_key, target_val
            );
            run_cmd("pkexec", &["sh", "-c", &script])
        }
        TweakKind::Service => {
            let svc_name = match tw.key.as_str() {
                "svc-bluetooth" => "bluetooth.service",
                "svc-cups" => "cups.service",
                "svc-avahi" => "avahi-daemon.service",
                _ => return (false, "Bilinmeyen servis".to_string()),
            };
            let action = if enable { "disable" } else { "enable" };
            run_cmd("pkexec", &["systemctl", action, "--now", svc_name])
        }
    }
}

// ---------- Cleanup Actions ----------

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CleanupAction {
    pub key: String,
    pub title: String,
    pub desc: String,
    pub needs_root: bool,
}

pub fn cleanup_actions() -> Vec<CleanupAction> {
    vec![
        CleanupAction {
            key: "pkg-cache".to_string(),
            title: "opt.clean.pkg_cache.title".to_string(),
            desc: "opt.clean.pkg_cache.desc".to_string(),
            needs_root: true,
        },
        CleanupAction {
            key: "orphans".to_string(),
            title: "opt.clean.orphans.title".to_string(),
            desc: "opt.clean.orphans.desc".to_string(),
            needs_root: true,
        },
        CleanupAction {
            key: "journal".to_string(),
            title: "opt.clean.journal.title".to_string(),
            desc: "opt.clean.journal.desc".to_string(),
            needs_root: true,
        },
        CleanupAction {
            key: "thumbnails".to_string(),
            title: "opt.clean.thumbnails.title".to_string(),
            desc: "opt.clean.thumbnails.desc".to_string(),
            needs_root: false,
        },
        CleanupAction {
            key: "user-cache".to_string(),
            title: "opt.clean.user_cache.title".to_string(),
            desc: "opt.clean.user_cache.desc".to_string(),
            needs_root: false,
        },
        CleanupAction {
            key: "trash".to_string(),
            title: "opt.clean.trash.title".to_string(),
            desc: "opt.clean.trash.desc".to_string(),
            needs_root: false,
        },
    ]
}

pub fn run_cleanup(key: &str) -> (bool, String) {
    match key {
        "pkg-cache" => {
            if has_cmd("dnf5") {
                run_cmd("pkexec", &["dnf5", "clean", "all"])
            } else if has_cmd("dnf") {
                run_cmd("pkexec", &["dnf", "clean", "all"])
            } else if has_cmd("apt-get") {
                run_cmd("pkexec", &["apt-get", "clean"])
            } else {
                (false, "Desteklenen paket yöneticisi bulunamadı".to_string())
            }
        }
        "orphans" => {
            if has_cmd("dnf5") {
                run_cmd("pkexec", &["dnf5", "autoremove", "-y"])
            } else if has_cmd("dnf") {
                run_cmd("pkexec", &["dnf", "autoremove", "-y"])
            } else if has_cmd("apt-get") {
                run_cmd("pkexec", &["apt-get", "autoremove", "-y"])
            } else {
                (false, "Desteklenen paket yöneticisi bulunamadı".to_string())
            }
        }
        "journal" => run_cmd("pkexec", &["journalctl", "--vacuum-time=3d"]),
        "thumbnails" => {
            let home = env::var("HOME").unwrap_or_default();
            let thumb_path = Path::new(&home).join(".cache/thumbnails");
            if thumb_path.exists() {
                if let Err(e) = fs::remove_dir_all(&thumb_path) {
                    return (false, e.to_string());
                }
                let _ = fs::create_dir_all(&thumb_path);
            }
            (true, "Küçük resim önbelleği temizlendi".to_string())
        }
        "user-cache" => {
            let home = env::var("HOME").unwrap_or_default();
            let cache_root = Path::new(&home).join(".cache");
            if cache_root.exists() {
                // Protected directories that may hold active compositor/session/shader/socket state
                let protected = [
                    "wayland",
                    "niri",
                    "solarui",
                    "quickshell",
                    "noctalia",
                    "mesa_shader_cache",
                    "mesa_shader_cache_db",
                    "fontconfig",
                    "ibus",
                    "gstreamer-1.0",
                    "pipewire",
                    "wireplumber",
                ];
                if let Ok(entries) = fs::read_dir(&cache_root) {
                    for entry in entries.flatten() {
                        let p = entry.path();
                        let fname = entry.file_name().to_string_lossy().to_string();
                        if protected.iter().any(|&prot| fname.starts_with(prot)) {
                            continue;
                        }
                        if p.is_dir() && !p.is_symlink() {
                            let _ = fs::remove_dir_all(&p);
                        } else if p.is_file() {
                            let _ = fs::remove_file(&p);
                        }
                    }
                }
            }
            (true, "Uygulama önbelleği (~/.cache) temizlendi".to_string())
        }
        "trash" => {
            let home = env::var("HOME").unwrap_or_default();
            let trash_root = Path::new(&home).join(".local/share/Trash");
            for sub in ["files", "info"] {
                let p = trash_root.join(sub);
                if p.exists() {
                    let _ = fs::remove_dir_all(&p);
                    let _ = fs::create_dir_all(&p);
                }
            }
            (true, "Çöp kutusu boşaltıldı".to_string())
        }
        _ => (false, "Bilinmeyen temizlik işlemi".to_string()),
    }
}

// ---------- ZRAM Management ----------

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ZramStatus {
    pub available: bool,
    pub active: bool,
    pub device: String,
    pub disksize_bytes: u64,
    pub used_bytes: u64,
    pub compressed_bytes: u64,
    pub algorithm: String,
    pub total_ram_bytes: u64,
}

pub fn zram_status() -> ZramStatus {
    let mut st = ZramStatus::default();

    // Total RAM
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                if let Some(num_str) = line.split_whitespace().nth(1) {
                    if let Ok(kb) = num_str.parse::<u64>() {
                        st.total_ram_bytes = kb * 1024;
                    }
                }
                break;
            }
        }
    }

    let zdev = Path::new("/sys/block/zram0");
    if !zdev.exists() {
        st.available = has_cmd("zramctl") || Path::new("/sys/class/zram-control").exists();
        return st;
    }

    st.available = true;
    st.device = String::from("zram0");

    if let Ok(size_str) = fs::read_to_string(zdev.join("disksize")) {
        let disksize = size_str.trim().parse::<u64>().unwrap_or(0);
        st.disksize_bytes = disksize;
        st.active = disksize > 0;
    }

    if let Ok(mm_stat) = fs::read_to_string(zdev.join("mm_stat")) {
        let parts: Vec<&str> = mm_stat.split_whitespace().collect();
        if parts.len() >= 2 {
            st.used_bytes = parts[0].parse().unwrap_or(0);
            st.compressed_bytes = parts[1].parse().unwrap_or(0);
        }
    }

    if let Ok(algo) = fs::read_to_string(zdev.join("comp_algorithm")) {
        if let (Some(start), Some(end)) = (algo.find('['), algo.find(']')) {
            st.algorithm = algo[start + 1..end].to_string();
        } else {
            st.algorithm = algo.split_whitespace().next().unwrap_or("").to_string();
        }
    }

    st
}

pub fn zram_available_algorithms() -> Vec<String> {
    if let Ok(algo) = fs::read_to_string("/sys/block/zram0/comp_algorithm") {
        return algo
            .split_whitespace()
            .map(|s| s.trim_matches(|c| c == '[' || c == ']').to_string())
            .collect();
    }
    vec!["zstd".to_string(), "lz4".to_string(), "lzo".to_string()]
}

pub fn zram_configure(size_mb: u64, algorithm: &str) -> (bool, String) {
    let conf = format!(
        "[zram0]\nzram-size = {}\ncompression-algorithm = {}\n",
        size_mb, algorithm
    );
    let script = format!(
        "cat > /etc/systemd/zram-generator.conf << 'EOF'\n{}EOF\nsystemctl daemon-reload 2>/dev/null || true; systemctl restart systemd-zram-setup@zram0.service 2>/dev/null || true",
        conf
    );
    run_cmd("pkexec", &["sh", "-c", &script])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_optimize_catalog() {
        let cat = build_catalog();
        assert!(!cat.is_empty());
        for tw in &cat {
            let state = query_tweak_state(tw);
            println!("Tweak {}: state {:?}", tw.key, state);
        }
    }

    #[test]
    fn test_zram_status() {
        let st = zram_status();
        println!("ZRAM Status: {:?}", st);
    }

    #[test]
    fn test_update_sysctl_conf_content() {
        let initial = "# BlazeOS sysctl\nvm.swappiness = 60\n";
        let after_vfs = update_sysctl_conf_content(initial, "vm.vfs_cache_pressure", "50");
        assert!(after_vfs.contains("vm.swappiness = 60"));
        assert!(after_vfs.contains("vm.vfs_cache_pressure = 50"));

        let after_swap = update_sysctl_conf_content(&after_vfs, "vm.swappiness", "10");
        assert!(after_swap.contains("vm.swappiness = 10"));
        assert!(!after_swap.contains("vm.swappiness = 60"));
        assert!(after_swap.contains("vm.vfs_cache_pressure = 50"));
    }
}
