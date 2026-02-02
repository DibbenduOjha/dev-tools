use serde::{Deserialize, Serialize};
use std::path::Path;
use tokio::fs;

/// 磁盘使用信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DiskUsage {
    pub category: String,
    pub path: String,
    pub size_bytes: u64,
    pub item_count: u32,
}

/// 获取目录大小
async fn get_dir_size(path: &Path) -> u64 {
    let mut size = 0u64;

    if let Ok(mut entries) = fs::read_dir(path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let entry_path = entry.path();
            if let Ok(metadata) = fs::metadata(&entry_path).await {
                if metadata.is_file() {
                    size += metadata.len();
                } else if metadata.is_dir() {
                    size += Box::pin(get_dir_size(&entry_path)).await;
                }
            }
        }
    }

    size
}

/// 统计目录下的项目数量
async fn count_items(path: &Path) -> u32 {
    let mut count = 0u32;

    if let Ok(mut entries) = fs::read_dir(path).await {
        while let Ok(Some(_)) = entries.next_entry().await {
            count += 1;
        }
    }

    count
}

/// 扫描磁盘使用情况
#[tauri::command]
pub async fn scan_disk_usage() -> Result<Vec<DiskUsage>, String> {
    let mut results = Vec::new();

    #[cfg(windows)]
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    #[cfg(not(windows))]
    let home = std::env::var("HOME").unwrap_or_default();

    if home.is_empty() {
        return Err("无法获取用户目录".to_string());
    }

    // npm 相关目录
    let npm_paths = vec![
        (format!("{}/AppData/Roaming/npm", home), "npm 全局包"),
        (format!("{}/AppData/Local/npm-cache", home), "npm 缓存"),
        (format!("{}/.npm", home), "npm 配置"),
    ];

    // pnpm 相关目录
    let pnpm_paths = vec![
        (format!("{}/AppData/Local/pnpm", home), "pnpm 全局"),
        (format!("{}/AppData/Local/pnpm-cache", home), "pnpm 缓存"),
        (format!("{}/AppData/Local/pnpm-store", home), "pnpm 存储"),
    ];

    // yarn 相关目录
    let yarn_paths = vec![
        (format!("{}/AppData/Local/Yarn", home), "yarn 数据"),
    ];

    // cargo 相关目录
    let cargo_paths = vec![
        (format!("{}/.cargo/bin", home), "cargo 可执行文件"),
        (format!("{}/.cargo/registry", home), "cargo 注册表"),
        (format!("{}/.cargo/git", home), "cargo git 缓存"),
    ];

    // pip 相关目录
    let pip_paths = vec![
        (format!("{}/AppData/Local/pip", home), "pip 缓存"),
    ];

    // Python 安装目录
    let python_paths = vec![
        (format!("{}/AppData/Local/Programs/Python", home), "Python 安装"),
    ];

    // Node.js 相关
    let node_paths = vec![
        (format!("{}/AppData/Roaming/nvm", home), "nvm 版本"),
        (format!("{}/AppData/Local/fnm_multishells", home), "fnm 缓存"),
    ];

    // 所有路径合并
    let all_paths: Vec<(String, &str)> = [
        npm_paths,
        pnpm_paths,
        yarn_paths,
        cargo_paths,
        pip_paths,
        python_paths,
        node_paths,
    ].concat();

    for (path_str, category) in all_paths {
        let path = Path::new(&path_str);
        if path.exists() {
            let size = get_dir_size(path).await;
            let count = count_items(path).await;

            if size > 0 {
                results.push(DiskUsage {
                    category: category.to_string(),
                    path: path_str,
                    size_bytes: size,
                    item_count: count,
                });
            }
        }
    }

    // 按大小排序
    results.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

    Ok(results)
}

/// 获取单个目录的详细信息
#[tauri::command]
pub async fn get_dir_details(path: String) -> Result<Vec<DiskUsage>, String> {
    let dir_path = Path::new(&path);

    if !dir_path.exists() {
        return Err("目录不存在".to_string());
    }

    let mut results = Vec::new();

    if let Ok(mut entries) = fs::read_dir(dir_path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let entry_path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            if let Ok(metadata) = fs::metadata(&entry_path).await {
                let size = if metadata.is_dir() {
                    get_dir_size(&entry_path).await
                } else {
                    metadata.len()
                };

                let count = if metadata.is_dir() {
                    count_items(&entry_path).await
                } else {
                    1
                };

                results.push(DiskUsage {
                    category: name,
                    path: entry_path.to_string_lossy().to_string(),
                    size_bytes: size,
                    item_count: count,
                });
            }
        }
    }

    // 按大小排序
    results.sort_by(|a, b| b.size_bytes.cmp(&a.size_bytes));

    Ok(results)
}
