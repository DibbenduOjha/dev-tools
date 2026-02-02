import { useState } from 'react';
import {
  Typography, Input, Select, Table, Button, Space,
  Tag, message, Modal, Empty, Spin
} from 'antd';
import {
  SearchOutlined, DownloadOutlined,
  ExclamationCircleOutlined
} from '@ant-design/icons';
import { invoke } from '@tauri-apps/api/core';
import type { ColumnsType } from 'antd/es/table';

const { Title, Text } = Typography;

type PackageSource = 'npm' | 'cargo' | 'pip';

interface PackageSearchResult {
  name: string;
  version: string;
  description: string | null;
  source: string;
}

export default function PackageStore() {
  const [source, setSource] = useState<PackageSource>('npm');
  const [searchText, setSearchText] = useState('');
  const [searching, setSearching] = useState(false);
  const [results, setResults] = useState<PackageSearchResult[]>([]);
  const [installing, setInstalling] = useState<Set<string>>(new Set());

  // 搜索包
  async function handleSearch() {
    if (!searchText.trim()) {
      message.warning('请输入搜索关键词');
      return;
    }

    setSearching(true);
    setResults([]);

    try {
      let data: PackageSearchResult[] = [];

      switch (source) {
        case 'npm':
          data = await invoke<PackageSearchResult[]>('search_npm_packages', {
            query: searchText
          });
          break;
        case 'cargo':
          data = await invoke<PackageSearchResult[]>('search_cargo_packages', {
            query: searchText
          });
          break;
        case 'pip':
          data = await invoke<PackageSearchResult[]>('search_pip_packages', {
            query: searchText
          });
          break;
      }

      setResults(data);

      if (data.length === 0) {
        message.info('未找到匹配的包');
      }
    } catch (e) {
      message.error(String(e));
    }

    setSearching(false);
  }

  // 确认安装
  function confirmInstall(pkg: PackageSearchResult) {
    Modal.confirm({
      title: '确认安装',
      icon: <ExclamationCircleOutlined />,
      content: (
        <div>
          <p>确定要安装 <strong>{pkg.name}</strong> 吗？</p>
          <p style={{ color: 'rgba(255,255,255,0.5)', fontSize: 12 }}>
            版本: {pkg.version} | 来源: {pkg.source}
          </p>
        </div>
      ),
      okText: '确认安装',
      cancelText: '取消',
      onOk: () => handleInstall(pkg),
    });
  }

  // 安装包
  async function handleInstall(pkg: PackageSearchResult) {
    const key = `${pkg.source}-${pkg.name}`;
    setInstalling(prev => new Set(prev).add(key));

    try {
      let result: string;

      switch (pkg.source) {
        case 'npm':
          result = await invoke<string>('install_npm_package', { name: pkg.name });
          break;
        case 'cargo':
          result = await invoke<string>('install_cargo_package', { name: pkg.name });
          break;
        case 'pip':
          result = await invoke<string>('install_pip_package', { name: pkg.name });
          break;
        default:
          throw new Error('不支持的包来源');
      }

      message.success(result);
    } catch (e) {
      message.error(String(e));
    }

    setInstalling(prev => {
      const next = new Set(prev);
      next.delete(key);
      return next;
    });
  }

  // 表格列定义
  const columns: ColumnsType<PackageSearchResult> = [
    {
      title: '包名',
      dataIndex: 'name',
      key: 'name',
      render: (name, record) => (
        <div>
          <div style={{ fontWeight: 500, color: '#fff' }}>{name}</div>
          {record.description && (
            <Text
              style={{ fontSize: 12, color: 'rgba(255,255,255,0.5)' }}
              ellipsis={{ tooltip: record.description }}
            >
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
      width: 120,
      render: (version) => <Tag>{version}</Tag>,
    },
    {
      title: '来源',
      dataIndex: 'source',
      key: 'source',
      width: 80,
      render: (src) => (
        <Tag color={src === 'npm' ? 'green' : src === 'cargo' ? 'orange' : 'blue'}>
          {src}
        </Tag>
      ),
    },
    {
      title: '操作',
      key: 'action',
      width: 100,
      render: (_, record) => {
        const key = `${record.source}-${record.name}`;
        const isInstalling = installing.has(key);
        return (
          <Button
            type="primary"
            size="small"
            icon={<DownloadOutlined />}
            loading={isInstalling}
            onClick={() => confirmInstall(record)}
          >
            安装
          </Button>
        );
      },
    },
  ];

  return (
    <div>
      {/* 页面标题 */}
      <div style={{ marginBottom: 24 }}>
        <Title level={3} style={{ margin: 0, color: '#fff' }}>
          包商店
        </Title>
        <Text style={{ color: 'rgba(255, 255, 255, 0.6)' }}>
          搜索并安装 npm、cargo、pip 包
        </Text>
      </div>

      {/* 搜索栏 */}
      <div style={{ marginBottom: 24 }}>
        <Space.Compact style={{ width: '100%' }}>
          <Select
            value={source}
            onChange={setSource}
            style={{ width: 120 }}
            options={[
              { value: 'npm', label: 'npm' },
              { value: 'cargo', label: 'cargo' },
              { value: 'pip', label: 'pip' },
            ]}
          />
          <Input
            placeholder="输入包名搜索..."
            value={searchText}
            onChange={(e) => setSearchText(e.target.value)}
            onPressEnter={handleSearch}
            style={{ flex: 1 }}
            allowClear
          />
          <Button
            type="primary"
            icon={<SearchOutlined />}
            onClick={handleSearch}
            loading={searching}
          >
            搜索
          </Button>
        </Space.Compact>
      </div>

      {/* 搜索结果 */}
      <div className="glass" style={{ overflow: 'hidden' }}>
        {searching ? (
          <div style={{ padding: 48, textAlign: 'center' }}>
            <Spin size="large" />
            <div style={{ marginTop: 16, color: 'rgba(255,255,255,0.6)' }}>
              正在搜索...
            </div>
          </div>
        ) : results.length > 0 ? (
          <Table
            columns={columns}
            dataSource={results}
            rowKey={(record) => `${record.source}-${record.name}`}
            pagination={{ pageSize: 10 }}
            size="middle"
          />
        ) : (
          <Empty
            description="输入关键词搜索包"
            style={{ padding: 48 }}
          />
        )}
      </div>
    </div>
  );
}
