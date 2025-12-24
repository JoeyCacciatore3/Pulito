use rusqlite::{Connection, Result};
use std::path::PathBuf;
use tauri::{AppHandle, Manager};
use std::sync::Mutex;

#[derive(Debug)]
pub struct AppState {
    pub db: Mutex<Option<rusqlite::Connection>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusqlite::Connection;
    use std::path::PathBuf;
    use tempfile::TempDir;

    fn create_test_db(path: &PathBuf) -> Result<Connection, rusqlite::Error> {
        Connection::open(path)
    }

    fn initialize_test_database(conn: &Connection) -> Result<()> {
        conn.execute_batch(
            r#"
            -- Scan history
            CREATE TABLE IF NOT EXISTS scan_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp TEXT NOT NULL,
                total_size INTEGER NOT NULL,
                total_items INTEGER NOT NULL,
                scan_time_ms INTEGER NOT NULL,
                cleaned_size INTEGER DEFAULT 0
            );

            -- Trash items
            CREATE TABLE IF NOT EXISTS trash_items (
                id TEXT PRIMARY KEY,
                original_path TEXT NOT NULL,
                trash_path TEXT NOT NULL,
                deleted_at TEXT NOT NULL,
                expires_at TEXT NOT NULL,
                size INTEGER NOT NULL,
                item_type TEXT NOT NULL,
                metadata TEXT
            );

            -- Settings
            CREATE TABLE IF NOT EXISTS settings (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL
            );

            -- DiskPulse: Cache growth events for cache feed
            CREATE TABLE IF NOT EXISTS cache_events (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL,
                size_change INTEGER NOT NULL,
                event_type TEXT NOT NULL,
                source TEXT,
                timestamp INTEGER NOT NULL
            );

            -- DiskPulse: Disk usage history
            CREATE TABLE IF NOT EXISTS disk_history (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                timestamp INTEGER NOT NULL,
                used_bytes INTEGER NOT NULL,
                total_bytes INTEGER NOT NULL,
                available_bytes INTEGER NOT NULL
            );

            -- DiskPulse: Monitoring state
            CREATE TABLE IF NOT EXISTS monitoring_state (
                key TEXT PRIMARY KEY,
                value TEXT NOT NULL,
                updated_at INTEGER NOT NULL
            );

            -- DiskPulse: File access tracking for old files detection
            CREATE TABLE IF NOT EXISTS file_access (
                id INTEGER PRIMARY KEY AUTOINCREMENT,
                path TEXT NOT NULL UNIQUE,
                size INTEGER NOT NULL,
                last_access INTEGER NOT NULL
            );

            -- Last scan results for Dashboard display
            CREATE TABLE IF NOT EXISTS last_scan_results (
                scan_type TEXT PRIMARY KEY,
                total_size INTEGER NOT NULL,
                total_items INTEGER NOT NULL,
                timestamp INTEGER NOT NULL,
                scan_data TEXT
            );

            -- Create indexes
            CREATE INDEX IF NOT EXISTS idx_trash_items_expires ON trash_items(expires_at);
            CREATE INDEX IF NOT EXISTS idx_cache_events_timestamp ON cache_events(timestamp);
            CREATE INDEX IF NOT EXISTS idx_cache_events_source ON cache_events(source);
            CREATE INDEX IF NOT EXISTS idx_disk_history_timestamp ON disk_history(timestamp);
            CREATE INDEX IF NOT EXISTS idx_file_access_last_access ON file_access(last_access);
            "#,
        )
    }

    #[test]
    fn test_database_initialization_creates_tables() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = create_test_db(&db_path).unwrap();

        // Initialize the database schema
        initialize_test_database(&conn).unwrap();

        // Verify all tables exist
        let tables: Vec<String> = conn
            .prepare(
                "SELECT name FROM sqlite_master WHERE type='table' ORDER BY name"
            )
            .unwrap()
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        assert!(tables.contains(&"cache_events".to_string()));
        assert!(tables.contains(&"disk_history".to_string()));
        assert!(tables.contains(&"file_access".to_string()));
        assert!(tables.contains(&"monitoring_state".to_string()));
        assert!(tables.contains(&"scan_history".to_string()));
        assert!(tables.contains(&"settings".to_string()));
        assert!(tables.contains(&"trash_items".to_string()));
    }

    #[test]
    fn test_disk_history_table_structure() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = create_test_db(&db_path).unwrap();
        initialize_test_database(&conn).unwrap();

        // Test inserting into disk_history
        conn.execute(
            "INSERT INTO disk_history (timestamp, used_bytes, total_bytes, available_bytes) VALUES (?, ?, ?, ?)",
            [1234567890i64, 1000000i64, 5000000i64, 4000000i64],
        ).unwrap();

        // Test reading from disk_history
        let count: i64 = conn
            .query_row("SELECT COUNT(*) FROM disk_history", [], |row| row.get(0))
            .unwrap();
        assert_eq!(count, 1);
    }

    #[test]
    fn test_monitoring_state_table_structure() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = create_test_db(&db_path).unwrap();
        initialize_test_database(&conn).unwrap();

        // Test inserting into monitoring_state
        conn.execute(
            "INSERT OR REPLACE INTO monitoring_state (key, value, updated_at) VALUES (?, ?, ?)",
            rusqlite::params!["test_key", "test_value", 1234567890i64],
        ).unwrap();

        // Test reading from monitoring_state
        let value: String = conn
            .query_row(
                "SELECT value FROM monitoring_state WHERE key = ?",
                ["test_key"],
                |row| row.get(0)
            )
            .unwrap();
        assert_eq!(value, "test_value");
    }

    #[test]
    fn test_file_access_table_structure() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = create_test_db(&db_path).unwrap();
        initialize_test_database(&conn).unwrap();

        // Test inserting into file_access
        conn.execute(
            "INSERT INTO file_access (path, size, last_access) VALUES (?, ?, ?)",
            rusqlite::params!["/test/path", 1000i64, 1234567890i64],
        ).unwrap();

        // Test reading from file_access
        let count: i64 = conn
            .query_row(
                "SELECT COUNT(*) FROM file_access WHERE last_access < ?",
                [1234567900i64],
                |row| row.get(0)
            )
            .unwrap();
        assert_eq!(count, 1);

        // Test UNIQUE constraint on path
        let result = conn.execute(
            "INSERT INTO file_access (path, size, last_access) VALUES (?, ?, ?)",
            rusqlite::params!["/test/path", 2000i64, 1234567891i64],
        );
        assert!(result.is_err()); // Should fail due to UNIQUE constraint
    }

    #[test]
    fn test_indexes_created() {
        let temp_dir = TempDir::new().unwrap();
        let db_path = temp_dir.path().join("test.db");
        let conn = create_test_db(&db_path).unwrap();
        initialize_test_database(&conn).unwrap();

        // Verify indexes exist
        let indexes: Vec<String> = conn
            .prepare(
                "SELECT name FROM sqlite_master WHERE type='index' AND name LIKE 'idx_%'"
            )
            .unwrap()
            .query_map([], |row| row.get::<_, String>(0))
            .unwrap()
            .map(|r| r.unwrap())
            .collect();

        assert!(indexes.contains(&"idx_trash_items_expires".to_string()));
        assert!(indexes.contains(&"idx_cache_events_timestamp".to_string()));
        assert!(indexes.contains(&"idx_cache_events_source".to_string()));
        assert!(indexes.contains(&"idx_disk_history_timestamp".to_string()));
        assert!(indexes.contains(&"idx_file_access_last_access".to_string()));
    }
}

