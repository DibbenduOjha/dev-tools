// 工具来源类型
export type ToolSource = 'npm' | 'cargo' | 'pip' | 'go' | 'script' | 'manual' | 'unknown';

// 工具信息
export interface ToolInfo {
  name: string;
  scope: string | null;
  full_name: string;
  version: string | null;
  source: ToolSource;
  install_path: string;
  size_bytes: number;
  description: string | null;
}

// 配置文件夹信息
export interface DotFolder {
  name: string;
  path: string;
  size_bytes: number;
  file_count: number;
  modified_at: string | null;
  related_tool: string | null;
}

// 配置文件信息
export interface ConfigFile {
  path: string;
  name: string;
  dir: string;
}
