import { useState, useEffect } from 'react';
import { Row, Col, Typography, Button } from 'antd';
import { ReloadOutlined } from '@ant-design/icons';
import { invoke } from '@tauri-apps/api/core';
import { ToolInfo } from '../../types';

const { Title, Text } = Typography;

export default function Dashboard() {
  const [npmCount, setNpmCount] = useState(0);
  const [cargoCount, setCargoCount] = useState(0);
  const [pipCount, setPipCount] = useState(0);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadData();
  }, []);

  async function loadData() {
    setLoading(true);
    try {
      const [npm, cargo, pip] = await Promise.all([
        invoke<ToolInfo[]>('scan_npm').catch(() => []),
        invoke<ToolInfo[]>('scan_cargo').catch(() => []),
        invoke<ToolInfo[]>('scan_pip').catch(() => []),
      ]);
      setNpmCount(npm.length);
      setCargoCount(cargo.length);
      setPipCount(pip.length);
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  }

  const totalCount = npmCount + cargoCount + pipCount;

  const stats = [
    { key: 'total', label: '工具总数', value: totalCount, icon: '📦', color: '#6366f1' },
    { key: 'npm', label: 'npm 包', value: npmCount, icon: '📗', color: '#22c55e' },
    { key: 'cargo', label: 'Cargo 工具', value: cargoCount, icon: '🦀', color: '#f97316' },
    { key: 'pip', label: 'Pip 包', value: pipCount, icon: '🐍', color: '#3b82f6' },
  ];

  return (
    <div>
      {/* Header */}
      <div style={{
        display: 'flex',
        alignItems: 'flex-start',
        justifyContent: 'space-between',
        marginBottom: 32
      }}>
        <div>
          <Title
            level={2}
            style={{
              margin: 0,
              color: '#fff',
              marginBottom: 8,
            }}
          >
            欢迎使用 DevTool Manager
          </Title>
          <Text style={{ color: 'rgba(255, 255, 255, 0.6)', fontSize: 16 }}>
            管理你的开发工具，智能扫描，一键操作
          </Text>
        </div>
        <Button
          icon={<ReloadOutlined spin={loading} />}
          onClick={loadData}
          loading={loading}
        >
          刷新数据
        </Button>
      </div>

      {/* Stats Cards */}
      <Row gutter={[16, 16]}>
        {stats.map((stat) => (
          <Col xs={24} sm={12} lg={6} key={stat.key}>
            <div className="stat-card">
              <div
                className="icon"
                style={{
                  background: `${stat.color}20`,
                }}
              >
                {stat.icon}
              </div>
              <div className="info">
                <div className="value">{stat.value}</div>
                <div className="label">{stat.label}</div>
              </div>
            </div>
          </Col>
        ))}
      </Row>

      {/* Info Cards */}
      <Row gutter={[16, 16]} style={{ marginTop: 24 }}>
        <Col xs={24} md={12}>
          <div className="glass" style={{ padding: 24 }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginBottom: 16 }}>
              <span style={{ fontSize: 28 }}>📊</span>
              <Title level={4} style={{ margin: 0, color: '#fff' }}>工具统计</Title>
            </div>
            <Text style={{ color: 'rgba(255, 255, 255, 0.7)', lineHeight: 1.8 }}>
              已扫描到 <span style={{ color: '#6366f1', fontWeight: 600 }}>{totalCount}</span> 个开发工具，
              包括 npm 全局包、Cargo 工具和 Pip 包。
              点击顶部「工具管理」查看详情并进行管理操作。
            </Text>
          </div>
        </Col>
        <Col xs={24} md={12}>
          <div className="glass" style={{ padding: 24 }}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 12, marginBottom: 16 }}>
              <span style={{ fontSize: 28 }}>⚡</span>
              <Title level={4} style={{ margin: 0, color: '#fff' }}>功能说明</Title>
            </div>
            <Text style={{ color: 'rgba(255, 255, 255, 0.7)', lineHeight: 1.8 }}>
              支持扫描、更新、卸载工具，以及编辑工具的配置文件。
              选择工具后可查看和编辑其 JSON/TOML/YAML 配置。
              所有操作均为异步执行，不会阻塞界面。
            </Text>
          </div>
        </Col>
      </Row>

      {/* Feature Highlights */}
      <div className="glass" style={{ padding: 24, marginTop: 24 }}>
        <Title level={4} style={{ margin: 0, color: '#fff', marginBottom: 20 }}>
          ✨ 功能亮点
        </Title>
        <Row gutter={[24, 16]}>
          <Col xs={24} sm={8}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
              <div style={{
                width: 40,
                height: 40,
                borderRadius: 8,
                background: 'rgba(99, 102, 241, 0.2)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                fontSize: 20,
              }}>
                🔍
              </div>
              <div>
                <div style={{ color: '#fff', fontWeight: 500 }}>智能扫描</div>
                <div style={{ color: 'rgba(255,255,255,0.5)', fontSize: 12 }}>自动识别多种包管理器</div>
              </div>
            </div>
          </Col>
          <Col xs={24} sm={8}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
              <div style={{
                width: 40,
                height: 40,
                borderRadius: 8,
                background: 'rgba(34, 197, 94, 0.2)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                fontSize: 20,
              }}>
                ⚙️
              </div>
              <div>
                <div style={{ color: '#fff', fontWeight: 500 }}>配置编辑</div>
                <div style={{ color: 'rgba(255,255,255,0.5)', fontSize: 12 }}>可视化编辑配置文件</div>
              </div>
            </div>
          </Col>
          <Col xs={24} sm={8}>
            <div style={{ display: 'flex', alignItems: 'center', gap: 12 }}>
              <div style={{
                width: 40,
                height: 40,
                borderRadius: 8,
                background: 'rgba(249, 115, 22, 0.2)',
                display: 'flex',
                alignItems: 'center',
                justifyContent: 'center',
                fontSize: 20,
              }}>
                🚀
              </div>
              <div>
                <div style={{ color: '#fff', fontWeight: 500 }}>异步操作</div>
                <div style={{ color: 'rgba(255,255,255,0.5)', fontSize: 12 }}>界面流畅不卡顿</div>
              </div>
            </div>
          </Col>
        </Row>
      </div>
    </div>
  );
}
