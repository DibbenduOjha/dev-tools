use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;
use tokio::fs;

/// 代理配置信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxyConfig {
    pub tool: String,
    pub proxy: Option<String>,
    pub registry: Option<String>,
}

/// 获取所有代理配置
#[tauri::command]
pub async fn get_proxy_configs() -> Result<Vec<ProxyConfig>, String> {
    let mut configs = Vec::new();

    // npm 代理
    configs.push(get_npm_proxy().await);

    // yarn 代理
    configs.push(get_yarn_proxy().await);

    // pnpm 代理
    configs.push(get_pnpm_proxy().await);

    // pip 代理
    configs.push(get_pip_proxy().await);

    Ok(configs)
}

async fn get_npm_proxy() -> ProxyConfig {
    let proxy = Command::new("npm")
        .args(["config", "get", "proxy"])
        .stdout(Stdio::piped())
        .output()
        .await
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s != "null");

    let registry = Command::new("npm")
        .args(["config", "get", "registry"])
        .stdout(Stdio::piped())
        .output()
        .await
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty());

    ProxyConfig {
        tool: "npm".to_string(),
        proxy,
        registry,
    }
}

async fn get_yarn_proxy() -> ProxyConfig {
    let proxy = Command::new("yarn")
        .args(["config", "get", "proxy"])
        .stdout(Stdio::piped())
        .output()
        .await
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s != "undefined");

    let registry = Command::new("yarn")
        .args(["config", "get", "registry"])
        .stdout(Stdio::piped())
        .output()
        .await
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s != "undefined");

    ProxyConfig {
        tool: "yarn".to_string(),
        proxy,
        registry,
    }
}

async fn get_pnpm_proxy() -> ProxyConfig {
    let proxy = Command::new("pnpm")
        .args(["config", "get", "proxy"])
        .stdout(Stdio::piped())
        .output()
        .await
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s != "undefined");

    let registry = Command::new("pnpm")
        .args(["config", "get", "registry"])
        .stdout(Stdio::piped())
        .output()
        .await
        .ok()
        .and_then(|o| String::from_utf8(o.stdout).ok())
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty() && s != "undefined");

    ProxyConfig {
        tool: "pnpm".to_string(),
        proxy,
        registry,
    }
}

async fn get_pip_proxy() -> ProxyConfig {
    #[cfg(windows)]
    let home = std::env::var("USERPROFILE").unwrap_or_default();
    #[cfg(not(windows))]
    let home = std::env::var("HOME").unwrap_or_default();

    let pip_conf = format!("{}/pip/pip.ini", home);
    let proxy = fs::read_to_string(&pip_conf)
        .await
        .ok()
        .and_then(|content| {
            content.lines()
                .find(|l| l.trim().starts_with("proxy"))
                .map(|l| l.split('=').nth(1).unwrap_or("").trim().to_string())
        })
        .filter(|s| !s.is_empty());

    let registry = fs::read_to_string(&pip_conf)
        .await
        .ok()
        .and_then(|content| {
            content.lines()
                .find(|l| l.trim().starts_with("index-url"))
                .map(|l| l.split('=').nth(1).unwrap_or("").trim().to_string())
        })
        .filter(|s| !s.is_empty());

    ProxyConfig {
        tool: "pip".to_string(),
        proxy,
        registry,
    }
}

/// 设置代理配置
#[tauri::command]
pub async fn set_proxy_config(tool: String, proxy: Option<String>, registry: Option<String>) -> Result<String, String> {
    match tool.as_str() {
        "npm" => set_npm_proxy(proxy, registry).await,
        "yarn" => set_yarn_proxy(proxy, registry).await,
        "pnpm" => set_pnpm_proxy(proxy, registry).await,
        _ => Err(format!("不支持的工具: {}", tool)),
    }
}

async fn set_npm_proxy(proxy: Option<String>, registry: Option<String>) -> Result<String, String> {
    if let Some(p) = proxy {
        if p.is_empty() {
            Command::new("npm").args(["config", "delete", "proxy"]).output().await.ok();
        } else {
            Command::new("npm").args(["config", "set", "proxy", &p]).output().await
                .map_err(|e| e.to_string())?;
        }
    }
    if let Some(r) = registry {
        Command::new("npm").args(["config", "set", "registry", &r]).output().await
            .map_err(|e| e.to_string())?;
    }
    Ok("npm 配置已更新".to_string())
}

async fn set_yarn_proxy(proxy: Option<String>, registry: Option<String>) -> Result<String, String> {
    if let Some(p) = proxy {
        if p.is_empty() {
            Command::new("yarn").args(["config", "delete", "proxy"]).output().await.ok();
        } else {
            Command::new("yarn").args(["config", "set", "proxy", &p]).output().await
                .map_err(|e| e.to_string())?;
        }
    }
    if let Some(r) = registry {
        Command::new("yarn").args(["config", "set", "registry", &r]).output().await
            .map_err(|e| e.to_string())?;
    }
    Ok("yarn 配置已更新".to_string())
}

async fn set_pnpm_proxy(proxy: Option<String>, registry: Option<String>) -> Result<String, String> {
    if let Some(p) = proxy {
        if p.is_empty() {
            Command::new("pnpm").args(["config", "delete", "proxy"]).output().await.ok();
        } else {
            Command::new("pnpm").args(["config", "set", "proxy", &p]).output().await
                .map_err(|e| e.to_string())?;
        }
    }
    if let Some(r) = registry {
        Command::new("pnpm").args(["config", "set", "registry", &r]).output().await
            .map_err(|e| e.to_string())?;
    }
    Ok("pnpm 配置已更新".to_string())
}
