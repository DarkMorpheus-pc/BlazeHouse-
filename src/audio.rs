use crate::capabilities::{has_cmd, run_cmd};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioDevice {
    pub id: String,
    pub name: String,
    pub is_default: bool,
    pub is_sink: bool,
    pub volume: Option<u32>,
    pub muted: bool,
}

pub fn server_name() -> String {
    if has_cmd("wpctl") {
        let (ok, out) = run_cmd("wpctl", &["status"]);
        if ok && out.contains("PipeWire") {
            return String::from("PipeWire");
        }
    }
    if has_cmd("pactl") {
        let (ok, out) = run_cmd("pactl", &["info"]);
        if ok {
            for line in out.lines() {
                if line.starts_with("Server Name:") {
                    return line
                        .split(':')
                        .nth(1)
                        .unwrap_or("PulseAudio")
                        .trim()
                        .to_string();
                }
            }
        }
        return String::from("PulseAudio");
    }
    String::from("Bilinmiyor")
}

pub fn outputs() -> Vec<AudioDevice> {
    if has_cmd("wpctl") {
        return parse_wpctl_devices(true);
    } else if has_cmd("pactl") {
        return parse_pactl_devices(true);
    }
    Vec::new()
}

pub fn inputs() -> Vec<AudioDevice> {
    if has_cmd("wpctl") {
        return parse_wpctl_devices(false);
    } else if has_cmd("pactl") {
        return parse_pactl_devices(false);
    }
    Vec::new()
}

pub fn output_volume() -> Option<u32> {
    if has_cmd("wpctl") {
        let (ok, out) = run_cmd("wpctl", &["get-volume", "@DEFAULT_AUDIO_SINK@"]);
        if ok {
            // "Volume: 0.65 [MUTED]" or "Volume: 0.65"
            let parts: Vec<&str> = out.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(val) = parts[1].parse::<f64>() {
                    return Some((val * 100.0).round() as u32);
                }
            }
        }
    } else if has_cmd("pactl") {
        let (ok, out) = run_cmd("pactl", &["get-sink-volume", "@DEFAULT_SINK@"]);
        if ok {
            if let Some(pos) = out.find('%') {
                let sub = &out[..pos];
                if let Some(num_str) = sub.split_whitespace().last() {
                    if let Ok(vol) = num_str.parse::<u32>() {
                        return Some(vol);
                    }
                }
            }
        }
    }
    None
}

pub fn set_output_volume(vol: u32) -> bool {
    let vol_clamped = vol.min(100);
    if has_cmd("wpctl") {
        let vol_f = vol_clamped as f64 / 100.0;
        let (ok, _) = run_cmd(
            "wpctl",
            &[
                "set-volume",
                "@DEFAULT_AUDIO_SINK@",
                &format!("{:.2}", vol_f),
            ],
        );
        ok
    } else if has_cmd("pactl") {
        let (ok, _) = run_cmd(
            "pactl",
            &[
                "set-sink-volume",
                "@DEFAULT_SINK@",
                &format!("{}%", vol_clamped),
            ],
        );
        ok
    } else {
        false
    }
}

pub fn muted() -> bool {
    if has_cmd("wpctl") {
        let (ok, out) = run_cmd("wpctl", &["get-volume", "@DEFAULT_AUDIO_SINK@"]);
        ok && out.contains("[MUTED]")
    } else if has_cmd("pactl") {
        let (ok, out) = run_cmd("pactl", &["get-sink-mute", "@DEFAULT_SINK@"]);
        ok && out.to_lowercase().contains("yes")
    } else {
        false
    }
}

pub fn set_muted(mute: bool) -> bool {
    if has_cmd("wpctl") {
        let flag = if mute { "1" } else { "0" };
        let (ok, _) = run_cmd("wpctl", &["set-mute", "@DEFAULT_AUDIO_SINK@", flag]);
        ok
    } else if has_cmd("pactl") {
        let flag = if mute { "1" } else { "0" };
        let (ok, _) = run_cmd("pactl", &["set-sink-mute", "@DEFAULT_SINK@", flag]);
        ok
    } else {
        false
    }
}

pub fn input_volume() -> Option<u32> {
    if has_cmd("wpctl") {
        let (ok, out) = run_cmd("wpctl", &["get-volume", "@DEFAULT_AUDIO_SOURCE@"]);
        if ok {
            let parts: Vec<&str> = out.split_whitespace().collect();
            if parts.len() >= 2 {
                if let Ok(val) = parts[1].parse::<f64>() {
                    return Some((val * 100.0).round() as u32);
                }
            }
        }
    } else if has_cmd("pactl") {
        let (ok, out) = run_cmd("pactl", &["get-source-volume", "@DEFAULT_SOURCE@"]);
        if ok {
            if let Some(pos) = out.find('%') {
                let sub = &out[..pos];
                if let Some(num_str) = sub.split_whitespace().last() {
                    if let Ok(vol) = num_str.parse::<u32>() {
                        return Some(vol);
                    }
                }
            }
        }
    }
    None
}

pub fn set_input_volume(vol: u32) -> bool {
    let vol_clamped = vol.min(100);
    if has_cmd("wpctl") {
        let vol_f = vol_clamped as f64 / 100.0;
        let (ok, _) = run_cmd(
            "wpctl",
            &[
                "set-volume",
                "@DEFAULT_AUDIO_SOURCE@",
                &format!("{:.2}", vol_f),
            ],
        );
        ok
    } else if has_cmd("pactl") {
        let (ok, _) = run_cmd(
            "pactl",
            &[
                "set-source-volume",
                "@DEFAULT_SOURCE@",
                &format!("{}%", vol_clamped),
            ],
        );
        ok
    } else {
        false
    }
}

