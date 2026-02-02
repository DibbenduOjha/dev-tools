use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::process::Stdio;
use tokio::process::Command;

/// 孤儿依赖信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OrphanDependency {
    pub name: String,
    pub version: String,
    pub source: String,
    pub reason: String,
    pub size_bytes: u64,
}

/// 扫描 npm 孤儿依赖
async fn scan_npm_orphans() -> Vec<OrphanDependency> {
    let mut orphans = Vec::new();

    let output = Command::new("npm")
        .args(["list", "-g", "--depth=0", "--json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await;

    if let Ok(output) = output {
        if let Ok(json_str) = String::from_utf8(output.stdout) {
            #[derive(Deserialize)]
            struct NpmList {
                dependencies: Option<std::collections::HashMap<String, NpmPkg>>,
            }
            #[derive(Deserialize)]
            struct NpmPkg {
                version: Option<String>,
            }

            if let Ok(list) = serde_json::from_str::<NpmList>(&json_str) {
                if let Some(deps) = list.dependencies {
                    let common_tools: HashSet<&str> = [
                        "npm", "pnpm", "yarn", "typescript", "ts-node",
                        "eslint", "prettier", "webpack", "vite", "create-react-app",
                        "create-vite", "nodemon", "pm2", "serve", "http-server",
                        "npx", "corepack", "nx", "turbo", "lerna",
                    ].into_iter().collect();

                    for (name, pkg) in deps {
                        if common_tools.contains(name.as_str()) {
                            continue;
                        }

                        let check = Command::new("npm")
                            .args(["view", &name, "bin", "--json"])
                            .stdout(Stdio::piped())
                            .stderr(Stdio::piped())
                            .output()
                            .await;

                        let has_bin = check
                            .map(|o| !o.stdout.is_empty() && o.stdout != b"null\n")
                            .unwrap_or(false);

                        if !has_bin {
                            orphans.push(OrphanDependency {
                                name: name.clone(),
                                version: pkg.version.unwrap_or_default(),
                                source: "npm".to_string(),
                                reason: "非 CLI 工具，可能是误装的库".to_string(),
                                size_bytes: 0,
                            });
                        }
                    }
                }
            }
        }
    }

    orphans
}

/// 扫描 pip 孤儿依赖
async fn scan_pip_orphans() -> Vec<OrphanDependency> {
    let mut orphans = Vec::new();

    let output = Command::new("pip")
        .args(["list", "--format=json"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await;

    if let Ok(output) = output {
        if let Ok(json_str) = String::from_utf8(output.stdout) {
            #[derive(Deserialize)]
            struct PipPkg {
                name: String,
                version: String,
            }

            if let Ok(packages) = serde_json::from_str::<Vec<PipPkg>>(&json_str) {
                let dep_output = Command::new("pip")
                    .args(["show", "--no-color"])
                    .args(packages.iter().map(|p| p.name.as_str()).collect::<Vec<_>>())
                    .stdout(Stdio::piped())
                    .stderr(Stdio::piped())
                    .output()
                    .await;

                let mut required_by: HashSet<String> = HashSet::new();

                if let Ok(dep_out) = dep_output {
                    let text = String::from_utf8_lossy(&dep_out.stdout);
                    for line in text.lines() {
                        if line.starts_with("Required-by:") {
                            let deps = line.trim_start_matches("Required-by:").trim();
                            for dep in deps.split(',') {
                                let dep = dep.trim();
                                if !dep.is_empty() {
                                    required_by.insert(dep.to_lowercase());
                                }
                            }
                        }
                    }
                }

                let common_tools: HashSet<&str> = [
                    "pip", "setuptools", "wheel", "virtualenv", "pipenv",
                    "poetry", "black", "flake8", "pylint", "mypy",
                    "pytest", "ipython", "jupyter", "notebook",
                ].into_iter().collect();

                for pkg in packages {
                    let name_lower = pkg.name.to_lowercase();
                    if common_tools.contains(name_lower.as_str()) {
                        continue;
                    }
                    if !required_by.contains(&name_lower) {
                        orphans.push(OrphanDependency {
                            name: pkg.name,
                            version: pkg.version,
                            source: "pip".to_string(),
                            reason: "没有被其他包依赖".to_string(),
                            size_bytes: 0,
                        });
                    }
                }
            }
        }
    }

    orphans
}

/// 扫描所有孤儿依赖
#[tauri::command]
pub async fn scan_orphan_dependencies() -> Result<Vec<OrphanDependency>, String> {
    let mut all_orphans = Vec::new();

    let (npm_orphans, pip_orphans) = tokio::join!(
        scan_npm_orphans(),
        scan_pip_orphans()
    );

    all_orphans.extend(npm_orphans);
    all_orphans.extend(pip_orphans);

    Ok(all_orphans)
}
