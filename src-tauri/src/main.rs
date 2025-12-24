// Prevents additional console window on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod cache;
mod commands;
mod db;
mod packages;
mod scanner;
mod trash;

use db::AppState;

use tauri::Manager;

use std::sync::Mutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

fn main() {
    // Initialize comprehensive logging with structured formatting
    // Log level can be controlled via RUST_LOG environment variable
    // Examples: RUST_LOG=debug, RUST_LOG=pulito=info, RUST_LOG=pulito::commands=debug
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("pulito=info"));

    // Configure log formatting - structured output with consistent formatting
    // In release builds, we keep it simple for readability while maintaining structure
    #[cfg(debug_assertions)]
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false) // Hide module paths for cleaner output
        .with_thread_ids(true)
        .with_thread_names(true)
        .with_file(true) // Include file in debug builds
        .with_line_number(true); // Include line number in debug builds

    #[cfg(not(debug_assertions))]
    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .with_file(false)
        .with_line_number(false);

    tracing_subscriber::registry()
        .with(fmt_layer)
        .with(filter)
        .init();

    tracing::info!(
        version = env!("CARGO_PKG_VERSION"),
        "🚀 Starting Pulito - System Cleanup Tool"
    );
    tracing::debug!("Application data will be stored in app data directory");
    tracing::debug!("Initializing Tauri application with plugins: shell, os, notification");

    // Generate TypeScript types early in debug mode
    #[cfg(debug_assertions)]
    {
        use specta::{TypeCollection, Type};
        use specta_typescript::Typescript;

        let output_dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("src/lib/generated");

        std::fs::create_dir_all(&output_dir).expect("Failed to create generated directory");

        // Register all the types we want to export
        let mut collection = TypeCollection::default();
        collection.register::<commands::SystemStats>();
        collection.register::<commands::AppSettings>();
        collection.register::<commands::TrashSettings>();
        collection.register::<commands::MonitoringSettings>();
        collection.register::<commands::NotificationSettings>();
        collection.register::<commands::ScanSettings>();
        collection.register::<commands::CacheEvent>();
        collection.register::<commands::DiskPulseHealth>();
        collection.register::<commands::OldFilesSummary>();
        collection.register::<commands::CacheItem>();
        collection.register::<commands::SystemHealthData>();
        collection.register::<commands::GpuInfo>();
        collection.register::<commands::Temperatures>();
        collection.register::<commands::NetworkInterfaceInfo>();
        collection.register::<commands::NetworkConnection>();
        collection.register::<commands::BatteryInfo>();
        collection.register::<commands::ProcessInfo>();
        collection.register::<commands::LoadAverage>();
        collection.register::<commands::TreeNode>();
        collection.register::<commands::CleanResult>();
        collection.register::<commands::CacheAnalytics>();
        collection.register::<commands::CacheContributor>();
        collection.register::<commands::CacheGrowthPoint>();
        collection.register::<scanner::ScanItem>();
        collection.register::<scanner::ScanResults>();
        collection.register::<scanner::ScanOptions>();
        collection.register::<scanner::FilesystemHealthResults>();
        collection.register::<scanner::StorageRecoveryResults>();
        collection.register::<scanner::DuplicateGroup>();
        collection.register::<trash::TrashItem>();
        collection.register::<trash::TrashMetadata>();
        collection.register::<trash::TrashData>();
        let types = collection;

        match Typescript::default()
            .bigint(specta_typescript::BigIntExportBehavior::Number)
            .export_to(&output_dir.join("types.ts"), &types) {
            Ok(_) => {
                // Post-process the generated types to use undefined instead of null
                if let Ok(content) = std::fs::read_to_string(&output_dir.join("types.ts")) {
                    let processed_content = content.replace(" | null", " | undefined");
                    if let Err(e) = std::fs::write(&output_dir.join("types.ts"), processed_content) {
                        tracing::error!("Failed to post-process TypeScript types: {}", e);
                    } else {
                        tracing::info!("TypeScript types exported and post-processed to use undefined instead of null");
                    }
                }
                tracing::info!("TypeScript types exported to: {}", output_dir.join("types.ts").display());
            }
            Err(e) => {
                tracing::error!("Failed to export TypeScript types: {}", e);
            }
        }
    }

    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .manage(AppState {
            db: Mutex::new(None),
        })
        .setup(|app| {
            tracing::debug!("Running application setup...");

            // Initialize database
            let app_handle = app.handle().clone();
            tracing::debug!("Initializing database...");
            match db::initialize_database(&app_handle) {
                Ok(_) => tracing::info!("Database initialized successfully"),
                Err(e) => {
                    tracing::error!("Failed to initialize database: {}", e);
                    return Err(e.into());
                }
            }

            // Set up system tray
            #[cfg(desktop)]
            {
                use tauri::tray::TrayIconBuilder;
                use tauri::image::Image;

                let app_handle_for_tray = app.handle().clone();

                // Create a default icon (white square)
                // In the future, we can load from file if image-png/image-ico features are enabled
                let default_icon = Image::new_owned(vec![255, 255, 255, 255], 1, 1);

                let tray = TrayIconBuilder::new()
                    .tooltip("Pulito - System Cleanup")
                    .icon(default_icon)
                    .on_tray_icon_event(move |_tray, event| {
                        match event {
                            tauri::tray::TrayIconEvent::Click { .. } => {
                                tracing::info!("Tray icon clicked - toggling main window");
                                if let Some(window) = app_handle_for_tray.get_webview_window("main") {
                                    if let Ok(visible) = window.is_visible() {
                                        if visible {
                                            let _ = window.hide();
                                        } else {
                                            let _ = window.show();
                                            let _ = window.set_focus();
                                        }
                                    }
                                }
                            }
                            tauri::tray::TrayIconEvent::DoubleClick { .. } => {
                                tracing::info!("Tray icon double-clicked - showing main window");
                                if let Some(window) = app_handle_for_tray.get_webview_window("main") {
                                    let _ = window.show();
                                    let _ = window.set_focus();
                                }
                            }
                            _ => {}
                        }
                    })
                    .build(app)?;

                // Store tray handle for dynamic icon updates
                app.manage(tray);
            }

            // TypeScript types are generated earlier in main() function

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::initialize_app,
            commands::get_system_stats,
            commands::get_system_health,
            commands::scan_filesystem_tree,
            commands::start_scan,
            commands::scan_filesystem_health,
            commands::scan_storage_recovery,
            commands::scan_for_old_files,
            commands::get_cache_analytics,
            commands::clean_items,
            commands::clear_cache,
            commands::clean_packages,
            commands::clear_logs,
            commands::get_trash_items,
            commands::restore_from_trash,
            commands::delete_from_trash,
            commands::empty_trash,
            commands::get_settings,
            commands::save_settings,
            // DiskPulse commands
            commands::start_diskpulse_monitoring,
            commands::stop_diskpulse_monitoring,
            commands::get_diskpulse_health,
            commands::get_old_files_summary,
            commands::get_recent_cache_events,
            commands::get_cache_items,
            commands::clear_cache_item,
            // commands::cleanup_old_files,
            commands::update_tray_icon,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
