use crate::models::DotFolder;
use crate::utils::get_home_dir;
use anyhow::Result;
use chrono::{DateTime, Utc};
use tokio::fs;

pub struct DotfilesScanner;

impl DotfilesScanner {
    /// 扫描用户目录下的配置文件夹 (异步)
    pub async fn scan() -> Result<Vec<DotFolder>> {
        let home = get_home_dir();
        let mut folders = Vec::new();

        let mut entries = fs::read_dir(&home).await?;

        while let Some(entry) = entries.next_entry().await? {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();

            // 只扫描以 . 开头的目录
            if !name.starts_with('.') || !path.is_dir() {
                continue;
            }

            // 跳过一些系统目录
            if name == "." || name == ".." {
                continue;
            }

            let metadata = fs::metadata(&path).await.ok();
            let modified_at = metadata
                .and_then(|m| m.modified().ok())
                .map(|t| DateTime::<Utc>::from(t));

            // 计算文件夹大小和文件数量
            let (size, count) = Self::calc_folder_size(&path).await;

            folders.push(DotFolder {
                name: name.clone(),
                path: path.to_string_lossy().to_string(),
                size_bytes: size,
                file_count: count,
                modified_at,
                related_tool: Self::guess_related_tool(&name),
            });
        }

        // 按名称排序
        folders.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(folders)
    }

    /// 计算文件夹大小 (异步)
    async fn calc_folder_size(path: &std::path::Path) -> (u64, u32) {
        let mut size = 0u64;
        let mut count = 0u32;

        if let Ok(mut entries) = fs::read_dir(path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let path = entry.path();
                if path.is_file() {
                    size += fs::metadata(&path).await.map(|m| m.len()).unwrap_or(0);
                    count += 1;
                }
            }
        }

        (size, count)
    }

    /// 猜测关联的工具
    fn guess_related_tool(name: &str) -> Option<String> {
        match name {
            ".npm" | ".npmrc" => Some("npm".to_string()),
            ".cargo" => Some("cargo".to_string()),
            ".rustup" => Some("rustup".to_string()),
            ".nvm" => Some("nvm".to_string()),
            ".fnm" => Some("fnm".to_string()),
            ".bun" => Some("bun".to_string()),
            ".deno" => Some("deno".to_string()),
            ".pip" => Some("pip".to_string()),
            ".vscode" => Some("vscode".to_string()),
            ".git" => Some("git".to_string()),
            ".ssh" => Some("ssh".to_string()),
            _ => None,
        }
    }
}
