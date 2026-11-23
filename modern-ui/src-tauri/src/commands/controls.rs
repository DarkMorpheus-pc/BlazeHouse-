use serde::Serialize;
use std::{fs, path::Path, process::Command};

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Control {
    id: String,
    label: String,
    description: String,
    group: String,
    value: Option<String>,
    options: Vec<String>,
    supported: bool,
}

fn output(program: &str, args: &[&str]) -> Result<String, String> {
    let result = Command::new(program)
        .args(args)
        .output()
        .map_err(|e| format!("{program}: {e}"))?;
    if !result.status.success() {
        return Err(format!(
            "{program}: {}",
            String::from_utf8_lossy(&result.stderr).trim()
        ));
    }
    Ok(String::from_utf8_lossy(&result.stdout).trim().to_string())
}

fn read(path: &str) -> Option<String> {
    fs::read_to_string(path).ok().map(|v| v.trim().to_string())
}

fn thresholds() -> Vec<std::path::PathBuf> {
    fs::read_dir("/sys/class/power_supply")
        .into_iter()
        .flatten()
        .flatten()
        .map(|entry| entry.path().join("charge_control_end_threshold"))
        .filter(|path| path.is_file())
        .collect()
}

fn control(
    id: &str,
    label: &str,
    description: &str,
    group: &str,
    value: Option<String>,
    options: Vec<String>,
) -> Control {
    let supported = value.is_some() && !options.is_empty();
    Control {
        id: id.into(),
        label: label.into(),
        description: description.into(),
        group: group.into(),
        value,
        options,
        supported,
    }
}

pub fn read_controls() -> Vec<Control> {
    let quantum = output(
        "pw-metadata",
        &["-n", "settings", "0", "clock.force-quantum"],
    )
    .ok()
    .and_then(|text| {
        text.split("value:'")
            .nth(1)
            .and_then(|part| part.split('\'').next())
            .map(String::from)
    });
    let wine_path = std::env::var("HOME")
        .ok()
        .map(|home| format!("{home}/.config/environment.d/90-blazehouse-gaming.conf"));
    let wine_value = wine_path.as_ref().map(|path| {
        if read(path)
            .unwrap_or_default()
            .lines()
            .any(|line| line == "WINE_FSYNC=1")
        {
            "1".to_string()
        } else {
            "0".to_string()
        }
    });
    let game_loaded = output(
        "systemctl",
        &[
            "--user",
            "show",
            "gamemoded.service",
            "--property=LoadState",
            "--value",
        ],
    )
    .ok()
    .as_deref()
        == Some("loaded");
    let game_state = if game_loaded {
        output(
            "systemctl",
            &[
                "--user",
                "show",
                "gamemoded.service",
                "--property=ActiveState",
                "--value",
            ],
        )
        .ok()
    } else {
        None
    };
    let governors = read("/sys/devices/system/cpu/cpu0/cpufreq/scaling_available_governors")
        .unwrap_or_default()
        .split_whitespace()
        .map(String::from)
        .collect();
    let profiles = output("powerprofilesctl", &["list"]).unwrap_or_default();
    let profile_options = ["power-saver", "balanced", "performance"]
        .into_iter()
        .filter(|profile| {
            profiles
                .lines()
                .any(|line| line.trim().trim_start_matches('*').trim() == format!("{profile}:"))
        })
        .map(String::from)
        .collect();
    let threshold = thresholds()
        .first()
        .and_then(|path| fs::read_to_string(path).ok())
        .map(|v| v.trim().to_string());
    vec![
        control("quantum", "PipeWire ses tamponu", "Kare cinsinden ses tamponu. 0 otomatik yönetimdir; küçük tamponlar işlemci yükünü artırabilir.", "gaming", quantum, vec!["0".into(), "64".into(), "128".into(), "256".into(), "512".into()]),
        control("fsync", "Wine Fsync tercihi", "Kullanıcı environment.d dosyasına kaydedilir; yeni oturumda devreye girer. Wine/Proton sürümünün Fsync desteği gerekir.", "gaming", wine_value, vec!["0".into(), "1".into()]),
        control("gamemode", "GameMode kullanıcı servisi", "Servisi başlatır veya durdurur. Oyunların GameMode kullanması için gamemoderun ile başlatılması gerekir.", "gaming", game_state, vec!["active".into(), "inactive".into()]),
        control("governor", "İşlemci frekans politikası", "Çekirdeğin bu işlemci için bildirdiği politikalar. Değişiklik tüm CPU çekirdeklerine uygulanır.", "gaming", read("/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor"), governors),
        control("profile", "Sistem güç profili", "power-profiles-daemon tarafından sunulan ve doğrulanan güç profili.", "power", output("powerprofilesctl", &["get"]).ok(), profile_options),
        control("threshold", "Batarya şarj sınırı", "Sürücünün desteklediği bataryalarda üst şarj sınırı. 100 değeri normal şarja döndürür.", "power", threshold, (60..=100).step_by(5).map(|n| n.to_string()).collect()),
    ]
}

