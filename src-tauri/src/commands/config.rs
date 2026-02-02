use std::path::Path;
use crate::utils::get_home_dir;
use crate::models::ConfigFile;
use tokio::fs;

/// 获取用户主目录
#[tauri::command]
pub fn get_home_path() -> Result<String, String> {
    Ok(get_home_dir().to_string_lossy().to_string())
}

/// 读取配置文件内容 (异步)
#[tauri::command]
pub async fn read_config_file(path: String) -> Result<String, String> {
    let file_path = Path::new(&path);

    if !file_path.exists() {
        return Err("文件不存在".to_string());
    }

    fs::read_to_string(file_path).await.map_err(|e| e.to_string())
}

/// 写入配置文件内容 (异步)
#[tauri::command]
pub async fn write_config_file(path: String, content: String) -> Result<String, String> {
    let file_path = Path::new(&path);

    fs::write(file_path, &content).await.map_err(|e| e.to_string())?;

    Ok("保存成功".to_string())
}

/// 列出目录下的 JSON 配置文件 (异步)
#[tauri::command]
pub async fn list_json_files(dir_path: String) -> Result<Vec<String>, String> {
    let path = Path::new(&dir_path);

    if !path.exists() || !path.is_dir() {
        return Ok(vec![]);
    }

    let mut files = Vec::new();

    let mut entries = fs::read_dir(path).await.map_err(|e| e.to_string())?;

    while let Some(entry) = entries.next_entry().await.map_err(|e| e.to_string())? {
        let file_path = entry.path();
        if file_path.is_file() {
            if let Some(ext) = file_path.extension() {
                if ext == "json" || ext == "toml" || ext == "yaml" || ext == "yml" {
                    files.push(file_path.to_string_lossy().to_string());
                }
            }
            // 也包含无扩展名但常见的配置文件
            if let Some(name) = file_path.file_name() {
                let name_str = name.to_string_lossy();
                if name_str == "config" || name_str.starts_with('.') {
                    files.push(file_path.to_string_lossy().to_string());
                }
            }
        }
    }

    files.sort();
    Ok(files)
}

/// 递归列出目录下的所有配置文件 (异步)
#[tauri::command]
pub async fn list_json_files_recursive(dir_path: String) -> Result<Vec<ConfigFile>, String> {
    let base_path = Path::new(&dir_path);

    if !base_path.exists() || !base_path.is_dir() {
        return Ok(vec![]);
    }

    let mut files = Vec::new();
    collect_config_files_recursive(base_path, base_path, &mut files).await;

    // 按目录和文件名排序
    files.sort_by(|a, b| {
        let dir_cmp = a.dir.cmp(&b.dir);
        if dir_cmp == std::cmp::Ordering::Equal {
            a.name.cmp(&b.name)
        } else {
            dir_cmp
        }
    });

    Ok(files)
}

/// 递归收集配置文件
async fn collect_config_files_recursive(
    base_path: &Path,
    current_path: &Path,
    files: &mut Vec<ConfigFile>,
) {
    if let Ok(mut entries) = fs::read_dir(current_path).await {
        while let Ok(Some(entry)) = entries.next_entry().await {
            let entry_path = entry.path();

            if entry_path.is_file() {
                let is_config = if let Some(ext) = entry_path.extension() {
                    let ext_str = ext.to_string_lossy().to_lowercase();
                    ext_str == "json" || ext_str == "toml" || ext_str == "yaml" || ext_str == "yml"
                } else {
                    false
                };

                if is_config {
                    let relative_dir = current_path
                        .strip_prefix(base_path)
                        .unwrap_or(Path::new(""))
                        .to_string_lossy()
                        .to_string();

                    let file_name = entry_path
                        .file_name()
                        .map(|n| n.to_string_lossy().to_string())
                        .unwrap_or_default();

                    files.push(ConfigFile {
                        path: entry_path.to_string_lossy().to_string(),
                        name: file_name,
                        dir: if relative_dir.is_empty() { ".".to_string() } else { relative_dir },
                    });
                }
            } else if entry_path.is_dir() {
                // 跳过一些不需要扫描的目录
                let dir_name = entry_path.file_name()
                    .map(|n| n.to_string_lossy().to_string())
                    .unwrap_or_default();

                // 只跳过特定的目录，允许扫描 .config 等配置目录
                let skip_dirs = [
                    "node_modules", "cache", "_cacache", ".git", ".svn",
                    ".hg", "target", "dist", "build", "__pycache__"
                ];

                if !skip_dirs.contains(&dir_name.as_str()) {
                    Box::pin(collect_config_files_recursive(base_path, &entry_path, files)).await;
                }
            }
        }
    }
}
