use crate::capabilities::{has_cmd, run_cmd};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedSettings {
    pub theme_mode: String,
    pub accent_color: String,
    pub language: String,
    pub reduce_motion: bool,
}

impl Default for SharedSettings {
    fn default() -> Self {
        Self {
            theme_mode: String::from("dark"),
            accent_color: String::from("#8A8F9C"),
            language: String::from("tr"),
            reduce_motion: false,
        }
    }
}

impl SharedSettings {
    pub fn config_path() -> PathBuf {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("/root"));
        PathBuf::from(home).join(".config/blazeos/shared.toml")
    }

    pub fn load() -> Self {
        let path = Self::config_path();
        if path.exists() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(settings) = toml::from_str::<SharedSettings>(&content) {
                    return settings;
                }
            }
        }
        let default_settings = Self::default();
        let _ = default_settings.save();
        default_settings
    }

    pub fn save(&self) -> Result<(), String> {
        let path = Self::config_path();
        if let Some(parent) = path.parent() {
            if let Err(e) = fs::create_dir_all(parent) {
                return Err(format!("Dizin oluşturulamadı: {}", e));
            }
        }

        let toml_str = match toml::to_string_pretty(self) {
            Ok(s) => s,
            Err(e) => return Err(format!("TOML dönüştürme hatası: {}", e)),
        };

        // Atomic write via tempfile in the same directory
        let parent_dir = path.parent().unwrap_or_else(|| Path::new("."));
        let temp_path = parent_dir.join(".shared.toml.tmp");

        if let Err(e) = fs::write(&temp_path, toml_str) {
            return Err(format!("Gelişigüzel dosya yazma hatası: {}", e));
        }

        if let Err(e) = fs::rename(&temp_path, &path) {
            let _ = fs::remove_file(&temp_path);
            return Err(format!("Atomik yeniden adlandırma hatası: {}", e));
        }

        self.sync_desktop_theme();

        Ok(())
    }

    pub fn sync_desktop_theme(&self) {
        let home = env::var("HOME").unwrap_or_else(|_| String::from("/root"));
        let home_path = PathBuf::from(&home);
        let solar_mode = match self.theme_mode.as_str() {
            "light" => "light",
            "system" | "auto" => "auto",
            _ => "dark",
        };

        // 1. Sync ~/.config/solarui/config.toml
        let solar_config_path = home_path.join(".config/solarui/config.toml");
        if let Some(parent) = solar_config_path.parent() {
            let _ = fs::create_dir_all(parent);
        }
        let mut doc: toml::Value = if solar_config_path.exists() {
            fs::read_to_string(&solar_config_path)
                .ok()
                .and_then(|s| toml::from_str(&s).ok())
                .unwrap_or_else(|| toml::Value::Table(toml::map::Map::new()))
        } else {
            toml::Value::Table(toml::map::Map::new())
        };
        if let Some(root_table) = doc.as_table_mut() {
            let theme_entry = root_table
                .entry("theme")
                .or_insert_with(|| toml::Value::Table(toml::map::Map::new()));
            if let Some(theme_table) = theme_entry.as_table_mut() {
                theme_table.insert(
                    "mode".to_string(),
                    toml::Value::String(solar_mode.to_string()),
                );
                theme_table.insert(
                    "accent_color".to_string(),
                    toml::Value::String(self.accent_color.clone()),
                );
            }
            if let Ok(serialized) = toml::to_string_pretty(&doc) {
                let _ = fs::write(&solar_config_path, serialized);
            }
        }

        // 2. Sync ~/.config/noctalia/settings.json
        let noctalia_path = home_path.join(".config/noctalia/settings.json");
        if noctalia_path.exists() || noctalia_path.parent().map(|p| p.exists()).unwrap_or(false) {
            let mut json_val: serde_json::Value = if noctalia_path.exists() {
                fs::read_to_string(&noctalia_path)
                    .ok()
                    .and_then(|s| serde_json::from_str(&s).ok())
                    .unwrap_or_else(|| serde_json::json!({}))
            } else {
                serde_json::json!({})
            };
            if let Some(obj) = json_val.as_object_mut() {
                obj.insert(
                    "darkMode".to_string(),
                    serde_json::Value::Bool(solar_mode != "light"),
                );
                obj.insert(
                    "themeMode".to_string(),
                    serde_json::Value::String(solar_mode.to_string()),
                );
                obj.insert(
                    "accentColor".to_string(),
                    serde_json::Value::String(self.accent_color.clone()),
                );
                if let Ok(s) = serde_json::to_string_pretty(&json_val) {
                    let _ = fs::write(&noctalia_path, s);
                }
            }
        }

        // 3. Sync gsettings (GNOME / GTK / Cinnamon)
        if has_cmd("gsettings") {
            let color_scheme = match solar_mode {
                "light" => "prefer-light",
                "auto" => "default",
                _ => "prefer-dark",
            };
            let _ = run_cmd(
                "gsettings",
                &[
                    "set",
                    "org.gnome.desktop.interface",
                    "color-scheme",
                    color_scheme,
                ],
            );
            let anim = if self.reduce_motion { "false" } else { "true" };
            let _ = run_cmd(
                "gsettings",
                &["set", "org.gnome.desktop.interface", "enable-animations", anim],
            );
        }

        // 4. Sync KDE Plasma color scheme if available
        if has_cmd("plasma-apply-colorscheme") {
            let scheme = if solar_mode == "light" {
                "BreezeLight"
            } else {
                "BreezeDark"
            };
            let _ = run_cmd("plasma-apply-colorscheme", &[scheme]);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_shared_settings_load_save() {
        let mut settings = SharedSettings::load();
        let old_theme = settings.theme_mode.clone();
        settings.theme_mode = String::from("dark");
        assert!(settings.save().is_ok());

        let reloaded = SharedSettings::load();
        assert_eq!(reloaded.theme_mode, "dark");

        // Restore
        settings.theme_mode = old_theme;
        let _ = settings.save();
    }
}
