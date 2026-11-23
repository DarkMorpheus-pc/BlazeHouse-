use iced::Color;

#[derive(Debug, Clone, Copy)]
pub struct ColorTokens {
    pub background: Color,
    pub surface: Color,
    pub surface_elevated: Color,
    pub surface_hover: Color,
    pub surface_active: Color,
    pub border: Color,
    pub border_subtle: Color,
    pub border_focus: Color,
    pub text_primary: Color,
    pub text_secondary: Color,
    pub text_muted: Color,
    pub text_disabled: Color,
    pub accent: Color,
    pub accent_hover: Color,
    pub accent_on: Color,
    pub success: Color,
    pub warning: Color,
    pub danger: Color,
    pub info: Color,
}

impl ColorTokens {
    pub fn dark() -> Self {
        Self {
            background: Color::from_rgb8(14, 16, 23),
            surface: Color::from_rgb8(20, 24, 34),
            surface_elevated: Color::from_rgb8(28, 33, 46),
            surface_hover: Color::from_rgb8(38, 45, 62),
            surface_active: Color::from_rgb8(48, 57, 78),
            border: Color::from_rgba(1.0, 1.0, 1.0, 0.08),
            border_subtle: Color::from_rgba(1.0, 1.0, 1.0, 0.05),
            border_focus: Color::from_rgb8(96, 165, 250),
            text_primary: Color::from_rgb8(249, 250, 251),
            text_secondary: Color::from_rgb8(156, 163, 175),
            text_muted: Color::from_rgb8(107, 114, 128),
            text_disabled: Color::from_rgb8(75, 85, 99),
            accent: Color::from_rgb8(96, 165, 250),
            accent_hover: Color::from_rgb8(59, 130, 246),
            accent_on: Color::from_rgb8(14, 16, 23),
            success: Color::from_rgb8(52, 211, 153),
            warning: Color::from_rgb8(251, 191, 36),
            danger: Color::from_rgb8(248, 113, 113),
            info: Color::from_rgb8(34, 211, 238),
        }
    }

    pub fn light() -> Self {
        Self {
            background: Color::from_rgb8(244, 245, 248),
            surface: Color::from_rgba(1.0, 1.0, 1.0, 0.75),
            surface_elevated: Color::from_rgba(1.0, 1.0, 1.0, 0.92),
            surface_hover: Color::from_rgba(0.95, 0.96, 0.98, 0.85),
            surface_active: Color::from_rgba(0.90, 0.92, 0.95, 0.95),
            border: Color::from_rgba(0.0, 0.0, 0.0, 0.08),
            border_subtle: Color::from_rgba(0.0, 0.0, 0.0, 0.04),
            border_focus: Color::from_rgb8(59, 130, 246),
            text_primary: Color::from_rgb8(17, 24, 39),
            text_secondary: Color::from_rgb8(75, 85, 99),
            text_muted: Color::from_rgb8(107, 114, 128),
            text_disabled: Color::from_rgb8(156, 163, 175),
            accent: Color::from_rgb8(59, 130, 246),
            accent_hover: Color::from_rgb8(37, 99, 235),
            accent_on: Color::from_rgb8(255, 255, 255),
            success: Color::from_rgb8(16, 185, 129),
            warning: Color::from_rgb8(245, 158, 11),
            danger: Color::from_rgb8(239, 68, 68),
            info: Color::from_rgb8(6, 182, 212),
        }
    }
}

pub struct SpacingTokens;

impl SpacingTokens {
    pub const XS: f32 = 4.0;
    pub const SM: f32 = 8.0;
    pub const MD: f32 = 12.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 20.0;
    pub const XXL: f32 = 24.0;
    pub const XXXL: f32 = 32.0;
    pub const XXXXL: f32 = 48.0;
}

pub struct RadiusTokens;

impl RadiusTokens {
    pub const SM: f32 = 6.0;
    pub const MD: f32 = 10.0;
    pub const LG: f32 = 16.0;
    pub const XL: f32 = 20.0;
    pub const FULL: f32 = 9999.0;
}

pub struct TypographyTokens;

impl TypographyTokens {
    pub const DISPLAY_SIZE: f32 = 28.0;
    pub const PAGE_TITLE_SIZE: f32 = 22.0;
    pub const HEADING_SIZE: f32 = 16.0;
    pub const BODY_PRIMARY_SIZE: f32 = 14.0;
    pub const BODY_MUTED_SIZE: f32 = 13.0;
    pub const CAPTION_SIZE: f32 = 11.0;
    pub const TECH_VAL_SIZE: f32 = 13.0;
    pub const TERMINAL_SIZE: f32 = 12.0;
}
