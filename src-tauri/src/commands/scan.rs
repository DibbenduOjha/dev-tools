use crate::models::{ToolInfo, DotFolder};
use crate::scanners::{NpmScanner, CargoScanner, PipScanner, DotfilesScanner};

/// 扫描 npm 全局包 (异步)
#[tauri::command]
pub async fn scan_npm() -> Result<Vec<ToolInfo>, String> {
    NpmScanner::scan().await.map_err(|e| e.to_string())
}

/// 扫描 cargo 工具 (异步)
#[tauri::command]
pub async fn scan_cargo() -> Result<Vec<ToolInfo>, String> {
    CargoScanner::scan().await.map_err(|e| e.to_string())
}

/// 扫描 pip 包 (异步)
#[tauri::command]
pub async fn scan_pip() -> Result<Vec<ToolInfo>, String> {
    PipScanner::scan().await.map_err(|e| e.to_string())
}

/// 扫描配置文件夹 (异步)
#[tauri::command]
pub async fn scan_dotfiles() -> Result<Vec<DotFolder>, String> {
    DotfilesScanner::scan().await.map_err(|e| e.to_string())
}
