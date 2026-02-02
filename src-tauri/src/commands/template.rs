use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;

/// 项目模板信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectTemplate {
    pub name: String,
    pub description: String,
    pub command: String,
    pub category: String,
}

/// 获取可用的项目模板
#[tauri::command]
pub async fn get_project_templates() -> Vec<ProjectTemplate> {
    vec![
        // React 模板
        ProjectTemplate {
            name: "React + Vite".to_string(),
            description: "使用 Vite 创建 React 项目".to_string(),
            command: "npm create vite@latest {name} -- --template react-ts".to_string(),
            category: "React".to_string(),
        },
        ProjectTemplate {
            name: "Next.js".to_string(),
            description: "创建 Next.js 全栈应用".to_string(),
            command: "npx create-next-app@latest {name}".to_string(),
            category: "React".to_string(),
        },
        // Vue 模板
        ProjectTemplate {
            name: "Vue + Vite".to_string(),
            description: "使用 Vite 创建 Vue 3 项目".to_string(),
            command: "npm create vite@latest {name} -- --template vue-ts".to_string(),
            category: "Vue".to_string(),
        },
        ProjectTemplate {
            name: "Nuxt".to_string(),
            description: "创建 Nuxt 3 全栈应用".to_string(),
            command: "npx nuxi@latest init {name}".to_string(),
            category: "Vue".to_string(),
        },
        // Tauri 模板
        ProjectTemplate {
            name: "Tauri + React".to_string(),
            description: "创建 Tauri 桌面应用".to_string(),
            command: "npm create tauri-app@latest {name}".to_string(),
            category: "Desktop".to_string(),
        },
        // Node.js 模板
        ProjectTemplate {
            name: "Express".to_string(),
            description: "创建 Express 后端项目".to_string(),
            command: "npx express-generator {name}".to_string(),
            category: "Node.js".to_string(),
        },
    ]
}

/// 创建项目
#[tauri::command]
pub async fn create_project(template_command: String, name: String, path: String) -> Result<String, String> {
    let cmd = template_command.replace("{name}", &name);
    let parts: Vec<&str> = cmd.split_whitespace().collect();

    if parts.is_empty() {
        return Err("无效的命令".to_string());
    }

    let output = Command::new(parts[0])
        .args(&parts[1..])
        .current_dir(&path)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行失败: {}", e))?;

    if output.status.success() {
        Ok(format!("项目 {} 创建成功", name))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(stderr.to_string())
    }
}
