use crate::models::{ToolInfo, ToolSource};
use crate::utils::get_command;
use anyhow::Result;
use tokio::process::Command;

pub struct PipScanner;

impl PipScanner {
    /// 扫描 pip 安装的包 (异步)
    pub async fn scan() -> Result<Vec<ToolInfo>> {
        // 尝试多种 pip 命令
        let pip_commands = vec![
            get_command("pip"),
            get_command("pip3"),
            "pip".to_string(),
            "pip3".to_string(),
        ];

        for pip_cmd in pip_commands {
            if let Ok(output) = Command::new(&pip_cmd)
                .args(["list", "--format=json"])
                .output()
                .await
            {
                if output.status.success() {
                    let json_str = String::from_utf8_lossy(&output.stdout);
                    if let Ok(packages) = serde_json::from_str::<Vec<serde_json::Value>>(&json_str) {
                        let mut tools = Vec::new();

                        for pkg in packages {
                            let name = pkg.get("name")
                                .and_then(|v| v.as_str())
                                .unwrap_or_default()
                                .to_string();

                            let version = pkg.get("version")
                                .and_then(|v| v.as_str())
                                .map(String::from);

                            if !name.is_empty() {
                                tools.push(ToolInfo {
                                    name: name.clone(),
                                    scope: None,
                                    full_name: name.clone(),
                                    version,
                                    source: ToolSource::Pip,
                                    install_path: String::new(),
                                    size_bytes: 0,
                                    description: None,
                                    installed_at: None,
                                    last_accessed: None,
                                });
                            }
                        }

                        return Ok(tools);
                    }
                }
            }
        }

        // 如果所有命令都失败，返回空列表而不是错误
        Ok(vec![])
    }
}
