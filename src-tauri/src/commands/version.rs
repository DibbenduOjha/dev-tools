use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;

/// 运行时版本信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuntimeVersion {
    pub name: String,
    pub version: Option<String>,
    pub path: Option<String>,
    pub manager: Option<String>,
}

/// 检测 Node 版本管理器
async fn detect_node_manager() -> Option<String> {
    if Command::new("fnm").args(["--version"]).output().await.is_ok() {
        return Some("fnm".to_string());
    }
    if Command::new("volta").args(["--version"]).output().await.is_ok() {
        return Some("volta".to_string());
    }
    None
}

/// 检测 Python 版本管理器
async fn detect_python_manager() -> Option<String> {
    if Command::new("pyenv").args(["--version"]).output().await.is_ok() {
        return Some("pyenv".to_string());
    }
    if Command::new("conda").args(["--version"]).output().await.is_ok() {
        return Some("conda".to_string());
    }
    None
}

/// 获取 Node.js 版本
async fn get_node_version() -> RuntimeVersion {
    let version = Command::new("node")
        .args(["--version"])
        .stdout(Stdio::piped())
        .output()
        .await
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string());

    let path = which::which("node")
        .ok()
        .map(|p| p.to_string_lossy().to_string());

    let manager = detect_node_manager().await;

    RuntimeVersion {
        name: "Node.js".to_string(),
        version,
        path,
        manager,
    }
}

/// 获取 Python 版本
async fn get_python_version() -> RuntimeVersion {
    let version = Command::new("python")
        .args(["--version"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .ok()
        .and_then(|o| {
            let out = String::from_utf8_lossy(&o.stdout);
            let err = String::from_utf8_lossy(&o.stderr);
            let v = if out.contains("Python") { out } else { err };
            Some(v.replace("Python ", "").trim().to_string())
        });

    let path = which::which("python")
        .ok()
        .map(|p| p.to_string_lossy().to_string());

    let manager = detect_python_manager().await;

    RuntimeVersion {
        name: "Python".to_string(),
        version,
        path,
        manager,
    }
}

/// 获取 Rust 版本
async fn get_rust_version() -> RuntimeVersion {
    let version = Command::new("rustc")
        .args(["--version"])
        .stdout(Stdio::piped())
        .output()
        .await
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| {
            s.replace("rustc ", "")
                .split_whitespace()
                .next()
                .unwrap_or("")
                .to_string()
        });

    let path = which::which("rustc")
        .ok()
        .map(|p| p.to_string_lossy().to_string());

    RuntimeVersion {
        name: "Rust".to_string(),
        version,
        path,
        manager: Some("rustup".to_string()),
    }
}

/// 获取所有运行时版本
#[tauri::command]
pub async fn get_runtime_versions() -> Vec<RuntimeVersion> {
    let (node, python, rust) = tokio::join!(
        get_node_version(),
        get_python_version(),
        get_rust_version()
    );
    vec![node, python, rust]
}
