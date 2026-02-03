use serde::{Deserialize, Serialize};
use tauri::AppHandle;
use tauri_plugin_updater::UpdaterExt;

/// 更新信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateInfo {
    pub available: bool,
    pub current_version: String,
    pub latest_version: Option<String>,
    pub release_notes: Option<String>,
}

/// 检查更新
#[tauri::command]
pub async fn check_for_updates(app: AppHandle) -> Result<UpdateInfo, String> {
    let current_version = app.package_info().version.to_string();

    match app.updater() {
        Ok(updater) => {
            match updater.check().await {
                Ok(Some(update)) => {
                    Ok(UpdateInfo {
                        available: true,
                        current_version,
                        latest_version: Some(update.version.clone()),
                        release_notes: update.body.clone(),
                    })
                }
                Ok(None) => {
                    Ok(UpdateInfo {
                        available: false,
                        current_version,
                        latest_version: None,
                        release_notes: None,
                    })
                }
                Err(e) => Err(format!("检查更新失败: {}", e)),
            }
        }
        Err(e) => Err(format!("初始化更新器失败: {}", e)),
    }
}

/// 下载并安装更新
#[tauri::command]
pub async fn download_and_install_update(app: AppHandle) -> Result<(), String> {
    let updater = app.updater().map_err(|e| e.to_string())?;

    let update = updater
        .check()
        .await
        .map_err(|e| format!("检查更新失败: {}", e))?
        .ok_or("没有可用更新")?;

    // 下载并安装更新
    update
        .download_and_install(|_, _| {}, || {})
        .await
        .map_err(|e| format!("下载安装失败: {}", e))?;

    Ok(())
}
