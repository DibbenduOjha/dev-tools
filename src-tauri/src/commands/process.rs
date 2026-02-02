use serde::{Deserialize, Serialize};
use sysinfo::System;

/// 进程信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessInfo {
    pub pid: u32,
    pub name: String,
    pub cpu_usage: f32,
    pub memory_mb: f64,
    pub status: String,
}

/// 扫描开发相关进程
#[tauri::command]
pub async fn scan_dev_processes() -> Result<Vec<ProcessInfo>, String> {
    let mut sys = System::new_all();
    sys.refresh_processes();

    // 开发相关的进程关键词
    let dev_keywords = [
        "node", "npm", "pnpm", "yarn", "deno", "bun",
        "python", "pip", "cargo", "rustc", "rust-analyzer",
        "code", "cursor", "idea", "webstorm", "vscode",
        "docker", "git", "java", "gradle", "maven",
        "webpack", "vite", "esbuild", "tsc", "eslint",
    ];

    let processes: Vec<ProcessInfo> = sys
        .processes()
        .iter()
        .filter_map(|(pid, process)| {
            let name = process.name().to_lowercase();
            let is_dev = dev_keywords.iter().any(|k| name.contains(k));

            if is_dev {
                Some(ProcessInfo {
                    pid: pid.as_u32(),
                    name: process.name().to_string(),
                    cpu_usage: process.cpu_usage(),
                    memory_mb: process.memory() as f64 / 1024.0 / 1024.0,
                    status: format!("{:?}", process.status()),
                })
            } else {
                None
            }
        })
        .collect();

    Ok(processes)
}
