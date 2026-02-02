import { useState, useEffect } from 'react';
import {
  Table, Button, Space, Tag, Typography, message, Modal,
  Select, Input, Drawer, Tree, Empty, Form, Switch, InputNumber
} from 'antd';
import {
  ReloadOutlined, DeleteOutlined, SyncOutlined,
  SearchOutlined, FileTextOutlined, SaveOutlined,
  ExclamationCircleOutlined, FolderOutlined, FileOutlined,
  SwapOutlined
} from '@ant-design/icons';
import { invoke } from '@tauri-apps/api/core';
import { ToolInfo, ConfigFile } from '../../types';
import { useAppStore } from '../../store';
import type { ColumnsType } from 'antd/es/table';
import type { DataNode } from 'antd/es/tree';

const { Title, Text } = Typography;

type ToolType = 'all' | 'npm' | 'cargo' | 'pip';

interface BatchItem {
  source: string;
  name: string;
}

interface BatchResult {
  name: string;
  success: boolean;
  message: string;
}

// 格式化文件大小
function formatSize(bytes: number): string {
  if (bytes === 0) return '-';
  const units = ['B', 'KB', 'MB', 'GB'];
  let i = 0;
  let size = bytes;
  while (size >= 1024 && i < units.length - 1) {
    size /= 1024;
    i++;
  }
  return `${size.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

export default function ToolsPage() {
  // 使用全局状态缓存
  const {
    tools, setTools,
    toolsLoading: loading, setToolsLoading: setLoading,
    toolsLastFetch, isCacheValid
  } = useAppStore();

  const [filterType, setFilterType] = useState<ToolType>('all');
  const [searchText, setSearchText] = useState('');
  const [actionLoading, setActionLoading] = useState<Set<string>>(new Set());

  // 批量操作状态
  const [selectedRowKeys, setSelectedRowKeys] = useState<React.Key[]>([]);
  const [batchLoading, setBatchLoading] = useState(false);

  // 版本切换状态
  const [versionModalOpen, setVersionModalOpen] = useState(false);
  const [versionTool, setVersionTool] = useState<ToolInfo | null>(null);
  const [versions, setVersions] = useState<string[]>([]);
  const [versionsLoading, setVersionsLoading] = useState(false);

  // Config drawer state
  const [drawerOpen, setDrawerOpen] = useState(false);
  const [selectedTool, setSelectedTool] = useState<ToolInfo | null>(null);
  const [configFiles, setConfigFiles] = useState<ConfigFile[]>([]);
  const [selectedFile, setSelectedFile] = useState<string | null>(null);
  const [fileContent, setFileContent] = useState('');
  const [jsonData, setJsonData] = useState<Record<string, unknown> | null>(null);
  const [saving, setSaving] = useState(false);
  const [form] = Form.useForm();

  useEffect(() => {
    // 如果缓存有效则不重新加载
    if (!isCacheValid(toolsLastFetch)) {
      loadTools();
    }
  }, []);

  async function loadTools() {
    setLoading(true);
    try {
      const [npm, cargo, pip] = await Promise.all([
        invoke<ToolInfo[]>('scan_npm').catch(() => []),
        invoke<ToolInfo[]>('scan_cargo').catch(() => []),
        invoke<ToolInfo[]>('scan_pip').catch(() => []),
      ]);
      setTools([...npm, ...cargo, ...pip]);
    } catch (e) {
      console.error(e);
      message.error('扫描失败');
    }
    setLoading(false);
  }

  function confirmUpdate(tool: ToolInfo) {
    Modal.confirm({
      title: '确认更新',
      icon: <ExclamationCircleOutlined />,
      content: (
        <div>
          <p>确定要更新 <strong>{tool.name}</strong> 吗？</p>
          <p style={{ color: 'rgba(255,255,255,0.5)', fontSize: 12 }}>
            当前版本: {tool.version || '未知'}
          </p>
        </div>
      ),
      okText: '确认更新',
      cancelText: '取消',
      onOk: () => handleUpdate(tool),
    });
  }

  function confirmUninstall(tool: ToolInfo) {
    Modal.confirm({
      title: '确认卸载',
      icon: <ExclamationCircleOutlined style={{ color: '#ff4d4f' }} />,
      content: (
        <div>
          <p>确定要卸载 <strong style={{ color: '#ff4d4f' }}>{tool.name}</strong> 吗？</p>
          <p style={{ color: 'rgba(255,255,255,0.5)', fontSize: 12 }}>
            此操作不可恢复，请谨慎操作。
          </p>
        </div>
      ),
      okText: '确认卸载',
      okType: 'danger',
      cancelText: '取消',
      onOk: () => handleUninstall(tool),
    });
  }

  async function handleUpdate(tool: ToolInfo) {
    const key = `${tool.source}-${tool.name}`;
    setActionLoading((prev) => new Set(prev).add(key));
    try {
      const result = await invoke<string>('update_tool', {
        source: tool.source,
        name: tool.full_name,
      });
      message.success(result);
      loadTools();
    } catch (e) {
      message.error(String(e));
    }
    setActionLoading((prev) => {
      const next = new Set(prev);
      next.delete(key);
      return next;
    });
  }

  async function handleUninstall(tool: ToolInfo) {
    const key = `${tool.source}-${tool.name}`;
    setActionLoading((prev) => new Set(prev).add(key));
    try {
      const result = await invoke<string>('uninstall_tool', {
        source: tool.source,
        name: tool.full_name,
      });
      message.success(result);
      loadTools();
    } catch (e) {
      message.error(String(e));
    }
    setActionLoading((prev) => {
      const next = new Set(prev);
      next.delete(key);
      return next;
    });
  }

  // 批量更新
  async function handleBatchUpdate() {
    if (selectedRowKeys.length === 0) return;

    const items: BatchItem[] = selectedRowKeys.map((key) => {
      const [source, ...nameParts] = String(key).split('-');
      return { source, name: nameParts.join('-') };
    });

    setBatchLoading(true);
    try {
      const results = await invoke<BatchResult[]>('batch_update_tools', { items });
      const success = results.filter((r) => r.success).length;
      const failed = results.filter((r) => !r.success).length;
      message.info(`更新完成: ${success} 成功, ${failed} 失败`);
      setSelectedRowKeys([]);
      loadTools();
    } catch (e) {
      message.error(String(e));
    }
    setBatchLoading(false);
  }

  // 批量卸载
  async function handleBatchUninstall() {
    if (selectedRowKeys.length === 0) return;

    Modal.confirm({
      title: '确认批量卸载',
      icon: <ExclamationCircleOutlined style={{ color: '#ff4d4f' }} />,
      content: `确定要卸载选中的 ${selectedRowKeys.length} 个工具吗？`,
      okText: '确认卸载',
      okType: 'danger',
      cancelText: '取消',
      onOk: async () => {
        const items: BatchItem[] = selectedRowKeys.map((key) => {
          const [source, ...nameParts] = String(key).split('-');
          return { source, name: nameParts.join('-') };
        });

        setBatchLoading(true);
        try {
          const results = await invoke<BatchResult[]>('batch_uninstall_tools', { items });
          const success = results.filter((r) => r.success).length;
          const failed = results.filter((r) => !r.success).length;
          message.info(`卸载完成: ${success} 成功, ${failed} 失败`);
          setSelectedRowKeys([]);
          loadTools();
        } catch (e) {
          message.error(String(e));
        }
        setBatchLoading(false);
      },
    });
  }

  // 打开版本选择弹窗
  async function openVersionModal(tool: ToolInfo) {
    if (tool.source !== 'npm') {
      message.warning('目前仅支持 npm 包的版本切换');
      return;
    }
    setVersionTool(tool);
    setVersionModalOpen(true);
    setVersionsLoading(true);
    try {
      const data = await invoke<string[]>('get_npm_versions', { name: tool.full_name });
      setVersions(data);
    } catch (e) {
      message.error(String(e));
    }
    setVersionsLoading(false);
  }

  // 安装指定版本
  async function handleInstallVersion(version: string) {
    if (!versionTool) return;
    setVersionModalOpen(false);
    const key = `${versionTool.source}-${versionTool.name}`;
    setActionLoading((prev) => new Set(prev).add(key));
    try {
      const result = await invoke<string>('install_npm_version', {
        name: versionTool.full_name,
        version,
      });
      message.success(result);
      loadTools();
    } catch (e) {
      message.error(String(e));
    }
    setActionLoading((prev) => {
      const next = new Set(prev);
      next.delete(key);
      return next;
    });
  }

  async function openConfigDrawer(tool: ToolInfo) {
    setSelectedTool(tool);
    setDrawerOpen(true);
    setSelectedFile(null);
    setJsonData(null);
    setConfigFiles([]);

    // 根据工具名称和类型获取配置文件路径
    try {
      const home = await invoke<string>('get_home_path');
      const configPaths: string[] = [];

      // 根据工具名称推断配置目录
      const toolName = tool.name.toLowerCase().replace(/-/g, '');

      // 常见的配置目录映射
      const knownConfigDirs: Record<string, string[]> = {
        'claudecode': ['.claude'],
        'claude': ['.claude'],
        'eslint': ['.eslintrc', '.config/eslint'],
        'prettier': ['.prettierrc', '.config/prettier'],
        'typescript': ['.config/typescript'],
        'npm': ['.npm', '.npmrc'],
        'pnpm': ['.pnpm', '.config/pnpm'],
        'yarn': ['.yarn', '.yarnrc'],
      };

      // 添加已知的配置目录
      if (knownConfigDirs[toolName]) {
        knownConfigDirs[toolName].forEach(dir => {
          configPaths.push(`${home}/${dir}`);
        });
      }

      // 尝试通用的配置目录模式
      configPaths.push(`${home}/.${tool.name.toLowerCase()}`);
      configPaths.push(`${home}/.config/${tool.name.toLowerCase()}`);

      // 根据工具类型添加默认目录
      if (tool.source === 'cargo') {
        configPaths.push(`${home}/.cargo`);
      } else if (tool.source === 'pip') {
        configPaths.push(`${home}/.pip`);
        configPaths.push(`${home}/.config/pip`);
      }

      // 去重并扫描所有可能的配置目录
      const uniquePaths = [...new Set(configPaths)];
      const allFiles: ConfigFile[] = [];

      for (const configPath of uniquePaths) {
        try {
          const files = await invoke<ConfigFile[]>('list_json_files_recursive', { dirPath: configPath });
          allFiles.push(...files);
        } catch {
          // 目录不存在，跳过
        }
      }

      setConfigFiles(allFiles);
    } catch (e) {
      console.error(e);
    }
  }

  async function loadConfigFile(path: string) {
    try {
      const content = await invoke<string>('read_config_file', { path });
      setFileContent(content);
      setSelectedFile(path);

      try {
        const parsed = JSON.parse(content);
        setJsonData(parsed);
        form.setFieldsValue(flattenObject(parsed));
      } catch {
        setJsonData(null);
      }
    } catch (e) {
      message.error(String(e));
    }
  }

  async function handleSaveConfig() {
    if (!selectedFile) return;

    setSaving(true);
    try {
      let contentToSave = fileContent;

      if (jsonData) {
        const values = form.getFieldsValue();
        const newData = unflattenObject(values);
        contentToSave = JSON.stringify(newData, null, 2);
      }

      await invoke<string>('write_config_file', {
        path: selectedFile,
        content: contentToSave,
      });
      message.success('保存成功');
    } catch (e) {
      message.error(String(e));
    }
    setSaving(false);
  }

  function flattenObject(obj: Record<string, unknown>, prefix = ''): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(obj)) {
      const newKey = prefix ? `${prefix}.${key}` : key;
      if (value && typeof value === 'object' && !Array.isArray(value)) {
        Object.assign(result, flattenObject(value as Record<string, unknown>, newKey));
      } else {
        result[newKey] = value;
      }
    }
    return result;
  }

  function unflattenObject(obj: Record<string, unknown>): Record<string, unknown> {
    const result: Record<string, unknown> = {};
    for (const [key, value] of Object.entries(obj)) {
      const keys = key.split('.');
      let current = result;
      for (let i = 0; i < keys.length - 1; i++) {
        if (!(keys[i] in current)) {
          current[keys[i]] = {};
        }
        current = current[keys[i]] as Record<string, unknown>;
      }
      current[keys[keys.length - 1]] = value;
    }
    return result;
  }

  function renderFormField(key: string, value: unknown) {
    if (typeof value === 'boolean') {
      return (
        <Form.Item key={key} name={key} label={key} valuePropName="checked">
          <Switch />
        </Form.Item>
      );
    }
    if (typeof value === 'number') {
      return (
        <Form.Item key={key} name={key} label={key}>
          <InputNumber style={{ width: '100%' }} />
        </Form.Item>
      );
    }
    return (
      <Form.Item key={key} name={key} label={key}>
        <Input />
      </Form.Item>
    );
  }

  // 构建配置文件树
  function buildConfigTree(): DataNode[] {
    const dirMap = new Map<string, ConfigFile[]>();

    configFiles.forEach((file) => {
      const dir = file.dir || '.';
      if (!dirMap.has(dir)) {
        dirMap.set(dir, []);
      }
      dirMap.get(dir)!.push(file);
    });

    const nodes: DataNode[] = [];
    const sortedDirs = Array.from(dirMap.keys()).sort();

    sortedDirs.forEach((dir) => {
      const files = dirMap.get(dir)!;
      const children: DataNode[] = files.map((file) => ({
        key: file.path,
        title: file.name,
        icon: <FileOutlined />,
        isLeaf: true,
      }));

      if (dir === '.') {
        nodes.push(...children);
      } else {
        nodes.push({
          key: `dir-${dir}`,
          title: dir,
          icon: <FolderOutlined />,
          children,
          selectable: false,
        });
      }
    });

    return nodes;
  }

  // Filter tools
  const filteredTools = tools.filter((tool) => {
    const matchType = filterType === 'all' || tool.source === filterType;
    const matchSearch = tool.name.toLowerCase().includes(searchText.toLowerCase()) ||
      (tool.scope?.toLowerCase().includes(searchText.toLowerCase()) ?? false);
    return matchType && matchSearch;
  });

  const sourceColors: Record<string, string> = {
    npm: 'green',
    cargo: 'orange',
    pip: 'blue',
  };

  const columns: ColumnsType<ToolInfo> = [
    {
      title: '作用域',
      dataIndex: 'scope',
      key: 'scope',
      width: 150,
      render: (scope) => (
        <Text style={{ color: 'rgba(255,255,255,0.5)' }}>
          {scope || '-'}
        </Text>
      ),
    },
    {
      title: '名称',
      dataIndex: 'name',
      key: 'name',
      sorter: (a, b) => a.name.localeCompare(b.name),
      render: (name, record) => (
        <div>
          <div style={{ fontWeight: 500, color: '#fff' }}>{name}</div>
          {record.description && (
            <Text style={{ fontSize: 12, color: 'rgba(255,255,255,0.5)' }}>
              {record.description}
            </Text>
          )}
        </div>
      ),
    },
    {
      title: '版本',
      dataIndex: 'version',
      key: 'version',
      width: 140,
      render: (version, record) => (
        <Space size="small">
          <Tag>{version || '-'}</Tag>
          {record.source === 'npm' && (
            <Button
              type="text"
              size="small"
              icon={<SwapOutlined />}
              onClick={() => openVersionModal(record)}
              title="切换版本"
              style={{ color: '#6366f1' }}
            />
          )}
        </Space>
      ),
    },
    {
      title: '类型',
      dataIndex: 'source',
      key: 'source',
      width: 80,
      render: (source) => (
        <Tag color={sourceColors[source] || 'default'}>
          {source}
        </Tag>
      ),
    },
    {
      title: '大小',
      dataIndex: 'size_bytes',
      key: 'size_bytes',
      width: 100,
      sorter: (a, b) => a.size_bytes - b.size_bytes,
      render: (size) => (
        <Text style={{ color: 'rgba(255,255,255,0.7)' }}>
          {formatSize(size)}
        </Text>
      ),
    },
    {
      title: '操作',
      key: 'action',
      width: 200,
      render: (_, record) => {
        const key = `${record.source}-${record.name}`;
        const isLoading = actionLoading.has(key);
        return (
          <Space size="small">
            <Button
              type="link"
              size="small"
              icon={<FileTextOutlined />}
              onClick={() => openConfigDrawer(record)}
            >
              配置
            </Button>
            <Button
              type="link"
              size="small"
              icon={<SyncOutlined spin={isLoading} />}
              onClick={() => confirmUpdate(record)}
              disabled={isLoading}
            >
              更新
            </Button>
            <Button
              type="link"
              size="small"
              danger
              icon={<DeleteOutlined />}
              onClick={() => confirmUninstall(record)}
              disabled={isLoading}
            >
              卸载
            </Button>
          </Space>
        );
      },
    },
  ];

  return (
    <div>
      {/* Header */}
      <div style={{ marginBottom: 24 }}>
        <Title level={3} style={{ margin: 0, color: '#fff' }}>
          工具管理
        </Title>
        <Text style={{ color: 'rgba(255, 255, 255, 0.6)' }}>
          管理已安装的开发工具，支持更新、卸载和配置编辑
        </Text>
      </div>

      {/* Toolbar */}
      <div style={{
        display: 'flex',
        justifyContent: 'space-between',
        marginBottom: 16,
        gap: 16,
        flexWrap: 'wrap',
      }}>
        <Space>
          <Select
            value={filterType}
            onChange={setFilterType}
            style={{ width: 140 }}
            options={[
              { value: 'all', label: '全部类型' },
              { value: 'npm', label: 'npm' },
              { value: 'cargo', label: 'cargo' },
              { value: 'pip', label: 'pip' },
            ]}
          />
          <Input
            placeholder="搜索工具..."
            prefix={<SearchOutlined style={{ color: 'rgba(255,255,255,0.4)' }} />}
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            style={{ width: 200 }}
            allowClear
          />
        </Space>
        <Space>
          {selectedRowKeys.length > 0 && (
            <>
              <Button
                type="primary"
                icon={<SyncOutlined />}
                onClick={handleBatchUpdate}
                loading={batchLoading}
              >
                批量更新 ({selectedRowKeys.length})
              </Button>
              <Button
                danger
                icon={<DeleteOutlined />}
                onClick={handleBatchUninstall}
                loading={batchLoading}
              >
                批量卸载
              </Button>
            </>
          )}
          <Button
            icon={<ReloadOutlined />}
            onClick={loadTools}
            loading={loading}
          >
            刷新
          </Button>
        </Space>
      </div>

      {/* Table */}
      <div className="glass" style={{ overflow: 'hidden' }}>
        <Table
          columns={columns}
          dataSource={filteredTools}
          rowKey={(record) => `${record.source}-${record.full_name}`}
          loading={loading}
          pagination={{ pageSize: 15, showSizeChanger: false }}
          size="middle"
          rowSelection={{
            selectedRowKeys,
            onChange: setSelectedRowKeys,
          }}
        />
      </div>

      {/* Config Drawer */}
      <Drawer
        title={selectedTool ? `${selectedTool.name} 配置文件` : '配置'}
        placement="right"
        width={520}
        open={drawerOpen}
        onClose={() => setDrawerOpen(false)}
        extra={
          selectedFile && (
            <Button
              type="primary"
              icon={<SaveOutlined />}
              onClick={handleSaveConfig}
              loading={saving}
            >
              保存
            </Button>
          )
        }
      >
        {configFiles.length === 0 ? (
          <Empty description="暂无配置文件" />
        ) : !selectedFile ? (
          <div>
            <Text style={{ color: 'rgba(255,255,255,0.6)', marginBottom: 16, display: 'block' }}>
              选择要编辑的配置文件：
            </Text>
            <Tree
              showIcon
              treeData={buildConfigTree()}
              onSelect={(_, info) => {
                if (info.node.isLeaf) {
                  loadConfigFile(info.node.key as string);
                }
              }}
              style={{ background: 'transparent' }}
            />
          </div>
        ) : jsonData ? (
          <Form form={form} layout="vertical" size="small">
            <Button
              type="link"
              onClick={() => setSelectedFile(null)}
              style={{ marginBottom: 16, padding: 0, color: '#6366f1' }}
            >
              ← 返回文件列表
            </Button>
            <div style={{ marginBottom: 16, padding: 8, background: 'rgba(99,102,241,0.1)', borderRadius: 6 }}>
              <Text style={{ color: 'rgba(255,255,255,0.6)', fontSize: 12 }}>
                {selectedFile}
              </Text>
            </div>
            {Object.entries(flattenObject(jsonData)).map(([key, value]) =>
              renderFormField(key, value)
            )}
          </Form>
        ) : (
          <div>
            <Button
              type="link"
              onClick={() => setSelectedFile(null)}
              style={{ marginBottom: 16, padding: 0, color: '#6366f1' }}
            >
              ← 返回文件列表
            </Button>
            <div style={{ marginBottom: 16, padding: 8, background: 'rgba(99,102,241,0.1)', borderRadius: 6 }}>
              <Text style={{ color: 'rgba(255,255,255,0.6)', fontSize: 12 }}>
                {selectedFile}
              </Text>
            </div>
            <Input.TextArea
              value={fileContent}
              onChange={(e) => setFileContent(e.target.value)}
              rows={20}
              style={{ fontFamily: 'monospace' }}
            />
          </div>
        )}
      </Drawer>

      {/* Version Selection Modal */}
      <Modal
        title={versionTool ? `选择 ${versionTool.name} 版本` : '选择版本'}
        open={versionModalOpen}
        onCancel={() => setVersionModalOpen(false)}
        footer={null}
        width={400}
      >
        {versionsLoading ? (
          <div style={{ textAlign: 'center', padding: 24 }}>
            <SyncOutlined spin style={{ fontSize: 24, marginBottom: 16 }} />
            <div>加载版本列表中...</div>
          </div>
        ) : versions.length === 0 ? (
          <Empty description="暂无版本信息" />
        ) : (
          <div>
            <Text style={{ color: 'rgba(255,255,255,0.6)', marginBottom: 12, display: 'block' }}>
              当前版本: <Tag color="blue">{versionTool?.version || '-'}</Tag>
            </Text>
            <div style={{ maxHeight: 300, overflowY: 'auto' }}>
              {versions.map((v) => (
                <div
                  key={v}
                  style={{
                    padding: '8px 12px',
                    marginBottom: 4,
                    borderRadius: 6,
                    background: v === versionTool?.version
                      ? 'rgba(99, 102, 241, 0.2)'
                      : 'rgba(255,255,255,0.05)',
                    border: v === versionTool?.version
                      ? '1px solid rgba(99, 102, 241, 0.5)'
                      : '1px solid transparent',
                    display: 'flex',
                    justifyContent: 'space-between',
                    alignItems: 'center',
                    cursor: 'pointer',
                    transition: 'all 0.2s',
                  }}
                  onMouseEnter={(e) => {
                    if (v !== versionTool?.version) {
                      e.currentTarget.style.background = 'rgba(255,255,255,0.1)';
                    }
                  }}
                  onMouseLeave={(e) => {
                    if (v !== versionTool?.version) {
                      e.currentTarget.style.background = 'rgba(255,255,255,0.05)';
                    }
                  }}
                >
                  <span>
                    {v}
                    {v === versionTool?.version && (
                      <Tag color="green" style={{ marginLeft: 8 }}>当前</Tag>
                    )}
                  </span>
                  {v !== versionTool?.version && (
                    <Button
                      type="primary"
                      size="small"
                      onClick={() => handleInstallVersion(v)}
                    >
                      安装
                    </Button>
                  )}
                </div>
              ))}
            </div>
          </div>
        )}
      </Modal>
    </div>
  );
}
