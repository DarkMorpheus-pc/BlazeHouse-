use iced::widget::{column, container, row, text, Column};
use iced::{Alignment, Element, Length, Task};

use crate::app::error::BlazeError;
use crate::app::input::InputService;
use crate::locale_input::InputDeviceInfo;
use crate::ui::components::button::{ButtonVariant, PillButton};
use crate::ui::components::divider::Divider;
use crate::ui::components::error_state::ErrorState;
use crate::ui::components::glass_card::GlassCard;
use crate::ui::components::info_row::InfoRow;
use crate::ui::components::loading_state::LoadingState;
use crate::ui::theme::Theme;
use crate::ui::tokens::{SpacingTokens, TypographyTokens};

#[derive(Debug, Clone)]
pub struct InputData {
    pub keyboard_layout: String,
    pub keyboard_model: String,
    pub available_layouts: Vec<&'static str>,
    pub mouse_speed: Option<f64>,
    pub mouse_natural_scroll: bool,
    pub touchpad_present: bool,
    pub touchpad_tap_to_click: bool,
    pub touchpad_natural_scroll: bool,
    pub devices: Vec<InputDeviceInfo>,
}

#[derive(Debug, Clone)]
pub enum InputMessage {
    LoadInput,
    InputLoaded(Result<Box<InputData>, BlazeError>),
    SetKeyboardLayout(String),
    SetMouseSpeed(f64),
    SetMouseNaturalScroll(bool),
    SetTouchpadTapToClick(bool),
    SetTouchpadNaturalScroll(bool),
    ActionFinished(Result<(), BlazeError>),
}

#[derive(Debug, Clone)]
pub enum InputViewState {
    Loading,
    Loaded(Box<InputData>),
    Error(BlazeError),
}

pub struct InputPage {
    pub state: InputViewState,
    pub is_refreshing: bool,
    pub is_acting: bool,
}

impl InputPage {
    pub fn new() -> (Self, Task<InputMessage>) {
        let page = Self {
            state: InputViewState::Loading,
            is_refreshing: true,
            is_acting: false,
        };
        let task = Task::perform(
            async move {
                let service = InputService::new();
                let (keyboard_layout, keyboard_model) = service
                    .get_keyboard_layout()
                    .unwrap_or(("tr".to_string(), "pc105".to_string()));
                let available_layouts = service.get_available_layouts();
                let mouse_speed = service.get_mouse_speed().unwrap_or(None);
                let mouse_natural_scroll = service.get_mouse_natural_scroll().unwrap_or(false);
                let touchpad_present = service.is_touchpad_present().unwrap_or(false);
                let touchpad_tap_to_click = service.get_touchpad_tap_to_click().unwrap_or(false);
                let touchpad_natural_scroll =
                    service.get_touchpad_natural_scroll().unwrap_or(false);
                let devices = service.get_input_devices().unwrap_or_default();

                Ok(Box::new(InputData {
                    keyboard_layout,
                    keyboard_model,
                    available_layouts,
                    mouse_speed,
                    mouse_natural_scroll,
                    touchpad_present,
                    touchpad_tap_to_click,
                    touchpad_natural_scroll,
                    devices,
                }))
            },
            InputMessage::InputLoaded,
        );
        (page, task)
    }

