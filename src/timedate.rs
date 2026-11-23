use crate::capabilities::{has_cmd, run_cmd};
use std::collections::HashMap;

pub fn timedate_status() -> HashMap<String, String> {
    let mut map = HashMap::new();
    if has_cmd("timedatectl") {
        let (ok, out) = run_cmd("timedatectl", &["status"]);
        if ok {
            for line in out.lines() {
                if let Some((k, v)) = line.split_once(':') {
                    let key = k.trim().to_string();
                    let val = v.trim().to_string();
                    if key.contains("Time zone") || key.contains("Timezone") {
                        let tz = val.split_whitespace().next().unwrap_or("").to_string();
                        map.insert("Timezone".to_string(), tz);
                    } else if key.contains("NTP service") || key.contains("NTP") {
                        let ntp = if val.contains("active") || val.contains("yes") {
                            "yes"
                        } else {
                            "no"
                        };
                        map.insert("NTP".to_string(), ntp.to_string());
                    } else if key.contains("synchronized") {
                        let sync = if val.contains("yes") { "yes" } else { "no" };
                        map.insert("NTPSynchronized".to_string(), sync.to_string());
                    } else if key.contains("RTC in local TZ") {
                        let rtc = if val.contains("yes") { "yes" } else { "no" };
                        map.insert("LocalRTC".to_string(), rtc.to_string());
                    }
                }
            }
        }
    }
    map
}

pub fn set_ntp(enabled: bool) -> bool {
    if has_cmd("timedatectl") {
        let flag = if enabled { "true" } else { "false" };
        let (ok, _) = run_cmd("timedatectl", &["set-ntp", flag]);
        return ok;
    }
    false
}

pub fn timezones() -> Vec<String> {
    if has_cmd("timedatectl") {
        let (ok, out) = run_cmd("timedatectl", &["list-timezones"]);
        if ok {
            return out.lines().map(|s| s.trim().to_string()).collect();
        }
    }
    vec![
        "Europe/Istanbul".to_string(),
        "UTC".to_string(),
        "Europe/London".to_string(),
        "America/New_York".to_string(),
    ]
}

pub fn set_timezone(tz: &str) -> bool {
    if has_cmd("timedatectl") {
        let (ok, _) = run_cmd("timedatectl", &["set-timezone", tz]);
        return ok;
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_timedate_status() {
        let st = timedate_status();
        println!("Timedate status: {:?}", st);
    }
}
