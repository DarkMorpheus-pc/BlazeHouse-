pub struct MotionTokens;

impl MotionTokens {
    pub const DURATION_FAST_MS: u64 = 150;
    pub const DURATION_NORMAL_MS: u64 = 250;
    pub const EASING_CUBIC: &'static str = "cubic-bezier(0.16, 1, 0.3, 1)";
}

#[derive(Debug, Clone, Copy)]
pub struct MotionConfig {
    pub reduce_motion: bool,
}

impl MotionConfig {
    pub fn duration_ms(&self, default_ms: u64) -> u64 {
        if self.reduce_motion {
            0
        } else {
            default_ms
        }
    }
}
