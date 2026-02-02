use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

/// 工具来源类型
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum ToolSource {
    Npm,
    Cargo,
    Pip,
    Go,
    Script,  // 脚本安装的工具 (scoop, rustup, fnm 等)
    Manual,  // 手动安装
    Unknown,
}

/// 工具信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ToolInfo {
    pub name: String,                          // 包名 (如: claude-code)
    pub scope: Option<String>,                 // 作用域 (如: @anthropic-ai)
    pub full_name: String,                     // 完整名称 (如: @anthropic-ai/claude-code)
    pub version: Option<String>,
    pub source: ToolSource,
    pub install_path: String,
    pub size_bytes: u64,
    pub description: Option<String>,
    pub installed_at: Option<DateTime<Utc>>,
    pub last_accessed: Option<DateTime<Utc>>,
}

/// 配置文件信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub path: String,
    pub name: String,
    pub dir: String,  // 相对目录
}

/// 配置文件夹信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DotFolder {
    pub name: String,
    pub path: String,
    pub size_bytes: u64,
    pub file_count: u32,
    pub modified_at: Option<DateTime<Utc>>,
    pub related_tool: Option<String>,
}

/// 扫描结果汇总
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScanSummary {
    pub total_tools: u32,
    pub total_size_bytes: u64,
    pub by_source: std::collections::HashMap<String, u32>,
}