    pub fn update(&mut self, message: InputMessage) -> Task<InputMessage> {
        match message {
            InputMessage::LoadInput => {
                if self.is_refreshing {
                    return Task::none();
                }
                self.is_refreshing = true;
                Task::perform(
                    async move {
                        let service = InputService::new();
                        let (keyboard_layout, keyboard_model) = service.get_keyboard_layout()?;
                        let available_layouts = service.get_available_layouts();
                        let mouse_speed = service.get_mouse_speed()?;
                        let mouse_natural_scroll = service.get_mouse_natural_scroll()?;
                        let touchpad_present = service.is_touchpad_present()?;
                        let touchpad_tap_to_click = service.get_touchpad_tap_to_click()?;
                        let touchpad_natural_scroll = service.get_touchpad_natural_scroll()?;
                        let devices = service.get_input_devices()?;

                        Ok(Box::new(InputData {
                            keyboard_layout,
                            keyboard_model,
                            available_layouts,
                            mouse_speed,
                            mouse_natural_scroll,
                            touchpad_present,
                            touchpad_tap_to_click,
                            touchpad_natural_scroll,
                            devices,
                        }))
                    },
                    InputMessage::InputLoaded,
                )
            }
            InputMessage::InputLoaded(res) => {
                self.is_refreshing = false;
                self.is_acting = false;
                match res {
                    Ok(data) => {
                        self.state = InputViewState::Loaded(data);
                    }
                    Err(e) => {
                        self.state = InputViewState::Error(e);
                    }
                }
                Task::none()
            }
            InputMessage::SetKeyboardLayout(layout) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                let target = layout.clone();
                Task::perform(
                    async move {
                        let service = InputService::new();
                        service.set_keyboard_layout(&target)
                    },
                    InputMessage::ActionFinished,
                )
            }
            InputMessage::SetMouseSpeed(speed) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let service = InputService::new();
                        service.set_mouse_speed(speed)
                    },
                    InputMessage::ActionFinished,
                )
            }
            InputMessage::SetMouseNaturalScroll(enable) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let service = InputService::new();
                        service.set_mouse_natural_scroll(enable)
                    },
                    InputMessage::ActionFinished,
                )
            }
            InputMessage::SetTouchpadTapToClick(enable) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let service = InputService::new();
                        service.set_touchpad_tap_to_click(enable)
                    },
                    InputMessage::ActionFinished,
                )
            }
            InputMessage::SetTouchpadNaturalScroll(enable) => {
                if self.is_acting {
                    return Task::none();
                }
                self.is_acting = true;
                Task::perform(
                    async move {
                        let service = InputService::new();
                        service.set_touchpad_natural_scroll(enable)
                    },
                    InputMessage::ActionFinished,
                )
            }
            InputMessage::ActionFinished(res) => {
                self.is_acting = false;
                match res {
                    Ok(()) => {
                        // Re-read input state to verify actual Linux input system state
                        self.is_refreshing = true;
                        Task::perform(
                            async move {
                                let service = InputService::new();
                                let (keyboard_layout, keyboard_model) =
                                    service.get_keyboard_layout()?;
                                let available_layouts = service.get_available_layouts();
                                let mouse_speed = service.get_mouse_speed()?;
                                let mouse_natural_scroll = service.get_mouse_natural_scroll()?;
                                let touchpad_present = service.is_touchpad_present()?;
                                let touchpad_tap_to_click = service.get_touchpad_tap_to_click()?;
                                let touchpad_natural_scroll =
                                    service.get_touchpad_natural_scroll()?;
                                let devices = service.get_input_devices()?;

                                Ok(Box::new(InputData {
                                    keyboard_layout,
                                    keyboard_model,
                                    available_layouts,
                                    mouse_speed,
                                    mouse_natural_scroll,
                                    touchpad_present,
                                    touchpad_tap_to_click,
                                    touchpad_natural_scroll,
                                    devices,
                                }))
                            },
                            InputMessage::InputLoaded,
                        )
                    }
                    Err(e) => {
                        self.state = InputViewState::Error(e);
                        Task::none()
                    }
                }
            }
        }
    }

    pub fn view<'a>(&'a self, theme: &'a Theme) -> Element<'a, InputMessage> {
        match self.state {
            InputViewState::Loading => LoadingState::build(
                "Klavye, fare ve dokunmatik yüzey parametreleri taranıyor...",
                theme,
            ),
            InputViewState::Error(ref err) => {
                let retry_btn = PillButton::build(
                    "🔄 Yeniden Dene",
                    ButtonVariant::Primary,
                    Some(InputMessage::LoadInput),
                    theme,
                );
                let col = column![ErrorState::build(err, theme), retry_btn]
                    .spacing(SpacingTokens::LG)
                    .align_x(Alignment::Center);
                container(col).width(Length::Fill).into()
            }
            InputViewState::Loaded(ref data) => {
                let sub_title =
                    text("Giriş Aygıtları Envanteri, Klavye Düzeni ve Fare/Touchpad Ayarları")
                        .size(TypographyTokens::CAPTION_SIZE)
                        .color(theme.colors.text_secondary);

                let refresh_btn = PillButton::build(
                    if self.is_refreshing {
                        "⏳ Yenileniyor..."
                    } else {
                        "🔄 Yenile"
                    },
                    ButtonVariant::Tonal,
                    if self.is_refreshing || self.is_acting {
                        None
                    } else {
                        Some(InputMessage::LoadInput)
                    },
                    theme,
                );

                let top_bar = row![sub_title, iced::widget::horizontal_space(), refresh_btn]
                    .align_y(Alignment::Center)
                    .width(Length::Fill);

                let mut page_col: Column<'a, InputMessage> = column![top_bar]
                    .spacing(SpacingTokens::XL)
                    .width(Length::Fill);

                // 1. Keyboard Layout Selection Card
                let mut kbd_row = row![text("Klavye Düzeni:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center);

                let layout_labels: [(&str, &str); 6] = [
                    ("tr", "tr (Türkçe Q)"),
                    ("us", "us (English US)"),
                    ("de", "de (Deutsch)"),
                    ("fr", "fr (Français)"),
                    ("es", "es (Español)"),
                    ("gb", "gb (English UK)"),
                ];

                for (l_code, l_label) in layout_labels {
                    let is_sel = data.keyboard_layout == l_code;
                    let is_avail = data.available_layouts.contains(&l_code);
                    let l_str = l_code.to_string();

                    let btn = PillButton::build(
                        l_label,
                        if is_sel {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_acting || is_sel || !is_avail {
                            None
                        } else {
                            Some(InputMessage::SetKeyboardLayout(l_str))
                        },
                        theme,
                    );
                    kbd_row = kbd_row.push(btn);
                }

                let kbd_card_content: Column<'a, InputMessage> = column![
                    InfoRow::build(
                        "Aktif Klavye Düzeni (Layout)",
                        &data.keyboard_layout,
                        true,
                        theme
                    ),
                    Divider::build(theme),
                    InfoRow::build(
                        "Klavye Modeli",
                        if data.keyboard_model.is_empty() {
                            "Standart PC (pc105)"
                        } else {
                            &data.keyboard_model
                        },
                        false,
                        theme
                    ),
                    Divider::build(theme),
                    kbd_row
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "KLAVYE DÜZENİ VE DİL YAPILANDIRMASI",
                    kbd_card_content.into(),
                    theme,
                ));

                // 2. Mouse Settings Card
                let mouse_speed_str = data
                    .mouse_speed
                    .map(|s| format!("{:.2}", s))
                    .unwrap_or_else(|| "0.00 (Standart)".to_string());

                let mouse_nat_btn = PillButton::build(
                    if self.is_acting {
                        "⏳ İşleniyor..."
                    } else if data.mouse_natural_scroll {
                        "📜 Doğal Kaydırma (Etkin)"
                    } else {
                        "📜 Standart Kaydırma"
                    },
                    if data.mouse_natural_scroll {
                        ButtonVariant::Primary
                    } else {
                        ButtonVariant::Tonal
                    },
                    if self.is_acting {
                        None
                    } else {
                        Some(InputMessage::SetMouseNaturalScroll(
                            !data.mouse_natural_scroll,
                        ))
                    },
                    theme,
                );

                let mut speed_row = row![text("Fare Hızı:")
                    .size(TypographyTokens::BODY_PRIMARY_SIZE)
                    .color(theme.colors.text_primary)]
                .spacing(SpacingTokens::MD)
                .align_y(Alignment::Center);

                let speeds: [(f64, &str); 3] = [
                    (-0.5, "Yavaş (-0.5)"),
                    (0.0, "Normal (0.0)"),
                    (0.5, "Hızlı (+0.5)"),
                ];

                for (sp, label) in speeds {
                    let is_sel = data
                        .mouse_speed
                        .map(|curr| (curr - sp).abs() < 0.1)
                        .unwrap_or(sp == 0.0);
                    let btn = PillButton::build(
                        label,
                        if is_sel {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_acting || is_sel {
                            None
                        } else {
                            Some(InputMessage::SetMouseSpeed(sp))
                        },
                        theme,
                    );
                    speed_row = speed_row.push(btn);
                }

                let mouse_card_content: Column<'a, InputMessage> = column![
                    InfoRow::build("Imleç Hızı Değeri", &mouse_speed_str, true, theme),
                    Divider::build(theme),
                    speed_row,
                    Divider::build(theme),
                    row![
                        InfoRow::build(
                            "Doğal Kaydırma (Natural Scroll)",
                            if data.mouse_natural_scroll {
                                "Etkin"
                            } else {
                                "Devre Dışı"
                            },
                            data.mouse_natural_scroll,
                            theme
                        ),
                        iced::widget::horizontal_space(),
                        mouse_nat_btn
                    ]
                    .align_y(Alignment::Center)
                    .width(Length::Fill)
                ]
                .spacing(SpacingTokens::SM);

                page_col = page_col.push(GlassCard::build(
                    "FARE (MOUSE) İMLEÇ VE KAYDIRMA AYARLARI",
                    mouse_card_content.into(),
                    theme,
                ));

                // 3. Touchpad Settings Card (if present)
                if data.touchpad_present {
                    let tap_btn = PillButton::build(
                        if self.is_acting {
                            "⏳ İşleniyor..."
                        } else if data.touchpad_tap_to_click {
                            "👇 Dokunarak Tıklama (Etkin)"
                        } else {
                            "👇 Dokunarak Tıklama (Kapalı)"
                        },
                        if data.touchpad_tap_to_click {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_acting {
                            None
                        } else {
                            Some(InputMessage::SetTouchpadTapToClick(
                                !data.touchpad_tap_to_click,
                            ))
                        },
                        theme,
                    );

                    let tp_nat_btn = PillButton::build(
                        if self.is_acting {
                            "⏳ İşleniyor..."
                        } else if data.touchpad_natural_scroll {
                            "📜 Doğal Kaydırma (Etkin)"
                        } else {
                            "📜 Standart Kaydırma"
                        },
                        if data.touchpad_natural_scroll {
                            ButtonVariant::Primary
                        } else {
                            ButtonVariant::Tonal
                        },
                        if self.is_acting {
                            None
                        } else {
                            Some(InputMessage::SetTouchpadNaturalScroll(
                                !data.touchpad_natural_scroll,
                            ))
                        },
                        theme,
                    );

                    let tp_card_content: Column<'a, InputMessage> = column![
                        row![
                            InfoRow::build(
                                "Dokunarak Tıklama (Tap-to-click)",
                                if data.touchpad_tap_to_click {
                                    "Etkin"
                                } else {
                                    "Devre Dışı"
                                },
                                data.touchpad_tap_to_click,
                                theme
                            ),
                            iced::widget::horizontal_space(),
                            tap_btn
                        ]
                        .align_y(Alignment::Center)
                        .width(Length::Fill),
                        Divider::build(theme),
                        row![
                            InfoRow::build(
                                "Dokunmatik Yüzey Doğal Kaydırma",
                                if data.touchpad_natural_scroll {
                                    "Etkin"
                                } else {
                                    "Devre Dışı"
                                },
                                data.touchpad_natural_scroll,
                                theme
                            ),
                            iced::widget::horizontal_space(),
                            tp_nat_btn
                        ]
                        .align_y(Alignment::Center)
                        .width(Length::Fill)
                    ]
                    .spacing(SpacingTokens::SM);

                    page_col = page_col.push(GlassCard::build(
                        "DOKUNMATİK YÜZEY (TOUCHPAD) AYARLARI",
                        tp_card_content.into(),
                        theme,
                    ));
                }

                // 4. Hardware Input Devices Inventory Card
                let mut dev_col: Column<'a, InputMessage> = column![].spacing(SpacingTokens::SM);

                if data.devices.is_empty() {
                    dev_col = dev_col.push(InfoRow::build(
                        "Giriş Aygıtları",
                        "Sistemde algılanan hardware input aygıtı bulunamadı",
                        false,
                        theme,
                    ));
                } else {
                    for (idx, dev) in data.devices.iter().enumerate() {
                        if idx > 0 {
                            dev_col = dev_col.push(Divider::build(theme));
                        }
                        dev_col =
                            dev_col.push(InfoRow::build(&dev.name, &dev.dev_type, true, theme));
                    }
                }

                page_col = page_col.push(GlassCard::build(
                    "DONANIM GİRİŞ AYGITLARI ENVANTERİ",
                    dev_col.into(),
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
    fn test_input_page_lifecycle() {
        let (mut page, _task) = InputPage::new();
        assert!(matches!(page.state, InputViewState::Loading));

        let service = InputService::new();
        let (keyboard_layout, keyboard_model) = service
            .get_keyboard_layout()
            .unwrap_or(("tr".to_string(), "pc105".to_string()));
        let available_layouts = service.get_available_layouts();
        let mouse_speed = service.get_mouse_speed().unwrap_or(None);
        let mouse_natural_scroll = service.get_mouse_natural_scroll().unwrap_or(false);
        let touchpad_present = service.is_touchpad_present().unwrap_or(false);
        let touchpad_tap_to_click = service.get_touchpad_tap_to_click().unwrap_or(false);
        let touchpad_natural_scroll = service.get_touchpad_natural_scroll().unwrap_or(false);
        let devices = service.get_input_devices().unwrap_or_default();

        let data = Box::new(InputData {
            keyboard_layout,
            keyboard_model,
            available_layouts,
            mouse_speed,
            mouse_natural_scroll,
            touchpad_present,
            touchpad_tap_to_click,
            touchpad_natural_scroll,
            devices,
        });

        let _ = page.update(InputMessage::InputLoaded(Ok(data)));
        assert!(!page.is_refreshing);
        assert!(matches!(page.state, InputViewState::Loaded(_)));
    }
}
