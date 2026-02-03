use serde::{Deserialize, Serialize};
use std::path::PathBuf;
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

/// 获取扩展的 PATH 环境变量
fn get_extended_path() -> String {
    let paths = std::env::var("PATH").unwrap_or_default();

    #[cfg(target_os = "macos")]
    {
        if let Some(home) = dirs::home_dir() {
            let extra_paths = [
                "/opt/homebrew/bin".to_string(),
                "/usr/local/bin".to_string(),
                format!("{}/.nvm/versions/node", home.display()),
                format!("{}/.asdf/shims", home.display()),
                format!("{}/.volta/bin", home.display()),
            ];
            for p in extra_paths {
                if !paths.contains(&p) {
                    paths = format!("{}:{}", p, paths);
                }
            }
        }
    }

    paths
}

/// 使用扩展 PATH 查找命令
fn find_command(cmd: &str) -> Option<PathBuf> {
    // 首先尝试标准 which
    if let Ok(path) = which::which(cmd) {
        return Some(path);
    }

    // 手动搜索扩展路径
    let sep = if cfg!(windows) { ';' } else { ':' };
    for dir in get_extended_path().split(sep) {
        let full_path = std::path::Path::new(dir).join(cmd);
        if full_path.exists() {
            return Some(full_path);
        }
        // Windows 下尝试添加 .exe 后缀
        #[cfg(windows)]
        {
            let exe_path = std::path::Path::new(dir).join(format!("{}.exe", cmd));
            if exe_path.exists() {
                return Some(exe_path);
            }
        }
    }

    None
}

/// 检测 Node 版本管理器
async fn detect_node_manager() -> Option<String> {
    let home = dirs::home_dir()?;

    // 检测 nvm (优先检测目录，更快)
    if home.join(".nvm").exists() {
        return Some("nvm".to_string());
    }
    // 检测 fnm
    if Command::new("fnm").args(["--version"]).output().await.is_ok() {
        return Some("fnm".to_string());
    }
    // 检测 volta
    if home.join(".volta").exists() || Command::new("volta").args(["--version"]).output().await.is_ok() {
        return Some("volta".to_string());
    }
    // 检测 asdf
    if home.join(".asdf").exists() {
        return Some("asdf".to_string());
    }
    // 检测 Homebrew (macOS)
    #[cfg(target_os = "macos")]
    {
        let brew_node = std::path::Path::new("/opt/homebrew/bin/node");
        let brew_node_intel = std::path::Path::new("/usr/local/bin/node");
        if brew_node.exists() || brew_node_intel.exists() {
            return Some("homebrew".to_string());
        }
    }

    None
}

/// 检测 Python 版本管理器
async fn detect_python_manager() -> Option<String> {
    let home = dirs::home_dir()?;

    // 检测 pyenv (优先检测目录，更快)
    if home.join(".pyenv").exists() {
        return Some("pyenv".to_string());
    }
    // 检测 conda
    if Command::new("conda").args(["--version"]).output().await.is_ok() {
        return Some("conda".to_string());
    }
    // 检测 asdf
    if home.join(".asdf").exists() {
        return Some("asdf".to_string());
    }
    // 检测 Homebrew (macOS)
    #[cfg(target_os = "macos")]
    {
        let brew_python = std::path::Path::new("/opt/homebrew/bin/python3");
        let brew_python_intel = std::path::Path::new("/usr/local/bin/python3");
        if brew_python.exists() || brew_python_intel.exists() {
            return Some("homebrew".to_string());
        }
    }

    None
}

/// 获取 Node.js 版本
async fn get_node_version() -> RuntimeVersion {
    let path = find_command("node");

    let version = if let Some(ref p) = path {
        Command::new(p)
            .args(["--version"])
            .stdout(Stdio::piped())
            .output()
            .await
            .ok()
            .and_then(|o| String::from_utf8(o.stdout).ok())
            .map(|s| s.trim().to_string())
    } else {
        None
    };

    let manager = detect_node_manager().await;

    RuntimeVersion {
        name: "Node.js".to_string(),
        version,
        path: path.map(|p| p.to_string_lossy().to_string()),
        manager,
    }
}

/// 获取 Python 版本
async fn get_python_version() -> RuntimeVersion {
    // 优先尝试 python3，然后 python
    let path = find_command("python3").or_else(|| find_command("python"));

    let version = if let Some(ref p) = path {
        Command::new(p)
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
            })
    } else {
        None
    };

    let manager = detect_python_manager().await;

    RuntimeVersion {
        name: "Python".to_string(),
        version,
        path: path.map(|p| p.to_string_lossy().to_string()),
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
