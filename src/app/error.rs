use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum BlazeError {
    CapabilityMissing {
        command: String,
        hint: String,
    },
    PermissionDenied {
        action: String,
        details: String,
    },
    PolkitCancelled,
    ExecutionError {
        command: String,
        stdout: String,
        stderr: String,
    },
    IoError(String),
    NotFound(String),
    InvalidArg(String),
}

impl fmt::Display for BlazeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            BlazeError::CapabilityMissing { command, hint } => {
                write!(f, "Sistem yeteneği eksik ({}): {}", command, hint)
            }
            BlazeError::PermissionDenied { action, details } => {
                write!(f, "Yetki reddedildi ({}): {}", action, details)
            }
            BlazeError::PolkitCancelled => write!(f, "Yönetici şifre doğrulaması iptal edildi"),
            BlazeError::ExecutionError {
                command,
                stdout,
                stderr,
            } => {
                write!(
                    f,
                    "Komut çalıştırma hatası [{}]: {} {}",
                    command, stdout, stderr
                )
            }
            BlazeError::IoError(msg) => write!(f, "Giriş/Çıkış hatası: {}", msg),
            BlazeError::NotFound(msg) => write!(f, "Bulunamadı: {}", msg),
            BlazeError::InvalidArg(msg) => write!(f, "Geçersiz parametre: {}", msg),
        }
    }
}

impl std::error::Error for BlazeError {}
