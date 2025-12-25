use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};
use std::io::{Read, Seek, SeekFrom};
use std::fs::File;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use walkdir::WalkDir;
use anyhow::{Context, Result as AnyhowResult};
use tokio::time::timeout;
use thiserror::Error;
use tauri::Emitter;

use crate::trash;

/// Scanner-specific error types
#[derive(Debug, Error)]
pub enum ScannerError {
    #[error("I/O error: {0}")]
    IoError(#[from] std::io::Error),
    #[error("Path validation failed: {0}")]
    PathValidationError(String),
    #[error("Memory limit exceeded: {0}")]
    MemoryLimitExceeded(String),
    #[error("Timeout exceeded")]
    Timeout,
    #[error("Operation cancelled")]
    #[allow(dead_code)] // Reserved for future cancellation support
    Cancelled,
}

/// Memory and performance limits for scanning operations
#[derive(Debug, Clone)]
pub struct ScanLimits {
    pub max_files: usize,
    pub max_depth: usize,
    pub max_memory_mb: usize,
    #[allow(dead_code)] // Reserved for future timeout configuration
    pub timeout_seconds: u64,
}

/// Check if current memory usage is within limits
/// Uses process-specific memory tracking for accurate measurement
async fn check_memory_limits(limits: &ScanLimits) -> Result<(), ScannerError> {
    use sysinfo::{System, Pid};

    // Use blocking task for system memory check (sysinfo operations are synchronous)
    let memory_limit_mb = limits.max_memory_mb;
    let current_memory_mb = tokio::task::spawn_blocking(move || -> Result<u64, ScannerError> {
        let mut system = System::new();
        system.refresh_memory();

        // Get current process ID
        let pid = Pid::from(std::process::id() as usize);

        // Refresh all processes to get current process info
        system.refresh_all();

        // Try to get process-specific memory usage
        let used_memory_mb = if let Some(process) = system.process(pid) {
            // Process found, use its memory usage
            let memory_bytes = process.memory();
            memory_bytes / (1024 * 1024) // Convert to MB
        } else {
            // Fallback to system-wide memory if process not found
            system.used_memory() / (1024 * 1024)
        };

        Ok(used_memory_mb)
    }).await
    .map_err(|e| ScannerError::MemoryLimitExceeded(format!("Memory check task failed: {}", e)))??;

    if current_memory_mb > memory_limit_mb as u64 {
        return Err(ScannerError::MemoryLimitExceeded(
            format!("Memory usage {}MB exceeds limit {}MB", current_memory_mb, memory_limit_mb)
        ));
    }

    Ok(())
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct ScanItem {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: u64,
    #[serde(rename = "type")]
    pub item_type: String,
    pub category: String,
    pub risk_level: u8,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<ScanItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependencies: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub dependents: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct FailedCategory {
    pub category: String,
    pub error: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct ScanResults {
    pub items: Vec<ScanItem>,
    pub total_size: u64,
    pub total_items: usize,
    pub scan_time_ms: u64,
    pub timestamp: String,
    #[serde(default)]
    pub failed_categories: Vec<FailedCategory>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct ScanOptions {
    pub include_caches: bool,
    pub include_packages: bool,
    pub include_large_files: bool,
    pub include_logs: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_files: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_depth: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_memory_mb: Option<usize>,
}

impl Default for ScanOptions {
    fn default() -> Self {
        Self {
            include_caches: true,
            include_packages: true,
            include_large_files: true,
            include_logs: true,
            max_files: None,
            max_depth: None,
            max_memory_mb: None,
        }
    }
}

/// Progress event structure for real-time scan updates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanProgress {
    pub category: String,
    pub progress: u8, // 0-100
    pub message: String,
    pub items_found: usize,
    pub current_size: u64,
}

/// Async version of main scan function with proper error handling and memory bounds
/// Emits progress events via app_handle if provided
pub async fn scan_system_async(
    options: &ScanOptions,
    app_handle: Option<&tauri::AppHandle>,
) -> Result<ScanResults, ScannerError> {
    let start = Instant::now();

    // Set memory and time limits for the scan (use provided limits or defaults)
    let scan_limits = ScanLimits {
        max_files: options.max_files.unwrap_or(50_000), // Limit to prevent excessive memory usage
        max_depth: options.max_depth.unwrap_or(10),     // Prevent infinite recursion
        max_memory_mb: options.max_memory_mb.unwrap_or(500), // 500MB memory limit
        timeout_seconds: 300, // 5 minute timeout (internal, not configurable)
    };

    let mut items = Vec::new();
    let mut total_size: u64 = 0;
    let mut total_items: usize = 0;
    let mut failed_categories = Vec::new();

    // Calculate total number of scan phases for progress tracking
    let total_phases = [
        options.include_caches,
        options.include_packages,
        options.include_logs,
        options.include_large_files,
    ]
    .iter()
    .filter(|&&enabled| enabled)
    .count();

    let mut completed_phases = 0;

    // Check memory usage periodically
    let memory_check_interval = Duration::from_secs(10);
    let mut last_memory_check = start;

    // Helper function to emit progress events
    // Note: We pass completed_phases as a parameter to avoid borrowing issues
    let emit_progress = |category: &str, phase_progress: u8, message: &str, items_found: usize, current_size: u64, phases_completed: usize| {
        if let Some(handle) = app_handle {
            let overall_progress = if total_phases > 0 {
                ((phases_completed * 100 + phase_progress as usize) / total_phases).min(100) as u8
            } else {
                0
            };

            let progress_event = ScanProgress {
                category: category.to_string(),
                progress: overall_progress,
                message: message.to_string(),
                items_found,
                current_size,
            };

            if let Err(e) = handle.emit("scan-progress", &progress_event) {
                tracing::warn!("Failed to emit scan progress event: {}", e);
            }
        }
    };

    if options.include_caches {
        emit_progress("caches", 0, "Scanning cache directories...", 0, 0, completed_phases);

        match scan_caches_async(&scan_limits).await {
            Ok(cache_items) => {
                let cache_size: u64 = cache_items.iter().map(|i| i.size).sum();
                let cache_count = cache_items.len();

                for item in &cache_items {
                    total_size += item.size;
                    total_items += 1;
                    if let Some(children) = &item.children {
                        total_items += children.len();
                    }
                }
                items.extend(cache_items);

                completed_phases += 1;
                emit_progress("caches", 100, &format!("Found {} cache items", cache_count), cache_count, cache_size, completed_phases);
            }
            Err(e) => {
                tracing::warn!("Cache scanning failed: {}", e);
                failed_categories.push(FailedCategory {
                    category: "caches".to_string(),
                    error: e.to_string(),
                });
                completed_phases += 1;
                emit_progress("caches", 100, &format!("Cache scan failed: {}", e), 0, 0, completed_phases);
                // Continue with other scans even if cache scan fails
            }
        }

        // Check memory usage
        let now = Instant::now();
        if now.duration_since(last_memory_check) > memory_check_interval {
            if let Err(e) = check_memory_limits(&scan_limits).await {
                return Err(ScannerError::MemoryLimitExceeded(e.to_string()));
            }
            last_memory_check = now;
        }
    }

    if options.include_packages {
        emit_progress("packages", 0, "Scanning package caches...", 0, 0, completed_phases);

        match scan_package_caches_async().await {
            Ok(package_items) => {
                let package_size: u64 = package_items.iter().map(|i| i.size).sum();
                let package_count = package_items.len();

                for item in &package_items {
                    total_size += item.size;
                    total_items += 1;
                }
                items.extend(package_items);

                completed_phases += 1;
                emit_progress("packages", 100, &format!("Found {} package cache items", package_count), package_count, package_size, completed_phases);
            }
            Err(e) => {
                tracing::warn!("Package cache scanning failed: {}", e);
                failed_categories.push(FailedCategory {
                    category: "packages".to_string(),
                    error: e.to_string(),
                });
                completed_phases += 1;
                emit_progress("packages", 100, &format!("Package scan failed: {}", e), 0, 0, completed_phases);
            }
        }

        // Check memory usage
        let now = Instant::now();
        if now.duration_since(last_memory_check) > memory_check_interval {
            if let Err(e) = check_memory_limits(&scan_limits).await {
                return Err(ScannerError::MemoryLimitExceeded(e.to_string()));
            }
            last_memory_check = now;
        }
    }

    if options.include_logs {
        emit_progress("logs", 0, "Scanning log files...", 0, 0, completed_phases);

        match scan_logs_async(&scan_limits).await {
            Ok(log_items) => {
                let log_size: u64 = log_items.iter().map(|i| i.size).sum();
                let log_count = log_items.len();

                for item in &log_items {
                    total_size += item.size;
                    total_items += 1;
                }
                items.extend(log_items);

                completed_phases += 1;
                emit_progress("logs", 100, &format!("Found {} log files", log_count), log_count, log_size, completed_phases);
            }
            Err(e) => {
                tracing::warn!("Log scanning failed: {}", e);
                failed_categories.push(FailedCategory {
                    category: "logs".to_string(),
                    error: e.to_string(),
                });
                completed_phases += 1;
                emit_progress("logs", 100, &format!("Log scan failed: {}", e), 0, 0, completed_phases);
            }
        }
    }

    if options.include_large_files {
        // Check memory usage
        let now = Instant::now();
        if now.duration_since(last_memory_check) > memory_check_interval {
            if let Err(e) = check_memory_limits(&scan_limits).await {
                return Err(ScannerError::MemoryLimitExceeded(e.to_string()));
            }
            // Note: This is the last memory check, so we don't update last_memory_check
        }

        emit_progress("large_files", 0, "Scanning for large files...", 0, 0, completed_phases);

        match scan_large_files_async(&scan_limits).await {
            Ok(large_files) => {
                let large_size: u64 = large_files.iter().map(|i| i.size).sum();
                let large_count = large_files.len();

                for item in &large_files {
                    total_size += item.size;
                    total_items += 1;
                }
                items.extend(large_files);

                completed_phases += 1;
                emit_progress("large_files", 100, &format!("Found {} large files", large_count), large_count, large_size, completed_phases);
            }
            Err(e) => {
                tracing::warn!("Large files scanning failed: {}", e);
                failed_categories.push(FailedCategory {
                    category: "large_files".to_string(),
                    error: e.to_string(),
                });
                completed_phases += 1;
                emit_progress("large_files", 100, &format!("Large files scan failed: {}", e), 0, 0, completed_phases);
            }
        }
    }

    let elapsed = start.elapsed();

    // Final memory check
    check_memory_limits(&scan_limits).await?;

    // Emit final completion event
    emit_progress("complete", 100, &format!("Scan complete: {} items found", total_items), total_items, total_size, completed_phases);

    Ok(ScanResults {
        items,
        total_size,
        total_items,
        scan_time_ms: elapsed.as_millis() as u64,
        timestamp: chrono::Utc::now().to_rfc3339(),
        failed_categories,
    })
}


/// Async version of cache scanning with proper error handling
async fn scan_caches_async(_limits: &ScanLimits) -> Result<Vec<ScanItem>, ScannerError> {
    let mut items = Vec::new();

    let home = dirs::home_dir()
        .ok_or_else(|| ScannerError::PathValidationError("Cannot determine home directory".to_string()))?;

    let cache_dirs = vec![
        (home.join(".cache"), "User Cache"),
        (home.join(".local/share/Trash"), "User Trash"),
        (home.join(".thumbnails"), "Thumbnails"),
    ];

    for (path, name) in cache_dirs {
        if path.exists() {
            // Clone path for the blocking task
            let path_clone = path.clone();
            // Use tokio::task::spawn_blocking for CPU-intensive directory size calculation
            let size = timeout(
                Duration::from_secs(30),
                tokio::task::spawn_blocking(move || trash::get_dir_size(&path_clone))
            ).await
            .map_err(|_| ScannerError::Timeout)?
            .map_err(|e| ScannerError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

            if size > 0 {
                let mut item = ScanItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: name.to_string(),
                    path: path.to_string_lossy().to_string(),
                    size,
                    item_type: "cache".to_string(),
                    category: "Cache".to_string(),
                    risk_level: 0,
                    description: "Cache directory - safe to remove".to_string(),
                    children: None,
                    dependencies: None,
                    dependents: None,
                };

                // Scan subdirectories with depth limit
                item.children = scan_cache_subdirs_async(&path, _limits.max_depth).await?;
                items.push(item);
            }
        }
    }

    // Browser caches
    let browser_caches = vec![
        (home.join(".cache/google-chrome"), "Chrome Cache"),
        (home.join(".cache/mozilla/firefox"), "Firefox Cache"),
        (home.join(".cache/chromium"), "Chromium Cache"),
    ];

    for (path, name) in browser_caches {
        if path.exists() {
            let path_clone = path.clone();
            let size = timeout(
                Duration::from_secs(30),
                tokio::task::spawn_blocking(move || trash::get_dir_size(&path_clone))
            ).await
            .map_err(|_| ScannerError::Timeout)?
            .map_err(|e| ScannerError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

            if size > 10 * 1024 * 1024 {
                items.push(ScanItem {
                    id: uuid::Uuid::new_v4().to_string(),
                    name: name.to_string(),
                    path: path.to_string_lossy().to_string(),
                    size,
                    item_type: "cache".to_string(),
                    category: "Browser".to_string(),
                    risk_level: 0,
                    description: "Browser cache - safe to remove".to_string(),
                    children: None,
                    dependencies: None,
                    dependents: None,
                });
            }
        }
    }

    Ok(items)
}


/// Async version of cache subdirectory scanning
async fn scan_cache_subdirs_async(path: &Path, _max_depth: usize) -> Result<Option<Vec<ScanItem>>, ScannerError> {
    let mut children = Vec::new();

    if let Ok(entries) = std::fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            if entry_path.is_dir() {
                // Use blocking task for directory size calculation
                let size = timeout(
                    Duration::from_secs(10),
                    tokio::task::spawn_blocking({
                        let path_clone = entry_path.clone();
                        move || trash::get_dir_size(&path_clone)
                    })
                ).await
                .map_err(|_| ScannerError::Timeout)?
                .map_err(|e| ScannerError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

                if size > 5 * 1024 * 1024 {
                    children.push(ScanItem {
                        id: uuid::Uuid::new_v4().to_string(),
                        name: entry_path.file_name()
                            .and_then(|n| n.to_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| entry_path.to_string_lossy().to_string()),
                        path: entry_path.to_string_lossy().to_string(),
                        size,
                        item_type: "directory".to_string(),
                        category: "Cache".to_string(),
                        risk_level: 0,
                        description: "Application cache".to_string(),
                        children: None,
                        dependencies: None,
                        dependents: None,
                    });
                }
            }
        }
    }

    if children.is_empty() {
        Ok(None)
    } else {
        children.sort_by(|a, b| b.size.cmp(&a.size));
        Ok(Some(children.into_iter().take(10).collect()))
    }
}


/// Async version of package cache scanning
async fn scan_package_caches_async() -> Result<Vec<ScanItem>, ScannerError> {
    let mut items = Vec::new();

    let home = dirs::home_dir()
        .ok_or_else(|| ScannerError::PathValidationError("Cannot determine home directory".to_string()))?;

    // APT cache
    let apt_cache = PathBuf::from("/var/cache/apt/archives");
    if apt_cache.exists() {
        let apt_cache_clone = apt_cache.clone();
        let size = timeout(
            Duration::from_secs(30),
            tokio::task::spawn_blocking(move || trash::get_dir_size(&apt_cache_clone))
        ).await
        .map_err(|_| ScannerError::Timeout)?
        .map_err(|e| ScannerError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        if size > 0 {
            items.push(ScanItem {
                id: uuid::Uuid::new_v4().to_string(),
                name: "APT Package Cache".to_string(),
                path: apt_cache.to_string_lossy().to_string(),
                size,
                item_type: "cache".to_string(),
                category: "Package Manager".to_string(),
                risk_level: 0,
                description: "Downloaded .deb packages - safe to remove".to_string(),
                children: None,
                dependencies: None,
                dependents: None,
            });
        }
    }

    // pip cache
    let pip_cache = home.join(".cache/pip");
    if pip_cache.exists() {
        let pip_cache_clone = pip_cache.clone();
        let size = timeout(
            Duration::from_secs(30),
            tokio::task::spawn_blocking(move || trash::get_dir_size(&pip_cache_clone))
        ).await
        .map_err(|_| ScannerError::Timeout)?
        .map_err(|e| ScannerError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        if size > 0 {
            items.push(ScanItem {
                id: uuid::Uuid::new_v4().to_string(),
                name: "pip Cache".to_string(),
                path: pip_cache.to_string_lossy().to_string(),
                size,
                item_type: "cache".to_string(),
                category: "Python".to_string(),
                risk_level: 0,
                description: "Python package cache - safe to remove".to_string(),
                children: None,
                dependencies: None,
                dependents: None,
            });
        }
    }

    // npm cache
    let npm_cache = home.join(".npm/_cacache");
    if npm_cache.exists() {
        let npm_cache_clone = npm_cache.clone();
        let size = timeout(
            Duration::from_secs(30),
            tokio::task::spawn_blocking(move || trash::get_dir_size(&npm_cache_clone))
        ).await
        .map_err(|_| ScannerError::Timeout)?
        .map_err(|e| ScannerError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

        if size > 0 {
            items.push(ScanItem {
                id: uuid::Uuid::new_v4().to_string(),
                name: "npm Cache".to_string(),
                path: npm_cache.to_string_lossy().to_string(),
                size,
                item_type: "cache".to_string(),
                category: "Node.js".to_string(),
                risk_level: 0,
                description: "Node.js package cache - safe to remove".to_string(),
                children: None,
                dependencies: None,
                dependents: None,
            });
        }
    }

    Ok(items)
}


/// Async version of log scanning with proper limits
async fn scan_logs_async(limits: &ScanLimits) -> Result<Vec<ScanItem>, ScannerError> {
    let mut items = Vec::new();

    let home = dirs::home_dir()
        .ok_or_else(|| ScannerError::PathValidationError("Cannot determine home directory".to_string()))?;

    // Clone limits data to move into the closure
    let max_depth = limits.max_depth;
    let max_files = limits.max_files;
    let home_clone = home.clone();

    // Use tokio::task::spawn_blocking for the synchronous WalkDir operation
    let log_items = timeout(
        Duration::from_secs(60),
        tokio::task::spawn_blocking(move || {
            let mut sync_items = Vec::new();
            for entry in WalkDir::new(home_clone.join(".local/share"))
                .max_depth(max_depth)
                .into_iter()
                .filter_map(|e| e.ok())
                .take(max_files)
            {
                let path = entry.path();
                if path.is_file() {
                    let name = path.file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| path.to_string_lossy().to_string());

                    if name.ends_with(".log") || name.ends_with(".log.1") || name.contains(".log.") {
                        if let Ok(metadata) = path.metadata() {
                            let size = metadata.len();
                            if size > 10 * 1024 * 1024 {
                                sync_items.push(ScanItem {
                                    id: uuid::Uuid::new_v4().to_string(),
                                    name: name.to_string(),
                                    path: path.to_string_lossy().to_string(),
                                    size,
                                    item_type: "file".to_string(),
                                    category: "Logs".to_string(),
                                    risk_level: 1,
                                    description: "Log file - review before removing".to_string(),
                                    children: None,
                                    dependencies: None,
                                    dependents: None,
                                });
                            }
                        }
                    }
                }
            }
            sync_items
        })
    ).await
    .map_err(|_| ScannerError::Timeout)?
    .map_err(|e| ScannerError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    items.extend(log_items);
    Ok(items)
}


/// Async version of large files scanning with proper limits
async fn scan_large_files_async(limits: &ScanLimits) -> Result<Vec<ScanItem>, ScannerError> {
    let mut items = Vec::new();

    let home = dirs::home_dir()
        .ok_or_else(|| ScannerError::PathValidationError("Cannot determine home directory".to_string()))?;

    let scan_dirs = vec![home.join("Downloads"), home.join("Documents")];
    let threshold = 100 * 1024 * 1024; // 100MB

    // Clone limits data to move into the closure
    let max_depth = limits.max_depth;
    let max_files = limits.max_files;
    let scan_dirs_clone = scan_dirs.clone();

    // Use tokio::task::spawn_blocking for the synchronous WalkDir operation
    let large_files = timeout(
        Duration::from_secs(120),
        tokio::task::spawn_blocking(move || {
            let mut sync_items = Vec::new();
            for dir in scan_dirs_clone {
                if dir.exists() {
                    for entry in WalkDir::new(&dir)
                        .max_depth(max_depth)
                        .into_iter()
                        .filter_map(|e| e.ok())
                        .take(max_files)
                    {
                        let path = entry.path();
                        if path.is_file() {
                            if let Ok(metadata) = path.metadata() {
                                let size = metadata.len();
                                if size > threshold {
                                    sync_items.push(ScanItem {
                                        id: uuid::Uuid::new_v4().to_string(),
                                        name: path.file_name()
                                            .and_then(|n| n.to_str())
                                            .map(|s| s.to_string())
                                            .unwrap_or_else(|| path.to_string_lossy().to_string()),
                                        path: path.to_string_lossy().to_string(),
                                        size,
                                        item_type: "file".to_string(),
                                        category: "Large Files".to_string(),
                                        risk_level: 2,
                                        description: "Large file - review before removing".to_string(),
                                        children: None,
                                        dependencies: None,
                                        dependents: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
            sync_items
        })
    ).await
    .map_err(|_| ScannerError::Timeout)?
    .map_err(|e| ScannerError::IoError(std::io::Error::new(std::io::ErrorKind::Other, e)))?;

    items.extend(large_files);
    items.sort_by(|a, b| b.size.cmp(&a.size));
    items.truncate(20); // Limit results
    Ok(items)
}


// Filesystem Health Check functions
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct FilesystemHealthResults {
    pub empty_directories: Vec<ScanItem>,
    pub broken_symlinks: Vec<ScanItem>,
    pub orphaned_temp_files: Vec<ScanItem>,
    pub total_size: u64,
    pub total_items: usize,
}

pub fn scan_filesystem_health() -> FilesystemHealthResults {
    let start_time = Instant::now();

    tracing::info!("Starting filesystem health check scan");

    let empty_dirs = scan_empty_directories();
    let broken_links = scan_broken_symlinks();
    let orphaned_temp = scan_orphaned_temp_files();

    let total_size = empty_dirs.iter().map(|i| i.size).sum::<u64>() +
                     broken_links.iter().map(|i| i.size).sum::<u64>() +
                     orphaned_temp.iter().map(|i| i.size).sum::<u64>();

    let total_items = empty_dirs.len() + broken_links.len() + orphaned_temp.len();

    let scan_time = start_time.elapsed().as_millis() as u64;
    tracing::info!("Filesystem health check completed in {}ms: {} items, {} bytes",
                   scan_time, total_items, total_size);

    FilesystemHealthResults {
        empty_directories: empty_dirs,
        broken_symlinks: broken_links,
        orphaned_temp_files: orphaned_temp,
        total_size,
        total_items,
    }
}

fn scan_empty_directories() -> Vec<ScanItem> {
    let mut items = Vec::new();

    if let Some(home) = dirs::home_dir() {
        let walker = WalkDir::new(&home)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok());

        for entry in walker {
            let path = entry.path();

            if path.is_dir() {
                // Check if directory is empty
                if let Ok(read_dir) = std::fs::read_dir(path) {
                    if read_dir.count() == 0 {
                        // Double-check it's still empty (in case of race condition)
                        if let Ok(recheck) = std::fs::read_dir(path) {
                            if recheck.count() == 0 {
                                let path_str = path.to_string_lossy().to_string();
                                items.push(ScanItem {
                                    id: format!("empty_dir_{}", items.len()),
                                    name: path.file_name()
                                        .and_then(|n| n.to_str())
                                        .map(|s| s.to_string())
                                        .unwrap_or_else(|| path.to_string_lossy().to_string()),
                                    path: path_str,
                                    size: 0,
                                    item_type: "directory".to_string(),
                                    category: "empty_directory".to_string(),
                                    risk_level: 0, // Safe to remove
                                    description: "Empty directory with no contents".to_string(),
                                    children: None,
                                    dependencies: None,
                                    dependents: None,
                                });
                            }
                        }
                    }
                }
            }
        }
    }

    tracing::info!("Found {} empty directories", items.len());
    items
}

fn scan_broken_symlinks() -> Vec<ScanItem> {
    let mut items = Vec::new();

    if let Some(home) = dirs::home_dir() {
        let walker = WalkDir::new(&home)
            .follow_links(false)
            .into_iter()
            .filter_map(|e| e.ok());

        for entry in walker {
            let path = entry.path();

            if let Ok(metadata) = std::fs::symlink_metadata(path) {
                if metadata.file_type().is_symlink() {
                    // Check if symlink target exists
                    if let Ok(target) = std::fs::read_link(path) {
                        if !target.exists() {
                            let path_str = path.to_string_lossy().to_string();
                            items.push(ScanItem {
                                id: format!("broken_link_{}", items.len()),
                                name: path.file_name()
                                    .and_then(|n| n.to_str())
                                    .map(|s| s.to_string())
                                    .unwrap_or_else(|| path.to_string_lossy().to_string()),
                                path: path_str,
                                size: 0,
                                item_type: "symlink".to_string(),
                                category: "broken_symlink".to_string(),
                                risk_level: 0, // Safe to remove
                                description: format!("Broken symlink pointing to non-existent target: {}",
                                                   target.display()),
                                children: None,
                                dependencies: None,
                                dependents: None,
                            });
                        }
                    }
                }
            }
        }
    }

    tracing::info!("Found {} broken symlinks", items.len());
    items
}

fn scan_orphaned_temp_files() -> Vec<ScanItem> {
    let mut items = Vec::new();

    if let Some(home) = dirs::home_dir() {
        // Common temp file patterns
        let temp_patterns = [
            "*.tmp", "*.temp", "*.swp", "*.bak", "*.orig",
            "*.old", "~*", "*~", "*.lock", "*.pid"
        ];

        // Common temp directories
        let temp_dirs = [
            home.join("tmp"),
            home.join(".tmp"),
            home.join("temp"),
            home.join("Temp"),
            home.join("TEMP"),
        ];

        // Also check common temp locations in home
        let walker = WalkDir::new(&home)
            .follow_links(false)
            .max_depth(3) // Don't go too deep
            .into_iter()
            .filter_map(|e| e.ok());

        for entry in walker {
            let path = entry.path();

            if path.is_file() {
                if let Some(filename) = path.file_name() {
                    let filename_str = filename.to_string_lossy();

                    // Check if it's in a temp directory or matches temp patterns
                    let is_in_temp_dir = temp_dirs.iter().any(|temp_dir| {
                        path.starts_with(temp_dir)
                    });

                    let matches_temp_pattern = temp_patterns.iter().any(|pattern| {
                        // Simple glob matching
                        if let Some(prefix_stripped) = pattern.strip_prefix("*.") {
                            if let Some(suffix_stripped) = prefix_stripped.strip_suffix('*') {
                                filename_str.contains(suffix_stripped)
                            } else {
                                filename_str.ends_with(prefix_stripped)
                            }
                        } else if let Some(suffix_stripped) = pattern.strip_suffix('*') {
                            filename_str.starts_with(suffix_stripped)
                        } else {
                            filename_str == *pattern
                        }
                    });

                    if is_in_temp_dir || matches_temp_pattern {
                        // Check if file is older than 30 days (orphaned temp file)
                        if let Ok(metadata) = std::fs::metadata(path) {
                            if let Ok(modified) = metadata.modified() {
                                let age_days = match modified.elapsed() {
                                    Ok(duration) => duration.as_secs() / (24 * 3600),
                                    Err(_) => {
                                        // File modified in the future (clock skew) - skip
                                        continue;
                                    }
                                };

                                if age_days > 30 {
                                    let path_str = path.to_string_lossy().to_string();
                                    let size = metadata.len();

                                    items.push(ScanItem {
                                        id: format!("orphaned_temp_{}", items.len()),
                                        name: filename.to_string_lossy().to_string(),
                                        path: path_str,
                                        size,
                                        item_type: "file".to_string(),
                                        category: "orphaned_temp".to_string(),
                                        risk_level: 1, // Low risk, review suggested
                                        description: format!("Orphaned temporary file, {} days old", age_days),
                                        children: None,
                                        dependencies: None,
                                        dependents: None,
                                    });
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    tracing::info!("Found {} orphaned temp files", items.len());
    items
}

// Storage Recovery Suite functions
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct StorageRecoveryResults {
    pub duplicates: Vec<DuplicateGroup>,
    pub large_files: Vec<ScanItem>,
    pub old_downloads: Vec<ScanItem>,
    pub total_duplicate_size: u64,
    pub total_large_files_size: u64,
    pub total_old_downloads_size: u64,
    pub total_recoverable_size: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct DuplicateGroup {
    pub id: String,
    pub files: Vec<ScanItem>,
    pub total_size: u64,
    pub group_size: usize,
}

/// Scan for storage recovery opportunities (duplicates, large files, old downloads)
/// Returns results even if some scans fail (partial success)
pub fn scan_storage_recovery() -> AnyhowResult<StorageRecoveryResults> {
    let start_time = Instant::now();

    tracing::info!("Starting storage recovery scan");

    // Run all scans - each can fail independently
    let duplicates = scan_duplicate_files()
        .context("Failed to scan for duplicate files")?;

    let large_files = scan_large_files_storage_recovery(1024 * 1024 * 1024) // 1GB threshold
        .context("Failed to scan for large files")?;

    let old_downloads = scan_old_downloads(90) // 90 days
        .context("Failed to scan for old downloads")?;

    let total_duplicate_size: u64 = duplicates.iter().map(|g| g.total_size).sum();
    let total_large_files_size: u64 = large_files.iter().map(|i| i.size).sum();
    let total_old_downloads_size: u64 = old_downloads.iter().map(|i| i.size).sum();
    let total_recoverable_size = total_duplicate_size + total_large_files_size + total_old_downloads_size;

    let scan_time = start_time.elapsed().as_millis() as u64;
    tracing::info!("Storage recovery scan completed in {}ms: {} duplicates, {} large files, {} old downloads, {} bytes recoverable",
                   scan_time, duplicates.len(), large_files.len(), old_downloads.len(), total_recoverable_size);

    Ok(StorageRecoveryResults {
        duplicates,
        large_files,
        old_downloads,
        total_duplicate_size,
        total_large_files_size,
        total_old_downloads_size,
        total_recoverable_size,
    })
}

/// Compute a content hash of a file using chunked reading (doesn't load entire file)
/// Samples: first 64KB, middle 64KB, and last 64KB
fn compute_file_hash_chunked(path: &Path) -> AnyhowResult<String> {
    let mut file = File::open(path)
        .with_context(|| format!("Failed to open file for hashing: {}", path.display()))?;

    let metadata = file.metadata()
        .with_context(|| format!("Failed to get metadata for: {}", path.display()))?;
    let file_size = metadata.len();

    let mut hasher = DefaultHasher::new();
    file_size.hash(&mut hasher);

    const CHUNK_SIZE: u64 = 64 * 1024; // 64KB chunks

    // Hash first chunk
    if file_size > 0 {
        let first_chunk_size = std::cmp::min(CHUNK_SIZE, file_size);
        let mut buffer = vec![0u8; first_chunk_size as usize];
        file.read_exact(&mut buffer)
            .with_context(|| format!("Failed to read first chunk of: {}", path.display()))?;
        buffer.hash(&mut hasher);
    }

    // Hash middle chunk if file is large enough
    if file_size > CHUNK_SIZE * 2 {
        let mid_start = file_size / 2;
        file.seek(SeekFrom::Start(mid_start))
            .with_context(|| format!("Failed to seek to middle of: {}", path.display()))?;

        let mid_chunk_size = std::cmp::min(CHUNK_SIZE, file_size - mid_start);
        let mut buffer = vec![0u8; mid_chunk_size as usize];
        file.read_exact(&mut buffer)
            .with_context(|| format!("Failed to read middle chunk of: {}", path.display()))?;
        buffer.hash(&mut hasher);
    }

    // Hash last chunk if file is large enough
    if file_size > CHUNK_SIZE {
        let last_start = file_size.saturating_sub(CHUNK_SIZE);
        file.seek(SeekFrom::Start(last_start))
            .with_context(|| format!("Failed to seek to end of: {}", path.display()))?;

        let last_chunk_size = file_size - last_start;
        let mut buffer = vec![0u8; last_chunk_size as usize];
        file.read_exact(&mut buffer)
            .with_context(|| format!("Failed to read last chunk of: {}", path.display()))?;
        buffer.hash(&mut hasher);
    }

    Ok(format!("{:x}", hasher.finish()))
}

/// Scan for duplicate files using chunked hashing (memory-efficient)
/// Limits scan to prevent excessive processing time
fn scan_duplicate_files() -> AnyhowResult<Vec<DuplicateGroup>> {
    let mut duplicates = Vec::new();

    let home = dirs::home_dir()
        .context("Cannot determine home directory")?;

    // Use a hash map to group files by size first, then by content hash
    let mut size_groups: std::collections::HashMap<u64, Vec<std::path::PathBuf>> = std::collections::HashMap::new();

    const MAX_FILES_TO_SCAN: usize = 10000; // Limit to prevent excessive scanning
    let mut files_scanned = 0;

    // First pass: group by size
    let walker = WalkDir::new(&home)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| {
            match e {
                Ok(entry) => Some(entry),
                Err(e) => {
                    tracing::debug!("WalkDir error (skipping): {}", e);
                    None
                }
            }
        })
        .filter(|e| e.file_type().is_file())
        .take(MAX_FILES_TO_SCAN);

    for entry in walker {
        files_scanned += 1;
        let path = entry.path();

        match std::fs::metadata(path) {
            Ok(metadata) => {
                let size = metadata.len();
                // Only consider files larger than 1KB to avoid too many small duplicates
                if size > 1024 {
                    size_groups.entry(size).or_default().push(path.to_path_buf());
                }
            }
            Err(e) => {
                tracing::debug!("Failed to get metadata for {}: {}", path.display(), e);
                // Continue with other files
            }
        }
    }

    if files_scanned >= MAX_FILES_TO_SCAN {
        tracing::warn!("Duplicate scan limited to {} files to prevent timeout", MAX_FILES_TO_SCAN);
    }

    // Second pass: check actual duplicates within same-size groups using chunked hashing
    for (size, paths) in size_groups {
        if paths.len() > 1 {
            let mut hash_groups: std::collections::HashMap<String, Vec<std::path::PathBuf>> = std::collections::HashMap::new();

            for path in paths {
                match compute_file_hash_chunked(&path) {
                    Ok(hash) => {
                        hash_groups.entry(hash).or_default().push(path);
                    }
                    Err(e) => {
                        tracing::debug!("Failed to hash file {}: {}", path.display(), e);
                        // Skip this file but continue with others
                    }
                }
            }

            // Create duplicate groups
            for (hash, paths) in hash_groups {
                if paths.len() > 1 {
                    let mut files = Vec::new();
                    for path in paths {
                        let path_str = path.to_string_lossy().to_string();
                        let name = path.file_name()
                            .and_then(|n| n.to_str())
                            .map(|s| s.to_string())
                            .unwrap_or_else(|| path_str.clone());

                        files.push(ScanItem {
                            id: format!("dup_{}_{}", hash, files.len()),
                            name,
                            path: path_str,
                            size,
                            item_type: "file".to_string(),
                            category: "duplicate".to_string(),
                            risk_level: 2, // Medium risk - review recommended
                            description: "Duplicate file - one copy can be safely removed".to_string(),
                            children: None,
                            dependencies: None,
                            dependents: None,
                        });
                    }

                    let group_size = files.len();
                    duplicates.push(DuplicateGroup {
                        id: format!("dup_group_{}", duplicates.len()),
                        files,
                        total_size: size * group_size as u64,
                        group_size,
                    });
                }
            }
        }
    }

    tracing::info!("Found {} duplicate groups", duplicates.len());
    Ok(duplicates)
}

/// Scan for large files (above specified threshold)
/// Limits scan to prevent excessive processing time
fn scan_large_files_storage_recovery(min_size: u64) -> AnyhowResult<Vec<ScanItem>> {
    let mut large_files = Vec::new();

    let home = dirs::home_dir()
        .context("Cannot determine home directory")?;

    const MAX_FILES_TO_SCAN: usize = 5000; // Limit to prevent timeout
    let mut files_scanned = 0;

    let walker = WalkDir::new(&home)
        .follow_links(false)
        .into_iter()
        .filter_map(|e| {
            match e {
                Ok(entry) => Some(entry),
                Err(e) => {
                    tracing::debug!("WalkDir error (skipping): {}", e);
                    None
                }
            }
        })
        .filter(|e| e.file_type().is_file())
        .take(MAX_FILES_TO_SCAN);

    for entry in walker {
        files_scanned += 1;
        let path = entry.path();

        match std::fs::metadata(path) {
            Ok(metadata) => {
                let size = metadata.len();
                if size >= min_size {
                    let path_str = path.to_string_lossy().to_string();
                    let name = path.file_name()
                        .and_then(|n| n.to_str())
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| path_str.clone());

                    large_files.push(ScanItem {
                        id: format!("large_file_{}", large_files.len()),
                        name,
                        path: path_str,
                        size,
                        item_type: "file".to_string(),
                        category: "large_file".to_string(),
                        risk_level: 3, // High risk - careful review required
                        description: format!("Large file: {}", format_bytes(size)),
                        children: None,
                        dependencies: None,
                        dependents: None,
                    });
                }
            }
            Err(e) => {
                tracing::debug!("Failed to get metadata for {}: {}", path.display(), e);
                // Continue with other files
            }
        }
    }

    if files_scanned >= MAX_FILES_TO_SCAN {
        tracing::warn!("Large files scan limited to {} files to prevent timeout", MAX_FILES_TO_SCAN);
    }

    // Sort by size descending
    large_files.sort_by(|a, b| b.size.cmp(&a.size));

    tracing::info!("Found {} large files", large_files.len());
    Ok(large_files)
}

/// Scan for old downloads (files in Downloads directory older than threshold)
fn scan_old_downloads(days_threshold: u64) -> AnyhowResult<Vec<ScanItem>> {
    let mut old_downloads = Vec::new();

    let home = dirs::home_dir()
        .context("Cannot determine home directory")?;

    let downloads_dir = home.join("Downloads");
    let threshold_seconds = days_threshold * 24 * 3600;

    if !downloads_dir.exists() {
        tracing::info!("Downloads directory does not exist, skipping old downloads scan");
        return Ok(old_downloads);
    }

    let walker = WalkDir::new(&downloads_dir)
        .follow_links(false)
        .max_depth(2) // Don't go too deep
        .into_iter()
        .filter_map(|e| {
            match e {
                Ok(entry) => Some(entry),
                Err(e) => {
                    tracing::debug!("WalkDir error (skipping): {}", e);
                    None
                }
            }
        })
        .filter(|e| e.file_type().is_file());

    for entry in walker {
        let path = entry.path();

        match std::fs::metadata(path) {
            Ok(metadata) => {
                match metadata.modified() {
                    Ok(modified) => {
                        let age_seconds = match modified.elapsed() {
                            Ok(duration) => duration.as_secs(),
                            Err(_) => {
                                // File modified in the future (clock skew) - skip
                                continue;
                            }
                        };

                        if age_seconds > threshold_seconds {
                            let path_str = path.to_string_lossy().to_string();
                            let size = metadata.len();
                            let name = path.file_name()
                                .and_then(|n| n.to_str())
                                .map(|s| s.to_string())
                                .unwrap_or_else(|| path_str.clone());

                            old_downloads.push(ScanItem {
                                id: format!("old_download_{}", old_downloads.len()),
                                name,
                                path: path_str,
                                size,
                                item_type: "file".to_string(),
                                category: "old_download".to_string(),
                                risk_level: 1, // Low risk - downloads can usually be removed
                                description: format!("Old download: {} days old", age_seconds / (24 * 3600)),
                                children: None,
                                dependencies: None,
                                dependents: None,
                            });
                        }
                    }
                    Err(e) => {
                        tracing::debug!("Failed to get modification time for {}: {}", path.display(), e);
                        // Continue with other files
                    }
                }
            }
            Err(e) => {
                tracing::debug!("Failed to get metadata for {}: {}", path.display(), e);
                // Continue with other files
            }
        }
    }

    // Sort by path (could be enhanced with actual timestamps)
    old_downloads.sort_by(|a, b| a.path.cmp(&b.path));

    tracing::info!("Found {} old downloads", old_downloads.len());
    Ok(old_downloads)
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    format!("{:.2} {}", size, UNITS[unit_index])
}
