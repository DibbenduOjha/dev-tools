import { useState, useEffect } from 'react';
import { Table, Button, Space, Tag, Typography, message, Popconfirm } from 'antd';
import { ReloadOutlined, DeleteOutlined, SyncOutlined } from '@ant-design/icons';
import { invoke } from '@tauri-apps/api/core';
import { ToolInfo } from '../../types';
import type { ColumnsType } from 'antd/es/table';

const { Title } = Typography;

export default function CargoTools() {
  const [tools, setTools] = useState<ToolInfo[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadTools();
  }, []);

  async function loadTools() {
    setLoading(true);
    try {
      const result = await invoke<ToolInfo[]>('scan_cargo');
      setTools(result);
    } catch (e) {
      console.error(e);
      message.error('扫描失败');
    }
    setLoading(false);
  }

  const columns: ColumnsType<ToolInfo> = [
    {
      title: '工具名',
      dataIndex: 'name',
      key: 'name',
      sorter: (a, b) => a.name.localeCompare(b.name),
    },
    {
      title: '版本',
      dataIndex: 'version',
      key: 'version',
      width: 120,
      render: (version) => version || '-',
    },
    {
      title: '来源',
      dataIndex: 'source',
      key: 'source',
      width: 100,
      render: () => <Tag color="orange">cargo</Tag>,
    },
    {
      title: '操作',
      key: 'action',
      width: 150,
      render: (_, record) => (
        <Space size="small">
          <Button
            type="link"
            size="small"
            icon={<SyncOutlined />}
            onClick={() => message.info(`更新 ${record.name} 功能开发中`)}
          >
            更新
          </Button>
          <Popconfirm
            title="确认卸载"
            description={`确定要卸载 ${record.name} 吗？`}
            onConfirm={() => message.info(`卸载 ${record.name} 功能开发中`)}
            okText="确定"
            cancelText="取消"
          >
            <Button type="link" size="small" danger icon={<DeleteOutlined />}>
              卸载
            </Button>
          </Popconfirm>
        </Space>
      ),
    },
  ];

  return (
    <div>
      <div style={{ display: 'flex', justifyContent: 'space-between', marginBottom: 16 }}>
        <Title level={3} style={{ margin: 0 }}>🦀 Cargo 工具</Title>
        <Button icon={<ReloadOutlined />} onClick={loadTools} loading={loading}>
          刷新
        </Button>
      </div>
      <Table
        columns={columns}
        dataSource={tools}
        rowKey="name"
        loading={loading}
        pagination={{ pageSize: 15 }}
        size="middle"
      />
    </div>
  );
}
