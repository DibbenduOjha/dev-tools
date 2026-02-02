use serde::{Deserialize, Serialize};
use std::env;

/// 环境变量信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnvVariable {
    pub name: String,
    pub value: String,
    pub is_path: bool,
}

/// 获取所有环境变量
#[tauri::command]
pub async fn get_env_variables() -> Result<Vec<EnvVariable>, String> {
    let mut variables: Vec<EnvVariable> = env::vars()
        .map(|(name, value)| {
            let is_path = name.to_uppercase().contains("PATH")
                || name.to_uppercase().ends_with("HOME")
                || name.to_uppercase().ends_with("DIR");
            EnvVariable { name, value, is_path }
        })
        .collect();

    variables.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
    Ok(variables)
}

/// 获取 PATH 环境变量的条目
#[tauri::command]
pub async fn get_path_entries() -> Result<Vec<String>, String> {
    let path = env::var("PATH").unwrap_or_default();

    #[cfg(windows)]
    let separator = ';';
    #[cfg(not(windows))]
    let separator = ':';

    let entries: Vec<String> = path
        .split(separator)
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string())
        .collect();

    Ok(entries)
}
