use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs;
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct PackageUpdate {
    pub id: String,
    pub name: String,
    pub current_version: String,
    pub new_version: String,
    pub size_mb: f32,
    #[serde(rename = "type")]
    pub pkg_type: String,
    pub source: String,
    pub description: String,
    pub changelog: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct UpgradeTargetInfo {
    pub series: String,
    pub target_releasever: String,
    pub ready: bool,
    pub min_blaze_release: String,
    pub notes: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateChannelInfo {
    pub distro_family: String,
    pub product_name: String,
    pub product_series: String,
    pub github_repo: String,
    pub channel_url: String,
    pub rpm_repo_url: String,
    pub current_releasever: String,
    pub online_reachable: bool,
    pub remote_series: Option<String>,
    pub remote_base_fedora: Option<String>,
    pub remote_packages: BTreeMap<String, String>,
    pub release_notes: String,
    pub upgrade_target: Option<UpgradeTargetInfo>,
    pub error: Option<String>,
}

#[derive(Debug, Deserialize)]
struct RemoteUpgradeTarget {
    #[serde(default)]
    series: String,
    #[serde(default)]
    target_releasever: String,
    #[serde(default)]
    ready: bool,
    #[serde(default)]
    min_blaze_release: String,
    #[serde(default)]
    notes: String,
}

#[derive(Debug, Deserialize)]
struct RemoteChannelManifest {
    #[serde(default)]
    product: Option<String>,
    #[serde(default)]
    series: Option<String>,
    #[serde(default)]
    base_fedora: Option<String>,
    #[serde(default)]
    repo_url: Option<String>,
    #[serde(default)]
    github_repo: Option<String>,
    #[serde(default)]
    packages: BTreeMap<String, String>,
    #[serde(default)]
    upgrade_target: Option<RemoteUpgradeTarget>,
}

#[derive(Debug, Deserialize)]
struct GitHubReleaseManifest {
    #[serde(default)]
    tag_name: Option<String>,
    #[serde(default)]
    name: Option<String>,
    #[serde(default)]
    body: Option<String>,
}

pub fn parse_channel_conf(content: &str) -> BTreeMap<String, String> {
    let mut map = BTreeMap::new();
    for raw_line in content.lines() {
        let line = raw_line.trim();
        if line.is_empty() || line.starts_with('#') || line.starts_with('[') {
            continue;
        }
        if let Some((k, v)) = line.split_once('=') {
            map.insert(k.trim().to_string(), v.trim().trim_matches('"').to_string());
        }
    }
    map
}

pub fn build_default_channel_info(
    channel_conf: Option<&str>,
    os_release: Option<&str>,
    distro: &str,
) -> UpdateChannelInfo {
    let mut version_id = String::new();
    let mut os_id = String::new();
    let mut os_like = String::new();

    if let Some(os_txt) = os_release {
        let os_map = parse_channel_conf(os_txt);
        version_id = os_map.get("VERSION_ID").cloned().unwrap_or_default();
        os_id = os_map.get("ID").cloned().unwrap_or_default().to_lowercase();
        os_like = os_map.get("ID_LIKE").cloned().unwrap_or_default().to_lowercase();
    }

    let is_fedora_family = os_id.contains("blazeos")
        || os_id.contains("fedora")
        || os_like.contains("fedora")
        || (os_id.is_empty() && distro == "fedora");

    let mut info = if is_fedora_family {
        UpdateChannelInfo {
            distro_family: "solar-evolution".to_string(),
            product_name: "Blaze SolarEvolution".to_string(),
            product_series: "5.05".to_string(),
            github_repo: "DarkMorpheus-pc/Blaze-SolarEvolution".to_string(),
            channel_url: "https://darkmorpheus-pc.github.io/Blaze-SolarEvolution/repo/channel.json".to_string(),
            rpm_repo_url: "https://darkmorpheus-pc.github.io/Blaze-SolarEvolution/repo/".to_string(),
            current_releasever: if version_id.is_empty() { "45".to_string() } else { version_id.clone() },
            online_reachable: false,
            remote_series: None,
            remote_base_fedora: None,
            remote_packages: BTreeMap::new(),
            release_notes: "Fedora tabanlı Blaze SolarEvolution resmi GitHub Pages RPM kanalı".to_string(),
            upgrade_target: None,
            error: None,
        }
    } else {
        UpdateChannelInfo {
            distro_family: "universe".to_string(),
            product_name: "Blaze Universe".to_string(),
            product_series: "Rolling".to_string(),
            github_repo: "DarkMorpheus-pc/Blaze-Universe-".to_string(),
            channel_url: "https://api.github.com/repos/DarkMorpheus-pc/Blaze-Universe-/releases/latest".to_string(),
            rpm_repo_url: String::new(),
            current_releasever: if version_id.is_empty() { distro.to_string() } else { version_id.clone() },
            online_reachable: false,
            remote_series: None,
            remote_base_fedora: None,
            remote_packages: BTreeMap::new(),
            release_notes: "Arch/Debian tabanlı Blaze Universe GitHub sürüm kanalı".to_string(),
            upgrade_target: None,
            error: None,
        }
    };

    if let Some(conf_txt) = channel_conf {
        let cfg = parse_channel_conf(conf_txt);
        if let Some(v) = cfg.get("distro_family").filter(|s| !s.is_empty()) {
            info.distro_family = v.clone();
        }
        if let Some(v) = cfg.get("product_name").filter(|s| !s.is_empty()) {
            info.product_name = v.clone();
        }
        if let Some(v) = cfg.get("product_series").filter(|s| !s.is_empty()) {
            info.product_series = v.clone();
        }
        if let Some(v) = cfg.get("github_repo").filter(|s| !s.is_empty()) {
            info.github_repo = v.clone();
        }
        if let Some(v) = cfg.get("repo_manifest_url").filter(|s| !s.is_empty()) {
            info.channel_url = v.clone();
        }
        if let Some(v) = cfg.get("rpm_repo_baseurl").filter(|s| !s.is_empty()) {
            info.rpm_repo_url = v.clone();
        }
        if let Some(v) = cfg.get("current_releasever").filter(|s| !s.is_empty()) {
            info.current_releasever = v.clone();
        }
    }

    info
}

pub fn apply_remote_channel_json(info: &mut UpdateChannelInfo, raw_json: &str) -> Result<(), String> {
    if info.channel_url.contains("api.github.com/") {
        let gh: GitHubReleaseManifest =
            serde_json::from_str(raw_json).map_err(|e| format!("GitHub JSON ayrıştırılamadı: {e}"))?;
        info.online_reachable = true;
        if let Some(tag) = gh.tag_name.filter(|s| !s.is_empty()) {
            info.remote_series = Some(tag);
        } else if let Some(name) = gh.name.filter(|s| !s.is_empty()) {
            info.remote_series = Some(name);
        }
        if let Some(body) = gh.body.filter(|s| !s.trim().is_empty()) {
            info.release_notes = body.lines().take(6).collect::<Vec<_>>().join(" ");
        }
        return Ok(());
    }

    let manifest: RemoteChannelManifest =
        serde_json::from_str(raw_json).map_err(|e| format!("channel.json ayrıştırılamadı: {e}"))?;
    info.online_reachable = true;
    if let Some(p) = manifest.product.filter(|s| !s.is_empty()) {
        info.product_name = p;
    }
    if let Some(r) = manifest.github_repo.filter(|s| !s.is_empty()) {
        info.github_repo = r;
    }
    if let Some(u) = manifest.repo_url.filter(|s| !s.is_empty()) {
        info.rpm_repo_url = u;
    }
    info.remote_series = manifest.series;
    info.remote_base_fedora = manifest.base_fedora;
    info.remote_packages = manifest.packages;
    if let Some(target) = manifest.upgrade_target {
        if !target.notes.trim().is_empty() {
            info.release_notes = target.notes.clone();
        }
        info.upgrade_target = Some(UpgradeTargetInfo {
            series: target.series,
            target_releasever: target.target_releasever,
            ready: target.ready,
            min_blaze_release: target.min_blaze_release,
            notes: target.notes,
        });
    }
    Ok(())
}

pub fn get_update_channel_blocking(distro: String) -> UpdateChannelInfo {
    let channel_conf = fs::read_to_string("/etc/blazeos/update-channel.conf").ok();
    let os_release = fs::read_to_string("/etc/os-release").ok();
    let mut info = build_default_channel_info(channel_conf.as_deref(), os_release.as_deref(), &distro);

    if info.channel_url.starts_with("https://") {
        let out = Command::new("curl")
            .args([
                "-fsSL",
                "--max-time",
                "8",
                "-A",
                "BlazeHouse/5.1",
                &info.channel_url,
            ])
            .env("LC_ALL", "C")
            .output();
        match out {
            Ok(res) if res.status.success() => {
                let body = String::from_utf8_lossy(&res.stdout);
                if let Err(err) = apply_remote_channel_json(&mut info, &body) {
                    info.online_reachable = false;
                    info.error = Some(err);
                }
            }
            Ok(res) => {
                info.online_reachable = false;
                let msg = String::from_utf8_lossy(&res.stderr).trim().to_string();
                info.error = Some(if msg.is_empty() {
                    format!("Kanal yanıt vermedi (curl çıkış kodu {:?})", res.status.code())
                } else {
                    msg
                });
            }
            Err(e) => {
                info.online_reachable = false;
                info.error = Some(format!("curl çalıştırılamadı: {e}"));
            }
        }
    }

    info
}

#[tauri::command]
pub async fn get_update_channel(distro: String) -> Result<UpdateChannelInfo, String> {
    tauri::async_runtime::spawn_blocking(move || get_update_channel_blocking(distro))
        .await
        .map_err(|e| e.to_string())
}

fn package(name: &str, old: &str, new: &str, source: &str) -> PackageUpdate {
    PackageUpdate {
        id: name.into(),
        name: name.into(),
        current_version: old.into(),
        new_version: new.into(),
        size_mb: 0.0,
        pkg_type: "feature".into(),
        source: source.into(),
        description: String::new(),
        changelog: vec![],
    }
}
pub fn check_system_updates_blocking(distro: String) -> Result<Vec<PackageUpdate>, String> {
    let (program, args, source, accepted) = match distro.as_str() {
        "arch" => ("pacman", vec!["-Qu"], "pacman", vec![0, 1]),
        "fedora" => (
            "dnf5",
            vec!["check-upgrade", "--quiet"],
            "dnf",
            vec![0, 100],
        ),
        "debian" => ("apt", vec!["list", "--upgradable"], "apt", vec![0]),
        _ => return Err("Desteklenmeyen paket yöneticisi".into()),
    };
    let output = Command::new(program)
        .args(args)
        .env("LC_ALL", "C")
        .output()
        .map_err(|e| format!("{program}: {e}"))?;
    if !accepted.contains(&output.status.code().unwrap_or(-1))
        || (output.status.code() == Some(1) && !output.stderr.is_empty())
    {
        return Err(format!(
            "{program}: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }
    let text = String::from_utf8_lossy(&output.stdout);
    let mut result = vec![];
    for line in text.lines() {
        let parts: Vec<_> = line.split_whitespace().collect();
        if source == "pacman" && parts.len() >= 4 && parts[2] == "->" {
            result.push(package(parts[0], parts[1], parts[3], source));
        }
        if source == "dnf" && parts.len() >= 3 && parts[0].contains('.') {
            result.push(package(
                parts[0],
                "Kurulu sürüm sorgulanmadı",
                parts[1],
                source,
            ));
        }
        if source == "apt" && parts.len() >= 3 && parts[0].contains('/') {
            let name = parts[0].split('/').next().unwrap_or(parts[0]);
            let old = line
                .split("upgradable from: ")
                .nth(1)
                .map(|s| s.trim_end_matches(']'))
                .unwrap_or("Bilinmiyor");
            result.push(package(name, old, parts[1], source));
        }
    }
    Ok(result)
}

#[tauri::command]
pub async fn check_system_updates(distro: String) -> Result<Vec<PackageUpdate>, String> {
    tauri::async_runtime::spawn_blocking(move || check_system_updates_blocking(distro))
        .await
        .map_err(|e| e.to_string())?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_solar_evolution_channel_from_conf_and_os_release() {
        let conf = "[channel]\ndistro_family=solar-evolution\nproduct_name=Blaze SolarEvolution\nproduct_series=5.05\ngithub_repo=DarkMorpheus-pc/Blaze-SolarEvolution\nrepo_manifest_url=https://darkmorpheus-pc.github.io/Blaze-SolarEvolution/repo/channel.json\nrpm_repo_baseurl=https://darkmorpheus-pc.github.io/Blaze-SolarEvolution/repo/\ncurrent_releasever=45\n";
        let os_rel = "ID=blazeos\nID_LIKE=\"fedora\"\nVERSION_ID=45\n";
        let info = build_default_channel_info(Some(conf), Some(os_rel), "fedora");
        assert_eq!(info.distro_family, "solar-evolution");
        assert_eq!(info.github_repo, "DarkMorpheus-pc/Blaze-SolarEvolution");
        assert_eq!(info.current_releasever, "45");
    }

    #[test]
    fn falls_back_to_blaze_universe_for_debian_or_arch() {
        let os_rel = "ID=debian\nVERSION_ID=\"13\"\n";
        let info = build_default_channel_info(None, Some(os_rel), "debian");
        assert_eq!(info.distro_family, "universe");
        assert_eq!(info.github_repo, "DarkMorpheus-pc/Blaze-Universe-");
        assert_eq!(info.current_releasever, "13");
    }

    #[test]
    fn parses_remote_channel_json_with_upgrade_target() {
        let mut info = build_default_channel_info(None, Some("ID=blazeos\nVERSION_ID=45\n"), "fedora");
        let payload = r#"{
            "product": "Blaze SolarEvolution",
            "series": "5.05",
            "base_fedora": "45",
            "repo_url": "https://darkmorpheus-pc.github.io/Blaze-SolarEvolution/repo/",
            "github_repo": "DarkMorpheus-pc/Blaze-SolarEvolution",
            "packages": {"blaze-release": "5.05-3.blaze", "blaze-house": "2.1.0-1.blaze"},
            "upgrade_target": {
                "series": "6.0",
                "target_releasever": "46",
                "ready": true,
                "min_blaze_release": "5.05-3.blaze",
                "notes": "Fedora 46 tabanina gecis hazir."
            }
        }"#;
        apply_remote_channel_json(&mut info, payload).unwrap();
        assert!(info.online_reachable);
        assert_eq!(info.remote_packages.get("blaze-house").map(String::as_str), Some("2.1.0-1.blaze"));
        let target = info.upgrade_target.unwrap();
        assert!(target.ready);
        assert_eq!(target.target_releasever, "46");
        assert_eq!(target.series, "6.0");
    }
}
