use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::hardware::{HardwareData, HardwareService};
use crate::ui::components::badge::{Badge, BadgeStyle};
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::components::metric_card::MetricCard;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

#[derive(Debug, Clone)]
pub enum HardwareMessage {
    LoadHardware,
    HardwareLoaded(Result<HardwareData, BlazeError>),
}

#[derive(Debug, Clone)]
pub enum HardwareViewState {
    Loading,
    Loaded(HardwareData),
    Error(BlazeError),
}

pub struct HardwarePage {
    pub state: HardwareViewState,
    pub is_refreshing: bool,
}

impl HardwarePage {
    pub fn new() -> (Self, Task<HardwareMessage>) {
        let page = Self {
            state: HardwareViewState::Loading,
            is_refreshing: true,
        };
        let task = Task::perform(
            async move {
                let service = HardwareService::new();
                service.get_hardware_data()
            },
            HardwareMessage::HardwareLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: HardwareMessage) -> Task<HardwareMessage> {
        match message {
            HardwareMessage::LoadHardware => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = HardwareService::new();
                        service.get_hardware_data()
                    },
                    HardwareMessage::HardwareLoaded,
                )
            }
            HardwareMessage::HardwareLoaded(res) => {
                self.is_refreshing = false;
                match res {
                    Ok(data) => {
                        self.state = HardwareViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = HardwareViewState::Error(e);
                    }
                }
                Task::none()
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, HardwareMessage> {
        match self.state {
            HardwareViewState::Loading => {
                LoadingState::build("Fiziksel donanım envanteri taranıyor...", theme)
            }
            HardwareViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(HardwareMessage::LoadHardware),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            HardwareViewState::Loaded(ref data) => {
                let sub_title =
                    text("Fiziksel Donanım Bileşenleri, Ekran Çıktıları ve Aygıt Envanteri")
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
                        Some(HardwareMessage::LoadHardware)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                // Top Metric Cards Summary (CPU model/count, Memory, GPU count, Disks count)
                let cpu_val = data
                    .inventory
                    .cpu
                    .first()
                    .map(|i| i.value.as_str())
                    .unwrap_or("Tanımsız CPU");
                let ram_val = data
                    .inventory
                    .memory
                    .first()
                    .map(|i| i.value.as_str())
                    .unwrap_or("Tanımsız RAM");
                let gpu_val = data
                    .inventory
                    .graphics
                    .first()
                    .map(|i| i.value.as_str())
                    .unwrap_or("Standart GPU");

                let metric_cpu =
                    MetricCard::build("İŞLEMCİ (CPU)", cpu_val, "Ana İşlemci Birimi", true, theme);

                let metric_ram = MetricCard::build(
                    "SİSTEM BELLEĞİ (RAM)",
                    ram_val,
                    "Fiziksel Bellek Kapasitesi",
                    false,
                    theme,
                );

                let metric_gpu = MetricCard::build(
                    "GRAFİK KARTI (GPU)",
                    gpu_val,
                    "Grafik Yonga Seti",
                    false,
                    theme,
                );

                let metric_grid = row![metric_cpu, metric_ram, metric_gpu]
                    .spacing(SpacingTokens::LG)
                    .width(Length::Fill);

                let mut page_col: Column<'a, HardwareMessage> = column![top_bar, metric_grid]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. CPU Section
                let mut cpu_col: Column<'a, HardwareMessage> = column![].spacing(SpacingTokens::SM);
                for (idx, item) in data.inventory.cpu.iter().enumerate() {
                    if idx > 0 {
                        cpu_col = cpu_col.push(Divider::build(theme));
                    }
                    cpu_col = cpu_col.push(InfoRow::build(&item.label, &item.value, false, theme));
                }
                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "İŞLEMCİ (CPU)",
                    "İşlemci Modeli, Çekirdek Yapısı ve Donanım Frekansı",
                    cpu_col.into(),
                    theme,
                ));

                // 2. Memory Section
                let mut mem_col: Column<'a, HardwareMessage> = column![].spacing(SpacingTokens::SM);
                for (idx, item) in data.inventory.memory.iter().enumerate() {
                    if idx > 0 {
                        mem_col = mem_col.push(Divider::build(theme));
                    }
                    mem_col = mem_col.push(InfoRow::build(&item.label, &item.value, true, theme));
                }
                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "SİSTEM BELLEĞİ (RAM)",
                    "Fiziksel RAM Kapasitesi ve Bellek Modül Bilgileri",
                    mem_col.into(),
                    theme,
                ));