#[allow(dead_code)]
pub fn get_db_path(app_handle: &AppHandle) -> Result<PathBuf, String> {
    let app_dir = app_handle
        .path()
        .app_data_dir()
        .map_err(|e| format!("Failed to get app data directory: {}", e))?;

    std::fs::create_dir_all(&app_dir)
        .map_err(|e| format!("Failed to create app data directory: {}", e))?;

    Ok(app_dir.join("pulito.db"))
}

#[allow(dead_code)] // False positive - used via extension trait in main.rs
pub fn initialize_database(app_handle: &AppHandle) -> Result<()> {
    let db_path = get_db_path(app_handle)
        .map_err(|e| rusqlite::Error::InvalidPath(std::path::PathBuf::from(e)))?;
    let conn = Connection::open(&db_path)?;

    // Create tables
    conn.execute_batch(
        r#"
        -- Scan history
        CREATE TABLE IF NOT EXISTS scan_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            total_size INTEGER NOT NULL,
            total_items INTEGER NOT NULL,
            scan_time_ms INTEGER NOT NULL,
            cleaned_size INTEGER DEFAULT 0
        );

        -- Trash items
        CREATE TABLE IF NOT EXISTS trash_items (
            id TEXT PRIMARY KEY,
            original_path TEXT NOT NULL,
            trash_path TEXT NOT NULL,
            deleted_at TEXT NOT NULL,
            expires_at TEXT NOT NULL,
            size INTEGER NOT NULL,
            item_type TEXT NOT NULL,
            metadata TEXT
        );

        -- Settings
        CREATE TABLE IF NOT EXISTS settings (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL
        );

        -- DiskPulse: Cache growth events for cache feed
        CREATE TABLE IF NOT EXISTS cache_events (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL,
            size_change INTEGER NOT NULL,
            event_type TEXT NOT NULL, -- 'growth', 'cleanup', 'new'
            source TEXT, -- 'chrome', 'pip', 'npm', etc.
            timestamp INTEGER NOT NULL
        );

        -- DiskPulse: Disk usage history
        CREATE TABLE IF NOT EXISTS disk_history (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp INTEGER NOT NULL,
            used_bytes INTEGER NOT NULL,
            total_bytes INTEGER NOT NULL,
            available_bytes INTEGER NOT NULL
        );

        -- DiskPulse: Monitoring state
        CREATE TABLE IF NOT EXISTS monitoring_state (
            key TEXT PRIMARY KEY,
            value TEXT NOT NULL,
            updated_at INTEGER NOT NULL
        );

        -- DiskPulse: File access tracking for old files detection
        CREATE TABLE IF NOT EXISTS file_access (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            path TEXT NOT NULL UNIQUE,
            size INTEGER NOT NULL,
            last_access INTEGER NOT NULL
        );

        -- Create indexes
        CREATE INDEX IF NOT EXISTS idx_trash_items_expires ON trash_items(expires_at);
        CREATE INDEX IF NOT EXISTS idx_cache_events_timestamp ON cache_events(timestamp);
        CREATE INDEX IF NOT EXISTS idx_cache_events_source ON cache_events(source);
        CREATE INDEX IF NOT EXISTS idx_disk_history_timestamp ON disk_history(timestamp);
        CREATE INDEX IF NOT EXISTS idx_file_access_last_access ON file_access(last_access);
        "#,
    )?;

    // Store connection in app state
    let state: tauri::State<AppState> = app_handle.state();
    let mut db = state.db.lock().unwrap();
    *db = Some(conn);

    tracing::info!("Database initialized at {:?}", db_path);
    Ok(())
}

/// Helper trait to access database from app handle
pub trait DbAccess {
    fn db<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>;
}

impl DbAccess for AppHandle {
    fn db<F, T>(&self, f: F) -> Result<T>
    where
        F: FnOnce(&Connection) -> Result<T>,
    {
        let state: tauri::State<AppState> = self.state();
        let db = state.db.lock()
            .map_err(|e| rusqlite::Error::FromSqlConversionFailure(0, rusqlite::types::Type::Null, Box::new(std::io::Error::other(format!("Mutex lock failed: {}", e)))))?;

        let conn = db.as_ref()
            .ok_or_else(|| rusqlite::Error::ExecuteReturnedResults)?;

        f(conn)
    }
}
