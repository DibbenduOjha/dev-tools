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

/// 获取扩展的 PATH 环境变量
fn get_extended_path_env() -> Vec<(String, String)> {
    let path = std::env::var("PATH").unwrap_or_default();

    #[cfg(target_os = "macos")]
    {
        let extra = ["/opt/homebrew/bin", "/usr/local/bin"];
        for p in extra {
            if !path.contains(p) {
                path = format!("{}:{}", p, path);
            }
        }
    }

    vec![("PATH".to_string(), path)]
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

    // pip 缓存 - 跨平台路径修复
    #[cfg(target_os = "macos")]
    let pip_cache = home.join("Library/Caches/pip");
    #[cfg(target_os = "windows")]
    let pip_cache = home.join("AppData/Local/pip/cache");
    #[cfg(not(any(target_os = "macos", target_os = "windows")))]
    let pip_cache = home.join(".cache/pip");

    caches.push(CacheInfo {
        name: "pip 缓存".to_string(),
        path: pip_cache.to_string_lossy().to_string(),
        size_bytes: get_dir_size(&pip_cache),
        exists: pip_cache.exists(),
    });

    // Gradle 缓存
    let gradle_cache = home.join(".gradle/caches");
    caches.push(CacheInfo {
        name: "Gradle 缓存".to_string(),
        path: gradle_cache.to_string_lossy().to_string(),
        size_bytes: get_dir_size(&gradle_cache),
        exists: gradle_cache.exists(),
    });

    // Maven 缓存
    let maven_cache = home.join(".m2/repository");
    caches.push(CacheInfo {
        name: "Maven 缓存".to_string(),
        path: maven_cache.to_string_lossy().to_string(),
        size_bytes: get_dir_size(&maven_cache),
        exists: maven_cache.exists(),
    });

    // Go modules 缓存
    let go_cache = home.join("go/pkg/mod/cache");
    caches.push(CacheInfo {
        name: "Go 缓存".to_string(),
        path: go_cache.to_string_lossy().to_string(),
        size_bytes: get_dir_size(&go_cache),
        exists: go_cache.exists(),
    });

    Ok(caches)
}

/// 清理 npm 缓存
#[tauri::command]
pub async fn clear_npm_cache() -> Result<String, String> {
    let output = Command::new("npm")
        .args(["cache", "clean", "--force"])
        .envs(get_extended_path_env())
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
        .envs(get_extended_path_env())
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
        .envs(get_extended_path_env())
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
        .envs(get_extended_path_env())
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

/// 清理 Gradle 缓存
#[tauri::command]
pub async fn clear_gradle_cache() -> Result<String, String> {
    let home = dirs::home_dir()
        .ok_or_else(|| "无法获取用户目录".to_string())?;

    let cache_path = home.join(".gradle/caches");

    if cache_path.exists() {
        std::fs::remove_dir_all(&cache_path)
            .map_err(|e| format!("清理失败: {}", e))?;
        std::fs::create_dir_all(&cache_path)
            .map_err(|e| format!("重建目录失败: {}", e))?;
    }

    Ok("Gradle 缓存清理完成".to_string())
}

/// 清理 Maven 缓存
#[tauri::command]
pub async fn clear_maven_cache() -> Result<String, String> {
    let home = dirs::home_dir()
        .ok_or_else(|| "无法获取用户目录".to_string())?;

    let cache_path = home.join(".m2/repository");

    if cache_path.exists() {
        std::fs::remove_dir_all(&cache_path)
            .map_err(|e| format!("清理失败: {}", e))?;
        std::fs::create_dir_all(&cache_path)
            .map_err(|e| format!("重建目录失败: {}", e))?;
    }

    Ok("Maven 缓存清理完成".to_string())
}

/// 清理 Go modules 缓存
#[tauri::command]
pub async fn clear_go_cache() -> Result<String, String> {
    let output = Command::new("go")
        .args(["clean", "-modcache"])
        .envs(get_extended_path_env())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行失败: {}", e))?;

    if output.status.success() {
        Ok("Go 缓存清理完成".to_string())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
