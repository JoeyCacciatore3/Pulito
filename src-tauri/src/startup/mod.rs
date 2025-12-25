use serde::{Deserialize, Serialize};
use specta::Type;
use std::path::PathBuf;
use std::fs;
use std::io::Write;
use dirs;
use tokio::time::{timeout, Duration};
use crate::commands::validate_path_comprehensive;
use crate::commands::SecurityContext;

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct StartupProgram {
    pub id: String,
    pub name: String,
    pub description: String,
    pub enabled: bool,
    pub location: String, // "xdg_autostart", "systemd_user", "systemd_system"
    pub file_path: String,
    pub impact: String, // "low", "medium", "high"
    pub exec_command: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, Type)]
#[specta(export)]
pub struct StartupProgramsList {
    pub programs: Vec<StartupProgram>,
    pub total_count: usize,
    pub enabled_count: usize,
}

#[tauri::command]
pub async fn get_startup_programs() -> Result<StartupProgramsList, String> {
    let timeout_duration = Duration::from_secs(10);

    timeout(timeout_duration, async {
        let mut programs = Vec::new();

        // Scan XDG autostart directory
        if let Some(config_dir) = dirs::config_dir() {
            let xdg_dir = config_dir.join("autostart");
            if xdg_dir.exists() {
                if let Ok(entries) = fs::read_dir(&xdg_dir) {
                    for entry in entries.flatten() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "desktop" {
                                if let Ok(program) = parse_desktop_file(entry.path()) {
                                    programs.push(program);
                                }
                            }
                        }
                    }
                }
            }

            // Scan systemd user services
            let systemd_user = config_dir.join("systemd/user");
            if systemd_user.exists() {
                if let Ok(entries) = fs::read_dir(&systemd_user) {
                    for entry in entries.flatten() {
                        if let Some(ext) = entry.path().extension() {
                            if ext == "service" {
                                if let Ok(program) = parse_service_file(entry.path()).await {
                                    programs.push(program);
                                }
                            }
                        }
                    }
                }
            }
        }

        let enabled_count = programs.iter().filter(|p| p.enabled).count();

        Ok(StartupProgramsList {
            total_count: programs.len(),
            enabled_count,
            programs,
        })
    })
    .await
    .map_err(|_| "Timeout getting startup programs".to_string())?
}

fn parse_desktop_file(path: PathBuf) -> Result<StartupProgram, String> {
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read desktop file: {}", e))?;

    let mut name = String::new();
    let mut exec = None;
    let mut comment = String::new();
    let mut hidden = false;
    let mut no_display = false;
    let mut only_show_in = Vec::new();
    let mut not_show_in = Vec::new();

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("Name=") {
            name = line[5..].to_string();
        } else if line.starts_with("Exec=") {
            exec = Some(line[5..].to_string());
        } else if line.starts_with("Comment=") {
            comment = line[8..].to_string();
        } else if line == "Hidden=true" {
            hidden = true;
        } else if line == "NoDisplay=true" {
            no_display = true;
        } else if line.starts_with("OnlyShowIn=") {
            only_show_in = line[11..].split(';').map(|s| s.to_string()).collect();
        } else if line.starts_with("NotShowIn=") {
            not_show_in = line[10..].split(';').map(|s| s.to_string()).collect();
        }
    }

    if name.is_empty() {
        name = path.file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("Unknown")
            .to_string();
    }

    // Determine if the program is enabled
    // A program is enabled if it's not explicitly hidden/disabled
    // and it's appropriate for the current desktop environment
    let mut enabled = !hidden && !no_display;

    // Check OnlyShowIn/NotShowIn for current desktop
    if enabled {
        // Get current desktop environment
        let current_desktop = std::env::var("XDG_CURRENT_DESKTOP")
            .unwrap_or_else(|_| "GNOME".to_string()); // Default fallback

        if !only_show_in.is_empty() && !only_show_in.contains(&current_desktop) {
            enabled = false;
        }
        if !not_show_in.is_empty() && not_show_in.contains(&current_desktop) {
            enabled = false;
        }
    }

    let id = format!("xdg_{}", path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("unknown")
        .replace(".", "_")
        .replace("-", "_"));

    Ok(StartupProgram {
        id,
        name,
        description: comment,
        enabled,
        location: "xdg_autostart".to_string(),
        file_path: path.to_string_lossy().to_string(),
        impact: "medium".to_string(), // Default, could be enhanced
        exec_command: exec,
    })
}

