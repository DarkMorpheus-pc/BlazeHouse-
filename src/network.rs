use crate::capabilities::{has_cmd, run_cmd};
use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WifiNetwork {
    pub ssid: String,
    pub signal: u32,
    pub security: String,
    pub active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ActiveConnection {
    pub name: String,
    pub conn_type: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkInterfaceInfo {
    pub name: String,
    pub if_type: String,
    pub state: String,
    pub connection: String,
    pub ip_address: String,
    pub gateway: String,
    pub dns: String,
}

pub fn wifi_enabled() -> bool {
    if has_cmd("nmcli") {
        let (ok, out) = run_cmd("nmcli", &["radio", "wifi"]);
        return ok && out.trim() == "enabled";
    }
    false
}

pub fn set_wifi_enabled(enabled: bool) -> bool {
    if has_cmd("nmcli") {
        let state = if enabled { "on" } else { "off" };
        let (ok, _) = run_cmd("nmcli", &["radio", "wifi", state]);
        return ok;
    }
    false
}

pub fn wifi_networks() -> Vec<WifiNetwork> {
    if !wifi_enabled() {
        return Vec::new();
    }

    let (ok, out) = run_cmd(
        "nmcli",
        &[
            "-t",
            "-f",
            "SSID,SIGNAL,SECURITY,ACTIVE",
            "device",
            "wifi",
            "list",
        ],
    );
    if !ok || out.trim().is_empty() {
        return Vec::new();
    }

    let mut networks = Vec::new();
    for line in out.lines() {
        let parts: Vec<&str> = line.split(':').collect();
        if parts.len() >= 4 {
            let ssid = parts[0].trim().to_string();
            if ssid.is_empty() {
                continue;
            }
            let signal = parts[1].trim().parse::<u32>().unwrap_or(0);
            let security = if parts[2].trim().is_empty() {
                "Açık".to_string()
            } else {
                parts[2].trim().to_string()
            };
            let active = parts[3].trim().to_lowercase() == "yes";

            networks.push(WifiNetwork {
                ssid,
                signal,
                security,
                active,
            });
        }
    }
    networks
}

pub fn connect_wifi(ssid: &str) -> bool {
    if has_cmd("nmcli") {
        let (ok, _) = run_cmd("nmcli", &["device", "wifi", "connect", "--", ssid]);
        return ok;
    }
    false
}

pub fn disconnect_wifi(ssid: &str) -> bool {
    if has_cmd("nmcli") {
        let (ok, _) = run_cmd("nmcli", &["connection", "down", "id", "--", ssid]);
        return ok;
    }
    false
}

pub fn parse_resolv_conf(content: &str) -> Vec<String> {
    let mut servers = Vec::new();
    for line in content.lines() {
        let line_no_comment = line.split(['#', ';']).next().unwrap_or("").trim();
        if let Some(rest) = line_no_comment.strip_prefix("nameserver") {
            let ns = rest.trim();
            if !ns.is_empty() && !servers.contains(&ns.to_string()) {
                servers.push(ns.to_string());
            }
        }
    }
    servers
}

pub fn get_dns_servers() -> Vec<String> {
    if let Ok(content) = fs::read_to_string("/etc/resolv.conf") {
        let servers = parse_resolv_conf(&content);
        if !servers.is_empty() {
            return servers;
        }
    }
    Vec::new()
}

pub fn active_connections() -> Vec<ActiveConnection> {
    if has_cmd("nmcli") {
        let (ok, out) = run_cmd(
            "nmcli",
            &["-t", "-f", "NAME,TYPE", "connection", "show", "--active"],
        );
        if ok {
            let mut conns = Vec::new();
            for line in out.lines() {
                if let Some((name, ctype)) = line.split_once(':') {
                    conns.push(ActiveConnection {
                        name: name.trim().to_string(),
                        conn_type: ctype.trim().to_string(),
                    });
                }
            }
            return conns;
        }
    }
    Vec::new()
}

pub fn network_interfaces() -> Vec<NetworkInterfaceInfo> {
    if !has_cmd("nmcli") {
        return Vec::new();
    }

    let (ok, out) = run_cmd("nmcli", &["device", "show"]);
    if !ok || out.trim().is_empty() {
        return Vec::new();
    }

    let mut interfaces = Vec::new();
    let mut current_name = String::new();
    let mut current_type = String::new();
    let mut current_state = String::new();
    let mut current_conn = String::new();
    let mut current_ip = String::new();
    let mut current_gw = String::new();
    let mut current_dns = String::new();

    let fallback_dns = get_dns_servers().join(", ");

    for line in out.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            if !current_name.is_empty() {
                interfaces.push(NetworkInterfaceInfo {
                    name: current_name.clone(),
                    if_type: current_type.clone(),
                    state: current_state.clone(),
                    connection: if current_conn.is_empty() || current_conn == "--" {
                        "Bağlı Değil".to_string()
                    } else {
                        current_conn.clone()
                    },
                    ip_address: if current_ip.is_empty() || current_ip == "--" {
                        "Yok".to_string()
                    } else {
                        current_ip.clone()
                    },
                    gateway: if current_gw.is_empty() || current_gw == "--" {
                        "Yok".to_string()
                    } else {
                        current_gw.clone()
                    },
                    dns: if current_dns.is_empty() || current_dns == "--" {
                        if !fallback_dns.is_empty() {
                            fallback_dns.clone()
                        } else {
                            "Yok".to_string()
                        }
                    } else {
                        current_dns.clone()
                    },
                });
                current_name.clear();
                current_type.clear();
                current_state.clear();
                current_conn.clear();
                current_ip.clear();
                current_gw.clear();
                current_dns.clear();
            }
            continue;
        }

        if let Some((key, val)) = trimmed.split_once(':') {
            let key = key.trim();
            let val = val.trim().to_string();
            match key {
                "GENERAL.DEVICE" => current_name = val,
                "GENERAL.TYPE" => current_type = val,
                "GENERAL.STATE" => current_state = val,
                "GENERAL.CONNECTION" => current_conn = val,
                "IP4.ADDRESS[1]" if current_ip.is_empty() => current_ip = val,
                "IP4.GATEWAY" if current_gw.is_empty() => current_gw = val,
                "IP6.GATEWAY" if current_gw.is_empty() && val != "--" => current_gw = val,
                "IP4.DNS[1]" if current_dns.is_empty() => current_dns = val,
                "IP6.DNS[1]" if current_dns.is_empty() && val != "--" => current_dns = val,
                _ => {}
            }
        }
    }

    if !current_name.is_empty() {
        interfaces.push(NetworkInterfaceInfo {
            name: current_name,
            if_type: current_type,
            state: current_state,
            connection: if current_conn.is_empty() || current_conn == "--" {
                "Bağlı Değil".to_string()
            } else {
                current_conn
            },
            ip_address: if current_ip.is_empty() || current_ip == "--" {
                "Yok".to_string()
            } else {
                current_ip
            },
            gateway: if current_gw.is_empty() || current_gw == "--" {
                "Yok".to_string()
            } else {
                current_gw
            },
            dns: if current_dns.is_empty() || current_dns == "--" {
                if !fallback_dns.is_empty() {
                    fallback_dns
                } else {
                    "Yok".to_string()
                }
            } else {
                current_dns
            },
        });
    }

    interfaces
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_network_info() {
        println!("Wi-Fi enabled: {}", wifi_enabled());
        let conns = active_connections();
        println!("Active Connections: {:?}", conns);
        let ifs = network_interfaces();
        println!("Interfaces: {:?}", ifs);
    }
}
