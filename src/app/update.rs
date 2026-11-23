use crate::app::error::BlazeError;
use crate::updater::{check_for_updates, UpdateInfo};

pub struct UpdateService;

impl UpdateService {
    pub fn new() -> Self {
        Self
    }

    pub async fn check_for_updates(&self, current_version: &str) -> Result<UpdateInfo, BlazeError> {
        let info = check_for_updates(current_version).await;
        Ok(info)
    }
}

impl Default for UpdateService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_update_service() {
        let service = UpdateService::new();
        let res = service.check_for_updates("2.0.0").await;
        assert!(res.is_ok());
    }
}
