use std::path::PathBuf;

/// 获取跨平台命令名称
pub fn get_command(cmd: &str) -> String {
    #[cfg(windows)]
    {
        format!("{}.cmd", cmd)
    }
    #[cfg(not(windows))]
    {
        cmd.to_string()
    }
}

/// 获取用户主目录
pub fn get_home_dir() -> PathBuf {
    dirs::home_dir().unwrap_or_default()
}

/// 获取配置目录
#[allow(dead_code)]
pub fn get_config_dir() -> PathBuf {
    dirs::config_dir().unwrap_or_default()
}
