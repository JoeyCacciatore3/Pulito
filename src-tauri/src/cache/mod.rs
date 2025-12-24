use std::collections::HashMap;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};

/// Cache entry with TTL (Time To Live)
#[derive(Debug, Clone)]
struct CacheEntry<T> {
    value: T,
    expires_at: Instant,
}

/// TTL-based cache for expensive operations
#[derive(Clone)]
pub struct CacheManager {
    // Directory size cache: path -> (size, expires_at)
    dir_sizes: Arc<RwLock<HashMap<PathBuf, CacheEntry<u64>>>>,
    // Scan results cache: scan_key -> (results, expires_at)
    scan_results: Arc<RwLock<HashMap<String, CacheEntry<ScanCacheEntry>>>>,
    // Default TTL for directory sizes (5 minutes)
    dir_size_ttl: Duration,
    // Default TTL for scan results (10 minutes)
    scan_result_ttl: Duration,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanCacheEntry {
    pub total_size: u64,
    pub total_items: usize,
    pub timestamp: String,
}

impl CacheManager {
    /// Create a new cache manager with default TTLs
    pub fn new() -> Self {
        Self {
            dir_sizes: Arc::new(RwLock::new(HashMap::new())),
            scan_results: Arc::new(RwLock::new(HashMap::new())),
            dir_size_ttl: Duration::from_secs(300), // 5 minutes
            scan_result_ttl: Duration::from_secs(600), // 10 minutes
        }
    }

    /// Create a cache manager with custom TTLs
    pub fn with_ttls(dir_size_ttl: Duration, scan_result_ttl: Duration) -> Self {
        Self {
            dir_sizes: Arc::new(RwLock::new(HashMap::new())),
            scan_results: Arc::new(RwLock::new(HashMap::new())),
            dir_size_ttl,
            scan_result_ttl,
        }
    }

    /// Get cached directory size, or None if not cached or expired
    pub async fn get_dir_size(&self, path: &PathBuf) -> Option<u64> {
        let cache = self.dir_sizes.read().await;
        if let Some(entry) = cache.get(path) {
            if entry.expires_at > Instant::now() {
                return Some(entry.value);
            }
        }
        None
    }

    /// Cache a directory size
    pub async fn set_dir_size(&self, path: PathBuf, size: u64) {
        let mut cache = self.dir_sizes.write().await;
        cache.insert(path, CacheEntry {
            value: size,
            expires_at: Instant::now() + self.dir_size_ttl,
        });
    }

    /// Get cached scan results, or None if not cached or expired
    pub async fn get_scan_results(&self, key: &str) -> Option<ScanCacheEntry> {
        let cache = self.scan_results.read().await;
        if let Some(entry) = cache.get(key) {
            if entry.expires_at > Instant::now() {
                return Some(entry.value.clone());
            }
        }
        None
    }

    /// Cache scan results
    pub async fn set_scan_results(&self, key: String, results: ScanCacheEntry) {
        let mut cache = self.scan_results.write().await;
        cache.insert(key, CacheEntry {
            value: results,
            expires_at: Instant::now() + self.scan_result_ttl,
        });
    }

    /// Clear expired entries from directory size cache
    pub async fn cleanup_dir_sizes(&self) {
        let now = Instant::now();
        let mut cache = self.dir_sizes.write().await;
        cache.retain(|_, entry| entry.expires_at > now);
    }

    /// Clear expired entries from scan results cache
    pub async fn cleanup_scan_results(&self) {
        let now = Instant::now();
        let mut cache = self.scan_results.write().await;
        cache.retain(|_, entry| entry.expires_at > now);
    }

    /// Clear all caches
    pub async fn clear_all(&self) {
        let mut dir_cache = self.dir_sizes.write().await;
        dir_cache.clear();
        drop(dir_cache);

        let mut scan_cache = self.scan_results.write().await;
        scan_cache.clear();
    }

    /// Get cache statistics
    pub async fn stats(&self) -> CacheStats {
        let dir_cache = self.dir_sizes.read().await;
        let scan_cache = self.scan_results.read().await;
        let now = Instant::now();

        let dir_size_count = dir_cache.len();
        let dir_size_expired = dir_cache.values()
            .filter(|entry| entry.expires_at <= now)
            .count();

        let scan_result_count = scan_cache.len();
        let scan_result_expired = scan_cache.values()
            .filter(|entry| entry.expires_at <= now)
            .count();

        CacheStats {
            dir_size_entries: dir_size_count,
            dir_size_expired,
            scan_result_entries: scan_result_count,
            scan_result_expired,
        }
    }
}

impl Default for CacheManager {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Debug, Clone)]
pub struct CacheStats {
    pub dir_size_entries: usize,
    pub dir_size_expired: usize,
    pub scan_result_entries: usize,
    pub scan_result_expired: usize,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_dir_size_cache() {
        let cache = CacheManager::with_ttls(
            Duration::from_secs(1),
            Duration::from_secs(1),
        );

        let path = PathBuf::from("/test/path");

        // Cache miss
        assert_eq!(cache.get_dir_size(&path).await, None);

        // Cache set
        cache.set_dir_size(path.clone(), 1024).await;

        // Cache hit
        assert_eq!(cache.get_dir_size(&path).await, Some(1024));

        // Wait for expiry
        tokio::time::sleep(Duration::from_millis(1100)).await;

        // Cache miss after expiry
        assert_eq!(cache.get_dir_size(&path).await, None);
    }

    #[tokio::test]
    async fn test_cache_cleanup() {
        let cache = CacheManager::with_ttls(
            Duration::from_millis(100),
            Duration::from_millis(100),
        );

        let path = PathBuf::from("/test/path");
        cache.set_dir_size(path, 1024).await;

        // Should have 1 entry
        let stats = cache.stats().await;
        assert_eq!(stats.dir_size_entries, 1);

        // Wait for expiry
        tokio::time::sleep(Duration::from_millis(150)).await;

        // Cleanup expired
        cache.cleanup_dir_sizes().await;

        // Should have 0 entries
        let stats = cache.stats().await;
        assert_eq!(stats.dir_size_entries, 0);
    }
}
