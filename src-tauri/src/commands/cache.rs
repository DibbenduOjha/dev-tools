use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use tokio::process::Command;
use std::process::Stdio;
use walkdir::WalkDir;

/// 缓存信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CacheInfo {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub exists: bool,
}

/// 获取目录大小
fn get_dir_size(path: &PathBuf) -> u64 {
    if !path.exists() {
        return 0;
    }

    WalkDir::new(path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
        .filter_map(|e| e.metadata().ok())
        .map(|m| m.len())
        .sum()
}

/// 扫描所有缓存目录
#[tauri::command]
pub async fn scan_caches() -> Result<Vec<CacheInfo>, String> {
    let home = dirs::home_dir()
        .ok_or_else(|| "无法获取用户目录".to_string())?;

    let mut caches = Vec::new();

    // npm 缓存
    let npm_cache = home.join(".npm/_cacache");
    caches.push(CacheInfo {
        name: "npm 缓存".to_string(),
        path: npm_cache.to_string_lossy().to_string(),
        size_bytes: get_dir_size(&npm_cache),
        exists: npm_cache.exists(),
    });

    // pnpm 缓存
    let pnpm_cache = home.join(".pnpm-store");
    caches.push(CacheInfo {
        name: "pnpm 缓存".to_string(),
        path: pnpm_cache.to_string_lossy().to_string(),
        size_bytes: get_dir_size(&pnpm_cache),
        exists: pnpm_cache.exists(),
    });

    // yarn 缓存
    let yarn_cache = home.join(".yarn/cache");
    caches.push(CacheInfo {
        name: "yarn 缓存".to_string(),
        path: yarn_cache.to_string_lossy().to_string(),
        size_bytes: get_dir_size(&yarn_cache),
        exists: yarn_cache.exists(),
    });

    // cargo 缓存
    let cargo_cache = home.join(".cargo/registry/cache");
    caches.push(CacheInfo {
        name: "cargo 缓存".to_string(),
        path: cargo_cache.to_string_lossy().to_string(),
        size_bytes: get_dir_size(&cargo_cache),
        exists: cargo_cache.exists(),
    });

    // pip 缓存
    let pip_cache = home.join(".cache/pip");
    caches.push(CacheInfo {
        name: "pip 缓存".to_string(),
        path: pip_cache.to_string_lossy().to_string(),
        size_bytes: get_dir_size(&pip_cache),
        exists: pip_cache.exists(),
    });

    Ok(caches)
}

/// 清理 npm 缓存
#[tauri::command]
pub async fn clear_npm_cache() -> Result<String, String> {
    let output = Command::new("npm")
        .args(["cache", "clean", "--force"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行失败: {}", e))?;

    if output.status.success() {
        Ok("npm 缓存清理完成".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// 清理 pnpm 缓存
#[tauri::command]
pub async fn clear_pnpm_cache() -> Result<String, String> {
    let output = Command::new("pnpm")
        .args(["store", "prune"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行失败: {}", e))?;

    if output.status.success() {
        Ok("pnpm 缓存清理完成".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// 清理 yarn 缓存
#[tauri::command]
pub async fn clear_yarn_cache() -> Result<String, String> {
    let output = Command::new("yarn")
        .args(["cache", "clean"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行失败: {}", e))?;

    if output.status.success() {
        Ok("yarn 缓存清理完成".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// 清理 cargo 缓存
#[tauri::command]
pub async fn clear_cargo_cache() -> Result<String, String> {
    let home = dirs::home_dir()
        .ok_or_else(|| "无法获取用户目录".to_string())?;

    let cache_path = home.join(".cargo/registry/cache");

    if cache_path.exists() {
        std::fs::remove_dir_all(&cache_path)
            .map_err(|e| format!("清理失败: {}", e))?;
        std::fs::create_dir_all(&cache_path)
            .map_err(|e| format!("重建目录失败: {}", e))?;
    }

    Ok("cargo 缓存清理完成".to_string())
}

/// 清理 pip 缓存
#[tauri::command]
pub async fn clear_pip_cache() -> Result<String, String> {
    let output = Command::new("pip")
        .args(["cache", "purge"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行失败: {}", e))?;

    if output.status.success() {
        Ok("pip 缓存清理完成".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
