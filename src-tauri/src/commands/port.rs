use serde::{Deserialize, Serialize};
use std::process::Stdio;
use tokio::process::Command;

/// 端口信息
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PortInfo {
    pub port: u16,
    pub protocol: String,
    pub pid: Option<u32>,
    pub process_name: Option<String>,
    pub state: String,
}

/// 扫描端口 (Windows)
#[tauri::command]
pub async fn scan_ports() -> Result<Vec<PortInfo>, String> {
    let output = Command::new("netstat")
        .args(["-ano"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行 netstat 失败: {}", e))?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut ports = Vec::new();

    for line in stdout.lines().skip(4) {
        if let Some(info) = parse_netstat_line(line) {
            ports.push(info);
        }
    }

    // 获取进程名称
    for port in &mut ports {
        if let Some(pid) = port.pid {
            port.process_name = get_process_name(pid).await;
        }
    }

    // 按端口排序
    ports.sort_by_key(|p| p.port);
    ports.dedup_by_key(|p| (p.port, p.protocol.clone()));

    Ok(ports)
}

/// 解析 netstat 输出行
fn parse_netstat_line(line: &str) -> Option<PortInfo> {
    let parts: Vec<&str> = line.split_whitespace().collect();
    if parts.len() < 4 {
        return None;
    }

    let protocol = parts[0].to_uppercase();
    if protocol != "TCP" && protocol != "UDP" {
        return None;
    }

    // 解析本地地址
    let local_addr = parts[1];
    let port = local_addr
        .rsplit(':')
        .next()?
        .parse::<u16>()
        .ok()?;

    // 状态和 PID
    let (state, pid) = if protocol == "TCP" && parts.len() >= 5 {
        let state = parts[3].to_string();
        let pid = parts[4].parse::<u32>().ok();
        (state, pid)
    } else if protocol == "UDP" && parts.len() >= 4 {
        let pid = parts[3].parse::<u32>().ok();
        ("*".to_string(), pid)
    } else {
        return None;
    };

    Some(PortInfo {
        port,
        protocol,
        pid,
        process_name: None,
        state,
    })
}

/// 获取进程名称
async fn get_process_name(pid: u32) -> Option<String> {
    let output = Command::new("tasklist")
        .args(["/FI", &format!("PID eq {}", pid), "/FO", "CSV", "/NH"])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    let line = stdout.lines().next()?;

    // CSV 格式: "进程名","PID",...
    let name = line.split(',').next()?;
    let name = name.trim_matches('"');

    if name.is_empty() || name.contains("没有") {
        None
    } else {
        Some(name.to_string())
    }
}

/// 杀死进程
#[tauri::command]
pub async fn kill_process(pid: u32) -> Result<String, String> {
    let output = Command::new("taskkill")
        .args(["/F", "/PID", &pid.to_string()])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(|e| format!("执行失败: {}", e))?;

    if output.status.success() {
        Ok(format!("已终止进程 {}", pid))
    } else {
        Err(String::from_utf8_lossy(&output.stderr).to_string())
    }
}
