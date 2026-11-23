use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::storage::{StorageData, StorageService};
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

#[derive(Debug, Clone)]
pub enum StorageMessage {
    LoadStorage,
    StorageLoaded(Result<StorageData, BlazeError>),
}

#[derive(Debug, Clone)]
pub enum StorageViewState {
    Loading,
    Loaded(StorageData),
    Error(BlazeError),
}

pub struct StoragePage {
    pub state: StorageViewState,
    pub is_refreshing: bool,
}

impl StoragePage {
    pub fn new() -> (Self, Task<StorageMessage>) {
        let page = Self {
            state: StorageViewState::Loading,
            is_refreshing: true,
        };
        let task = Task::perform(
            async move {
                let service = StorageService::new();
                service.get_storage_data()
            },
            StorageMessage::StorageLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: StorageMessage) -> Task<StorageMessage> {
        match message {
            StorageMessage::LoadStorage => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = StorageService::new();
                        service.get_storage_data()
                    },
                    StorageMessage::StorageLoaded,
                )
            }
            StorageMessage::StorageLoaded(res) => {
                self.is_refreshing = false;
                match res {
                    Ok(data) => {
                        self.state = StorageViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = StorageViewState::Error(e);
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, StorageMessage> {
        match self.state {
            StorageViewState::Loading => {
                LoadingState::build("Depolama birimleri ve önbellek taranıyor...", theme)
            }
            StorageViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(StorageMessage::LoadStorage),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            StorageViewState::Loaded(ref data) => {
                let sub_title =
                    text("Sistem Depolama Birimleri, Mount Noktaları ve Önbellek Analiz Envanteri")
                        .size(TypographyTokens::CAPTION_SIZE)
                        .color(theme.colors.text_secondary);

                let refresh_btn = PillButton::build(
                    if self.is_refreshing {
                        "⏳ Yenileniyor..."
                    } else {
                        "🔄 Yenile"
                    },
                    ButtonVariant::Tonal,
                    if self.is_refreshing {
                        None
                    } else {
                        Some(StorageMessage::LoadStorage)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, StorageMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. Root Disk Summary
                let total_gb = data.root_usage.total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                let free_gb = data.root_usage.free_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                let used_gb = data.root_usage.used_bytes as f64 / (1024.0 * 1024.0 * 1024.0);

                let root_summary_str = format!(
                    "%{:.1} Kullanımda ({:.1} GB Boş / {:.1} GB Toplam)",
                    data.root_usage.percent, free_gb, total_gb
                );

                let root_col: Column<'a, StorageMessage> = column![
                    InfoRow::build("Bağlantı Noktası (Mount)", "/", true, theme),
                    Divider::build(theme),
                    InfoRow::build(
                        "Kullanılan Alan",
                        &format!("{:.2} GB", used_gb),
                        true,
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build("Boş Alan", &format!("{:.2} GB", free_gb), true, theme),
                    Divider::build(theme),
                    InfoRow::build("Kullanım Durumu", &root_summary_str, true, theme),
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "KÖK DİZİN DEPOLAMA ÖZETİ (/)",
                    root_col.into(),
                    theme,
                ));

                // 2. All Mounted Disks & Partitions
                let mut disks_col: Column<'a, StorageMessage> =
                    column![].spacing(SpacingTokens::SM);
                if data.all_disks.is_empty() {
                    disks_col = disks_col.push(InfoRow::build(
                        "Disk Envanteri",
                        "Bağlı Disk Bulunamadı",
                        false,
                        theme,
                    ));
                } else {
                    for (idx, disk) in data.all_disks.iter().enumerate() {
                        if idx > 0 {
                            disks_col = disks_col.push(Divider::build(theme));
                        }
                        let d_total_gb = disk.total_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                        let d_free_gb = disk.free_bytes as f64 / (1024.0 * 1024.0 * 1024.0);
                        let disk_desc = format!(
                            "Mount: {} | FS: {} | %{:.1} (%{:.1} GB Boş / {:.1} GB Toplam) | Çıkarılabilir: {}",
                            disk.mount_point,
                            if disk.file_system.is_empty() { "Bilinmiyor" } else { &disk.file_system },
                            disk.percent,
                            d_free_gb,
                            d_total_gb,
                            if disk.is_removable { "Evet" } else { "Hayır" }
                        );
                        let label = if disk.name.is_empty() {
                            "Fiziksel Bölüm"
                        } else {
                            &disk.name
                        };
                        disks_col = disks_col.push(InfoRow::build(label, &disk_desc, true, theme));
                    }
                }

                page_col = page_col.push(GlassCard::build(
                    "TÜM BAĞLI DİSKLER VE BÖLÜMLER",
                    disks_col.into(),
                    theme,
                ));

                // 3. Cache & Temp Storage Analysis
                let mut cache_col: Column<'a, StorageMessage> =
                    column![].spacing(SpacingTokens::SM);

                if let Some(ref err_msg) = data.caches_error {
                    cache_col = cache_col.push(InfoRow::build(
                        "Önbellek Durumu",
                        &format!("Önbellek analizi yapılamadı ({})", err_msg),
                        false,
                        theme,
                    ));
                } else if data.caches.is_empty() {
                    cache_col = cache_col.push(InfoRow::build(
                        "Önbellek Envanteri",
                        "Analiz edilecek önbellek veya geçici dosya bulunamadı",
                        false,
                        theme,
                    ));
                } else {
                    for (idx, item) in data.caches.iter().enumerate() {
                        if idx > 0 {
                            cache_col = cache_col.push(Divider::build(theme));
                        }
                        cache_col = cache_col.push(InfoRow::build(
                            &item.label,
                            &item.desc_str(),
                            true,
                            theme,
                        ));
                    }
                }

                page_col = page_col.push(GlassCard::build(
                    "ÖNBELLEK VE GEÇİCİ DOSYA ANALİZİ",
                    cache_col.into(),
                    theme,
                ));

                // Read-only notification footer
                let ro_text = text(
                    "ℹ️ Depolama yönetimi read-only moddadır; biçimlendirme ve bölümleme işlemleri kısıtlanmıştır.",
                )
                .size(TypographyTokens::CAPTION_SIZE)
                .color(theme.colors.text_disabled);

                page_col = page_col.push(ro_text);

                page_col.into()
            }
        }
    }
}

impl crate::storage::CacheItem {
    fn desc_str(&self) -> String {
        let size_mb = self.size_bytes as f64 / (1024.0 * 1024.0);
        if size_mb >= 1024.0 {
            format!("{:.2} GB — {}", size_mb / 1024.0, self.path)
        } else {
            format!("{:.2} MB — {}", size_mb, self.path)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_page_lifecycle() {
        let (mut page, _task) = StoragePage::new();
        assert!(matches!(page.state, StorageViewState::Loading));

        let service = StorageService::new();
        let data = service.get_storage_data().unwrap();
        let _ = page.update(StorageMessage::StorageLoaded(Ok(data.clone())));

        assert!(!page.is_refreshing);
        if let StorageViewState::Loaded(ref d) = page.state {
            assert!(d.root_usage.total_bytes > 0);
        } else {
            panic!("Expected StorageViewState::Loaded");
        }
    }
}
