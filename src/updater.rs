use crate::capabilities::run_cmd;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub update_available: bool,
    pub current_version: String,
    pub latest_version: String,
    pub release_notes: String,
    pub download_url: String,
}

pub async fn check_for_updates(current_version: &str) -> UpdateInfo {
    let mut info = UpdateInfo {
        update_available: false,
        current_version: current_version.to_string(),
        latest_version: current_version.to_string(),
        release_notes: String::new(),
        download_url: String::new(),
    };

    // Use curl CLI or http request to check latest release
    let url = "https://api.github.com/repos/Blaze-Galaxy/blaze-house/releases/latest";
    let (ok, out) = run_cmd(
        "curl",
        &[
            "-s",
            "--connect-timeout",
            "2",
            "--max-time",
            "4",
            "-H",
            "User-Agent: BlazeHouse-Updater",
            url,
        ],
    );

    if ok && !out.trim().is_empty() {
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&out) {
            if let Some(tag) = json.get("tag_name").and_then(|v| v.as_str()) {
                let latest = tag.trim_start_matches('v');
                info.latest_version = latest.to_string();
                if latest != current_version {
                    info.update_available = true;
                }

                if let Some(body) = json.get("body").and_then(|v| v.as_str()) {
                    info.release_notes = body.to_string();
                }

                if let Some(html_url) = json.get("html_url").and_then(|v| v.as_str()) {
                    info.download_url = html_url.to_string();
                }
            }
        }
    }

    info
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_check_for_updates() {
        let info = check_for_updates("2.0.0").await;
        println!("Update check result: {:?}", info);
        assert!(!info.current_version.is_empty());
    }
}
