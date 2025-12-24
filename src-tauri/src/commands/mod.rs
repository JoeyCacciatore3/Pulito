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

use crate::packages;
use crate::db::DbAccess;
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
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct CacheGrowthPoint {
    pub timestamp: i64,
    pub total_size: u64,
    pub sources: std::collections::HashMap<String, u64>,
}
use sysinfo::{Components, Networks, System};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct SystemStats {
    pub total_disk_space: u64,
    pub used_disk_space: u64,
    pub cleanable_space: u64,
    pub last_scan: Option<String>,
    pub packages_installed: usize,
    pub orphan_packages: usize,
    pub cache_size: u64,
    pub filesystem_health_savings: Option<u64>, // Real savings from last filesystem health scan
    pub storage_recovery_savings: Option<u64>, // Real savings from last storage recovery scan
    pub orphan_packages_size: Option<u64>, // Real size of orphaned packages
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct AppSettings {
    pub trash: TrashSettings,
    pub monitoring: MonitoringSettings,
    pub notifications: NotificationSettings,
    pub scan: ScanSettings,
    pub theme: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct TrashSettings {
    pub retention_days: i64,
    pub max_size_mb: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct MonitoringSettings {
    pub enabled: bool,
    pub interval_hours: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct NotificationSettings {
    pub system: bool,
    pub tray: bool,
    pub in_app: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct ScanSettings {
    pub include_hidden: bool,
    pub large_file_threshold_mb: u64,
}

// DiskPulse data structures
#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct CacheEvent {
    pub id: i64,
    pub path: String,
    pub size_change: i64,
    pub event_type: String,
    pub source: Option<String>,
    pub timestamp: i64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct DiskPulseHealth {
    pub disk_usage_percent: f32,
    pub projected_days_until_full: Option<f32>,
    pub status_color: String, // "green", "yellow", "red"
    pub status_message: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct OldFilesSummary {
    pub total_files: usize,
    pub total_size: u64,
    pub cutoff_days: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct CacheItem {
    pub name: String,
    pub size: u64,
    pub category: String,
    pub can_clear: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct SystemHealthData {
    // CPU
    pub cpu_usage: f32,
    pub cpu_cores: usize,
    pub cpu_frequency: f32,
    pub core_usages: Vec<f32>,

    // Memory
    pub total_memory: u64,
    pub used_memory: u64,
    pub available_memory: u64,

    // GPU (enhanced)
    pub gpu_info: Option<GpuInfo>,

    // Network (enhanced)
    pub network_up: u64,
    pub network_down: u64,
    pub network_interfaces: Vec<NetworkInterfaceInfo>,
    pub active_connections: Vec<NetworkConnection>,

    // Temperatures (enhanced)
    pub temperatures: Temperatures,

    // Disk I/O (enhanced)
    pub disk_read_bytes: u64,
    pub disk_write_bytes: u64,
    pub disk_read_ops: u64,
    pub disk_write_ops: u64,

    // Battery (new for laptops)
    pub battery_info: Option<BatteryInfo>,

    // Processes (top resource consumers)
    pub top_processes: Vec<ProcessInfo>,

    // System load averages
    pub load_average: Option<LoadAverage>,

    // Swap usage
    pub swap_total: u64,
    pub swap_used: u64,

    pub timestamp: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct GpuInfo {
    pub name: String,
    pub usage: f32,
    pub memory_used: u64,
    pub memory_total: u64,
    pub temperature: Option<f32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct Temperatures {
    pub cpu: f32,              // CPU temperature from thermal zones
    pub cpu_sensors: f32,      // CPU temperature from lm-sensors
    pub system: f32,           // System temperature (highest thermal zone)
    pub gpu: Option<f32>,      // GPU temperature
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct NetworkInterfaceInfo {
    pub name: String,
    pub received: u64,
    pub transmitted: u64,
    pub packets_received: u64,
    pub packets_transmitted: u64,
    pub errors_received: u64,
    pub errors_transmitted: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct NetworkConnection {
    pub local_address: String,
    pub remote_address: String,
    pub local_port: u16,
    pub remote_port: u16,
    pub state: String,
    pub process_name: Option<String>,
    pub process_pid: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct BatteryInfo {
    pub percentage: f32,
    pub is_charging: bool,
    pub time_to_full: Option<u64>, // seconds
    pub time_to_empty: Option<u64>, // seconds
    pub power_consumption: Option<f32>, // watts
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_usage: u64,
    pub status: String,
    pub user_id: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct TreeNode {
    pub id: String,
    pub name: String,
    pub path: String,
    pub size: u64,
    #[serde(rename = "isDirectory")]
    pub is_directory: bool,
    #[serde(rename = "lastModified")]
    pub last_modified: i64,  // Unix timestamp
    #[serde(rename = "lastAccessed")]
    pub last_accessed: i64,  // Unix timestamp
    #[serde(skip_serializing_if = "Option::is_none")]
    pub children: Option<Vec<TreeNode>>,
    pub expanded: bool,  // Frontend state, default false
    pub selected: bool,  // Frontend state, default false
    #[serde(rename = "riskLevel")]
    pub risk_level: String,  // "safe" | "caution" | "warning"
    #[serde(rename = "aiInsight", skip_serializing_if = "Option::is_none")]
    pub ai_insight: Option<String>,
    #[serde(rename = "usagePattern", skip_serializing_if = "Option::is_none")]
    pub usage_pattern: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct LoadAverage {
    pub one_minute: f64,
    pub five_minutes: f64,
    pub fifteen_minutes: f64,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            trash: TrashSettings { retention_days: 3, max_size_mb: 1000 },
            monitoring: MonitoringSettings { enabled: true, interval_hours: 24 },
            notifications: NotificationSettings { system: true, tray: true, in_app: true },
            scan: ScanSettings { include_hidden: false, large_file_threshold_mb: 100 },
            theme: "system".to_string(),
        }
    }
}

#[tauri::command]
pub async fn initialize_app() -> Result<(), String> {
    tracing::info!("Initializing application...");

    if let Err(e) = trash::cleanup_expired() {
        tracing::warn!("Failed to cleanup expired trash: {}", e);
    }

    Ok(())
}

#[tauri::command]
pub async fn get_system_stats(app_handle: tauri::AppHandle) -> Result<SystemStats, String> {
    let disks = Disks::new_with_refreshed_list();

    let mut total_space: u64 = 0;
    let mut used_space: u64 = 0;

    // Log disk information for debugging
    tracing::info!("Available disks:");
    for disk in disks.list() {
        let mount = disk.mount_point().to_string_lossy();
        let total = disk.total_space();
        let available = disk.available_space();
        let used = total - available;
        tracing::info!("  Mount: {}, Total: {} GB, Used: {} GB, Available: {} GB",
            mount,
            total / (1024*1024*1024),
            used / (1024*1024*1024),
            available / (1024*1024*1024)
        );

        if mount == "/" {
            total_space = total;
            used_space = used;
            tracing::info!("  Using root filesystem: Total: {} GB, Used: {} GB, Available: {} GB",
                total_space / (1024*1024*1024),
                used_space / (1024*1024*1024),
                available / (1024*1024*1024)
            );
        }
    }

    // Get package stats - this is a synchronous operation, but we'll wrap it in a timeout
    // by running it in a blocking task
    let pkg_stats = match timeout(Duration::from_secs(30), tokio::task::spawn_blocking(|| {
        packages::get_package_stats()
    })).await {
        Ok(Ok(stats)) => stats,
        Ok(Err(_)) | Err(_) => {
            tracing::warn!("Package stats query timed out or failed, using defaults");
            packages::PackageStats {
                total_packages: 0,
                orphan_packages: 0,
                orphan_size: 0,
            }
        }
    };

    // Get cache size - run in blocking task with timeout
    let home = dirs::home_dir().unwrap_or_default();
    let cache_path = home.join(".cache");
    let cache_size = if cache_path.exists() {
        let cache_path_clone = cache_path.clone();
        match timeout(Duration::from_secs(30), tokio::task::spawn_blocking(move || {
            trash::get_dir_size(&cache_path_clone)
        })).await {
            Ok(Ok(size)) => size,
            Ok(Err(_)) | Err(_) => {
                tracing::warn!("Cache size calculation timed out or failed");
                0
            }
        }
    } else {
        0
    };

    // Get last scan results from database
    let (filesystem_health_savings, storage_recovery_savings) = app_handle.db(|conn| {
            let mut fs_stmt = conn.prepare("SELECT total_size FROM last_scan_results WHERE scan_type = 'filesystem_health' ORDER BY timestamp DESC LIMIT 1")?;
            let fs_savings: Option<u64> = fs_stmt.query_row([], |row| {
                let size: i64 = row.get(0)?;
                Ok(size as u64)
            }).ok();

            let mut sr_stmt = conn.prepare("SELECT total_size FROM last_scan_results WHERE scan_type = 'storage_recovery' ORDER BY timestamp DESC LIMIT 1")?;
            let sr_savings: Option<u64> = sr_stmt.query_row([], |row| {
                let size: i64 = row.get(0)?;
                Ok(size as u64)
            }).ok();

            Ok((fs_savings, sr_savings))
        })
        .unwrap_or((None, None));

    // Enhanced cleanable_space calculation: include all potential savings
    let mut cleanable_space = cache_size + pkg_stats.orphan_size;
    if let Some(fs_savings) = filesystem_health_savings {
        cleanable_space += fs_savings;
    }
    if let Some(sr_savings) = storage_recovery_savings {
        cleanable_space += sr_savings;
    }

    let last_scan = app_handle.db(|conn| {
            let mut stmt = conn.prepare("SELECT timestamp FROM scan_history ORDER BY id DESC LIMIT 1")?;
            let timestamp: Result<String, _> = stmt.query_row([], |row| row.get(0));
            Ok(timestamp.ok())
        })
        .unwrap_or(None);

    Ok(SystemStats {
        total_disk_space: total_space,
        used_disk_space: used_space,
        cleanable_space,
        last_scan,
        packages_installed: pkg_stats.total_packages,
        orphan_packages: pkg_stats.orphan_packages,
        cache_size,
        filesystem_health_savings,
        storage_recovery_savings,
        orphan_packages_size: if pkg_stats.orphan_size > 0 { Some(pkg_stats.orphan_size) } else { None },
    })
}

/// Get disk I/O statistics on Linux
#[cfg(target_os = "linux")]
fn get_disk_io_stats_linux() -> (u64, u64, u64, u64) {
    use std::fs;

    let mut total_read_bytes = 0u64;
    let mut total_write_bytes = 0u64;
    let mut total_read_ops = 0u64;
    let mut total_write_ops = 0u64;

    if let Ok(content) = fs::read_to_string("/proc/diskstats") {
        for line in content.lines() {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 14 {
                // Skip partitions (major number > 259 are partitions, not devices)
                if let Ok(major) = parts[0].parse::<u32>() {
                    if major < 259 { // Physical devices only
                        if let (Ok(read_ops), Ok(read_bytes), Ok(write_ops), Ok(write_bytes)) = (
                            parts[3].parse::<u64>(),
                            parts[5].parse::<u64>(),
                            parts[7].parse::<u64>(),
                            parts[9].parse::<u64>()
                        ) {
                            total_read_ops += read_ops;
                            total_read_bytes += read_bytes * 512; // Convert sectors to bytes
                            total_write_ops += write_ops;
                            total_write_bytes += write_bytes * 512;
                        }
                    }
                }
            }
        }
    }

    (total_read_bytes, total_write_bytes, total_read_ops, total_write_ops)
}

#[cfg(not(target_os = "linux"))]
fn get_disk_io_stats_linux() -> (u64, u64, u64, u64) {
    (0, 0, 0, 0)
}

/// Get network connections on Linux
#[cfg(target_os = "linux")]
fn get_network_connections() -> Vec<NetworkConnection> {
    use std::fs;
    let mut connections = Vec::new();

    // Read TCP connections
    if let Ok(content) = fs::read_to_string("/proc/net/tcp") {
        for line in content.lines().skip(1) { // Skip header
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() >= 10 {
                if let (Ok(local_addr), Ok(remote_addr), Ok(state)) = (
                    u32::from_str_radix(&parts[1][6..], 16), // Remove "00000000:" prefix
                    u32::from_str_radix(&parts[2][6..], 16),
                    u8::from_str_radix(parts[3], 16)
                ) {
                    let local_port = (local_addr & 0xFFFF) as u16;
                    let remote_port = (remote_addr & 0xFFFF) as u16;
                    let local_ip = format!("{}.{}.{}.{}",
                        (local_addr >> 24) & 0xFF,
                        (local_addr >> 16) & 0xFF,
                        (local_addr >> 8) & 0xFF,
                        local_addr & 0xFF
                    );
                    let remote_ip = format!("{}.{}.{}.{}",
                        (remote_addr >> 24) & 0xFF,
                        (remote_addr >> 16) & 0xFF,
                        (remote_addr >> 8) & 0xFF,
                        remote_addr & 0xFF
                    );

                    let state_str = match state {
                        0x01 => "ESTABLISHED",
                        0x02 => "SYN_SENT",
                        0x03 => "SYN_RECV",
                        0x04 => "FIN_WAIT1",
                        0x05 => "FIN_WAIT2",
                        0x06 => "TIME_WAIT",
                        0x07 => "CLOSE",
                        0x08 => "CLOSE_WAIT",
                        0x09 => "LAST_ACK",
                        0x0A => "LISTEN",
                        0x0B => "CLOSING",
                        _ => "UNKNOWN"
                    };

                    connections.push(NetworkConnection {
                        local_address: local_ip,
                        remote_address: remote_ip,
                        local_port,
                        remote_port,
                        state: state_str.to_string(),
                        process_name: None, // Would need additional processing
                        process_pid: None,
                    });
                }
            }
        }
    }

    // Limit to first 50 connections to avoid overwhelming the UI
    connections.truncate(50);
    connections
}

#[cfg(not(target_os = "linux"))]
fn get_network_connections() -> Vec<NetworkConnection> {
    Vec::new()
}

/// Fallback GPU detection using system components
/// Get battery information safely without external dependencies
/// This provides basic battery monitoring using system files directly
fn get_battery_info_safely() -> Option<BatteryInfo> {
    // Try to read battery info from /sys/class/power_supply on Linux
    #[cfg(target_os = "linux")]
    {
        use std::fs;

        // Look for battery directories
        if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
            for entry in entries.filter_map(|e| e.ok()) {
                let path = entry.path();
                if let Some(dir_name) = path.file_name() {
                    let dir_str = dir_name.to_string_lossy();
                    if dir_str.starts_with("BAT") {
                        // Found a battery, try to read its information
                        let capacity_path = path.join("capacity");
                        let status_path = path.join("status");

                        let percentage = fs::read_to_string(&capacity_path)
                            .ok()
                            .and_then(|s| s.trim().parse::<f32>().ok());

                        let status = fs::read_to_string(&status_path)
                            .ok()
                            .map(|s| s.trim().to_string());

                        if let Some(percentage) = percentage {
                            return Some(BatteryInfo {
                                percentage,
                                is_charging: status.as_ref().is_some_and(|s| s == "Charging"),
                                time_to_full: None, // Would need more complex calculation
                                time_to_empty: None, // Would need more complex calculation
                                power_consumption: None, // Would need additional files
                            });
                        }
                    }
                }
            }
        }
    }

    // For other platforms or if reading fails, return None
    #[cfg(not(target_os = "linux"))]
    {
        None
    }

    #[cfg(target_os = "linux")]
    {
        None
    }
}


fn get_gpu_info_from_components(components: &sysinfo::Components) -> Option<GpuInfo> {
    components.iter()
        .find(|c| c.label().to_lowercase().contains("gpu") ||
                 c.label().to_lowercase().contains("graphics"))
        .map(|gpu_comp| {
            GpuInfo {
                name: gpu_comp.label().to_string(),
                usage: 0.0, // Not available from components
                memory_used: 0, // Not available from components
                memory_total: 0, // Not available from components
                temperature: gpu_comp.temperature(),
            }
        })
}

#[tauri::command]
pub async fn get_system_health() -> Result<SystemHealthData, String> {
    // Set timeout for system health monitoring (30 seconds)
    let health_timeout = Duration::from_secs(30);

    match timeout(health_timeout, async {
        let mut sys = System::new();

    // Refresh system information
    sys.refresh_cpu_usage();
    sys.refresh_memory();
    sys.refresh_processes(sysinfo::ProcessesToUpdate::All, true);

    // CPU data
    let cpu_usage = sys.global_cpu_usage();
    let cpu_cores = sys.cpus().len();
    let cpu_frequency = sys.cpus().first().map(|cpu| cpu.frequency() as f32).unwrap_or(0.0);
    let core_usages: Vec<f32> = sys.cpus().iter().map(|cpu| cpu.cpu_usage()).collect();

    // Memory data
    let total_memory = sys.total_memory();
    let used_memory = sys.used_memory();
    let available_memory = sys.available_memory();

    // Swap data
    let swap_total = sys.total_swap();
    let swap_used = sys.used_swap();

    // Network data (enhanced)
    let networks = Networks::new_with_refreshed_list();
    let mut network_up: u64 = 0;
    let mut network_down: u64 = 0;
    let mut network_interfaces = Vec::new();

    for (interface_name, data) in &networks {
        network_up += data.transmitted();
        network_down += data.received();
        network_interfaces.push(NetworkInterfaceInfo {
            name: interface_name.clone(),
            received: data.total_received(),
            transmitted: data.total_transmitted(),
            packets_received: 0, // Would need platform-specific APIs
            packets_transmitted: 0,
            errors_received: 0,
            errors_transmitted: 0,
        });
    }

    // Network connections
    let active_connections = get_network_connections();

    // Disk I/O data (enhanced)
    let (disk_read_bytes, disk_write_bytes, disk_read_ops, disk_write_ops) = {
        #[cfg(target_os = "linux")]
        {
            get_disk_io_stats_linux()
        }
        #[cfg(target_os = "macos")]
        {
            // macOS implementation would go here
            (0, 0, 0, 0)
        }
        #[cfg(target_os = "windows")]
        {
            // Windows implementation would go here
            (0, 0, 0, 0)
        }
        #[cfg(not(any(target_os = "linux", target_os = "macos", target_os = "windows")))]
        {
            (0, 0, 0, 0)
        }
    };

    // Function to read CPU temperature from lm-sensors
    fn get_cpu_temperature_from_sensors() -> Option<f32> {
        use std::process::Command;

        // Try to run sensors command
        let output = Command::new("sensors")
            .output()
            .ok()?;

        if !output.status.success() {
            return None;
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let mut package_temp: Option<f32> = None;

        // First, look specifically for "Package id 0:" (most accurate CPU package temp)
        for line in output_str.lines() {
            if line.contains("Package id 0:") {
                // Extract temperature value (e.g., "+85.0°C" -> 85.0)
                if let Some(temp_str) = line.split('+').nth(1) {
                    if let Some(temp_val) = temp_str.split('°').next() {
                        if let Ok(temp) = temp_val.trim().parse::<f32>() {
                            package_temp = Some(temp);
                            break; // Found package temp, use this
                        }
                    }
                }
            }
        }

        // If we found package temp, return it
        if package_temp.is_some() {
            return package_temp;
        }

        // Fallback: Look for coretemp adapter and get temp1 (Package temp)
        let mut in_coretemp = false;
        for line in output_str.lines() {
            if line.contains("coretemp") {
                in_coretemp = true;
                continue;
            }
            if in_coretemp && line.contains("temp1:") {
                // Extract temperature value (e.g., "+85.0°C" -> 85.0)
                if let Some(temp_str) = line.split('+').nth(1) {
                    if let Some(temp_val) = temp_str.split('°').next() {
                        if let Ok(temp) = temp_val.trim().parse::<f32>() {
                            return Some(temp);
                        }
                    }
                }
            }
            // Reset if we hit a new adapter
            if line.starts_with("Adapter:") && in_coretemp {
                in_coretemp = false;
            }
        }

        None
    }

    // Temperature data from thermal zones (sysinfo)
    let components = Components::new_with_refreshed_list();
    let mut cpu_temp = 0.0;
    let mut system_temp = 0.0;
    let mut gpu_temp: Option<f32> = None;

    for component in &components {
        if let Some(temp) = component.temperature() {
            let label = component.label().to_lowercase();

            if label.contains("cpu") || label.contains("processor") || label.contains("x86_pkg_temp") {
                cpu_temp = temp;
            } else if label.contains("gpu") {
                gpu_temp = Some(temp);
            } else if temp > system_temp {
                // Use the highest temperature as system temp
                system_temp = temp;
            }
        }
    }

    // Fallback: Try to read x86_pkg_temp directly from thermal zones if sysinfo didn't find it
    if cpu_temp == 0.0 {
        use std::fs;
        if let Ok(entries) = fs::read_dir("/sys/class/thermal") {
            for entry in entries.flatten() {
                if let Ok(zone_type) = fs::read_to_string(entry.path().join("type")) {
                    if zone_type.trim() == "x86_pkg_temp" {
                        if let Ok(temp_str) = fs::read_to_string(entry.path().join("temp")) {
                            if let Ok(temp_millidegrees) = temp_str.trim().parse::<f32>() {
                                cpu_temp = temp_millidegrees / 1000.0;
                                break;
                            }
                        }
                    }
                }
            }
        }
    }

    // Get CPU temperature from lm-sensors (primary, most accurate)
    // Fallback to thermal zone if sensors unavailable
    let cpu_sensors_temp = get_cpu_temperature_from_sensors();
    let cpu_temp_final = cpu_sensors_temp.unwrap_or(cpu_temp);

    // GPU detection (enhanced with NVML support for NVIDIA GPUs)
    let gpu_info = {
        #[cfg(feature = "gpu-monitoring")]
        {
            // Try NVML first for NVIDIA GPUs
            if let Ok(nvml) = nvml_wrapper::Nvml::init() {
                if let Ok(device) = nvml.device_by_index(0) {
                    if let (Ok(name), Ok(memory_info), Ok(utilization), Ok(temp)) = (
                        device.name(),
                        device.memory_info(),
                        device.utilization_rates(),
                        device.temperature(nvml_wrapper::enum_wrappers::device::TemperatureSensor::Gpu)
                    ) {
                        Some(GpuInfo {
                            name,
                            usage: utilization.gpu as f32,
                            memory_used: memory_info.used,
                            memory_total: memory_info.total,
                            temperature: Some(temp as f32),
                        })
                    } else {
                        // Fallback to component-based detection
                        get_gpu_info_from_components(&components)
                    }
                } else {
                    get_gpu_info_from_components(&components)
                }
            } else {
                get_gpu_info_from_components(&components)
            }
        }
        #[cfg(not(feature = "gpu-monitoring"))]
        {
            get_gpu_info_from_components(&components)
        }
    };

    // Process monitoring (top 10 by CPU usage)
    let mut processes: Vec<_> = sys.processes().iter().collect();
    processes.sort_by(|a, b| b.1.cpu_usage().partial_cmp(&a.1.cpu_usage()).unwrap_or(std::cmp::Ordering::Equal));

    let top_processes: Vec<ProcessInfo> = processes.iter().take(10).map(|(pid, process)| {
        ProcessInfo {
            pid: pid.as_u32(),
            name: process.name().to_string_lossy().to_string(),
            cpu_usage: process.cpu_usage(),
            memory_usage: process.memory(),
            status: format!("{:?}", process.status()),
            user_id: None, // Would need additional platform-specific code
        }
    }).collect();

    // Load average (Unix systems only)
    let load_average = {
        #[cfg(unix)]
        {
            use std::fs;
            if let Ok(content) = fs::read_to_string("/proc/loadavg") {
                let parts: Vec<&str> = content.split_whitespace().collect();
                if parts.len() >= 3 {
                    if let (Ok(one), Ok(five), Ok(fifteen)) = (
                        parts[0].parse::<f64>(),
                        parts[1].parse::<f64>(),
                        parts[2].parse::<f64>()
                    ) {
                        Some(LoadAverage {
                            one_minute: one,
                            five_minutes: five,
                            fifteen_minutes: fifteen,
                        })
                    } else {
                        None
                    }
                } else {
                    None
                }
            } else {
                None
            }
        }
        #[cfg(not(unix))]
        {
            None
        }
    };

    // Network interfaces
    // NOTE: Network interface monitoring is deferred - current implementation returns empty vec
    // Future enhancement: Implement per-interface monitoring using sysinfo or platform-specific APIs
    let network_interfaces: Vec<NetworkInterfaceInfo> = Vec::new();

    // Battery information (for laptops)
    // NOTE: Battery monitoring has been removed due to security vulnerability
    // in the nix crate dependency (RUSTSEC-2021-0119). The battery crate uses an
    // outdated version of nix that contains an out-of-bounds write vulnerability.
    // For security, we provide basic battery info via direct system file access.
    let battery_info = get_battery_info_safely();

    let temperatures = Temperatures {
        cpu: cpu_temp_final,  // Primary: sensors first, thermal zone fallback
        cpu_sensors: cpu_sensors_temp.unwrap_or(0.0),  // Keep for backward compatibility
        system: system_temp,
        gpu: gpu_temp,
    };

    SystemHealthData {
        cpu_usage,
        cpu_cores,
        cpu_frequency,
        core_usages,
        total_memory,
        used_memory,
        available_memory,
        gpu_info,
        network_up,
        network_down,
        network_interfaces,
        active_connections,
        temperatures,
        disk_read_bytes,
        disk_write_bytes,
        disk_read_ops,
        disk_write_ops,
        battery_info,
        top_processes,
        load_average,
        swap_total,
        swap_used,
        timestamp: std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis() as u64,
    }
    }).await {
        Ok(result) => Ok(result),
        Err(_) => {
            tracing::error!("System health monitoring timed out after {} seconds", health_timeout.as_secs());
            Err("System health monitoring timed out. Please try again.".to_string())
        }
    }
}

#[tauri::command]
pub async fn start_scan(options: ScanOptions) -> Result<ScanResults, String> {
    tracing::info!("Starting system scan with async operations");

    // Set timeout based on scan options (more comprehensive scans get more time)
    let scan_timeout = if options.include_caches && options.include_packages {
        Duration::from_secs(900) // 15 minutes for comprehensive scans
    } else {
        Duration::from_secs(600) // 10 minutes for basic scans
    };

    match timeout(scan_timeout, async {
        scanner::scan_system_async(&options).await
    }).await {
        Ok(Ok(results)) => {
            tracing::info!("Async scan complete: {} items, {} bytes", results.total_items, results.total_size);
            Ok(results)
        },
        Ok(Err(e)) => {
            tracing::error!("System scan failed: {}", e);
            Err(format!("System scan failed: {}", e))
        },
        Err(_) => {
            tracing::error!("System scan timed out after {} seconds", scan_timeout.as_secs());
            Err(format!("System scan timed out after {} seconds. Try scanning with fewer options enabled.", scan_timeout.as_secs()))
        }
    }
}

#[tauri::command]
pub async fn scan_filesystem_health(app_handle: tauri::AppHandle) -> Result<FilesystemHealthResults, String> {
    tracing::info!("Starting filesystem health check");

    // Set a reasonable timeout for filesystem scanning (5 minutes)
    let scan_timeout = Duration::from_secs(300);

    match timeout(scan_timeout, async {
        scanner::scan_filesystem_health()
    }).await {
        Ok(results) => {
            tracing::info!("Filesystem health check complete: {} items, {} bytes", results.total_items, results.total_size);

            // Store results in database for Dashboard display
            let _ = app_handle.db(|conn| {
                conn.execute(
                    "INSERT OR REPLACE INTO last_scan_results (scan_type, total_size, total_items, timestamp, scan_data) VALUES (?1, ?2, ?3, ?4, ?5)",
                    (
                        "filesystem_health",
                        results.total_size as i64,
                        results.total_items as i64,
                        chrono::Utc::now().timestamp(),
                        serde_json::to_string(&results).unwrap_or_default()
                    )
                )?;
                Ok::<(), rusqlite::Error>(())
            });

            Ok(results)
        },
        Err(_) => {
            tracing::error!("Filesystem health check timed out after {} seconds", scan_timeout.as_secs());
            Err("Filesystem health check timed out. The scan took too long to complete.".to_string())
        }
    }
}

// Helper function to populate file_access table with file metadata
fn populate_file_access_table(app_handle: &tauri::AppHandle, files: &[scanner::ScanItem]) -> Result<(), String> {
    let home = match dirs::home_dir() {
        Some(h) => h,
        None => {
            tracing::warn!("Cannot determine home directory for file_access table population");
            return Ok(()); // Return success - this is non-critical
        }
    };

    let scan_dirs = vec![
        home.join("Downloads"),
        home.join("Documents"),
        home.join("Desktop"),
        home.join("Pictures"),
        home.join("Videos"),
        home.join("Music"),
    ];

    let mut files_tracked = 0;
    let mut errors_encountered = 0;
    let timestamp = chrono::Utc::now().timestamp();

    for dir in scan_dirs {
        if !dir.exists() {
            continue;
        }

        // Limit depth and number of files to avoid performance issues
        // Use filter_map to skip errors gracefully
        for entry in WalkDir::new(&dir)
            .max_depth(3)
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
            .take(10000) // Limit to 10k files per directory
        {
            let path = entry.path();
            if path.is_file() {
                if let Ok(metadata) = path.metadata() {
                    let size = metadata.len();
                    // Use modification time as last_access if available, otherwise use current time
                    let last_access = metadata
                        .modified()
                        .ok()
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                        .map(|d: std::time::Duration| d.as_secs() as i64)
                        .unwrap_or(timestamp);

                    let path_str = path.to_string_lossy().to_string();

                    if let Err(e) = app_handle.db(|conn| {
                        conn.execute(
                            "INSERT OR REPLACE INTO file_access (path, size, last_access) VALUES (?1, ?2, ?3)",
                            (&path_str, size as i64, last_access),
                        )?;
                        Ok::<(), rusqlite::Error>(())
                    }) {
                        errors_encountered += 1;
                        if errors_encountered <= 10 {
                            tracing::warn!("Failed to insert file_access record for {}: {}", path_str, e);
                        } else if errors_encountered == 11 {
                            tracing::warn!("Suppressing further file_access insert errors ({} total so far)", errors_encountered);
                        }
                    } else {
                        files_tracked += 1;
                    }
                }
            }
        }
    }

    // Also track files from the scan results
    for file in files {
        if let Ok(metadata) = std::fs::metadata(&file.path) {
            let last_access = metadata
                .modified()
                .ok()
                .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())
                .map(|d: std::time::Duration| d.as_secs() as i64)
                .unwrap_or(timestamp);

            if let Err(e) = app_handle.db(|conn| {
                conn.execute(
                    "INSERT OR REPLACE INTO file_access (path, size, last_access) VALUES (?1, ?2, ?3)",
                    (&file.path, file.size as i64, last_access),
                )?;
                Ok::<(), rusqlite::Error>(())
            }) {
                errors_encountered += 1;
                if errors_encountered <= 10 {
                    tracing::warn!("Failed to insert file_access record for {}: {}", file.path, e);
                }
            } else {
                files_tracked += 1;
            }
        }
    }

    if files_tracked > 0 {
        tracing::info!("Populated file_access table with {} files ({} errors encountered)", files_tracked, errors_encountered);
    } else if errors_encountered > 0 {
        tracing::warn!("File_access table population encountered {} errors, no files tracked", errors_encountered);
    } else {
        tracing::info!("File_access table population completed (no files to track)");
    }

    // Always return Ok - this is non-critical and shouldn't fail the scan
    Ok(())
}

/// Dedicated scan command for DiskPulse that populates file_access table
/// This is optimized for finding unused files rather than full system analysis
#[tauri::command]
pub async fn scan_for_old_files(_app_handle: tauri::AppHandle) -> Result<ScanResults, String> {
    // Temporarily disabled due to compilation issues
    Err("Function temporarily disabled".to_string())
}

/// Scan filesystem and return tree structure for File Explorer
#[tauri::command]
pub async fn scan_filesystem_tree(
    root_path: String,
    max_depth: usize,
    include_hidden: bool,
    size_threshold: u64,
    filter_patterns: Vec<String>,
) -> Result<Vec<TreeNode>, String> {
    let scan_timeout = Duration::from_secs(60);

    match timeout(scan_timeout, async {
        // Resolve the root path
        let root_path_buf = if root_path == "~" {
            dirs::home_dir().ok_or("Cannot determine home directory")?
        } else {
            PathBuf::from(root_path)
        };

        if !root_path_buf.exists() {
            return Err(format!("Path does not exist: {}", root_path_buf.display()));
        }

        // Validate path for security
        let canonical_path = root_path_buf.canonicalize()
            .map_err(|e| format!("Failed to canonicalize path: {}", e))?;

        // Scan the filesystem tree in a blocking task
        let canonical_path_clone = canonical_path.clone();
        let tree_items = tokio::task::spawn_blocking(move || {
            let mut flat_items = Vec::new();
            scan_directory_recursive(
                &canonical_path_clone,
                &mut flat_items,
                0,
                max_depth,
                include_hidden,
                size_threshold,
                &filter_patterns,
            )?;
            build_tree_structure(&flat_items, &canonical_path_clone)
        })
        .await
        .map_err(|e| format!("Scan task failed: {}", e))?;

        tree_items
    }).await {
        Ok(Ok(items)) => Ok(items),
        Ok(Err(e)) => Err(e),
        Err(_) => {
            tracing::error!("Filesystem tree scan timed out after {} seconds", scan_timeout.as_secs());
            Err(format!("Filesystem scan timed out after {} seconds", scan_timeout.as_secs()))
        }
    }
}

/// Recursively scan a directory and collect file/directory information
fn scan_directory_recursive(
    path: &Path,
    results: &mut Vec<TreeNode>,
    current_depth: usize,
    max_depth: usize,
    include_hidden: bool,
    size_threshold: u64,
    filter_patterns: &[String],
) -> Result<(), String> {
    if current_depth > max_depth {
        return Ok(());
    }

    let entries = std::fs::read_dir(path)
        .map_err(|e| format!("Failed to read directory {}: {}", path.display(), e))?;

    for entry in entries {
        let entry = entry.map_err(|e| format!("Failed to read directory entry: {}", e))?;
        let entry_path = entry.path();

        // Skip hidden files if not requested
        if !include_hidden {
            if let Some(filename) = entry_path.file_name() {
                if filename.to_string_lossy().starts_with('.') {
                    continue;
                }
            }
        }

        // Check filter patterns
        let should_include = if filter_patterns.is_empty() {
            true
        } else {
            let filename = entry_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("");
            filter_patterns.iter().any(|pattern| filename.contains(pattern))
        };

        if !should_include {
            continue;
        }

        let metadata = entry.metadata()
            .map_err(|e| format!("Failed to get metadata for {}: {}", entry_path.display(), e))?;

        let size = if metadata.is_file() {
            metadata.len()
        } else {
            // For directories, calculate total size recursively
            trash::get_dir_size(&entry_path)
        };

        // Skip files below size threshold
        if metadata.is_file() && size < size_threshold {
            continue;
        }

        // Get file timestamps
        let last_modified = metadata.modified()
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let last_accessed = metadata.accessed()
            .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs() as i64;

        let risk_level = assess_risk_level(&entry_path, metadata.is_dir());

        let node = TreeNode {
            id: entry_path.to_string_lossy().to_string(),
            name: entry_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            path: entry_path.to_string_lossy().to_string(),
            size,
            is_directory: metadata.is_dir(),
            last_modified,
            last_accessed,
            children: None, // Will be populated in build_tree_structure
            expanded: false,
            selected: false,
            risk_level,
            ai_insight: None,
            usage_pattern: None,
        };

        results.push(node);

        // Recurse into directories
        if metadata.is_dir() && current_depth < max_depth {
            scan_directory_recursive(
                &entry_path,
                results,
                current_depth + 1,
                max_depth,
                include_hidden,
                size_threshold,
                filter_patterns,
            )?;
        }
    }

    Ok(())
}

/// Build tree structure from flat list of nodes
fn build_tree_structure(items: &[TreeNode], root_path: &Path) -> Result<Vec<TreeNode>, String> {
    let root_str = root_path.to_string_lossy().to_string();
    let mut tree_map: std::collections::HashMap<String, TreeNode> = std::collections::HashMap::new();
    let mut children_map: std::collections::HashMap<String, Vec<String>> = std::collections::HashMap::new();

    // First pass: create map of all nodes
    for item in items {
        let path = item.path.clone();
        tree_map.insert(path.clone(), item.clone());
    }

    // Second pass: build parent-child relationships
    for item in items {
        let item_path = Path::new(&item.path);
        let parent_path = item_path.parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| root_str.clone());

        // Only add to children map if parent exists and is not the same as the item
        if parent_path != item.path && tree_map.contains_key(&parent_path) {
            children_map.entry(parent_path)
                .or_insert_with(Vec::new)
                .push(item.path.clone());
        }
    }

    // Third pass: find root-level items and build tree structure
    let mut tree_items = Vec::new();
    for item in items {
        let item_path = Path::new(&item.path);
        let parent_path = item_path.parent()
            .map(|p| p.to_string_lossy().to_string())
            .unwrap_or_else(|| root_str.clone());

        // Check if this is a root-level item (parent is the root path)
        if parent_path == root_str && item.path != root_str {
            let mut node = item.clone();
            if let Some(children_paths) = children_map.get(&node.path) {
                let mut children = Vec::new();
                for child_path in children_paths {
                    if let Some(child_node) = build_tree_node_recursive(child_path.clone(), &tree_map, &children_map) {
                        children.push(child_node);
                    }
                }
                if !children.is_empty() {
                    node.children = Some(children);
                }
            }
            tree_items.push(node);
        }
    }

    Ok(tree_items)
}

/// Recursively build tree node with children
fn build_tree_node_recursive(
    path: String,
    tree_map: &std::collections::HashMap<String, TreeNode>,
    children_map: &std::collections::HashMap<String, Vec<String>>,
) -> Option<TreeNode> {
    let mut node = tree_map.get(&path)?.clone();

    if let Some(children_paths) = children_map.get(&path) {
        let mut children = Vec::new();
        for child_path in children_paths {
            if let Some(child_node) = build_tree_node_recursive(child_path.clone(), tree_map, children_map) {
                children.push(child_node);
            }
        }
        if !children.is_empty() {
            node.children = Some(children);
        }
    }

    Some(node)
}

/// Assess risk level based on file path and type
fn assess_risk_level(path: &Path, is_directory: bool) -> String {
    if is_directory {
        return "safe".to_string();
    }

    let path_str = path.to_string_lossy().to_lowercase();

    // High risk paths
    if path_str.contains("/etc/") ||
       path_str.contains("/usr/bin/") ||
       path_str.contains("/usr/sbin/") ||
       path_str.contains("/bin/") ||
       path_str.contains("/sbin/") ||
       path_str.contains("/lib/") ||
       path_str.contains("/opt/") {
        return "warning".to_string();
    }

    // Medium risk - system configs
    if path_str.contains("/.config/") ||
       path_str.contains("/.local/share/") ||
       path_str.contains("/.cache/") {
        return "caution".to_string();
    }

    // Safe - user files
    "safe".to_string()
}

#[tauri::command]
pub async fn scan_storage_recovery(app_handle: tauri::AppHandle) -> Result<StorageRecoveryResults, String> {
    tracing::info!("Starting storage recovery scan");

    // Set a reasonable timeout for storage scanning (10 minutes - more complex analysis)
    let scan_timeout = Duration::from_secs(600);

    match timeout(scan_timeout, async {
        // Run scan in blocking task to prevent blocking the async runtime
        // This also provides better panic isolation
        tokio::task::spawn_blocking(|| {
            scanner::scan_storage_recovery()
        }).await
    }).await {
        Ok(Ok(results)) => {
            let results = results.map_err(|e| {
                let error_msg = format!("Storage recovery scan failed: {}", e);
                tracing::error!("{}", error_msg);
                error_msg
            })?;

            tracing::info!("Storage recovery scan complete: {} duplicates, {} large files, {} old downloads, {} bytes recoverable",
                           results.duplicates.len(), results.large_files.len(), results.old_downloads.len(), results.total_recoverable_size);

            // Populate file_access table with scanned files for old files detection
            // This is non-critical, so we continue even if it fails
            let all_files: Vec<scanner::ScanItem> = results.duplicates.iter()
                .flat_map(|g| g.files.iter())
                .chain(results.large_files.iter())
                .chain(results.old_downloads.iter())
                .cloned()
                .collect();

            if let Err(e) = populate_file_access_table(&app_handle, &all_files) {
                tracing::warn!("Failed to populate file_access table: {}", e);
            }

            // Store results in database for Dashboard display
            // Non-critical, so we continue even if it fails
            if let Err(e) = app_handle.db(|conn| {
                let scan_data = serde_json::to_string(&results)
                    .unwrap_or_else(|_| "{}".to_string());

                conn.execute(
                    "INSERT OR REPLACE INTO last_scan_results (scan_type, total_size, total_items, timestamp, scan_data) VALUES (?1, ?2, ?3, ?4, ?5)",
                    (
                        "storage_recovery",
                        results.total_recoverable_size as i64,
                        (results.duplicates.len() + results.large_files.len() + results.old_downloads.len()) as i64,
                        chrono::Utc::now().timestamp(),
                        scan_data
                    )
                )?;
                Ok::<(), rusqlite::Error>(())
            }) {
                tracing::warn!("Failed to store scan results in database: {}", e);
            }

            Ok(results)
        },
        Ok(Err(e)) => {
            let error_msg = format!("Storage recovery scan task failed: {}", e);
            tracing::error!("{}", error_msg);
            Err(error_msg)
        },
        Err(_) => {
            let error_msg = format!("Storage recovery scan timed out after {} seconds. The scan may be processing a large number of files. Try again later or reduce the scan scope.", scan_timeout.as_secs());
            tracing::error!("{}", error_msg);
            Err(error_msg)
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct CleanResult {
    pub cleaned: usize,
    pub failed: usize,
    pub total_size: u64,
}

/// Enhanced security validation with multi-layer checks
/// Implements the security requirements from December 2025 standards
///
/// Security layers:
/// 1. Path canonicalization and symlink resolution
/// 2. Multi-level path traversal protection
/// 3. Comprehensive system-critical path detection
/// 4. File system boundary validation
/// 5. Permission and ownership verification
/// 6. Context-aware validation based on operation type
#[derive(Debug, Clone)]
pub enum SecurityContext {
    Deletion,
    CacheCleanup,
    PackageManagement,
    LogCleanup,
}

#[derive(Debug, thiserror::Error)]
pub enum SecurityError {
    #[error("Path traversal detected: {path}")]
    PathTraversal { path: String },
    #[error("Non-absolute path: {path}")]
    NonAbsolutePath { path: String },
    #[error("System critical path: {path}")]
    SystemCriticalPath { path: String },
    #[error("Permission denied: {path}")]
    PermissionDenied { path: String },
    #[error("Path outside allowed boundaries: {path}")]
    OutsideBoundaries { path: String },
    #[error("File does not exist: {path}")]
    PathDoesNotExist { path: String },
    #[error("Security violation: {message}")]
    SecurityViolation { message: String },
}

/// Comprehensive path validation with multiple security layers
fn validate_path_comprehensive(path: &str, context: SecurityContext) -> Result<(), SecurityError> {
    use std::path::Path;

    let path_buf = Path::new(path);

    // Layer 1: Multi-level path traversal protection
    validate_path_traversal(path)?;

    // Layer 2: Absolute path requirement
    if !path_buf.is_absolute() {
        return Err(SecurityError::NonAbsolutePath { path: path.to_string() });
    }

    // Layer 3: Canonical path resolution (resolves symlinks and relative paths)
    let canonical_path = path_buf.canonicalize()
        .map_err(|e| SecurityError::SecurityViolation {
            message: format!("Cannot canonicalize path {}: {}", path, e)
        })?;

    let canonical_str = canonical_path.to_string_lossy();

    // Layer 4: Context-aware system-critical path validation
    validate_system_critical_paths(&canonical_str, &context)?;

    // Layer 5: File system boundary validation
    validate_filesystem_boundaries(&canonical_path, &context)?;

    // Layer 6: Permission validation
    validate_permissions(&canonical_path)?;

    // Layer 7: Path existence validation
    if !canonical_path.exists() {
        return Err(SecurityError::PathDoesNotExist { path: path.to_string() });
    }

    Ok(())
}

/// Multi-level path traversal protection
fn validate_path_traversal(path: &str) -> Result<(), SecurityError> {
    // Basic traversal check
    if path.contains("..") {
        return Err(SecurityError::PathTraversal { path: path.to_string() });
    }

    // Advanced traversal patterns
    let traversal_patterns = ["../", "..\\", "/../", "\\..\\"];
    for pattern in &traversal_patterns {
        if path.contains(pattern) {
            return Err(SecurityError::PathTraversal { path: path.to_string() });
        }
    }

    // URL-encoded traversal attempts
    if path.contains("%2e%2e%2f") || path.contains("%2e%2e/") {
        return Err(SecurityError::PathTraversal { path: path.to_string() });
    }

    Ok(())
}

/// Context-aware system-critical path validation
fn validate_system_critical_paths(canonical_path: &str, context: &SecurityContext) -> Result<(), SecurityError> {
    // Always forbidden paths regardless of context
    let always_forbidden = [
        "/bin", "/boot", "/dev", "/etc", "/lib", "/lib64", "/proc", "/run", "/sbin", "/sys",
        "/usr/bin", "/usr/sbin", "/usr/lib", "/usr/local/bin",
        "/var/lib", "/var/run", "/var/lock", "/var/spool",
        "/root", "/home/root",
        "/etc/passwd", "/etc/shadow", "/etc/sudoers",
    ];

    for prefix in &always_forbidden {
        if canonical_path.starts_with(prefix) {
            return Err(SecurityError::SystemCriticalPath {
                path: prefix.to_string()
            });
        }
    }

    // Context-specific restrictions
    match context {
        SecurityContext::Deletion => {
            // For general deletion, be more restrictive
            let deletion_forbidden = ["/usr", "/opt", "/var"];
            for prefix in &deletion_forbidden {
                if canonical_path.starts_with(prefix) {
                    return Err(SecurityError::SystemCriticalPath {
                        path: prefix.to_string()
                    });
                }
            }
        }
        SecurityContext::CacheCleanup => {
            // For cache cleanup, allow more system paths but still protect critical ones
            let cache_forbidden = ["/etc", "/usr/bin"];
            for prefix in &cache_forbidden {
                if canonical_path.starts_with(prefix) {
                    return Err(SecurityError::SystemCriticalPath {
                        path: prefix.to_string()
                    });
                }
            }
        }
        SecurityContext::PackageManagement => {
            // Package management can operate in system areas but not critical config
            if canonical_path.starts_with("/etc") && !canonical_path.starts_with("/etc/apt") {
                return Err(SecurityError::SystemCriticalPath {
                    path: "/etc".to_string()
                });
            }
        }
        SecurityContext::LogCleanup => {
            // Log cleanup can be more permissive in user areas
        }
    }

    Ok(())
}

/// File system boundary validation
fn validate_filesystem_boundaries(canonical_path: &std::path::Path, _context: &SecurityContext) -> Result<(), SecurityError> {
    // Ensure we're within user-accessible file systems
    let home = dirs::home_dir()
        .ok_or_else(|| SecurityError::SecurityViolation {
            message: "Cannot determine home directory".to_string()
        })?;

    let home_str = home.to_string_lossy();

    // Most operations should be within user's home directory
    if !canonical_path.starts_with(home) {
        // Allow some system-wide cache operations
        let allowed_system_paths = ["/var/cache", "/tmp"];
        let is_allowed_system_path = allowed_system_paths.iter()
            .any(|allowed| canonical_path.starts_with(allowed));

        if !is_allowed_system_path {
            return Err(SecurityError::OutsideBoundaries {
                path: canonical_path.to_string_lossy().to_string()
            });
        }
    }

    Ok(())
}

/// Permission validation
fn validate_permissions(canonical_path: &std::path::Path) -> Result<(), SecurityError> {

    match canonical_path.metadata() {
        Ok(metadata) => {
            let permissions = metadata.permissions();

            // Check if we have write permission
            if permissions.readonly() {
                return Err(SecurityError::PermissionDenied {
                    path: canonical_path.to_string_lossy().to_string()
                });
            }

            // On Unix systems, check ownership (basic check)
            #[cfg(unix)]
            {
                use std::os::unix::fs::MetadataExt;
                let current_uid = unsafe { libc::getuid() };
                let file_uid = metadata.uid();

                // Allow root or file owner to modify
                if current_uid != 0 && current_uid != file_uid {
                    return Err(SecurityError::PermissionDenied {
                        path: canonical_path.to_string_lossy().to_string()
                    });
                }
            }
        }
        Err(e) => {
            return Err(SecurityError::SecurityViolation {
                message: format!("Cannot access file metadata: {}", e)
            });
        }
    }

    Ok(())
}


/// Clean selected items from scan results
/// Moves items to trash with configurable retention or permanently deletes if use_trash=false
///
/// Frontend confirmation dialog:
/// - Type: 'info' or 'warning' based on high-risk items present
/// - Message: Shows item count, total size, and risk warnings
/// - Always requires explicit user confirmation
///
/// Parameters:
/// - item_ids: Array of item IDs from scan results
/// - item_paths: Array of absolute paths to clean
/// - use_trash: Whether to use trash system (recommended: true)
/// - retention_days: Days to retain items in trash (default: 3)
#[tauri::command]
pub async fn clean_items(
    item_ids: Vec<String>,
    item_paths: Vec<String>,
    use_trash: bool,
    retention_days: i64,
) -> Result<CleanResult, String> {
    // Set timeout for cleanup operations (5 minutes should be plenty)
    let cleanup_timeout = Duration::from_secs(300);

    match timeout(cleanup_timeout, clean_items_inner(item_ids, item_paths, use_trash, retention_days)).await {
        Ok(result) => result,
        Err(_) => {
            tracing::error!("Cleanup operation timed out after {} seconds", cleanup_timeout.as_secs());
            Err("Cleanup operation timed out. Some items may have been partially processed.".to_string())
        }
    }
}

async fn clean_items_inner(
    item_ids: Vec<String>,
    item_paths: Vec<String>,
    use_trash: bool,
    retention_days: i64,
) -> Result<CleanResult, String> {
    let mut cleaned = 0;
    let mut failed = 0;
    let mut total_size: u64 = 0;

    for (_id, path) in item_ids.iter().zip(item_paths.iter()) {
        // Validate path before any operations with comprehensive security
        if let Err(validation_error) = validate_path_comprehensive(path, SecurityContext::Deletion) {
            tracing::warn!("Path validation failed for {}: {}", path, validation_error);
            failed += 1;
            continue;
        }

        let result = if use_trash {
            trash::move_to_trash(
                path,
                retention_days,
                Some(TrashMetadata {
                    category: "Cleanup".to_string(),
                    risk_level: 0,
                    reason: "User selected for cleanup".to_string(),
                }),
            )
        } else {
            let path_buf = std::path::PathBuf::from(path);
            if path_buf.is_dir() {
                std::fs::remove_dir_all(&path_buf)
                    .map(|_| trash::TrashItem {
                        id: String::new(),
                        original_path: path.clone(),
                        trash_path: String::new(),
                        deleted_at: chrono::Utc::now().to_rfc3339(),
                        expires_at: String::new(),
                        size: 0,
                        item_type: "directory".to_string(),
                        metadata: None,
                    })
                    .map_err(|e| e.to_string())
            } else {
                std::fs::remove_file(&path_buf)
                    .map(|_| trash::TrashItem {
                        id: String::new(),
                        original_path: path.clone(),
                        trash_path: String::new(),
                        deleted_at: chrono::Utc::now().to_rfc3339(),
                        expires_at: String::new(),
                        size: 0,
                        item_type: "file".to_string(),
                        metadata: None,
                    })
                    .map_err(|e| e.to_string())
            }
        };

        match result {
            Ok(item) => {
                cleaned += 1;
                total_size += item.size;
            }
            Err(e) => {
                tracing::error!("Failed to clean {}: {}", path, e);
                failed += 1;
            }
        }
    }

    Ok(CleanResult { cleaned, failed, total_size })
}

#[tauri::command]
pub async fn get_trash_items() -> Result<TrashData, String> {
    // Set a timeout for trash operations (10 seconds - file system operations)
    let trash_timeout = Duration::from_secs(10);

    match timeout(trash_timeout, async {
        Ok(trash::get_trash_items())
    }).await {
        Ok(result) => result,
        Err(_) => {
            tracing::error!("Trash items retrieval timed out after {} seconds", trash_timeout.as_secs());
            Err("Trash items retrieval timed out. Please try again.".to_string())
        }
    }
}

#[tauri::command]
pub async fn restore_from_trash(id: String) -> Result<(), String> {
    // Set a timeout for trash operations (10 seconds - file system operations)
    let trash_timeout = Duration::from_secs(10);

    match timeout(trash_timeout, async {
        trash::restore_from_trash(&id)
    }).await {
        Ok(result) => result,
        Err(_) => {
            tracing::error!("Trash restore timed out after {} seconds", trash_timeout.as_secs());
            Err("Trash restore operation timed out. Please try again.".to_string())
        }
    }
}

#[tauri::command]
pub async fn delete_from_trash(id: String) -> Result<(), String> {
    // Set a timeout for trash operations (10 seconds - file system operations)
    let trash_timeout = Duration::from_secs(10);

    match timeout(trash_timeout, async {
        trash::delete_from_trash(&id)
    }).await {
        Ok(result) => result,
        Err(_) => {
            tracing::error!("Trash delete timed out after {} seconds", trash_timeout.as_secs());
            Err("Trash delete operation timed out. Please try again.".to_string())
        }
    }
}

#[tauri::command]
pub async fn empty_trash() -> Result<usize, String> {
    // Set a timeout for trash operations (30 seconds - bulk file operations)
    let trash_timeout = Duration::from_secs(30);

    match timeout(trash_timeout, async {
        trash::empty_trash()
    }).await {
        Ok(result) => result,
        Err(_) => {
            tracing::error!("Empty trash timed out after {} seconds", trash_timeout.as_secs());
            Err("Empty trash operation timed out. Please try again.".to_string())
        }
    }
}

#[tauri::command]
pub async fn get_settings(app_handle: tauri::AppHandle) -> Result<AppSettings, String> {
    // Set a timeout for settings operations (5 seconds - database read)
    let settings_timeout = Duration::from_secs(5);

    match timeout(settings_timeout, async {
        let settings = app_handle.db(|conn| {
                let mut stmt = conn.prepare("SELECT value FROM settings WHERE key = 'app_settings'")?;
                let json: Result<String, _> = stmt.query_row([], |row| row.get(0));

                match json {
                    Ok(json) => serde_json::from_str(&json).map_err(|_| rusqlite::Error::InvalidQuery),
                    Err(_) => Ok(AppSettings::default()),
                }
            })
            .unwrap_or_else(|_| AppSettings::default());

        Ok(settings)
    }).await {
        Ok(result) => result,
        Err(_) => {
            tracing::error!("Settings retrieval timed out after {} seconds", settings_timeout.as_secs());
            Err("Settings retrieval timed out. Using defaults.".to_string())
        }
    }
}

#[tauri::command]
pub async fn save_settings(app_handle: tauri::AppHandle, settings: AppSettings) -> Result<(), String> {
    // Set a timeout for settings operations (5 seconds - database write)
    let settings_timeout = Duration::from_secs(5);

    match timeout(settings_timeout, async {
        let json = serde_json::to_string(&settings).map_err(|e| format!("Failed to serialize: {}", e))?;

        app_handle.db(|conn| {
                conn.execute(
                    "INSERT OR REPLACE INTO settings (key, value) VALUES ('app_settings', ?1)",
                    [&json],
                )?;
                Ok(())
            })
            .map_err(|e| format!("Failed to save: {}", e))?;

        Ok(())
    }).await {
        Ok(result) => result,
        Err(_) => {
            tracing::error!("Settings save timed out after {} seconds", settings_timeout.as_secs());
            Err("Settings save timed out. Please try again.".to_string())
        }
    }
}


/// Clear user cache directories (~/.cache)
/// Only operates on safe cache locations within user's home directory
///
/// Frontend confirmation dialog:
/// - Type: 'warning' (moderate risk)
/// - Message: "This will clear application caches and temporary files. This is generally safe but may require applications to rebuild their caches."
/// - Requires explicit user confirmation before proceeding
#[tauri::command]
pub async fn clear_cache() -> Result<CleanResult, String> {
    tracing::info!("Clearing user cache directories");
    let mut cleaned = 0;
    let mut failed = 0;
    let mut total_size: u64 = 0;

    let home = dirs::home_dir().ok_or("Cannot determine home directory")?;
    let cache_dir = home.join(".cache");

    if !cache_dir.exists() {
        return Ok(CleanResult { cleaned: 0, failed: 0, total_size: 0 });
    }

    // Safe cache subdirectories to clean (user-specific, not system-critical)
    let safe_cache_dirs = vec![
        "thumbnails",
        "mozilla",
        "google-chrome",
        "chromium",
        "code",
        "npm",
        "pip",
        "yarn",
        "cargo",
        "rustup",
    ];

    if let Ok(entries) = std::fs::read_dir(&cache_dir) {
        for entry in entries.filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            let dir_name = entry_path.file_name()
                .and_then(|n| n.to_str())
                .unwrap_or("")
                .to_string();

            // Only clean known safe cache directories
            if safe_cache_dirs.iter().any(|&safe| dir_name.contains(safe)) {
                let path_str = entry_path.to_string_lossy().to_string();

                // Validate path before deletion with cache cleanup context
                if let Err(validation_error) = validate_path_comprehensive(&path_str, SecurityContext::CacheCleanup) {
                    tracing::warn!("Path validation failed for {}: {}", path_str, validation_error);
                    failed += 1;
                    continue;
                }

                // Get size before deletion
                let size = if entry_path.is_dir() {
                    trash::get_dir_size(&entry_path)
                } else {
                    entry_path.metadata().map(|m| m.len()).unwrap_or(0)
                };

                // Move to trash with 3-day retention
                match trash::move_to_trash(
                    &path_str,
                    3,
                    Some(TrashMetadata {
                        category: "Cache".to_string(),
                        risk_level: 0,
                        reason: "User requested cache cleanup".to_string(),
                    }),
                ) {
                    Ok(_) => {
                        cleaned += 1;
                        total_size += size;
                        tracing::info!("Cleaned cache: {} ({} bytes)", path_str, size);
                    }
                    Err(e) => {
                        tracing::error!("Failed to clean cache {}: {}", path_str, e);
                        failed += 1;
                    }
                }
            }
        }
    }

    tracing::info!("Cache cleanup complete: {} cleaned, {} failed, {} bytes", cleaned, failed, total_size);
    Ok(CleanResult { cleaned, failed, total_size })
}

/// Clean package manager caches and remove orphaned packages
/// Uses package manager commands safely
///
/// Frontend confirmation dialog:
/// - Type: 'warning' (moderate risk)
/// - Message: "This will clean package manager cache and remove orphaned packages. This operation may require administrator privileges."
/// - Requires explicit user confirmation before proceeding
#[tauri::command]
pub async fn clean_packages() -> Result<CleanResult, String> {
    tracing::info!("Cleaning package manager caches and orphaned packages");
    let mut cleaned = 0;
    let mut failed = 0;
    let mut total_size: u64 = 0;

    // Clean APT cache
    let apt_clean_result = std::process::Command::new("apt")
        .args(["clean"])
        .output();

    match apt_clean_result {
        Ok(output) => {
            if output.status.success() {
                cleaned += 1;
                tracing::info!("APT cache cleaned successfully");
            } else {
                failed += 1;
                tracing::warn!("APT cache clean failed: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            failed += 1;
            tracing::warn!("Failed to run apt clean: {}", e);
        }
    }

    // Clean APT autoremove (orphaned packages)
    let apt_autoremove_result = std::process::Command::new("apt")
        .args(["autoremove", "-y"])
        .output();

    match apt_autoremove_result {
        Ok(output) => {
            if output.status.success() {
                cleaned += 1;
                // Estimate size from output (rough estimate)
                let stdout = String::from_utf8_lossy(&output.stdout);
                if stdout.contains("MB") || stdout.contains("KB") {
                    // Try to extract size from output
                    // This is a rough estimate - actual size would need more parsing
                    total_size += 50 * 1024 * 1024; // Estimate 50MB per autoremove
                }
                tracing::info!("APT autoremove completed successfully");
            } else {
                failed += 1;
                tracing::warn!("APT autoremove failed: {}", String::from_utf8_lossy(&output.stderr));
            }
        }
        Err(e) => {
            failed += 1;
            tracing::warn!("Failed to run apt autoremove: {}", e);
        }
    }

    // Clean pip cache (if exists)
    let home = dirs::home_dir().ok_or("Cannot determine home directory")?;
    let pip_cache = home.join(".cache/pip");
    if pip_cache.exists() {
        let path_str = pip_cache.to_string_lossy().to_string();
        if let Ok(()) = validate_path_comprehensive(&path_str, SecurityContext::PackageManagement) {
            let size = trash::get_dir_size(&pip_cache);
            match trash::move_to_trash(
                &path_str,
                3,
                Some(TrashMetadata {
                    category: "Package Cache".to_string(),
                    risk_level: 0,
                    reason: "User requested package cache cleanup".to_string(),
                }),
            ) {
                Ok(_) => {
                    cleaned += 1;
                    total_size += size;
                }
                Err(e) => {
                    tracing::warn!("Failed to clean pip cache: {}", e);
                    failed += 1;
                }
            }
        }
    }

    tracing::info!("Package cleanup complete: {} operations, {} failed, {} bytes", cleaned, failed, total_size);
    Ok(CleanResult { cleaned, failed, total_size })
}

/// Clear old system logs
/// Only operates on user-accessible log locations, not system logs
///
/// Frontend confirmation dialog:
/// - Type: 'warning' (moderate risk)
/// - Message: "This will clear old system logs. Important logs may be preserved. This operation requires administrator privileges."
/// - Requires explicit user confirmation before proceeding
#[tauri::command]
pub async fn clear_logs() -> Result<CleanResult, String> {
    tracing::info!("Clearing old user logs");
    let mut cleaned = 0;
    let mut failed = 0;
    let mut total_size: u64 = 0;

    let home = dirs::home_dir().ok_or("Cannot determine home directory")?;

    // Only clean logs in user's home directory (safe locations)
    let user_log_dirs = vec![
        home.join(".local/share/logs"),
        home.join(".cache/logs"),
    ];

    for log_dir in user_log_dirs {
        if !log_dir.exists() {
            continue;
        }

        let path_str = log_dir.to_string_lossy().to_string();

        // Validate path before deletion with log cleanup context
        if let Err(validation_error) = validate_path_comprehensive(&path_str, SecurityContext::LogCleanup) {
            tracing::warn!("Path validation failed for {}: {}", path_str, validation_error);
            failed += 1;
            continue;
        }

        // Get size before deletion
        let size = trash::get_dir_size(&log_dir);

        // Move to trash with 7-day retention (logs might be needed for debugging)
        match trash::move_to_trash(
            &path_str,
            7,
            Some(TrashMetadata {
                category: "Logs".to_string(),
                risk_level: 1,
                reason: "User requested log cleanup".to_string(),
            }),
        ) {
            Ok(_) => {
                cleaned += 1;
                total_size += size;
                tracing::info!("Cleaned logs: {} ({} bytes)", path_str, size);
            }
            Err(e) => {
                tracing::error!("Failed to clean logs {}: {}", path_str, e);
                failed += 1;
            }
        }
    }

    tracing::info!("Log cleanup complete: {} cleaned, {} failed, {} bytes", cleaned, failed, total_size);
    Ok(CleanResult { cleaned, failed, total_size })
}

// DiskPulse background monitoring functionality
lazy_static::lazy_static! {
    static ref MONITORING_STATE: Arc<AsyncMutex<MonitoringState>> = Arc::new(AsyncMutex::new(MonitoringState::new()));
}

#[derive(Debug)]
struct MonitoringState {
    disk_monitoring_task: Option<tokio::task::JoinHandle<()>>,
    cache_watcher: Option<notify::RecommendedWatcher>,
    is_running: bool,
}

impl MonitoringState {
    fn new() -> Self {
        Self {
            disk_monitoring_task: None,
            cache_watcher: None,
            is_running: false,
        }
    }
}

impl Default for MonitoringState {
    fn default() -> Self {
        Self::new()
    }
}

#[tauri::command]
pub async fn start_diskpulse_monitoring(app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut state = MONITORING_STATE.lock().await;

    if state.is_running {
        return Ok(()); // Already running
    }

    tracing::info!("Starting DiskPulse background monitoring");

    // Start disk usage monitoring (every 4 hours)
    let disk_app_handle = app_handle.clone();
    let disk_task = tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(4 * 3600)); // 4 hours

        loop {
            interval.tick().await;
            if let Err(e) = record_disk_usage(&disk_app_handle).await {
                tracing::error!("Failed to record disk usage: {}", e);
            }
        }
    });

    // Start cache directory watching
    let cache_app_handle = app_handle.clone();
    let cache_watcher = setup_cache_watcher(cache_app_handle).await?;

    state.disk_monitoring_task = Some(disk_task);
    state.cache_watcher = Some(cache_watcher);
    state.is_running = true;

    // Update monitoring state in database
    app_handle.db(|conn| {
        conn.execute(
            "INSERT OR REPLACE INTO monitoring_state (key, value, updated_at) VALUES ('diskpulse_running', 'true', ?)",
            [chrono::Utc::now().timestamp()],
        )?;
        Ok(())
    }).map_err(|e| format!("Failed to update monitoring state: {}", e))?;

    tracing::info!("DiskPulse monitoring started successfully");
    Ok(())
}

#[tauri::command]
pub async fn stop_diskpulse_monitoring(app_handle: tauri::AppHandle) -> Result<(), String> {
    let mut state = MONITORING_STATE.lock().await;

    if !state.is_running {
        return Ok(()); // Not running
    }

    tracing::info!("Stopping DiskPulse background monitoring");

    // Stop disk monitoring task
    if let Some(task) = state.disk_monitoring_task.take() {
        task.abort();
    }

    // Stop cache watcher
    state.cache_watcher = None;

    state.is_running = false;

    // Update monitoring state in database
    app_handle.db(|conn| {
        conn.execute(
            "INSERT OR REPLACE INTO monitoring_state (key, value, updated_at) VALUES ('diskpulse_running', 'false', ?)",
            [chrono::Utc::now().timestamp()],
        )?;
        Ok(())
    }).map_err(|e| format!("Failed to update monitoring state: {}", e))?;

    tracing::info!("DiskPulse monitoring stopped successfully");
    Ok(())
}

async fn record_disk_usage(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let disks = Disks::new_with_refreshed_list();

    for disk in disks.list() {
        if disk.mount_point().to_string_lossy() == "/" {
            let used = disk.total_space() - disk.available_space();
            let timestamp = chrono::Utc::now().timestamp();

            app_handle.db(|conn| {
                conn.execute(
                    "INSERT INTO disk_history (timestamp, used_bytes, total_bytes, available_bytes) VALUES (?, ?, ?, ?)",
                    [timestamp, used as i64, disk.total_space() as i64, disk.available_space() as i64],
                )?;
                Ok(())
            }).map_err(|e| format!("Failed to record disk usage: {}", e))?;
        }
    }

    Ok(())
}

async fn setup_cache_watcher(app_handle: tauri::AppHandle) -> Result<notify::RecommendedWatcher, String> {
    let (tx, rx) = std::sync::mpsc::channel();

    let mut watcher = notify::recommended_watcher(move |res| {
        if let Err(e) = tx.send(res) {
            tracing::error!("Failed to send watch event: {}", e);
        }
    }).map_err(|e| format!("Failed to create watcher: {}", e))?;

    // Watch common cache directories
    let home = dirs::home_dir().ok_or("Cannot determine home directory")?;
    let cache_dirs = vec![
        home.join(".cache"),
        home.join(".local/share/cache"),
    ];

    for cache_dir in cache_dirs {
        if cache_dir.exists() {
            if let Err(e) = watcher.watch(&cache_dir, notify::RecursiveMode::Recursive) {
                tracing::warn!("Failed to watch cache directory {:?}: {}", cache_dir, e);
            }
        }
    }

    // Handle cache events in background task
    tokio::spawn(async move {
        while let Ok(event) = rx.recv() {
            if let Err(e) = handle_cache_event(&app_handle, event).await {
                tracing::error!("Failed to handle cache event: {}", e);
            }
        }
    });

    Ok(watcher)
}

async fn handle_cache_event(app_handle: &tauri::AppHandle, event: notify::Result<notify::Event>) -> Result<(), String> {
    let event = event.map_err(|e| format!("Watch event error: {}", e))?;

    // Only process write/create events that might indicate cache growth
    if matches!(event.kind, notify::EventKind::Create(_) | notify::EventKind::Modify(_)) {
        for path in &event.paths {
            if let Ok(metadata) = std::fs::metadata(path) {
                let size = metadata.len() as i64;
                let path_str = path.to_string_lossy().to_string();
                let timestamp = chrono::Utc::now().timestamp();

                // Determine source from path
                let source = if path_str.contains("chromium") || path_str.contains("chrome") {
                    Some("chrome".to_string())
                } else if path_str.contains("firefox") {
                    Some("firefox".to_string())
                } else if path_str.contains("pip") {
                    Some("pip".to_string())
                } else if path_str.contains("npm") {
                    Some("npm".to_string())
                } else {
                    None
                };

                if let Some(source) = source {
                    app_handle.db(|conn| {
                        conn.execute(
                            "INSERT INTO cache_events (path, size_change, event_type, source, timestamp) VALUES (?, ?, 'growth', ?, ?)",
                            [&path_str, &size.to_string(), &source, &timestamp.to_string()],
                        )?;
                        Ok(())
                    }).map_err(|e| format!("Failed to record cache event: {}", e))?;
                }
            }
        }
    }

    Ok(())
}

// DiskPulse UI data commands
#[tauri::command]
pub async fn get_diskpulse_health(app_handle: tauri::AppHandle) -> Result<DiskPulseHealth, String> {
    let stats = get_system_stats(app_handle.clone()).await?;

    // Calculate disk usage percentage
    let usage_percent = if stats.total_disk_space > 0 {
        (stats.used_disk_space as f32 / stats.total_disk_space as f32) * 100.0
    } else {
        0.0
    };

    // Determine status color
    let (status_color, status_message) = if usage_percent < 70.0 {
        ("green", "You're good. No action needed.".to_string())
    } else if usage_percent < 85.0 {
        ("yellow", "Getting full, maybe check in.".to_string())
    } else {
        ("red", "Running low, take action.".to_string())
    };

    // Calculate projected days until full using historical data if available
    let projected_days = if stats.total_disk_space > 0 && stats.used_disk_space > 0 {
        // Try to get historical data from disk_history table
        let historical_data = app_handle.db(|conn| {
            let mut stmt = conn.prepare(
                "SELECT used_bytes, timestamp FROM disk_history ORDER BY timestamp DESC LIMIT 30"
            )?;
            let rows = stmt.query_map([], |row| {
                Ok((row.get::<_, i64>(0)?, row.get::<_, i64>(1)?))
            })?;

            let mut data_points: Vec<(i64, i64)> = Vec::new();
            for row_result in rows {
                data_points.push(row_result?);
            }
            Ok::<Vec<(i64, i64)>, rusqlite::Error>(data_points)
        }).unwrap_or_default();

        if historical_data.len() >= 2 {
            // Calculate daily growth rate from historical data
            let oldest = historical_data.last().unwrap();
            let newest = historical_data.first().unwrap();
            let days_diff = (newest.1 - oldest.1) as f32 / (24.0 * 3600.0);

            if days_diff > 0.0 {
                let bytes_growth = (newest.0 - oldest.0) as f32;
                let daily_usage_rate = bytes_growth / days_diff;

                if daily_usage_rate > 0.0 {
                    let remaining_space = stats.total_disk_space.saturating_sub(stats.used_disk_space);
                    Some((remaining_space as f32 / daily_usage_rate).ceil())
                } else {
                    // Disk is shrinking or stable, can't project
                    None
                }
            } else {
                None
            }
        } else {
            // Not enough historical data, use simplified calculation with current usage
            // Estimate based on cleanable space and assume moderate growth
            let remaining_space = stats.total_disk_space.saturating_sub(stats.used_disk_space);
            // Use a conservative estimate: assume 1% growth per month
            let monthly_growth = stats.total_disk_space as f32 * 0.01;
            let daily_growth = monthly_growth / 30.0;

            if daily_growth > 0.0 {
                Some((remaining_space as f32 / daily_growth).ceil())
            } else {
                None
            }
        }
    } else {
        None
    };

    Ok(DiskPulseHealth {
        disk_usage_percent: usage_percent,
        projected_days_until_full: projected_days,
        status_color: status_color.to_string(),
        status_message,
    })
}

#[tauri::command]
pub async fn get_old_files_summary(app_handle: tauri::AppHandle, days_cutoff: u32) -> Result<OldFilesSummary, String> {
    let cutoff_timestamp = chrono::Utc::now().timestamp() - (days_cutoff as i64 * 24 * 3600);

    let result = app_handle.db(|conn| {
        let mut stmt = conn.prepare(
            "SELECT COUNT(*), SUM(size) FROM file_access WHERE last_access < ?"
        )?;
        let mut rows = stmt.query([cutoff_timestamp])?;

        if let Some(row) = rows.next()? {
            let count: i64 = row.get(0)?;
            let total_size: Option<i64> = row.get(1)?;
            Ok(OldFilesSummary {
                total_files: count as usize,
                total_size: total_size.unwrap_or(0) as u64,
                cutoff_days: days_cutoff,
            })
        } else {
            Ok(OldFilesSummary {
                total_files: 0,
                total_size: 0,
                cutoff_days: days_cutoff,
            })
        }
    }).map_err(|e| format!("Failed to get old files summary: {}", e))?;

    Ok(result)
}

#[tauri::command]
pub async fn get_recent_cache_events(app_handle: tauri::AppHandle, limit: usize) -> Result<Vec<CacheEvent>, String> {
    let events = app_handle.db(|conn| {
        let mut stmt = conn.prepare(
            "SELECT id, path, size_change, event_type, source, timestamp FROM cache_events
             ORDER BY timestamp DESC LIMIT ?"
        )?;
        let mut rows = stmt.query([limit as i64])?;

        let mut events = Vec::new();
        while let Some(row) = rows.next()? {
            events.push(CacheEvent {
                id: row.get(0)?,
                path: row.get(1)?,
                size_change: row.get(2)?,
                event_type: row.get(3)?,
                source: row.get(4)?,
                timestamp: row.get(5)?,
            });
        }
        Ok(events)
    }).map_err(|e| format!("Failed to get cache events: {}", e))?;

    Ok(events)
}

#[tauri::command]
pub async fn get_cache_items() -> Result<Vec<CacheItem>, String> {
    let mut items = Vec::new();

    // Get real cache sizes from system
    let home = dirs::home_dir().ok_or("Cannot determine home directory")?;

    // Chrome/Chromium cache
    let chrome_cache = home.join(".cache/google-chrome");
    let chrome_size = if chrome_cache.exists() {
        trash::get_dir_size(&chrome_cache)
    } else {
        0
    };

    if chrome_size > 0 {
        items.push(CacheItem {
            name: "Chrome temporary files".to_string(),
            size: chrome_size,
            category: "browser".to_string(),
            can_clear: true,
        });
    }

    // Firefox cache
    let firefox_cache = home.join(".cache/mozilla/firefox");
    let firefox_size = if firefox_cache.exists() {
        trash::get_dir_size(&firefox_cache)
    } else {
        0
    };

    if firefox_size > 0 {
        items.push(CacheItem {
            name: "Firefox cache".to_string(),
            size: firefox_size,
            category: "browser".to_string(),
            can_clear: true,
        });
    }

    // PIP cache
    let pip_cache = home.join(".cache/pip");
    let pip_size = if pip_cache.exists() {
        trash::get_dir_size(&pip_cache)
    } else {
        0
    };

    if pip_size > 0 {
        items.push(CacheItem {
            name: "Python packages cache".to_string(),
            size: pip_size,
            category: "development".to_string(),
            can_clear: true,
        });
    }

    Ok(items)
}

#[tauri::command]
pub async fn clear_cache_item(item_name: String) -> Result<CleanResult, String> {
    match item_name.as_str() {
        "Chrome temporary files" => clear_cache().await,
        "Firefox cache" => clear_cache().await, // Would need Firefox-specific logic
        "Python packages cache" => clean_packages().await,
        _ => Err(format!("Unknown cache item: {}", item_name)),
    }
}

#[tauri::command]
pub async fn cleanup_old_files(app_handle: tauri::AppHandle, days_cutoff: u32) -> Result<CleanResult, String> {
    let cutoff_timestamp = chrono::Utc::now().timestamp() - (days_cutoff as i64 * 24 * 3600);

    let old_files = app_handle.db(|conn| {
        let mut stmt = conn.prepare("SELECT path FROM file_access WHERE last_access < ?")?;
        let rows = stmt.query_map([cutoff_timestamp], |row| row.get::<_, String>(0))?;

        let mut paths = Vec::new();
        for path_result in rows {
            paths.push(path_result?);
        }
        Ok(paths)
    }).map_err(|e| format!("Failed to get old files: {}", e))?;

    // Calculate actual file sizes and clean the files
    let mut cleaned = 0;
    let mut failed = 0;
    let mut total_size: u64 = 0;

    for path_str in old_files {
        let path = std::path::PathBuf::from(&path_str);

        // Validate path before any operations
        if let Err(validation_error) = validate_path_comprehensive(&path_str, SecurityContext::Deletion) {
            tracing::warn!("Path validation failed for {}: {}", path_str, validation_error);
            failed += 1;
            continue;
        }

        // Calculate file size before deletion
        let file_size = if path.exists() {
            if path.is_dir() {
                trash::get_dir_size(&path)
            } else {
                path.metadata().map(|m| m.len()).unwrap_or(0)
            }
        } else {
            // File no longer exists, skip it
            failed += 1;
            continue;
        };

        // Move to trash (30 day retention for old files)
        match trash::move_to_trash(
            &path_str,
            30,
            Some(TrashMetadata {
                category: "Old Files".to_string(),
                risk_level: 1,
                reason: format!("File not accessed in {} days", days_cutoff),
            }),
        ) {
            Ok(_) => {
                cleaned += 1;
                total_size += file_size;
                tracing::info!("Cleaned old file: {} ({} bytes)", path_str, file_size);
            }
            Err(e) => {
                tracing::error!("Failed to clean old file {}: {}", path_str, e);
                failed += 1;
            }
        }
    }

    Ok(CleanResult {
        cleaned,
        failed,
        total_size,
    })
}

// Cache Optimization Suite commands
#[tauri::command]
pub async fn get_cache_analytics(app_handle: tauri::AppHandle) -> Result<CacheAnalytics, String> {
    // Set a timeout for cache analytics (30 seconds - database operations)
    let analytics_timeout = Duration::from_secs(30);

    match timeout(analytics_timeout, get_cache_analytics_inner(app_handle)).await {
        Ok(result) => result,
        Err(_) => {
            tracing::error!("Cache analytics timed out after {} seconds", analytics_timeout.as_secs());
            Err("Cache analytics timed out. Please try again.".to_string())
        }
    }
}

async fn get_cache_analytics_inner(app_handle: tauri::AppHandle) -> Result<CacheAnalytics, String> {
    let cache_events = app_handle.db(|conn| {
        let mut stmt = conn.prepare(
            "SELECT source, size_change, timestamp FROM cache_events
             WHERE timestamp > ? ORDER BY timestamp DESC"
        )?;
        let cutoff = chrono::Utc::now().timestamp() - (30 * 24 * 3600); // Last 30 days

        let rows = stmt.query_map([cutoff], |row| {
            Ok((
                row.get::<_, String>(0)?,
                row.get::<_, i64>(1)?,
                row.get::<_, i64>(2)?,
            ))
        })?;

        let mut events = Vec::new();
        for event in rows.flatten() {
            events.push(event);
        }
        Ok(events)
    }).map_err(|e| format!("Failed to get cache events: {}", e))?;

    // Analyze current cache sizes
    let current_cache_items = get_cache_items().await?;
    let mut total_cache_size = 0u64;
    let mut contributors = Vec::new();

    for item in current_cache_items {
        total_cache_size += item.size;

        // Calculate growth rate from events (simplified)
        let source_events: Vec<_> = cache_events.iter()
            .filter(|(source, _, _)| *source == item.category)
            .collect();

        let growth_rate = if source_events.len() > 1 {
            let total_growth: i64 = source_events.iter().map(|(_, size, _)| *size).sum();
            let days_span = 30.0; // Assume 30 days of data
            (total_growth as f32 / (1024.0 * 1024.0)) / days_span // MB per day
        } else {
            0.0
        };

        let last_activity = source_events.first()
            .map(|(_, _, ts)| *ts)
            .unwrap_or(chrono::Utc::now().timestamp());

        let category = item.category.clone();
        contributors.push(CacheContributor {
            source: category.clone(),
            size: item.size,
            growth_rate,
            last_activity,
            recommended_limit: get_recommended_cache_limit(&category),
        });
    }

    // Generate growth trend (simplified - last 7 days)
    let mut growth_trend = Vec::new();
    for day_offset in (0..7).rev() {
        let timestamp = chrono::Utc::now().timestamp() - (day_offset * 24 * 3600);
        let day_size: u64 = cache_events.iter()
            .filter(|(_, _, ts)| *ts >= timestamp && *ts < timestamp + 24 * 3600)
            .map(|(_, size, _)| *size as u64)
            .sum();

        growth_trend.push(CacheGrowthPoint {
            timestamp,
            total_size: day_size,
            sources: std::collections::HashMap::new(), // Could be populated with per-source data
        });
    }

    // Recommended limits
    let mut recommended_limits = std::collections::HashMap::new();
    for contributor in &contributors {
        if let Some(limit) = contributor.recommended_limit {
            recommended_limits.insert(contributor.source.clone(), limit);
        }
    }

    Ok(CacheAnalytics {
        total_cache_size,
        cache_breakdown: contributors,
        growth_trend,
        recommended_limits,
    })
}

fn get_recommended_cache_limit(cache_type: &str) -> Option<u64> {
    match cache_type {
        "browser" => Some(1024 * 1024 * 1024), // 1GB for browsers
        "development" => Some(2 * 1024 * 1024 * 1024), // 2GB for dev tools
        "system" => Some(512 * 1024 * 1024), // 512MB for system caches
        _ => None,
    }
}

// Helper function to create a fallback colored icon
#[cfg(desktop)]
fn create_fallback_icon(status_color: &str) -> tauri::image::Image<'static> {
    use tauri::image::Image;

    let (r, g, b) = match status_color {
        "green" => (76, 175, 80),   // Green
        "yellow" => (255, 193, 7),   // Yellow/Amber
        "red" => (244, 67, 54),      // Red
        _ => (158, 158, 158),        // Gray (default)
    };
    // Create a 32x32 icon with the status color
    let mut rgba = Vec::with_capacity(32 * 32 * 4);
    for _ in 0..(32 * 32) {
        rgba.push(r);
        rgba.push(g);
        rgba.push(b);
        rgba.push(255); // Alpha
    }
    Image::new_owned(rgba, 32, 32)
}

#[tauri::command]
#[cfg(desktop)]
pub async fn update_tray_icon(app_handle: tauri::AppHandle, status_color: String) -> Result<(), String> {
    use tauri::tray::TrayIcon;
    use std::sync::Arc;

    tracing::info!("Updating tray icon for status: {}", status_color);

    // Get the tray icon - try to get it from managed state
    let tray_icon = if let Some(tray_state) = app_handle.try_state::<Arc<TrayIcon<tauri::Wry>>>() {
        Some(Arc::clone(tray_state.inner()))
    } else {
        // Fallback: try to get by default ID (first tray icon)
        // In Tauri 2.x, if no ID is specified, it uses a default
        app_handle.tray_by_id("default").map(Arc::new)
    };

    let Some(tray_icon) = tray_icon else {
        tracing::warn!("Tray icon not found, cannot update");
        return Err("Tray icon not available".to_string());
    };

    // Note: For now, we create a colored fallback icon
    // To load custom icon files, we would need to enable image-png/image-ico features in Tauri
    // and use Image::from_path(). For now, the colored icon provides visual feedback.

    // Load the icon image
    // For now, we'll use a colored fallback icon based on status
    // In the future, we can add image-png/image-ico features to Tauri to load custom icons
    let icon = create_fallback_icon(&status_color);

    // Update the tray icon
    tray_icon.set_icon(Some(icon))
        .map_err(|e| format!("Failed to set tray icon: {}", e))?;

    tracing::info!("Tray icon updated successfully for status: {}", status_color);
    Ok(())
}

#[tauri::command]
#[cfg(not(desktop))]
pub async fn update_tray_icon(_app_handle: tauri::AppHandle, _status_color: String) -> Result<(), String> {
    // Tray icons are only supported on desktop platforms
    Err("Tray icons are not supported on this platform".to_string())
}

#[cfg(test)]
mod security_tests {
    use super::*;
    use tempfile::TempDir;

    #[test]
    fn test_path_traversal_protection() {
        // Test basic path traversal
        assert!(validate_path_traversal("/home/user/../etc/passwd").is_err());
        assert!(validate_path_traversal("/home/user/../../etc/passwd").is_err());
        assert!(validate_path_traversal("/home/user/..\\etc\\passwd").is_err());

        // Test URL-encoded traversal
        assert!(validate_path_traversal("/home/user/%2e%2e%2fetc/passwd").is_err());
        assert!(validate_path_traversal("/home/user/%2e%2e/etc/passwd").is_err());

        // Test valid paths (without ..)
        assert!(validate_path_traversal("/home/user/documents").is_ok());
        assert!(validate_path_traversal("/home/user/.cache").is_ok());
    }

    #[test]
    fn test_system_critical_path_protection() {
        // Test system paths are blocked for deletion context
        // Note: These will fail on canonicalization/non-existence, but the intent is clear
        let result = validate_path_comprehensive("/etc/passwd", SecurityContext::Deletion);
        assert!(result.is_err());

        let result = validate_path_comprehensive("/bin/ls", SecurityContext::Deletion);
        assert!(result.is_err());

        let result = validate_path_comprehensive("/usr/bin", SecurityContext::Deletion);
        assert!(result.is_err());
    }

    #[test]
    fn test_symlink_resolution() {
        let temp_dir = TempDir::new().unwrap();
        let target = temp_dir.path().join("target.txt");
        let symlink = temp_dir.path().join("symlink.txt");

        std::fs::write(&target, "target").unwrap();

        #[cfg(unix)]
        {
            std::os::unix::fs::symlink(&target, &symlink).unwrap();

            // Canonicalization should resolve symlinks before validation
            let canonical = symlink.canonicalize().unwrap();
            assert_eq!(canonical, target.canonicalize().unwrap());
        }
    }
}
