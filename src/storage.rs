use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::Path;
use sysinfo::Disks;

use crate::capabilities::has_cmd;
use crate::optimize::run_cleanup;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskUsageInfo {
    pub mount_point: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub percent: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskDetailsInfo {
    pub name: String,
    pub mount_point: String,
    pub file_system: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub free_bytes: u64,
    pub percent: f64,
    pub is_removable: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheItem {
    pub key: String,
    pub label: String,
    pub path: String,
    pub size_bytes: u64,
    pub removable: bool,
}

pub fn disk_usage(path_str: &str) -> DiskUsageInfo {
    let disks = Disks::new_with_refreshed_list();
    for disk in &disks {
        if disk.mount_point().to_string_lossy() == path_str {
            let total = disk.total_space();
            let free = disk.available_space();
            let used = total.saturating_sub(free);
            let percent = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            return DiskUsageInfo {
                mount_point: path_str.to_string(),
                total_bytes: total,
                used_bytes: used,
                free_bytes: free,
                percent,
            };
        }
    }

    if let Some(disk) = disks.first() {
        let total = disk.total_space();
        let free = disk.available_space();
        let used = total.saturating_sub(free);
        let percent = if total > 0 {
            (used as f64 / total as f64) * 100.0
        } else {
            0.0
        };
        return DiskUsageInfo {
            mount_point: path_str.to_string(),
            total_bytes: total,
            used_bytes: used,
            free_bytes: free,
            percent,
        };
    }

    DiskUsageInfo {
        mount_point: path_str.to_string(),
        total_bytes: 0,
        used_bytes: 0,
        free_bytes: 0,
        percent: 0.0,
    }
}

pub fn list_all_disks() -> Vec<DiskDetailsInfo> {
    let disks = Disks::new_with_refreshed_list();
    disks
        .iter()
        .map(|disk| {
            let total = disk.total_space();
            let free = disk.available_space();
            let used = total.saturating_sub(free);
            let percent = if total > 0 {
                (used as f64 / total as f64) * 100.0
            } else {
                0.0
            };
            DiskDetailsInfo {
                name: disk.name().to_string_lossy().to_string(),
                mount_point: disk.mount_point().to_string_lossy().to_string(),
                file_system: disk.file_system().to_string_lossy().to_string(),
                total_bytes: total,
                used_bytes: used,
                free_bytes: free,
                percent,
                is_removable: disk.is_removable(),
            }
        })
        .collect()
}

pub fn analyze_caches() -> Vec<CacheItem> {
    let mut items = Vec::new();
    let home = env::var("HOME").unwrap_or_default();

    // 1. Package cache
    if has_cmd("dnf5") && Path::new("/var/cache/libdnf5").exists() {
        let mut size = dir_size(Path::new("/var/cache/libdnf5"), 6);
        if Path::new("/var/cache/dnf").exists() {
            size += dir_size(Path::new("/var/cache/dnf"), 6);
        }
        items.push(CacheItem {
            key: "pkg-cache".to_string(),
            label: "Paket önbelleği (dnf5)".to_string(),
            path: "/var/cache/libdnf5".to_string(),
            size_bytes: size,
            removable: true,
        });
    } else if (has_cmd("dnf") || has_cmd("dnf5"))
        && (Path::new("/var/cache/dnf").exists() || Path::new("/var/cache/libdnf5").exists())
    {
        let p = if Path::new("/var/cache/libdnf5").exists() {
            "/var/cache/libdnf5"
        } else {
            "/var/cache/dnf"
        };
        let mut size = 0;
        if Path::new("/var/cache/libdnf5").exists() {
            size += dir_size(Path::new("/var/cache/libdnf5"), 6);
        }
        if Path::new("/var/cache/dnf").exists() {
            size += dir_size(Path::new("/var/cache/dnf"), 6);
        }
        items.push(CacheItem {
            key: "pkg-cache".to_string(),
            label: "Paket önbelleği (dnf)".to_string(),
            path: p.to_string(),
            size_bytes: size,
            removable: true,
        });
    } else if has_cmd("apt-get") && Path::new("/var/cache/apt/archives").exists() {
        items.push(CacheItem {
            key: "pkg-cache".to_string(),
            label: "Paket önbelleği (apt)".to_string(),
            path: "/var/cache/apt/archives".to_string(),
            size_bytes: dir_size(Path::new("/var/cache/apt/archives"), 6),
            removable: true,
        });
    }

    // 2. Thumbnails
    let thumb_path = Path::new(&home).join(".cache/thumbnails");
    if thumb_path.exists() {
        items.push(CacheItem {
            key: "thumbnails".to_string(),
            label: "Küçük resim önbelleği".to_string(),
            path: thumb_path.to_string_lossy().to_string(),
            size_bytes: dir_size(&thumb_path, 6),
            removable: true,
        });
    }

    // 3. User Cache
    let user_cache = Path::new(&home).join(".cache");
    if user_cache.exists() {
        items.push(CacheItem {
            key: "user-cache".to_string(),
            label: "Uygulama önbelleği (~/.cache)".to_string(),
            path: user_cache.to_string_lossy().to_string(),
            size_bytes: dir_size(&user_cache, 6),
            removable: true,
        });
    }

    // 4. Trash
    let trash_path = Path::new(&home).join(".local/share/Trash");
    if trash_path.exists() {
        items.push(CacheItem {
            key: "trash".to_string(),
            label: "Çöp kutusu".to_string(),
            path: trash_path.to_string_lossy().to_string(),
            size_bytes: dir_size(&trash_path, 6),
            removable: true,
        });
    }

    // 5. Journal logs
    let journal_path = Path::new("/var/log/journal");
    if journal_path.exists() {
        items.push(CacheItem {
            key: "journal".to_string(),
            label: "Sistem günlükleri (journald)".to_string(),
            path: journal_path.to_string_lossy().to_string(),
            size_bytes: dir_size(journal_path, 6),
            removable: true,
        });
    }

    items
}

pub fn clean_cache_item(key: &str) -> (bool, String) {
    run_cleanup(key)
}

pub fn dir_size(path: &Path, max_depth: usize) -> u64 {
    dir_size_recursive(path, 0, max_depth)
}

fn dir_size_recursive(path: &Path, current_depth: usize, max_depth: usize) -> u64 {
    if current_depth > max_depth {
        return 0;
    }
    let mut total = 0;
    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.flatten() {
            let p = entry.path();
            if p.is_dir() {
                if !p.is_symlink() {
                    total += dir_size_recursive(&p, current_depth + 1, max_depth);
                }
            } else if let Ok(meta) = p.metadata() {
                total += meta.len();
            }
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_disk_usage() {
        let usage = disk_usage("/");
        println!("Disk Usage for /: {:?}", usage);
        assert!(usage.total_bytes > 0);
    }

    #[test]
    fn test_list_all_disks() {
        let disks = list_all_disks();
        println!("All Disks: {:?}", disks);
        assert!(!disks.is_empty());
    }

    #[test]
    fn test_analyze_caches() {
        let caches = analyze_caches();
        println!("Caches analyzed: {:?}", caches);
    }
}
