import { useState, useEffect } from 'react';
import { Typography, Button, Table, Input, Tag, Tabs, List, message } from 'antd';
import { ReloadOutlined, SearchOutlined, FolderOutlined } from '@ant-design/icons';
import { invoke } from '@tauri-apps/api/core';
import type { ColumnsType } from 'antd/es/table';

const { Title, Text } = Typography;

interface EnvVariable {
  name: string;
  value: string;
  is_path: boolean;
}

export default function EnvManager() {
  const [loading, setLoading] = useState(false);
  const [envVars, setEnvVars] = useState<EnvVariable[]>([]);
  const [pathEntries, setPathEntries] = useState<string[]>([]);
  const [searchText, setSearchText] = useState('');

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    setLoading(true);
    try {
      const [vars, paths] = await Promise.all([
        invoke<EnvVariable[]>('get_env_variables'),
        invoke<string[]>('get_path_entries'),
      ]);
      setEnvVars(vars);
      setPathEntries(paths);
    } catch (e) {
      message.error(String(e));
    }
    setLoading(false);
  }

  const filteredVars = envVars.filter(
    (v) =>
      v.name.toLowerCase().includes(searchText.toLowerCase()) ||
      v.value.toLowerCase().includes(searchText.toLowerCase())
  );

  const columns: ColumnsType<EnvVariable> = [
    {
      title: '变量名',
      dataIndex: 'name',
      key: 'name',
      width: 250,
      render: (name, record) => (
        <Text style={{ color: record.is_path ? '#6366f1' : '#fff', fontFamily: 'monospace' }}>
          {name}
        </Text>
      ),
    },
    {
      title: '值',
      dataIndex: 'value',
      key: 'value',
      ellipsis: true,
      render: (value) => (
        <Text style={{ color: 'rgba(255,255,255,0.7)', fontFamily: 'monospace', fontSize: 12 }}>
          {value}
        </Text>
      ),
    },
    {
      title: '类型',
      key: 'type',
      width: 80,
      render: (_, record) => record.is_path ? <Tag color="blue">路径</Tag> : <Tag>普通</Tag>,
    },
  ];

  const tabItems = [
    {
      key: 'all',
      label: '所有变量',
      children: (
        <Table
          columns={columns}
          dataSource={filteredVars}
          rowKey="name"
          loading={loading}
          pagination={{ pageSize: 15 }}
          size="small"
        />
      ),
    },
    {
      key: 'path',
      label: 'PATH 条目',
      children: (
        <List
          loading={loading}
          dataSource={pathEntries}
          renderItem={(item, index) => (
            <List.Item>
              <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
                <Tag color="blue">{index + 1}</Tag>
                <FolderOutlined style={{ color: '#6366f1' }} />
                <Text style={{ fontFamily: 'monospace', color: 'rgba(255,255,255,0.8)' }}>
                  {item}
                </Text>
              </div>
            </List.Item>
          )}
        />
      ),
    },
  ];

  return (
    <div>
      <div style={{ marginBottom: 24 }}>
        <Title level={3} style={{ margin: 0, color: '#fff' }}>
          环境变量
        </Title>
        <Text style={{ color: 'rgba(255, 255, 255, 0.6)' }}>
          查看系统环境变量和 PATH 配置
        </Text>
      </div>

      <div style={{ marginBottom: 16, display: 'flex', justifyContent: 'space-between' }}>
        <Input
          placeholder="搜索变量..."
          prefix={<SearchOutlined />}
          value={searchText}
          onChange={(e) => setSearchText(e.target.value)}
          style={{ width: 250 }}
          allowClear
        />
        <Button icon={<ReloadOutlined />} onClick={loadData} loading={loading}>
          刷新
        </Button>
      </div>

      <div className="glass" style={{ padding: 16 }}>
        <Tabs items={tabItems} />
      </div>
    </div>
  );
}