use chrono::{DateTime, Duration, Utc};
use serde::{Deserialize, Serialize};
use specta::Type;
use std::fs;
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct TrashItem {
    pub id: String,
    pub original_path: String,
    pub trash_path: String,
    pub deleted_at: String,
    pub expires_at: String,
    pub size: u64,
    pub item_type: String,
    pub metadata: Option<TrashMetadata>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct TrashMetadata {
    pub category: String,
    pub risk_level: u8,
    pub reason: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct TrashData {
    pub items: Vec<TrashItem>,
    pub total_size: u64,
    pub total_items: usize,
}

pub fn get_trash_dir() -> PathBuf {
    let home = dirs::home_dir().unwrap_or_else(|| PathBuf::from("/tmp"));
    let trash_dir = home.join(".local/share/linux-cleaner/trash");

    if !trash_dir.exists() {
        fs::create_dir_all(&trash_dir).ok();
    }

    trash_dir
}

fn get_metadata_path() -> PathBuf {
    get_trash_dir().join("metadata.json")
}

fn load_trash_metadata() -> Vec<TrashItem> {
    let metadata_path = get_metadata_path();

    if metadata_path.exists() {
        if let Ok(content) = fs::read_to_string(&metadata_path) {
            if let Ok(items) = serde_json::from_str::<Vec<TrashItem>>(&content) {
                return items;
            }
        }
    }

    Vec::new()
}

fn save_trash_metadata(items: &[TrashItem]) -> Result<(), std::io::Error> {
    let metadata_path = get_metadata_path();
    let content = serde_json::to_string_pretty(items)?;
    fs::write(metadata_path, content)
}

fn remove_path(path: &Path) -> Result<(), std::io::Error> {
    if path.is_dir() {
        fs::remove_dir_all(path)
    } else {
        fs::remove_file(path)
    }
}

pub fn move_to_trash(
    path: &str,
    retention_days: i64,
    metadata: Option<TrashMetadata>,
) -> Result<TrashItem, String> {
    let source_path = PathBuf::from(path);

    if !source_path.exists() {
        return Err(format!("Path does not exist: {}", path));
    }

    let id = uuid::Uuid::new_v4().to_string();
    let trash_dir = get_trash_dir();
    let trash_path = trash_dir.join(&id);

    let size = if source_path.is_dir() {
        get_dir_size(&source_path)
    } else {
        source_path.metadata().map(|m| m.len()).unwrap_or(0)
    };

    let item_type = if source_path.is_dir() { "directory" } else { "file" }.to_string();

    fs::rename(&source_path, &trash_path).map_err(|e| format!("Failed to move to trash: {}", e))?;

    let now = Utc::now();
    let expires = now + Duration::days(retention_days);

    let item = TrashItem {
        id,
        original_path: path.to_string(),
        trash_path: trash_path.to_string_lossy().to_string(),
        deleted_at: now.to_rfc3339(),
        expires_at: expires.to_rfc3339(),
        size,
        item_type,
        metadata,
    };

    let mut items = load_trash_metadata();
    items.push(item.clone());
    save_trash_metadata(&items).map_err(|e| format!("Failed to save metadata: {}", e))?;

    Ok(item)
}

pub fn restore_from_trash(id: &str) -> Result<(), String> {
    let mut items = load_trash_metadata();

    let item_idx = items.iter().position(|i| i.id == id)
        .ok_or_else(|| format!("Item not found in trash: {}", id))?;

    let item = &items[item_idx];
    let trash_path = PathBuf::from(&item.trash_path);
    let original_path = PathBuf::from(&item.original_path);

    if !trash_path.exists() {
        items.remove(item_idx);
        save_trash_metadata(&items).ok();
        return Err("Item no longer exists in trash".to_string());
    }

    if let Some(parent) = original_path.parent() {
        fs::create_dir_all(parent).map_err(|e| format!("Failed to create parent: {}", e))?;
    }

    if original_path.exists() {
        return Err(format!("Cannot restore: path already exists: {}", item.original_path));
    }

    fs::rename(&trash_path, &original_path).map_err(|e| format!("Failed to restore: {}", e))?;

    items.remove(item_idx);
    save_trash_metadata(&items).map_err(|e| format!("Failed to update metadata: {}", e))?;

    Ok(())
}

pub fn delete_from_trash(id: &str) -> Result<(), String> {
    let mut items = load_trash_metadata();

    let item_idx = items.iter().position(|i| i.id == id)
        .ok_or_else(|| format!("Item not found in trash: {}", id))?;

    let item = &items[item_idx];
    let trash_path = PathBuf::from(&item.trash_path);

    if trash_path.exists() {
        remove_path(&trash_path).map_err(|e| format!("Failed to delete: {}", e))?;
    }

    items.remove(item_idx);
    save_trash_metadata(&items).map_err(|e| format!("Failed to update metadata: {}", e))?;

    Ok(())
}

pub fn empty_trash() -> Result<usize, String> {
    let items = load_trash_metadata();
    let count = items.len();

    for item in &items {
        let trash_path = PathBuf::from(&item.trash_path);
        if trash_path.exists() {
            remove_path(&trash_path).ok();
        }
    }

    save_trash_metadata(&[]).map_err(|e| format!("Failed to clear metadata: {}", e))?;

    Ok(count)
}

pub fn get_trash_items() -> TrashData {
    let items = load_trash_metadata();
    let total_size: u64 = items.iter().map(|i| i.size).sum();
    let total_items = items.len();

    TrashData { items, total_size, total_items }
}

pub fn cleanup_expired() -> Result<usize, String> {
    let items = load_trash_metadata();
    let now = Utc::now();
    let mut removed = 0;
    let mut remaining = Vec::new();

    for item in items {
        let expires: DateTime<Utc> = item.expires_at.parse().unwrap_or(now);

        if expires <= now {
            let trash_path = PathBuf::from(&item.trash_path);
            if trash_path.exists() {
                remove_path(&trash_path).ok();
            }
            removed += 1;
        } else {
            remaining.push(item);
        }
    }

    save_trash_metadata(&remaining).map_err(|e| format!("Failed to update metadata: {}", e))?;

    Ok(removed)
}

pub fn get_dir_size(path: &Path) -> u64 {
    let mut size: u64 = 0;

    if let Ok(entries) = fs::read_dir(path) {
        for entry in entries.filter_map(|e| e.ok()) {
            let entry_path = entry.path();
            if entry_path.is_file() {
                if let Ok(metadata) = entry_path.metadata() {
                    size += metadata.len();
                }
            } else if entry_path.is_dir() {
                size += get_dir_size(&entry_path);
            }
        }
    }

    size
}