#[tauri::command]
pub async fn get_system_controls() -> Result<Vec<Control>, String> {
    tauri::async_runtime::spawn_blocking(read_controls)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_system_control(id: String, value: String) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || {
        let controls = read_controls();
        let control = controls.iter().find(|item| item.id == id).ok_or("Bilinmeyen ayar")?;
        if !control.supported || !control.options.contains(&value) {
            return Err("Bu ayar veya değer sistem tarafından desteklenmiyor".to_string());
        }
        match id.as_str() {
            "quantum" => { output("pw-metadata", &["-n", "settings", "0", "clock.force-quantum", &value])?; }
            "fsync" => {
                let home = std::env::var("HOME").map_err(|e| e.to_string())?;
                let dir = Path::new(&home).join(".config/environment.d");
                fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
                fs::write(dir.join("90-blazehouse-gaming.conf"), format!("WINE_FSYNC={value}\n")).map_err(|e| e.to_string())?;
            }
            "gamemode" => { output("systemctl", &["--user", if value == "active" { "start" } else { "stop" }, "gamemoded.service"])?; }
            "profile" => { output("powerprofilesctl", &["set", &value])?; }
            "governor" => {
                // The value is checked against the kernel's available governors before entering the shell.
                if !value.chars().all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-') { return Err("Geçersiz politika".into()); }
                let script = format!("set -eu; for f in /sys/devices/system/cpu/cpu[0-9]*/cpufreq/scaling_governor; do [ -f \"$f\" ] || continue; printf '%s' '{value}' > \"$f\"; [ \"$(cat \"$f\")\" = '{value}' ]; done");
                output("pkexec", &["sh", "-c", &script])?;
            }
            "threshold" => {
                let paths = thresholds();
                if paths.is_empty() { return Err("Şarj eşiği desteklenmiyor".into()); }
                for path in paths {
                    let path = path.to_str().ok_or("Geçersiz batarya yolu")?;
                    if !Path::new(path).starts_with("/sys/class/power_supply") { return Err("Geçersiz yol".into()); }
                    output("pkexec", &["sh", "-c", "printf '%s' \"$1\" > \"$2\"", "blaze-house", &value, path])?;
                    if read(path).as_deref() != Some(value.as_str()) { return Err("Batarya eşiği doğrulanamadı".into()); }
                }
            }
            _ => return Err("Bilinmeyen ayar".into()),
        }
        let verified = read_controls().into_iter().find(|item| item.id == id).and_then(|item| item.value);
        if verified.as_deref() != Some(value.as_str()) { return Err("İşlem sonrası sistem değeri doğrulanamadı".into()); }
        Ok(vec![format!("{}: {}", control.label, value), "Sistem yeniden okunarak doğrulandı.".into()])
    }).await.map_err(|e| e.to_string())?
}
