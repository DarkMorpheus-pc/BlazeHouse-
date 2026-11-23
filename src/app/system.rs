use crate::app::error::BlazeError;
use crate::system_info::SystemInfo;

#[derive(Debug, Clone)]
pub struct OverviewData {
    pub os_name: String,
    pub version_id: String,
    pub edition_line: String,
    pub kernel: String,
    pub hostname: String,
    pub arch: String,
    pub uptime: String,
    pub cpu_model: String,
    pub cpu_cores: String,
    pub ram_summary: String,
    pub gpu_summary: String,
    pub storage_summary: String,
}

#[derive(Debug, Clone)]
pub struct SystemDetailsData {
    pub os_name: String,
    pub version_id: String,
    pub edition_line: String,
    pub kernel: String,
    pub hostname: String,
    pub arch: String,
    pub desktop_env: String,
    pub session_type: String,
    pub uptime: String,
}

pub struct SystemService;

impl SystemService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_system_info(&self) -> Result<SystemInfo, BlazeError> {
        let info = SystemInfo::gather();
        Ok(info)
    }

    pub fn get_overview_data(&self) -> Result<OverviewData, BlazeError> {
        let sys = SystemInfo::gather();
        let hw = crate::hardware_info::gather_hardware();
        let storage = crate::storage::disk_usage("/");

        let cpu_model = hw
            .cpu
            .first()
            .map(|c| c.value.clone())
            .unwrap_or_else(|| "İşlemci Bilgisi Alınamadı".to_string());
        let cpu_cores = hw
            .cpu
            .get(1)
            .map(|c| c.value.clone())
            .unwrap_or_else(|| "Bilinmiyor".to_string());
        let ram_summary = format!("{:.2} GiB Toplam Bellek", sys.ram_gib);
        let gpu_summary = hw
            .graphics
            .first()
            .map(|g| g.value.clone())
            .unwrap_or_else(|| "Grafik Bağdaştırıcısı Yok".to_string());

        let free_gb = storage.free_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
        let total_gb = storage.total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);

        let storage_summary = format!(
            "%{:.1} Kullanımda ({:.1} GB Boş / {:.1} GB Toplam)",
            storage.percent, free_gb, total_gb
        );

        Ok(OverviewData {
            os_name: sys.os_name,
            version_id: sys.version_id,
            edition_line: sys.edition_line,
            kernel: sys.kernel,
            hostname: sys.hostname,
            arch: sys.arch,
            uptime: sys.uptime,
            cpu_model,
            cpu_cores,
            ram_summary,
            gpu_summary,
            storage_summary,
        })
    }

    pub fn get_system_details_data(&self) -> Result<SystemDetailsData, BlazeError> {
        let sys = SystemInfo::gather();
        let desktop_env = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_else(|_| "GNOME / BlazeOS Desktop".to_string());
        let session_type =
            std::env::var("XDG_SESSION_TYPE").unwrap_or_else(|_| "wayland".to_string());

        Ok(SystemDetailsData {
            os_name: sys.os_name,
            version_id: sys.version_id,
            edition_line: sys.edition_line,
            kernel: sys.kernel,
            hostname: sys.hostname,
            arch: sys.arch,
            desktop_env,
            session_type,
            uptime: sys.uptime,
        })
    }
}

impl Default for SystemService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_system_service() {
        let service = SystemService::new();
        let res = service.get_system_info();
        assert!(res.is_ok());

        let ov_res = service.get_overview_data();
        assert!(ov_res.is_ok());

        let dt_res = service.get_system_details_data();
        assert!(dt_res.is_ok());
        let dt = dt_res.unwrap();
        assert!(!dt.desktop_env.is_empty());
        assert!(!dt.session_type.is_empty());
    }
}
