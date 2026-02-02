use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;

/// 包搜索结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PackageSearchResult {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub source: String,
}

/// 搜索 npm 包
#[tauri::command]
pub async fn search_npm_packages(query: String) -> Result<Vec<PackageSearchResult>, String> {
    let output = Command::new("npm")
        .args(["search", "--json", &query])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行 npm search 失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let json_str = String::from_utf8_lossy(&output.stdout);

    #[derive(Deserialize)]
    struct NpmSearchItem {
        name: String,
        version: String,
        description: Option<String>,
    }

    let items: Vec<NpmSearchItem> = serde_json::from_str(&json_str)
        .map_err(|e| format!("解析 npm 搜索结果失败: {}", e))?;

    Ok(items
        .into_iter()
        .take(20)
        .map(|item| PackageSearchResult {
            name: item.name,
            version: item.version,
            description: item.description,
            source: "npm".to_string(),
        })
        .collect())
}

/// 搜索 cargo 包 (通过 crates.io API)
#[tauri::command]
pub async fn search_cargo_packages(query: String) -> Result<Vec<PackageSearchResult>, String> {
    let url = format!(
        "https://crates.io/api/v1/crates?q={}&per_page=20",
        urlencoding::encode(&query)
    );

    let client = reqwest::Client::builder()
        .user_agent("devtool-manager/0.1.0")
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let response = client
        .get(&url)
        .send()
        .await
        .map_err(|e| format!("请求 crates.io 失败: {}", e))?;

    if !response.status().is_success() {
        return Err(format!("crates.io 返回错误: {}", response.status()));
    }

    #[derive(Deserialize)]
    struct CratesResponse {
        crates: Vec<CrateInfo>,
    }

    #[derive(Deserialize)]
    struct CrateInfo {
        name: String,
        max_version: String,
        description: Option<String>,
    }

    let data: CratesResponse = response
        .json()
        .await
        .map_err(|e| format!("解析 crates.io 响应失败: {}", e))?;

    Ok(data
        .crates
        .into_iter()
        .map(|c| PackageSearchResult {
            name: c.name,
            version: c.max_version,
            description: c.description,
            source: "cargo".to_string(),
        })
        .collect())
}

/// 搜索 pip 包 (通过 PyPI API)
#[tauri::command]
pub async fn search_pip_packages(query: String) -> Result<Vec<PackageSearchResult>, String> {
    // PyPI 没有官方搜索 API，使用 pip index versions 或者简单的包信息查询
    // 这里使用 PyPI JSON API 查询单个包信息
    let url = format!("https://pypi.org/pypi/{}/json", query);

    let client = reqwest::Client::builder()
        .user_agent("devtool-manager/0.1.0")
        .build()
        .map_err(|e| format!("创建 HTTP 客户端失败: {}", e))?;

    let response = client.get(&url).send().await;

    match response {
        Ok(resp) if resp.status().is_success() => {
            #[derive(Deserialize)]
            struct PyPIResponse {
                info: PyPIInfo,
            }

            #[derive(Deserialize)]
            struct PyPIInfo {
                name: String,
                version: String,
                summary: Option<String>,
            }

            let data: PyPIResponse = resp
                .json()
                .await
                .map_err(|e| format!("解析 PyPI 响应失败: {}", e))?;

            Ok(vec![PackageSearchResult {
                name: data.info.name,
                version: data.info.version,
                description: data.info.summary,
                source: "pip".to_string(),
            }])
        }
        _ => {
            // 如果精确匹配失败，返回空结果并提示用户
            Ok(vec![])
        }
    }
}

/// 安装 npm 包
#[tauri::command]
pub async fn install_npm_package(name: String) -> Result<String, String> {
    let output = Command::new("npm")
        .args(["install", "-g", &name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行 npm install 失败: {}", e))?;

    if output.status.success() {
        Ok(format!("成功安装 {}", name))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

/// 安装 cargo 包
#[tauri::command]
pub async fn install_cargo_package(name: String) -> Result<String, String> {
    let output = Command::new("cargo")
        .args(["install", &name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行 cargo install 失败: {}", e))?;

    if output.status.success() {
        Ok(format!("成功安装 {}", name))
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        // cargo install 的正常输出可能在 stderr
        if stderr.contains("Installed package") || stderr.contains("Replacing") {
            Ok(format!("成功安装 {}", name))
        } else {
            Err(stderr.to_string())
        }
    }
}

/// 安装 pip 包
#[tauri::command]
pub async fn install_pip_package(name: String) -> Result<String, String> {
    let output = Command::new("pip")
        .args(["install", &name])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行 pip install 失败: {}", e))?;

    if output.status.success() {
        Ok(format!("成功安装 {}", name))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}

// URL 编码辅助模块
mod urlencoding {
    pub fn encode(s: &str) -> String {
        let mut result = String::new();
        for c in s.chars() {
            match c {
                'a'..='z' | 'A'..='Z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                    result.push(c);
                }
                _ => {
                    for byte in c.to_string().bytes() {
                        result.push_str(&format!("%{:02X}", byte));
                    }
                }
            }
        }
        result
    }
}

/// 获取 npm 包的版本列表
#[tauri::command]
pub async fn get_npm_versions(name: String) -> Result<Vec<String>, String> {
    let output = Command::new("npm")
        .args(["view", &name, "versions", "--json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行失败: {}", e))?;

    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).to_string());
    }

    let json_str = String::from_utf8_lossy(&output.stdout);
    let versions: Vec<String> = serde_json::from_str(&json_str)
        .map_err(|e| format!("解析失败: {}", e))?;

    // 返回最近的 20 个版本（倒序）
    Ok(versions.into_iter().rev().take(20).collect())
}

/// 安装指定版本的 npm 包
#[tauri::command]
pub async fn install_npm_version(name: String, version: String) -> Result<String, String> {
    let pkg = format!("{}@{}", name, version);
    let output = Command::new("npm")
        .args(["install", "-g", &pkg])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行失败: {}", e))?;

    if output.status.success() {
        Ok(format!("成功安装 {} 版本 {}", name, version))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