                // 3. GPU Section
                let mut gpu_col: Column<'a, HardwareMessage> = column![].spacing(SpacingTokens::SM);
                for (idx, item) in data.inventory.graphics.iter().enumerate() {
                    if idx > 0 {
                        gpu_col = gpu_col.push(Divider::build(theme));
                    }
                    gpu_col = gpu_col.push(InfoRow::build(&item.label, &item.value, false, theme));
                }
                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "GRAFİK KARTI (GPU)",
                    "Grafik İşlemci Yonga Seti, Üretici ve PCI Yolu",
                    gpu_col.into(),
                    theme,
                ));

                // 4. Displays Section (Section-level error handling)
                let mut disp_col: Column<'a, HardwareMessage> =
                    column![].spacing(SpacingTokens::SM);
                if let Some(ref err_msg) = data.displays_error {
                    disp_col = disp_col.push(InfoRow::build(
                        "Ekran Durumu",
                        &format!("Ekran bilgisi alınamadı ({})", err_msg),
                        false,
                        theme,
                    ));
                } else if data.displays.is_empty() {
                    disp_col = disp_col.push(InfoRow::build(
                        "Görüntü Çıkışı",
                        "Bağlı Harici Ekran veya Görüntü Sunucusu Bulunamadı",
                        false,
                        theme,
                    ));
                } else {
                    for (idx, disp) in data.displays.iter().enumerate() {
                        if idx > 0 {
                            disp_col = disp_col.push(Divider::build(theme));
                        }
                        let mode_info = if disp.current.is_empty() {
                            "Varsayılan Çözünürlük".to_string()
                        } else {
                            disp.current.clone()
                        };
                        let disp_desc = format!(
                            "{} (Ölçek: {:.2}x, Birincil: {})",
                            mode_info,
                            disp.scale,
                            if disp.primary { "Evet" } else { "Hayır" }
                        );

                        let primary_badge = if disp.primary {
                            Badge::build("PRIMARY", BadgeStyle::Success, theme)
                        } else {
                            Badge::build("CONNECTED", BadgeStyle::Neutral, theme)
                        };

                        let disp_row = row![
                            InfoRow::build(&disp.name, &disp_desc, true, theme),
                            iced::widget::horizontal_space(),
                            primary_badge
                        ]
                        .align_y(Alignment::Center)
                        .width(Length::Fill);

                        disp_col = disp_col.push(disp_row);
                    }
                }
                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "EKRAN VE GÖRÜNTÜ ÇIKIŞLARI",
                    "Aktif Bağlı Monitörler, Çözünürlük ve Ölçekleme Parametreleri",
                    disp_col.into(),
                    theme,
                ));

                // 5. Storage Devices Section
                let mut storage_col: Column<'a, HardwareMessage> =
                    column![].spacing(SpacingTokens::SM);
                for (idx, item) in data.inventory.storage.iter().enumerate() {
                    if idx > 0 {
                        storage_col = storage_col.push(Divider::build(theme));
                    }
                    storage_col =
                        storage_col.push(InfoRow::build(&item.label, &item.value, true, theme));
                }
                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "FİZİKSEL DEPOLAMA BİRİMLERİ",
                    "Sabit Diskler, NVMe / SSD Sürücüler ve Bölümler",
                    storage_col.into(),
                    theme,
                ));

                // 6. Network Hardware Section
                let mut net_col: Column<'a, HardwareMessage> = column![].spacing(SpacingTokens::SM);
                for (idx, item) in data.inventory.network.iter().enumerate() {
                    if idx > 0 {
                        net_col = net_col.push(Divider::build(theme));
                    }
                    net_col = net_col.push(InfoRow::build(&item.label, &item.value, true, theme));
                }
                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "AĞ DONANIMLARI VE KARTLAR",
                    "Kablolu / Kablosuz Ağ Adaptörleri ve MAC Adresleri",
                    net_col.into(),
                    theme,
                ));

                // 7. Board & DMI Section
                let mut board_col: Column<'a, HardwareMessage> =
                    column![].spacing(SpacingTokens::SM);
                for (idx, item) in data.inventory.board.iter().enumerate() {
                    if idx > 0 {
                        board_col = board_col.push(Divider::build(theme));
                    }
                    board_col =
                        board_col.push(InfoRow::build(&item.label, &item.value, false, theme));
                }
                page_col = page_col.push(GlassCard::build_with_subtitle(
                    "ANAKART VE DMI BİLGİLERİ",
                    "Anakart Üreticisi, Model ve BIOS Sürüm Numarası",
                    board_col.into(),
                    theme,
                ));

                page_col.into()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hardware_page_lifecycle() {
        let (mut page, _task) = HardwarePage::new();
        assert!(matches!(page.state, HardwareViewState::Loading));

        let service = HardwareService::new();
        let data = service.get_hardware_data().unwrap();
        let _ = page.update(HardwareMessage::HardwareLoaded(Ok(data.clone())));

        assert!(!page.is_refreshing);
        if let HardwareViewState::Loaded(ref d) = page.state {
            assert!(!d.inventory.cpu.is_empty());
        } else {
            panic!("Expected HardwareViewState::Loaded");
        }
    }

    #[test]
    fn test_hardware_page_error_and_refresh() {
        let (mut page, _task) = HardwarePage::new();
        let err = BlazeError::ExecutionError {
            command: "test".to_string(),
            stdout: "".to_string(),
            stderr: "Failed".to_string(),
        };
        let _ = page.update(HardwareMessage::HardwareLoaded(Err(err)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, HardwareViewState::Error(_)));

        let _ = page.update(HardwareMessage::LoadHardware);
        assert!(page.is_refreshing);
    }
}