async fn parse_service_file(path: PathBuf) -> Result<StartupProgram, String> {
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read service file: {}", e))?;

    let mut description = String::new();
    let mut exec_start = None;

    for line in content.lines() {
        let line = line.trim();
        if line.starts_with("Description=") {
            description = line[12..].to_string();
        } else if line.starts_with("ExecStart=") {
            exec_start = Some(line[10..].to_string());
        }
    }

    let name = path.file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("Unknown Service")
        .to_string();

    // Check if service is enabled using systemctl
    // Note: systemctl --user is-enabled returns:
    // 0 = enabled, 1 = disabled, 3 = not found/invalid
    let enabled_result = std::process::Command::new("systemctl")
        .args(&["--user", "is-enabled", &name])
        .output();

    let enabled = match enabled_result {
        Ok(output) => {
            // Exit code 0 means enabled, anything else means disabled/not found
            output.status.code() == Some(0)
        }
        Err(_) => {
            // If systemctl fails completely, assume disabled
            false
        }
    };

    let id = format!("systemd_{}", name.replace(".", "_").replace("-", "_"));

    Ok(StartupProgram {
        id,
        name,
        description,
        enabled,
        location: "systemd_user".to_string(),
        file_path: path.to_string_lossy().to_string(),
        impact: "medium".to_string(),
        exec_command: exec_start,
    })
}

#[tauri::command]
pub async fn toggle_startup_program(
    id: String,
    enabled: bool,
) -> Result<(), String> {
    let timeout_duration = Duration::from_secs(5);

    timeout(timeout_duration, async {
        // Get the program first
        let programs_result = get_startup_programs().await?;
        let program = programs_result.programs
            .into_iter()
            .find(|p| p.id == id)
            .ok_or_else(|| "Program not found".to_string())?;

        // Validate path
        validate_path_comprehensive(&program.file_path, SecurityContext::StartupManagement)
            .map_err(|e| format!("Security validation failed: {}", e))?;

        match program.location.as_str() {
            "xdg_autostart" => {
                toggle_xdg_autostart(&program.file_path, enabled)?;
            }
            "systemd_user" => {
                toggle_systemd_service(&program.name, enabled)?;
            }
            _ => {
                return Err("Unsupported startup location".to_string());
            }
        }

        Ok(())
    })
    .await
    .map_err(|_| "Timeout toggling startup program".to_string())?
}

fn toggle_xdg_autostart(file_path: &str, enabled: bool) -> Result<(), String> {
    let path = PathBuf::from(file_path);
    let content = fs::read_to_string(&path)
        .map_err(|e| format!("Failed to read desktop file: {}", e))?;

    let mut lines: Vec<String> = content.lines().map(String::from).collect();
    let mut hidden_index = None;

    for (i, line) in lines.iter().enumerate() {
        if line.trim().starts_with("Hidden=") {
            hidden_index = Some(i);
            break;
        }
    }

    if enabled {
        // Remove Hidden line
        if let Some(idx) = hidden_index {
            lines.remove(idx);
        }
    } else {
        // Add or update Hidden line
        let hidden_line = "Hidden=true".to_string();
        if let Some(idx) = hidden_index {
            lines[idx] = hidden_line;
        } else {
            // Find [Desktop Entry] section and add after it
            let mut insert_index = 0;
            for (i, line) in lines.iter().enumerate() {
                if line.trim() == "[Desktop Entry]" {
                    insert_index = i + 1;
                    break;
                }
            }
            lines.insert(insert_index, hidden_line);
        }
    }

    let mut file = fs::File::create(&path)
        .map_err(|e| format!("Failed to create desktop file: {}", e))?;
    file.write_all(lines.join("\n").as_bytes())
        .map_err(|e| format!("Failed to write desktop file: {}", e))?;

    Ok(())
}

fn toggle_systemd_service(service_name: &str, enabled: bool) -> Result<(), String> {
    let status = if enabled {
        std::process::Command::new("systemctl")
            .args(&["--user", "enable", service_name])
            .status()
    } else {
        std::process::Command::new("systemctl")
            .args(&["--user", "disable", service_name])
            .status()
    };

    status
        .map_err(|e| format!("Failed to execute systemctl: {}", e))?
        .success()
        .then_some(())
        .ok_or_else(|| "systemctl command failed".to_string())
}
