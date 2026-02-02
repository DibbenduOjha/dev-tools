use crate::models::{ToolInfo, ToolSource};
use crate::utils::get_command;
use anyhow::Result;
use tokio::process::Command;
use std::path::Path;

pub struct NpmScanner;

impl NpmScanner {
    /// 扫描 npm 全局安装的包 (异步)
    pub async fn scan() -> Result<Vec<ToolInfo>> {
        let npm_cmd = get_command("npm");

        // 获取全局安装路径
        let prefix_output = Command::new(&npm_cmd)
            .args(["config", "get", "prefix"])
            .output()
            .await?;

        let global_path = String::from_utf8_lossy(&prefix_output.stdout)
            .trim()
            .to_string();

        let output = Command::new(&npm_cmd)
            .args(["list", "-g", "--json", "--depth=0"])
            .output()
            .await?;

        if !output.status.success() {
            return Ok(vec![]);
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        let data: serde_json::Value = serde_json::from_str(&json_str)?;

        let mut tools = Vec::new();

        if let Some(deps) = data.get("dependencies").and_then(|d| d.as_object()) {
            for (full_name, info) in deps {
                let version = info.get("version")
                    .and_then(|v| v.as_str())
                    .map(String::from);

                // 解析 scope 和 name
                let (scope, name) = Self::parse_package_name(full_name);

                // 计算包大小
                let package_path = if global_path.is_empty() {
                    String::new()
                } else {
                    #[cfg(windows)]
                    let path = format!("{}/node_modules/{}", global_path, full_name);
                    #[cfg(not(windows))]
                    let path = format!("{}/lib/node_modules/{}", global_path, full_name);
                    path
                };

                let size_bytes = if !package_path.is_empty() {
                    Self::calc_dir_size(Path::new(&package_path)).await
                } else {
                    0
                };

                tools.push(ToolInfo {
                    name,
                    scope,
                    full_name: full_name.clone(),
                    version,
                    source: ToolSource::Npm,
                    install_path: package_path,
                    size_bytes,
                    description: None,
                    installed_at: None,
                    last_accessed: None,
                });
            }
        }

        Ok(tools)
    }

    /// 解析包名，分离 scope 和 name
    fn parse_package_name(full_name: &str) -> (Option<String>, String) {
        if full_name.starts_with('@') {
            if let Some(pos) = full_name.find('/') {
                let scope = full_name[..pos].to_string();
                let name = full_name[pos + 1..].to_string();
                return (Some(scope), name);
            }
        }
        (None, full_name.to_string())
    }

    /// 递归计算目录大小
    async fn calc_dir_size(path: &Path) -> u64 {
        let mut size = 0u64;

        if let Ok(mut entries) = tokio::fs::read_dir(path).await {
            while let Ok(Some(entry)) = entries.next_entry().await {
                let entry_path = entry.path();
                if entry_path.is_file() {
                    if let Ok(meta) = tokio::fs::metadata(&entry_path).await {
                        size += meta.len();
                    }
                } else if entry_path.is_dir() {
                    size += Box::pin(Self::calc_dir_size(&entry_path)).await;
                }
            }
        }

        size
    }
}
