use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum NavCategory {
    Overview,
    Hardware,
    Connectivity,
    Personalization,
    Performance,
    SecurityAndSystem,
}

impl NavCategory {
    pub fn title(&self) -> &'static str {
        match self {
            NavCategory::Overview => "Genel Bakış & Sistem",
            NavCategory::Hardware => "Donanım & Depolama",
            NavCategory::Connectivity => "Bağlantılar & Ses",
            NavCategory::Personalization => "Giriş & Kişiselleştirme",
            NavCategory::Performance => "Performans & Tuning",
            NavCategory::SecurityAndSystem => "Güvenlik & Sistem",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Route {
    General,
    SystemDetails,
    Displays,
    Storage,
    Network,
    Bluetooth,
    Sound,
    Input,
    Customize,
    LanguageRegion,
    Optimize,
    Zram,
    Battery,
    Security,
    Users,
    Accessibility,
    Notifications,
    About,
}

impl Route {
    pub fn title(&self) -> &'static str {
        match self {
            Route::General => "Genel Bakış",
            Route::SystemDetails => "Sistem Detayları",
            Route::Displays => "Ekranlar",
            Route::Storage => "Depolama",
            Route::Network => "Ağ & Wi-Fi",
            Route::Bluetooth => "Bluetooth",
            Route::Sound => "Ses",
            Route::Input => "Klavye & Fare",
            Route::Customize => "Görünüm & Tema",
            Route::LanguageRegion => "Dil & Bölge",
            Route::Optimize => "Optimizasyon & Temizlik",
            Route::Zram => "ZRAM Takas Belleği",
            Route::Battery => "Pil & Güç Profili",
            Route::Security => "Güvenlik & Gizlilik",
            Route::Users => "Kullanıcılar & Oturum",
            Route::Accessibility => "Erişilebilirlik",
            Route::Notifications => "Bildirimler",
            Route::About => "Hakkında & Güncelleme",
        }
    }

    pub fn category(&self) -> NavCategory {
        match self {
            Route::General | Route::SystemDetails => NavCategory::Overview,
            Route::Displays | Route::Storage => NavCategory::Hardware,
            Route::Network | Route::Bluetooth | Route::Sound => NavCategory::Connectivity,
            Route::Input | Route::Customize | Route::LanguageRegion => NavCategory::Personalization,
            Route::Optimize | Route::Zram | Route::Battery => NavCategory::Performance,
            Route::Security
            | Route::Users
            | Route::Accessibility
            | Route::Notifications
            | Route::About => NavCategory::SecurityAndSystem,
        }
    }

    pub fn icon_name(&self) -> &'static str {
        match self {
            Route::General => "⌂",
            Route::SystemDetails => "ℹ",
            Route::Displays => "🖥",
            Route::Storage => "🖴",
            Route::Network => "🌐",
            Route::Bluetooth => "ᛒ",
            Route::Sound => "🔊",
            Route::Input => "⌨",
            Route::Customize => "🎨",
            Route::LanguageRegion => "🌍",
            Route::Optimize => "⚡",
            Route::Zram => "⚙",
            Route::Battery => "🔋",
            Route::Security => "🛡",
            Route::Users => "👤",
            Route::Accessibility => "♿",
            Route::Notifications => "🔔",
            Route::About => "✦",
        }
    }

    pub fn all_routes() -> &'static [Route] {
        &[
            Route::General,
            Route::SystemDetails,
            Route::Displays,
            Route::Storage,
            Route::Network,
            Route::Bluetooth,
            Route::Sound,
            Route::Input,
            Route::Customize,
            Route::LanguageRegion,
            Route::Optimize,
            Route::Zram,
            Route::Battery,
            Route::Security,
            Route::Users,
            Route::Accessibility,
            Route::Notifications,
            Route::About,
        ]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_routes_count() {
        assert_eq!(Route::all_routes().len(), 18);
    }
}
