use serde::{Deserialize, Serialize};
use specta::Type;
use sysinfo::Disks;
use std::sync::Arc;
use std::path::{Path, PathBuf};
use tokio::sync::Mutex as AsyncMutex;
use tokio::time::{timeout, Duration};
use notify::Watcher;
use walkdir::WalkDir;
use tauri::Manager;
use dirs;

// use crate::packages;
use crate::db::AppState;
use crate::scanner::{self, ScanOptions, ScanResults, FilesystemHealthResults, StorageRecoveryResults};
use crate::trash::{self, TrashData, TrashMetadata};

// Helper function for database access - temporarily commented out
// fn with_db<F, T>(app_handle: &tauri::AppHandle, f: F) -> Result<T, String>
// where
//     F: FnOnce(&rusqlite::Connection) -> Result<T, rusqlite::Error>,
// {
//     let state: tauri::State<AppState> = app_handle.state();
//     let db = state.db.lock()
//         .map_err(|e| format!("Mutex lock failed: {}", e))?;

//     let conn = db.as_ref()
//         .ok_or_else(|| "Database not initialized".to_string())?;

//     f(conn).map_err(|e| format!("Database error: {}", e))
// }

// Cache analytics structures
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct CacheAnalytics {
    pub total_cache_size: u64,
    pub cache_breakdown: Vec<CacheContributor>,
    pub growth_trend: Vec<CacheGrowthPoint>,
    pub recommended_limits: std::collections::HashMap<String, u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct CacheContributor {
    pub source: String,
    pub size: u64,
    pub growth_rate: f32, // MB per day
    pub last_activity: i64,
    pub recommended_limit: Option<u64>,
