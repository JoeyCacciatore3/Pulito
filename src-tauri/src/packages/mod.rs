use serde::{Deserialize, Serialize};
use std::process::Command;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub size: u64,
    pub description: String,
    pub dependencies: Vec<String>,
    pub dependents: Vec<String>,
    pub is_orphan: bool,
    pub package_manager: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageStats {
    pub total_packages: usize,
    pub orphan_packages: usize,
    pub orphan_size: u64,
}

/// Get list of installed apt packages
pub fn get_apt_packages() -> Vec<PackageInfo> {
    let mut packages = Vec::new();

    let output = Command::new("dpkg-query")
        .args(["-W", "-f", "${Package}|${Version}|${Installed-Size}|${Status}|${Description}\n"])
        .output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            let parts: Vec<&str> = line.split('|').collect();
            if parts.len() >= 4 && parts[3].contains("installed") {
                packages.push(PackageInfo {
                    name: parts[0].to_string(),
                    version: parts[1].to_string(),
                    size: parts[2].parse::<u64>().unwrap_or(0) * 1024,
                    description: parts.get(4).unwrap_or(&"").to_string(),
                    dependencies: Vec::new(),
                    dependents: Vec::new(),
                    is_orphan: false,
                    package_manager: "apt".to_string(),
                });
            }
        }
    }

    packages
}

/// Get orphaned packages
pub fn get_orphan_packages() -> Vec<PackageInfo> {
    let mut orphans = Vec::new();

    let output = Command::new("apt").args(["--dry-run", "autoremove"]).output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.starts_with("Remv ") {
                let parts: Vec<&str> = line.split_whitespace().collect();
                if parts.len() >= 2 {
                    if let Some(info) = get_package_info(parts[1]) {
                        orphans.push(PackageInfo { is_orphan: true, ..info });
                    }
                }
            }
        }
    }

    orphans
}

/// Get detailed info for a specific package
pub fn get_package_info(name: &str) -> Option<PackageInfo> {
    let output = Command::new("dpkg-query")
        .args(["-W", "-f", "${Package}|${Version}|${Installed-Size}|${Description}\n", name])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next()?;
    let parts: Vec<&str> = line.split('|').collect();

    if parts.len() >= 3 {
        Some(PackageInfo {
            name: parts[0].to_string(),
            version: parts[1].to_string(),
            size: parts[2].parse::<u64>().unwrap_or(0) * 1024,
            description: parts.get(3).unwrap_or(&"").to_string(),
            dependencies: get_package_dependencies(name),
            dependents: get_package_dependents(name),
            is_orphan: false,
            package_manager: "apt".to_string(),
        })
    } else {
        None
    }
}

/// Get dependencies of a package
pub fn get_package_dependencies(name: &str) -> Vec<String> {
    let mut deps = Vec::new();

    let output = Command::new("apt-cache").args(["depends", "--installed", name]).output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
            if line.trim_start().starts_with("Depends:") {
                let dep = line.trim_start().trim_start_matches("Depends:").split_whitespace().next().unwrap_or("").to_string();
                if !dep.is_empty() && !dep.starts_with('<') {
                    deps.push(dep);
                }
            }
        }
    }

    deps
}

/// Get reverse dependencies
pub fn get_package_dependents(name: &str) -> Vec<String> {
    let mut dependents = Vec::new();

    let output = Command::new("apt-cache").args(["rdepends", "--installed", name]).output();

    if let Ok(output) = output {
        let stdout = String::from_utf8_lossy(&output.stdout);
        let mut skip_header = true;
        for line in stdout.lines() {
            if skip_header {
                if line.starts_with("Reverse Depends:") {
                    skip_header = false;
                }
                continue;
            }
            let dep = line.trim().to_string();
            if !dep.is_empty() && !dep.starts_with('|') {
                dependents.push(dep);
            }
        }
    }

    dependents
}

/// Get package statistics
pub fn get_package_stats() -> PackageStats {
    let orphans = get_orphan_packages();
    let total_packages = get_apt_packages().len();
    let orphan_size: u64 = orphans.iter().map(|p| p.size).sum();

    PackageStats {
        total_packages,
        orphan_packages: orphans.len(),
        orphan_size,
    }
}
