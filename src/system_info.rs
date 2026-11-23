use std::fs;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlazeEdition {
    Air,
    Blaze,
    Fire,
    Unknown(String),
}

impl BlazeEdition {
    pub fn name(&self) -> &'static str {
        match self {
            BlazeEdition::Air => "Blaze Air",
            BlazeEdition::Blaze => "Blaze",
            BlazeEdition::Fire => "Blaze Fire",
            BlazeEdition::Unknown(_) => "BlazeOS",
        }
    }

    pub fn code_name(&self) -> &'static str {
        match self {
            BlazeEdition::Air => "air",
            BlazeEdition::Blaze => "bear",
            BlazeEdition::Fire => "chameleon",
            BlazeEdition::Unknown(_) => "generic",
        }
    }
}

#[derive(Debug, Clone)]
pub struct SystemInfo {
    pub edition: BlazeEdition,
    pub edition_line: String,
    pub ram_gib: f64,
    pub os_name: String,
    pub version_id: String,
    pub kernel: String,
    pub hostname: String,
    pub arch: String,
    pub uptime: String,
}

impl SystemInfo {
    pub fn gather() -> Self {
        let edition = detect_edition();
        let edition_line = format!("{} 13 Chameleon", edition.name());
        let ram_gib = read_ram_gib();
        let (os_name, version_id) = read_os_release();
        let kernel = read_kernel_version();
        let hostname = read_hostname();
        let arch = std::env::consts::ARCH.to_string();
        let uptime = read_uptime();

        Self {
            edition,
            edition_line,
            ram_gib,
            os_name,
            version_id,
            kernel,
            hostname,
            arch,
            uptime,
        }
    }
}

fn detect_edition() -> BlazeEdition {
    // Check /etc/blaze-edition if it exists
    if let Ok(content) = fs::read_to_string("/etc/blaze-edition") {
        let trimmed = content.trim().to_lowercase();
        if trimmed.contains("air") {
            return BlazeEdition::Air;
        } else if trimmed.contains("fire") || trimmed.contains("chameleon") {
            return BlazeEdition::Fire;
        } else if trimmed.contains("bear") || trimmed.contains("blaze") {
            return BlazeEdition::Blaze;
        }
    }

    // Fallback: check hostname or os-release
    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        let lower = content.to_lowercase();
        if lower.contains("air") {
            return BlazeEdition::Air;
        } else if lower.contains("chameleon") || lower.contains("fire") {
            return BlazeEdition::Fire;
        }
    }

    BlazeEdition::Blaze
}

fn read_ram_gib() -> f64 {
    if let Ok(meminfo) = fs::read_to_string("/proc/meminfo") {
        for line in meminfo.lines() {
            if line.starts_with("MemTotal:") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Ok(kb) = parts[1].parse::<u64>() {
                        return kb as f64 / (1024.0 * 1024.0);
                    }
                }
            }
        }
    }
    0.0
}

fn read_os_release() -> (String, String) {
    let mut name = String::from("BlazeOS");
    let mut version = String::from("13");

    if let Ok(content) = fs::read_to_string("/etc/os-release") {
        for line in content.lines() {
            if let Some(stripped) = line.strip_prefix("NAME=") {
                name = stripped.trim_matches('"').to_string();
            } else if let Some(stripped) = line.strip_prefix("VERSION_ID=") {
                version = stripped.trim_matches('"').to_string();
            }
        }
    }

    (name, version)
}

fn read_kernel_version() -> String {
    if let Ok(version) = fs::read_to_string("/proc/sys/kernel/osrelease") {
        version.trim().to_string()
    } else {
        String::from("Linux")
    }
}

fn read_hostname() -> String {
    if let Ok(host) = fs::read_to_string("/proc/sys/kernel/hostname") {
        host.trim().to_string()
    } else {
        String::from("localhost")
    }
}

fn read_uptime() -> String {
    if let Ok(uptime_str) = fs::read_to_string("/proc/uptime") {
        if let Some(first_part) = uptime_str.split_whitespace().next() {
            if let Ok(secs_f) = first_part.parse::<f64>() {
                let total_secs = secs_f as u64;
                let days = total_secs / 86400;
                let hours = (total_secs % 86400) / 3600;
                let mins = (total_secs % 3600) / 60;

                if days > 0 {
                    return format!("{} gün {} saat {} dk", days, hours, mins);
                } else if hours > 0 {
                    return format!("{} saat {} dk", hours, mins);
                } else {
                    return format!("{} dk", mins);
                }
            }
        }
    }
    String::from("Bilinmiyor")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_info_gather() {
        let info = SystemInfo::gather();
        assert!(info.ram_gib > 0.0);
        assert!(!info.kernel.is_empty());
        assert!(!info.hostname.is_empty());
        assert!(!info.arch.is_empty());
        assert!(!info.uptime.is_empty());
        println!("Gathered SystemInfo: {:?}", info);
    }
}