pub fn set_default_output(id_or_name: &str) -> bool {
    if has_cmd("wpctl") {
        let (ok, _) = run_cmd("wpctl", &["set-default", id_or_name]);
        ok
    } else if has_cmd("pactl") {
        let (ok, _) = run_cmd("pactl", &["set-default-sink", id_or_name]);
        ok
    } else {
        false
    }
}

pub fn set_default_input(id_or_name: &str) -> bool {
    if has_cmd("wpctl") {
        let (ok, _) = run_cmd("wpctl", &["set-default", id_or_name]);
        ok
    } else if has_cmd("pactl") {
        let (ok, _) = run_cmd("pactl", &["set-default-source", id_or_name]);
        ok
    } else {
        false
    }
}

pub fn parse_wpctl_devices(is_sink: bool) -> Vec<AudioDevice> {
    let (ok, out) = run_cmd("wpctl", &["status"]);
    if !ok {
        return Vec::new();
    }
    parse_wpctl_devices_from_str(&out, is_sink)
}

pub fn parse_wpctl_devices_from_str(out: &str, is_sink: bool) -> Vec<AudioDevice> {
    let mut devices = Vec::new();
    let mut in_section = false;
    let target_section = if is_sink { "Sinks:" } else { "Sources:" };

    for line in out.lines() {
        if line.contains(target_section) {
            in_section = true;
            continue;
        }
        let cleaned_for_header = line
            .trim_start_matches(|c: char| {
                c.is_whitespace() || matches!(c, '│' | '├' | '─' | '└' | '*')
            })
            .trim();
        if in_section
            && (line.contains("Streams:")
                || line.contains("Filters:")
                || line.contains("Source endpoints:")
                || line.contains("Sink endpoints:")
                || cleaned_for_header.is_empty()
                || cleaned_for_header.ends_with(':'))
            && !line.contains(target_section)
        {
            in_section = false;
        }
        if in_section {
            let is_default = line.contains('*');
            let stripped = line.trim_start_matches(|c: char| {
                c.is_whitespace() || matches!(c, '│' | '├' | '─' | '└' | '*')
            });
            if let Some((id_part, name_part)) = stripped.split_once('.') {
                let id = id_part
                    .trim_matches(|c: char| {
                        c.is_whitespace() || matches!(c, '│' | '├' | '─' | '└' | '*')
                    })
                    .to_string();
                let name = name_part
                    .split('[')
                    .next()
                    .unwrap_or(name_part)
                    .trim()
                    .to_string();
                if !id.is_empty() && id.chars().all(|c| c.is_ascii_digit()) && !name.is_empty() {
                    devices.push(AudioDevice {
                        id,
                        name,
                        is_default,
                        is_sink,
                        volume: None,
                        muted: false,
                    });
                }
            }
        }
    }
    devices
}

fn parse_pactl_devices(is_sink: bool) -> Vec<AudioDevice> {
    let arg = if is_sink { "sinks" } else { "sources" };
    let (ok, out) = run_cmd("pactl", &["list", arg, "short"]);
    if !ok {
        return Vec::new();
    }

    let mut devices = Vec::new();
    for line in out.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 2 {
            let id = parts[0].to_string();
            let name = parts[1].to_string();
            devices.push(AudioDevice {
                id,
                name,
                is_default: false,
                is_sink,
                volume: None,
                muted: false,
            });
        }
    }
    devices
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_server_name() {
        let s = server_name();
        println!("Audio Server Name: {}", s);
        assert!(!s.is_empty());
    }

    #[test]
    fn test_audio_devices() {
        let outs = outputs();
        let ins = inputs();
        println!("Audio Outputs: {:?}", outs);
        println!("Audio Inputs: {:?}", ins);
    }

    #[test]
    fn test_parse_wpctl_devices() {
        let sample = "\
Audio
 ├─ Devices:
 │      42. Built-in Audio                      [alsa]
 │  
 ├─ Sinks:
 │  *   52. Built-in Audio Analog Stereo        [vol: 0.65]
 │      58. HDMI / DisplayPort 1 Output         [vol: 1.00 MUTED]
 │  
 ├─ Sink endpoints:
 │  
 ├─ Sources:
 │  *   53. Built-in Audio Analog Stereo        [vol: 0.80]
 │  
 ├─ Source endpoints:
 │  
 └─ Streams:
";
        let sinks = parse_wpctl_devices_from_str(sample, true);
        assert_eq!(sinks.len(), 2);
        assert_eq!(sinks[0].id, "52");
        assert_eq!(sinks[0].name, "Built-in Audio Analog Stereo");
        assert!(sinks[0].is_default);
        assert_eq!(sinks[1].id, "58");
        assert_eq!(sinks[1].name, "HDMI / DisplayPort 1 Output");
        assert!(!sinks[1].is_default);

        let sources = parse_wpctl_devices_from_str(sample, false);
        assert_eq!(sources.len(), 1);
        assert_eq!(sources[0].id, "53");
        assert_eq!(sources[0].name, "Built-in Audio Analog Stereo");
        assert!(sources[0].is_default);
    }
}
