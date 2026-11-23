use std::process::Command;
use tauri::Emitter;

fn ensure_policykit_available() -> Result<(), String> {
    let service = "/usr/share/dbus-1/system-services/org.freedesktop.PolicyKit1.service";
    if !std::path::Path::new(service).is_file() {
        return Err(format!(
            "Yetkilendirme servisi eksik ({service}). BlazeOS polkit paketi onarılmalı."
        ));
    }
    let active = Command::new("systemctl")
        .args(["is-active", "--quiet", "polkit.service"])
        .status()
        .map(|status| status.success())
        .unwrap_or(false);
    if !active {
        // Request normal system-bus activation. This operation itself does not
        // grant privileges; polkit still authenticates the following pkexec.
        let _ = Command::new("busctl")
            .args([
                "--system",
                "call",
                "org.freedesktop.DBus",
                "/org/freedesktop/DBus",
                "org.freedesktop.DBus",
                "StartServiceByName",
                "su",
                "org.freedesktop.PolicyKit1",
                "0",
            ])
            .status();
    }
    Ok(())
}

fn output_text(output: &std::process::Output) -> String {
    let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
    let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
    if stdout.is_empty() {
        stderr
    } else {
        stdout
    }
}

pub fn clean_storage_blocking(category_ids: Vec<String>) -> Result<Vec<String>, String> {
    if category_ids.is_empty()
        || category_ids
            .iter()
            .any(|id| !matches!(id.as_str(), "pkg_cache" | "system_logs" | "thumbnails"))
    {
        return Err("Temizlenecek geçerli bir kategori seçin".into());
    }
    let mut steps = Vec::new();
    let mut clean_cmds = Vec::new();

    for id in &category_ids {
        match id.as_str() {
            "pkg_cache" => {
                if std::path::Path::new("/etc/arch-release").exists() {
                    clean_cmds
                        .push("find /var/cache/pacman/pkg -mindepth 1 -maxdepth 1 -type f -delete");
                    steps.push("Arch Linux pacman paket önbelleği temizlendi".to_string());
                } else if std::path::Path::new("/etc/fedora-release").exists() {
                    clean_cmds.push("dnf clean all");
                    steps.push("Fedora DNF önbelleği temizlendi".to_string());
                } else {
                    clean_cmds.push("apt-get clean");
                    steps.push("Debian APT önbelleği temizlendi".to_string());
                }
            }
            "system_logs" => {
                clean_cmds.push("journalctl --vacuum-time=7d");
                steps.push("Systemd journald logları 7 günlük sınıra vakumlandı".to_string());
            }
            "thumbnails" => {
                let home =
                    std::env::var("HOME").map_err(|_| "HOME dizini bulunamadı".to_string())?;
                let thumbnail_dir = std::path::Path::new(&home).join(".cache/thumbnails");
                if thumbnail_dir.exists() {
                    for entry in std::fs::read_dir(&thumbnail_dir).map_err(|e| e.to_string())? {
                        let path = entry.map_err(|e| e.to_string())?.path();
                        if path.is_dir() && !path.is_symlink() {
                            std::fs::remove_dir_all(path)
                        } else {
                            std::fs::remove_file(path)
                        }
                        .map_err(|e| format!("Önizleme önbelleği temizlenemedi: {e}"))?;
                    }
                }
                steps.push("Kullanıcı önizleme (thumbnail) önbelleği temizlendi".to_string());
            }
            _ => {}
        }
    }

    let script = clean_cmds.join(" && ");
    if !script.is_empty() {
        ensure_policykit_available()?;
        let output = Command::new("pkexec")
            .arg("sh")
            .arg("-c")
            .arg(&script)
            .output()
            .map_err(|e| format!("Polkit başlatılamadı: {e}"))?;
        if !output.status.success() {
            return Err(format!("Temizlik başarısız: {}", output_text(&output)));
        }
    }
    steps.push("BAŞARILI: Seçilen disk ve sistem önbellekleri temizlendi.".to_string());
    Ok(steps)
}

