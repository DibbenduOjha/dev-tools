import { useState, useEffect } from 'react';
import {
  Typography, Card, Row, Col, Tag, Button, Spin, Empty
} from 'antd';
import { ReloadOutlined } from '@ant-design/icons';
import { invoke } from '@tauri-apps/api/core';

const { Title, Text } = Typography;

interface RuntimeVersion {
  name: string;
  version: string | null;
  path: string | null;
  manager: string | null;
}

// 版本卡片组件
interface VersionCardProps {
  runtime: RuntimeVersion;
  color: string;
}

function VersionCard({ runtime, color }: VersionCardProps) {
  return (
    <Card
      className="glass"
      styles={{ body: { padding: 20 } }}
    >
      <div style={{ display: 'flex', alignItems: 'center', marginBottom: 16 }}>
        <div
          style={{
            width: 48,
            height: 48,
            borderRadius: 12,
            background: `linear-gradient(135deg, ${color}40, ${color}20)`,
            display: 'flex',
            alignItems: 'center',
            justifyContent: 'center',
            marginRight: 16,
            border: `2px solid ${color}60`,
          }}
        >
          <Text style={{ fontSize: 24 }}>
            {runtime.name === 'Node.js' ? '⬢' :
             runtime.name === 'Python' ? '🐍' : '🦀'}
          </Text>
        </div>
        <div>
          <Text strong style={{ color: '#fff', fontSize: 18 }}>
            {runtime.name}
          </Text>
          {runtime.manager && (
            <Tag color="purple" style={{ marginLeft: 8 }}>
              {runtime.manager}
            </Tag>
          )}
        </div>
      </div>

      <div style={{ marginBottom: 8 }}>
        <Text style={{ color: 'rgba(255,255,255,0.5)' }}>版本</Text>
        <div>
          <Text style={{ color, fontSize: 20, fontWeight: 600 }}>
            {runtime.version || '未安装'}
          </Text>
        </div>
      </div>

      {runtime.path && (
        <div>
          <Text style={{ color: 'rgba(255,255,255,0.5)' }}>路径</Text>
          <div>
            <Text
              style={{ color: 'rgba(255,255,255,0.7)', fontSize: 12 }}
              ellipsis={{ tooltip: runtime.path }}
            >
              {runtime.path}
            </Text>
          </div>
        </div>
      )}
    </Card>
  );
}

export default function VersionManager() {
  const [versions, setVersions] = useState<RuntimeVersion[]>([]);
  const [loading, setLoading] = useState(false);

  useEffect(() => {
    loadVersions();
  }, []);

  async function loadVersions() {
    setLoading(true);
    try {
      const data = await invoke<RuntimeVersion[]>('get_runtime_versions');
      setVersions(data);
    } catch (e) {
      console.error(e);
    }
    setLoading(false);
  }

  // 获取运行时图标颜色
  const getColor = (name: string) => {
    switch (name) {
      case 'Node.js': return '#68a063';
      case 'Python': return '#3776ab';
      case 'Rust': return '#dea584';
      default: return '#6366f1';
    }
  };

  return (
    <div>
      {/* 页面标题 */}
      <div style={{ marginBottom: 24 }}>
        <Title level={3} style={{ margin: 0, color: '#fff' }}>
          版本管理
        </Title>
        <Text style={{ color: 'rgba(255, 255, 255, 0.6)' }}>
          查看已安装的开发环境版本
        </Text>
      </div>

      {/* 刷新按钮 */}
      <div style={{ marginBottom: 16 }}>
        <Button
          icon={<ReloadOutlined />}
          onClick={loadVersions}
          loading={loading}
        >
          刷新
        </Button>
      </div>

      {/* 版本卡片列表 */}
      {loading ? (
        <div style={{ textAlign: 'center', padding: 48 }}>
          <Spin size="large" />
        </div>
      ) : versions.length === 0 ? (
        <Empty description="未检测到运行时" />
      ) : (
        <Row gutter={[16, 16]}>
          {versions.map((rt) => (
            <Col xs={24} sm={12} lg={8} key={rt.name}>
              <VersionCard runtime={rt} color={getColor(rt.name)} />
            </Col>
          ))}
        </Row>
      )}
    </div>
  );
}
