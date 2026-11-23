#[derive(Debug, Clone)]
pub struct A11yInfo {
    pub label: String,
    pub tooltip: Option<String>,
    pub focusable: bool,
}

impl A11yInfo {
    pub fn new(label: &str, tooltip: Option<&str>) -> Self {
        Self {
            label: label.to_string(),
            tooltip: tooltip.map(|s| s.to_string()),
            focusable: true,
        }
    }
}
