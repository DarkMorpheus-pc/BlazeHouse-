use crate::app::error::BlazeError;
use crate::storage::{
    analyze_caches, clean_cache_item, disk_usage, list_all_disks, CacheItem, DiskDetailsInfo,
    DiskUsageInfo,
};

#[derive(Debug, Clone)]
pub struct StorageData {
    pub root_usage: DiskUsageInfo,
    pub all_disks: Vec<DiskDetailsInfo>,
    pub caches: Vec<CacheItem>,
    pub caches_error: Option<String>,
}

pub struct StorageService;

impl StorageService {
    pub fn new() -> Self {
        Self
    }

    pub fn get_disk_usage(&self, path: &str) -> Result<DiskUsageInfo, BlazeError> {
        let usage = disk_usage(path);
        Ok(usage)
    }

    pub fn get_all_disks(&self) -> Result<Vec<DiskDetailsInfo>, BlazeError> {
        let disks = list_all_disks();
        Ok(disks)
    }

    pub fn analyze_caches(&self) -> Result<Vec<CacheItem>, BlazeError> {
        let items = analyze_caches();
        Ok(items)
    }

    pub fn clean_cache_item(&self, key: &str) -> Result<(), BlazeError> {
        let (ok, msg) = clean_cache_item(key);
        if ok {
            Ok(())
        } else if msg.contains("dismissed") || msg.contains("cancel") {
            Err(BlazeError::PolkitCancelled)
        } else {
            Err(BlazeError::ExecutionError {
                command: format!("clean_cache_item({})", key),
                stdout: String::new(),
                stderr: msg,
            })
        }
    }

    pub fn get_storage_data(&self) -> Result<StorageData, BlazeError> {
        let root_usage = disk_usage("/");
        let all_disks = list_all_disks();
        let (caches, caches_error) = match self.analyze_caches() {
            Ok(c) => (c, None),
            Err(e) => (Vec::new(), Some(e.to_string())),
        };

        Ok(StorageData {
            root_usage,
            all_disks,
            caches,
            caches_error,
        })
    }
}

impl Default for StorageService {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_storage_service() {
        let service = StorageService::new();
        let usage = service.get_disk_usage("/");
        assert!(usage.is_ok());
        let disks = service.get_all_disks();
        assert!(disks.is_ok());
        let caches = service.analyze_caches();
        assert!(caches.is_ok());
        let data = service.get_storage_data();
        assert!(data.is_ok());
    }
}
