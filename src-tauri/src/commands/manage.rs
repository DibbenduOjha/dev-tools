use crate::managers::NpmManager;
use serde::{Deserialize, Serialize};

/// 批量操作项
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchItem {
    pub source: String,
    pub name: String,
}

/// 批量操作结果
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BatchResult {
    pub name: String,
    pub success: bool,
    pub message: String,
}

/// 更新工具 (异步)
#[tauri::command]
pub async fn update_tool(source: String, name: String) -> Result<String, String> {
    match source.as_str() {
        "npm" => NpmManager::update(&name).await.map_err(|e| e.to_string()),
        "cargo" => Err("Cargo 更新功能开发中".to_string()),
        "pip" => Err("Pip 更新功能开发中".to_string()),
        _ => Err(format!("不支持的工具来源: {}", source)),
    }
}

/// 卸载工具 (异步)
#[tauri::command]
pub async fn uninstall_tool(source: String, name: String) -> Result<String, String> {
    match source.as_str() {
        "npm" => NpmManager::uninstall(&name).await.map_err(|e| e.to_string()),
        "cargo" => Err("Cargo 卸载功能开发中".to_string()),
        "pip" => Err("Pip 卸载功能开发中".to_string()),
        _ => Err(format!("不支持的工具来源: {}", source)),
    }
}

/// 批量更新工具
#[tauri::command]
pub async fn batch_update_tools(items: Vec<BatchItem>) -> Vec<BatchResult> {
    let mut results = Vec::new();
    for item in items {
        let result = update_tool(item.source.clone(), item.name.clone()).await;
        results.push(BatchResult {
            name: item.name,
            success: result.is_ok(),
            message: result.unwrap_or_else(|e| e),
        });
    }
    results
}

/// 批量卸载工具
#[tauri::command]
pub async fn batch_uninstall_tools(items: Vec<BatchItem>) -> Vec<BatchResult> {
    let mut results = Vec::new();
    for item in items {
        let result = uninstall_tool(item.source.clone(), item.name.clone()).await;
        results.push(BatchResult {
            name: item.name,
            success: result.is_ok(),
            message: result.unwrap_or_else(|e| e),
        });
    }
    results
}