pub fn run_privileged_command_blocking(command: String) -> Result<Vec<String>, String> {
    const ALLOWED: &[&str] = &[
        "apt-get update && apt-get upgrade -y",
        "pacman -Syu --noconfirm",
    ];
    if !ALLOWED.contains(&command.as_str()) {
        return Err("Güvenlik nedeniyle bu komut izin listesinde değil".to_string());
    }
    ensure_policykit_available()?;
    let output = Command::new("pkexec")
        .arg("sh")
        .arg("-c")
        .arg(&command)
        .output()
        .map_err(|e| format!("Komut çalıştırılamadı: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut lines: Vec<String> = stdout.lines().map(|s| s.to_string()).collect();

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!(
            "HATA (Çıkış Kodu {:?}): {}",
            output.status.code(),
            stderr
        ));
    } else if lines.is_empty() {
        lines.push("Komut başarıyla yürütüldü (Çıkış Kodu 0).".to_string());
    }

    Ok(lines)
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflineUpdateProgress {
    pub phase: String,
    pub label: String,
    pub percent: u8,
    pub current: u32,
    pub total: u32,
    pub package: String,
    pub line: String,
    pub done: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct OfflineUpdateStatus {
    pub prepared: bool,
    pub state_summary: String,
}

pub fn strip_ansi_and_controls(raw: &str) -> String {
    let mut out = String::with_capacity(raw.len());
    let mut chars = raw.chars().peekable();
    while let Some(c) = chars.next() {
        if c == '\x1b' {
            match chars.peek().copied() {
                Some('[') => {
                    chars.next();
                    for nc in chars.by_ref() {
                        if ('@'..='~').contains(&nc) {
                            break;
                        }
                    }
                }
                Some(']') => {
                    chars.next();
                    while let Some(nc) = chars.next() {
                        if nc == '\x07' {
                            break;
                        }
                        if nc == '\x1b' && chars.peek() == Some(&'\\') {
                            chars.next();
                            break;
                        }
                    }
                }
                Some(_) => {
                    chars.next();
                }
                None => {}
            }
        } else if c == '\r' || c == '\x08' || (c.is_control() && c != '\t') {
            continue;
        } else {
            out.push(c);
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn extract_bracket_fraction(line: &str) -> Option<(u32, u32)> {
    let bytes = line.as_bytes();
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'[' {
            if let Some(rel_end) = line[i + 1..].find(']') {
                let inner = line[i + 1..i + 1 + rel_end].trim();
                if let Some((a, b)) = inner.split_once('/') {
                    let a_trim = a.trim();
                    let b_trim = b.trim();
                    if !a_trim.is_empty()
                        && !b_trim.is_empty()
                        && a_trim.chars().all(|c| c.is_ascii_digit())
                        && b_trim.chars().all(|c| c.is_ascii_digit())
                    {
                        if let (Ok(cur), Ok(tot)) = (a_trim.parse::<u32>(), b_trim.parse::<u32>()) {
                            if tot > 0 && cur <= tot {
                                return Some((cur, tot));
                            }
                        }
                    }
                }
                i += rel_end + 2;
                continue;
            }
        }
        i += 1;
    }
    None
}

fn extract_inline_percent(line: &str) -> Option<u8> {
    let mut best = None;
    for tok in line.split(|c: char| c.is_whitespace() || matches!(c, '(' | ')' | '[' | ']' | '|')) {
        if let Some(num_str) = tok.strip_suffix('%') {
            if !num_str.is_empty() && num_str.chars().all(|c| c.is_ascii_digit()) {
                if let Ok(v) = num_str.parse::<u32>() {
                    if v <= 100 {
                        best = Some(v as u8);
                    }
                }
            }
        }
    }
    best
}

#[derive(Debug, Clone)]
pub struct ProgressTracker {
    pub phase: String,
    pub label: String,
    pub percent: u8,
    pub current: u32,
    pub total: u32,
    pub package: String,
    downloading_started: bool,
}

impl Default for ProgressTracker {
    fn default() -> Self {
        Self {
            phase: "starting".to_string(),
            label: "Yetkilendirme ve DNF5 hazırlanıyor...".to_string(),
            percent: 3,
            current: 0,
            total: 0,
            package: String::new(),
            downloading_started: false,
        }
    }
}

impl ProgressTracker {
    pub fn process_line(&mut self, clean: &str) -> Option<OfflineUpdateProgress> {
        if clean.is_empty() {
            return None;
        }
        let lower = clean.to_lowercase();
        let mut target_pct = self.percent;

        if lower.contains("updating and loading repositories")
            || lower.contains("depolar güncelleniyor")
            || lower.contains("repositories loaded")
            || lower.contains("depolar yüklendi")
        {
            self.phase = "metadata".to_string();
            self.label = "Paket depoları güncelleniyor...".to_string();
            target_pct = target_pct.max(10);
        } else if lower.contains("downloading packages")
            || lower.contains("paketler indiriliyor")
            || lower.contains("package") && lower.contains("yükseltiliyor")
        {
            self.downloading_started = true;
            self.phase = "downloading".to_string();
            self.label = "Güncelleme paketleri indiriliyor...".to_string();
            target_pct = target_pct.max(20);
        } else if lower.contains("running transaction test")
            || lower.contains("işlem sınaması")
            || lower.contains("transaction test succeeded")
        {
            self.phase = "verifying".to_string();
            self.label = "Paket bütünlüğü ve işlem sınaması doğrulanıyor...".to_string();
            target_pct = target_pct.max(92);
        } else if lower.contains("transaction summary")
            || lower.contains("işlem özeti")
            || lower.contains("installing:")
            || lower.contains("kuruluyor:")
            || lower.contains("yükseltiliyor:")
        {
            if !self.downloading_started {
                self.phase = "downloading".to_string();
                self.label = "Paketler çözümleniyor ve indiriliyor...".to_string();
                target_pct = target_pct.max(25);
            } else {
                self.phase = "transaction".to_string();
                self.label = "Çevrimdışı işlem özeti kaydediliyor...".to_string();
                target_pct = target_pct.max(94);
            }
        } else if lower.contains("transaction stored to be performed offline")
            || lower.contains("offline reboot")
            || lower.contains("tamamlandı")
            || lower.contains("complete!")
        {
            self.phase = "verifying".to_string();
            self.label = "Çevrimdışı kurulum kaydı tamamlanıyor...".to_string();
            target_pct = target_pct.max(97);
        }

        if let Some((cur, tot)) = extract_bracket_fraction(clean) {
            self.current = cur;
            self.total = tot;
            let looks_like_pkg = self.downloading_started
                || tot > 15
                || lower.contains(".fc")
                || lower.contains("değiştiriliyor")
                || lower.contains("yükseltiliyor")
                || lower.contains("kuruluyor");
            if looks_like_pkg {
                self.downloading_started = true;
                if self.phase != "verifying" && self.phase != "transaction" {
                    self.phase = "downloading".to_string();
                    self.label = format!("Paketler hazırlanıyor ({cur}/{tot})...");
                }
                let mapped = 18 + ((cur as u64 * 76) / (tot.max(1) as u64)) as u8;
                target_pct = target_pct.max(mapped.min(96));
            } else {
                if self.phase == "starting" {
                    self.phase = "metadata".to_string();
                    self.label = "Depo verileri indiriliyor...".to_string();
                }
                let mapped = 8 + ((cur as u64 * 12) / (tot.max(1) as u64)) as u8;
                target_pct = target_pct.max(mapped.min(20));
            }

            if let Some(after_bracket) = clean.split(']').nth(1) {
                let pkg_txt = after_bracket.trim();
                if !pkg_txt.is_empty() && pkg_txt.len() <= 140 {
                    self.package = pkg_txt.to_string();
                }
            }
        } else if let Some(inline) = extract_inline_percent(clean) {
            if self.phase == "metadata" || self.phase == "starting" {
                self.phase = "metadata".to_string();
                let mapped = 8 + ((inline as u16 * 10) / 100) as u8;
                target_pct = target_pct.max(mapped);
            } else if self.phase == "downloading" && self.total == 0 {
                let mapped = 20 + ((inline as u16 * 70) / 100) as u8;
                target_pct = target_pct.max(mapped);
            }
        }

        if target_pct > self.percent {
            self.percent = target_pct.min(99);
        }

        Some(OfflineUpdateProgress {
            phase: self.phase.clone(),
            label: self.label.clone(),
            percent: self.percent,
            current: self.current,
            total: self.total,
            package: if self.package.is_empty() {
                clean.chars().take(110).collect()
            } else {
                self.package.clone()
            },
            line: clean.to_string(),
            done: false,
            error: None,
        })
    }
}

fn check_offline_update_status_blocking() -> OfflineUpdateStatus {
    let symlink_exists = std::fs::symlink_metadata("/system-update").is_ok();
    let state_path = std::path::Path::new("/usr/lib/sysimage/libdnf5/offline/offline-transaction-state.toml");
    let pending_upgrade_path = std::path::Path::new("/var/lib/blazeos-update/pending-system-upgrade-releasever");
    let pending_upgrade_ver = std::fs::read_to_string(pending_upgrade_path)
        .unwrap_or_default()
        .trim()
        .to_string();
    let state_content = std::fs::read_to_string(state_path).unwrap_or_default();
    let has_staged_state = state_path.is_file()
        && !state_content.trim().is_empty()
        && !state_content.contains("status = \"complete\"");
    let prepared = symlink_exists || has_staged_state || !pending_upgrade_ver.is_empty();
    let state_summary = if !pending_upgrade_ver.is_empty() {
        format!(
            "Fedora {pending_upgrade_ver} ana sürüm yükseltmesi hazırlandı; yeniden başlatmada uygulanacak"
        )
    } else if symlink_exists {
        "Yeniden başlatmada kuruluma hazır (/system-update etkin)".to_string()
    } else if has_staged_state {
        "Çevrimdışı güncelleme işlemi kaydedildi; yeniden başlatmaya hazır".to_string()
    } else {
        String::new()
    };
    OfflineUpdateStatus {
        prepared,
        state_summary,
    }
}

#[tauri::command]
pub async fn get_offline_update_status() -> Result<OfflineUpdateStatus, String> {
    Ok(tokio::task::spawn_blocking(check_offline_update_status_blocking)
        .await
        .map_err(|e| e.to_string())?)
}

fn validate_target_releasever(target_ver: &str) -> Result<String, String> {
    let trimmed = target_ver.trim();
    if trimmed.len() == 2 && trimmed.chars().all(|c| c.is_ascii_digit()) {
        Ok(trimmed.to_string())
    } else {
        Err("Geçersiz hedef sürüm numarası (2 haneli olmalı, örn. 46).".to_string())
    }
}

fn run_blazeos_update_helper(action: &str) -> Result<Vec<String>, String> {
    if !matches!(action, "reboot" | "cancel" | "system-upgrade-reboot") {
        return Err("Geçersiz BlazeOS güncelleme işlemi".to_string());
    }
    let helper = "/usr/libexec/blazeos-update";
    if !std::path::Path::new(helper).is_file() {
        return Err(format!(
            "BlazeOS çevrimdışı güncelleme yardımcısı eksik ({helper}). Sistem paketini onarın."
        ));
    }
    ensure_policykit_available()?;
    let output = Command::new("pkexec")
        .arg(helper)
        .arg(action)
        .output()
        .map_err(|e| format!("Yetkilendirme penceresi başlatılamadı: {e}"))?;
    if !output.status.success() {
        return Err(format!(
            "Güncelleme işlemi başarısız: {}",
            output_text(&output)
        ));
    }
    let mut lines: Vec<String> = String::from_utf8_lossy(&output.stdout)
        .lines()
        .map(str::to_string)
        .collect();
    if lines.is_empty() {
        lines.push(match action {
            "reboot" => "Çevrimdışı güncelleme için yeniden başlatılıyor.",
            "system-upgrade-reboot" => "Yeni BlazeOS ana sürüm yükseltmesi için yeniden başlatılıyor.",
            _ => "Hazırlanmış çevrimdışı güncelleme iptal edildi.",
        }
        .to_string());
    }
    Ok(lines)
}

async fn pump_stream<R: tokio::io::AsyncRead + Unpin>(
    mut reader: R,
    tx: tokio::sync::mpsc::UnboundedSender<String>,
) {
    use tokio::io::AsyncReadExt;
    let mut buf = [0u8; 4096];
    let mut pending: Vec<u8> = Vec::with_capacity(512);
    loop {
        match reader.read(&mut buf).await {
            Ok(0) | Err(_) => {
                if !pending.is_empty() {
                    let s = String::from_utf8_lossy(&pending).into_owned();
                    let _ = tx.send(s);
                }
                break;
            }
            Ok(n) => {
                for &b in &buf[..n] {
                    if b == b'\n' || b == b'\r' {
                        if !pending.is_empty() {
                            let s = String::from_utf8_lossy(&pending).into_owned();
                            let _ = tx.send(s);
                            pending.clear();
                        }
                    } else {
                        pending.push(b);
                    }
                }
            }
        }
    }
}

async fn run_blazeos_update_stream(
    app: tauri::AppHandle,
    helper_args: Vec<String>,
    start_label: &str,
    start_line: &str,
    ready_label: &str,
    ready_pkg: &str,
) -> Result<Vec<String>, String> {
    let helper = "/usr/libexec/blazeos-update";
    if !std::path::Path::new(helper).is_file() {
        return Err(format!(
            "BlazeOS çevrimdışı güncelleme yardımcısı eksik ({helper}). Sistem paketini onarın."
        ));
    }
    tauri::async_runtime::spawn_blocking(ensure_policykit_available)
        .await
        .map_err(|e| e.to_string())??;

    let initial = OfflineUpdateProgress {
        phase: "starting".to_string(),
        label: start_label.to_string(),
        percent: 2,
        current: 0,
        total: 0,
        package: String::new(),
        line: start_line.to_string(),
        done: false,
        error: None,
    };
    let _ = app.emit("offline-update-progress", &initial);

    let mut cmd = tokio::process::Command::new("pkexec");
    cmd.arg(helper);
    for arg in &helper_args {
        cmd.arg(arg);
    }
    let mut child = cmd
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::piped())
        .spawn()
        .map_err(|e| format!("Yetkilendirme penceresi başlatılamadı: {e}"))?;

    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<String>();
    let out_task = child.stdout.take().map(|s| tokio::spawn(pump_stream(s, tx.clone())));
    let err_task = child.stderr.take().map(|s| tokio::spawn(pump_stream(s, tx.clone())));
    drop(tx);

    let mut tracker = ProgressTracker::default();
    let mut logs: Vec<String> = Vec::new();
    while let Some(raw_line) = rx.recv().await {
        let clean = strip_ansi_and_controls(&raw_line);
        if clean.is_empty() {
            continue;
        }
        if logs.last().map(|s| s.as_str()) != Some(clean.as_str()) {
            if logs.len() >= 2500 {
                logs.remove(0);
            }
            logs.push(clean.clone());
        }
        if let Some(ev) = tracker.process_line(&clean) {
            let _ = app.emit("offline-update-progress", &ev);
        }
    }

    if let Some(t) = out_task {
        let _ = t.await;
    }
    if let Some(t) = err_task {
        let _ = t.await;
    }

    let status = child
        .wait()
        .await
        .map_err(|e| format!("Güncelleme işlemi beklenemedi: {e}"))?;

    if !status.success() {
        let tail = logs
            .iter()
            .rev()
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
            .into_iter()
            .rev()
            .collect::<Vec<_>>()
            .join(" | ");
        let err_msg = if tail.is_empty() {
            format!("Güncelleme işlemi başarısız oldu (çıkış kodu: {:?}).", status.code())
        } else {
            format!("Güncelleme işlemi başarısız: {tail}")
        };
        let fail_ev = OfflineUpdateProgress {
            phase: "error".to_string(),
            label: "Güncelleme hazırlanamadı".to_string(),
            percent: tracker.percent,
            current: tracker.current,
            total: tracker.total,
            package: String::new(),
            line: err_msg.clone(),
            done: true,
            error: Some(err_msg.clone()),
        };
        let _ = app.emit("offline-update-progress", &fail_ev);
        return Err(err_msg);
    }

    if logs.is_empty() {
        logs.push(ready_label.to_string());
    }
    let ready_ev = OfflineUpdateProgress {
        phase: "ready".to_string(),
        label: ready_label.to_string(),
        percent: 100,
        current: tracker.total.max(tracker.current),
        total: tracker.total.max(tracker.current),
        package: ready_pkg.to_string(),
        line: ready_label.to_string(),
        done: true,
        error: None,
    };
    let _ = app.emit("offline-update-progress", &ready_ev);
    Ok(logs)
}

#[tauri::command]
pub async fn prepare_offline_update(app: tauri::AppHandle) -> Result<Vec<String>, String> {
    run_blazeos_update_stream(
        app,
        vec!["prepare".to_string()],
        "Yetkilendirme ve DNF5 hazırlanıyor...",
        "DNF5 çevrimdışı güncelleme oturumu başlatılıyor...",
        "İndirildi ve yeniden başlatmada kuruluma hazır.",
        "Çevrimdışı işlem doğrulandı",
    )
    .await
}

#[tauri::command]
pub async fn prepare_system_upgrade(
    app: tauri::AppHandle,
    target_ver: String,
) -> Result<Vec<String>, String> {
    let valid_ver = validate_target_releasever(&target_ver)?;
    run_blazeos_update_stream(
        app,
        vec!["system-upgrade-download".to_string(), valid_ver.clone()],
        &format!("Fedora {valid_ver} ana sürüm yükseltmesi hazırlanıyor..."),
        &format!("Anlık görüntü alınıyor ve Fedora {valid_ver} paketleri indiriliyor..."),
        &format!("Fedora {valid_ver} ana sürüm yükseltmesi yeniden başlatmaya hazır."),
        &format!("system-upgrade --releasever={valid_ver}"),
    )
    .await
}

#[tauri::command]
pub async fn reboot_to_offline_update() -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(|| run_blazeos_update_helper("reboot"))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn reboot_to_system_upgrade() -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(|| run_blazeos_update_helper("system-upgrade-reboot"))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn cancel_offline_update() -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(|| run_blazeos_update_helper("cancel"))
        .await
        .map_err(|e| e.to_string())?
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealStorageCategory {
    pub id: String,
    pub name: String,
    pub size_gb: f32,
    #[serde(rename = "itemCount")]
    pub count: u32,
    pub description: String,
    pub path: String,
    pub color: String,
    #[serde(rename = "isCleanable")]
    pub safe: bool,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RealStorageDetails {
    pub disk_name: String,
    pub mount_point: String,
    pub total_disk_gb: f32,
    pub used_disk_gb: f32,
    pub free_disk_gb: f32,
    pub used_pct: u32,
    pub categories: Vec<RealStorageCategory>,
}

fn get_dir_size_gb(path: &str) -> f32 {
    if let Ok(output) = Command::new("du").args(["-sb", path]).output() {
        if !output.status.success() {
            return -1.0;
        }
        let text = String::from_utf8_lossy(&output.stdout);
        if let Some(first) = text.split_whitespace().next() {
            if let Ok(bytes) = first.parse::<f32>() {
                return (bytes / (1024.0 * 1024.0 * 1024.0) * 10.0).round() / 10.0;
            }
        }
    }
    -1.0
}

pub fn get_storage_details_blocking() -> Result<RealStorageDetails, String> {
    use sysinfo::Disks;
    let disks = Disks::new_with_refreshed_list();
    let root_disk = disks
        .iter()
        .find(|d| d.mount_point() == std::path::Path::new("/"))
        .ok_or_else(|| "Kök dosya sistemi bilgisi okunamadı".to_string())?;

    let total_disk_gb =
        (root_disk.total_space() as f32 / (1024.0 * 1024.0 * 1024.0) * 10.0).round() / 10.0;

    let free_disk_gb =
        (root_disk.available_space() as f32 / (1024.0 * 1024.0 * 1024.0) * 10.0).round() / 10.0;

    let used_disk_gb = ((total_disk_gb - free_disk_gb) * 10.0).round() / 10.0;
    let used_pct = if total_disk_gb > 0.0 {
        ((used_disk_gb / total_disk_gb) * 100.0).round() as u32
    } else {
        0
    };

    let disk_name = std::fs::read_to_string("/sys/block/nvme0n1/device/model")
        .or_else(|_| std::fs::read_to_string("/sys/block/sda/device/model"))
        .map(|s| s.trim().to_string())
        .unwrap_or_else(|_| "Kök dosya sistemi".to_string());

    let home = std::env::var("HOME").map_err(|_| "HOME dizini bulunamadı".to_string())?;
    let pkg_cache_gb = if std::path::Path::new("/var/cache/pacman/pkg").exists() {
        get_dir_size_gb("/var/cache/pacman/pkg")
    } else if std::path::Path::new("/var/cache/apt/archives").exists() {
        get_dir_size_gb("/var/cache/apt/archives")
    } else {
        get_dir_size_gb("/var/cache/dnf")
    };

    let journal_gb = get_dir_size_gb("/var/log/journal");
    let thumbnails_path = format!("{}/.cache/thumbnails", home);
    let thumbnails_gb = get_dir_size_gb(&thumbnails_path);
    let sys_files_gb =
        (used_disk_gb - pkg_cache_gb.max(0.0) - journal_gb.max(0.0) - thumbnails_gb.max(0.0))
            .max(0.0);

    let categories = vec![
        RealStorageCategory {
            id: "system_files".to_string(),
            name: "Diğer Kullanılan Alan".to_string(),
            size_gb: (sys_files_gb * 10.0).round() / 10.0,
            count: 0,
            description: "Temizlenebilir önbellekler dışındaki gerçek kullanılan disk alanı"
                .to_string(),
            path: "/".to_string(),
            color: "#F59E0B".to_string(),
            safe: false,
        },
        RealStorageCategory {
            id: "pkg_cache".to_string(),
            name: "Paket Yöneticisi Önbelleği".to_string(),
            size_gb: pkg_cache_gb,
            count: 0,
            description: "İndirilmiş ancak kurulmuş eski paket arşivleri".to_string(),
            path: "/var/cache/pacman/pkg".to_string(),
            color: "#EC4899".to_string(),
            safe: true,
        },
        RealStorageCategory {
            id: "system_logs".to_string(),
            name: "Sistem & Journal Logları".to_string(),
            size_gb: journal_gb,
            count: 0,
            description: "systemd-journald eski sistem geçmiş logları".to_string(),
            path: "/var/log/journal".to_string(),
            color: "#8B5CF6".to_string(),
            safe: true,
        },
        RealStorageCategory {
            id: "thumbnails".to_string(),
            name: "Önizleme & Uygulama Önbelleği".to_string(),
            size_gb: thumbnails_gb,
            count: 0,
            description: "Dosya yöneticisi thumbnail ve geçici resim tamponları".to_string(),
            path: "~/.cache/thumbnails".to_string(),
            color: "#F97316".to_string(),
            safe: true,
        },
    ];

    Ok(RealStorageDetails {
        disk_name,
        mount_point: "/".to_string(),
        total_disk_gb,
        used_disk_gb,
        free_disk_gb,
        used_pct,
        categories,
    })
}

#[tauri::command]
pub async fn clean_storage(category_ids: Vec<String>) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || clean_storage_blocking(category_ids))
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn get_storage_details() -> Result<RealStorageDetails, String> {
    tauri::async_runtime::spawn_blocking(move || get_storage_details_blocking())
        .await
        .map_err(|e| e.to_string())?
}

#[tauri::command]
pub async fn run_privileged_command(command: String) -> Result<Vec<String>, String> {
    tauri::async_runtime::spawn_blocking(move || run_privileged_command_blocking(command))
        .await
        .map_err(|e| e.to_string())?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strip_ansi_removes_escape_sequences_cleanly() {
        let raw = "\x1b[32m[12/40]\x1b[0m \x1b[1m7zip\x1b[0m   1.4 MiB/s | 1.2 MiB   \x1b[K";
        assert_eq!(strip_ansi_and_controls(raw), "[12/40] 7zip 1.4 MiB/s | 1.2 MiB");
    }

    #[test]
    fn progress_tracker_tracks_monotonic_dnf5_phases() {
        let mut tracker = ProgressTracker::default();
        let meta = tracker
            .process_line("Depolar güncelleniyor ve yükleniyor:")
            .unwrap();
        assert_eq!(meta.phase, "metadata");
        assert!(meta.percent >= 10);

        let r1 = tracker
            .process_line("[1/3] fedora                     100% | 1.2 MiB/s")
            .unwrap();
        assert!(r1.percent >= meta.percent && r1.percent <= 20);

        let dl_head = tracker.process_line("Paketler indiriliyor:").unwrap();
        assert_eq!(dl_head.phase, "downloading");
        assert!(dl_head.percent >= 20);

        let p1 = tracker
            .process_line("[100/200] 7zip x86_64 26.06-1.fc45 updates 1.2 MiB")
            .unwrap();
        assert_eq!(p1.phase, "downloading");
        assert_eq!(p1.current, 100);
        assert_eq!(p1.total, 200);
        assert!(p1.percent >= 50 && p1.percent < 97);

        let p_sum = tracker.process_line("[200/200] İşlem özeti:").unwrap();
        assert!(p_sum.percent >= p1.percent && p_sum.percent <= 99);
    }

    #[test]
    fn validates_major_upgrade_target_releasever() {
        assert_eq!(validate_target_releasever("46").as_deref(), Ok("46"));
        assert_eq!(validate_target_releasever(" 47 ").as_deref(), Ok("47"));
        assert!(validate_target_releasever("").is_err());
        assert!(validate_target_releasever("46; rm -rf /").is_err());
        assert!(validate_target_releasever("fc46").is_err());
    }
}
