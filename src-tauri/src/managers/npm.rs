use crate::utils::get_command;
use anyhow::Result;
use tokio::process::Command;

pub struct NpmManager;

impl NpmManager {
    /// 更新 npm 全局包 (异步)
    pub async fn update(name: &str) -> Result<String> {
        let npm_cmd = get_command("npm");

        let output = Command::new(&npm_cmd)
            .args(["update", "-g", name])
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(format!("更新成功: {}\n{}", name, stdout))
        } else {
            Err(anyhow::anyhow!("更新失败: {}", stderr))
        }
    }

    /// 卸载 npm 全局包 (异步)
    pub async fn uninstall(name: &str) -> Result<String> {
        let npm_cmd = get_command("npm");

        let output = Command::new(&npm_cmd)
            .args(["uninstall", "-g", name])
            .output()
            .await?;

        let stdout = String::from_utf8_lossy(&output.stdout).to_string();
        let stderr = String::from_utf8_lossy(&output.stderr).to_string();

        if output.status.success() {
            Ok(format!("卸载成功: {}\n{}", name, stdout))
        } else {
            Err(anyhow::anyhow!("卸载失败: {}", stderr))
        }
    }
}
