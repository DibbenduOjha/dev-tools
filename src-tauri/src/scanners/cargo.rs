use crate::models::{ToolInfo, ToolSource};
use crate::utils::get_home_dir;
use anyhow::Result;
use tokio::fs;
use std::path::Path;

pub struct CargoScanner;

impl CargoScanner {
    /// 扫描 cargo 安装的工具 (异步)
    pub async fn scan() -> Result<Vec<ToolInfo>> {
        let mut tools = Vec::new();

        let cargo_home = get_home_dir().join(".cargo");
        let crates_toml = cargo_home.join(".crates.toml");

        if !crates_toml.exists() {
            return Ok(tools);
        }

        let content = fs::read_to_string(&crates_toml).await?;
        let parsed: toml::Value = toml::from_str(&content)?;

        if let Some(v1) = parsed.get("v1").and_then(|v| v.as_table()) {
            for (key, _) in v1 {
                // key 格式: "package_name version (registry+url)"
                let parts: Vec<&str> = key.split_whitespace().collect();
                if parts.len() >= 2 {
                    let name = parts[0].to_string();
                    let version = parts[1].to_string();

                    // 计算工具大小 (bin 目录下的可执行文件)
                    let bin_path = cargo_home.join("bin");
                    let size_bytes = Self::calc_tool_size(&bin_path, &name).await;

                    tools.push(ToolInfo {
                        name: name.clone(),
                        scope: None,
                        full_name: name.clone(),
                        version: Some(version),
                        source: ToolSource::Cargo,
                        install_path: bin_path.to_string_lossy().to_string(),
                        size_bytes,
                        description: None,
                        installed_at: None,
                        last_accessed: None,
                    });
                }
            }
        }

        Ok(tools)
    }

    /// 计算工具大小
    async fn calc_tool_size(bin_path: &Path, name: &str) -> u64 {
        // 尝试查找可执行文件
        #[cfg(windows)]
        let exe_name = format!("{}.exe", name);
        #[cfg(not(windows))]
        let exe_name = name.to_string();

        let exe_path = bin_path.join(&exe_name);
        if let Ok(meta) = fs::metadata(&exe_path).await {
            return meta.len();
        }

        0
    }
}
