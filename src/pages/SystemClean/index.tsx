import { useState, useEffect } from 'react';
import {
  Typography, Card, Button, Space, Progress,
  message, Modal, Row, Col, Statistic
} from 'antd';
import {
  DeleteOutlined, ReloadOutlined,
  ExclamationCircleOutlined
} from '@ant-design/icons';
import { invoke } from '@tauri-apps/api/core';
import { useAppStore } from '../../store';

const { Title, Text } = Typography;

interface CacheInfo {
  name: string;
  path: string;
  size_bytes: number;
  exists: boolean;
}

// 格式化文件大小
function formatSize(bytes: number): string {
  if (bytes === 0) return '0 B';
  const units = ['B', 'KB', 'MB', 'GB'];
  let i = 0;
  let size = bytes;
  while (size >= 1024 && i < units.length - 1) {
    size /= 1024;
    i++;
  }
  return `${size.toFixed(i === 0 ? 0 : 1)} ${units[i]}`;
}

// 缓存卡片组件
interface CacheCardProps {
  cache: CacheInfo;
  totalSize: number;
  cleaning: boolean;
  onClear: () => void;
}

function CacheCard({ cache, totalSize, cleaning, onClear }: CacheCardProps) {
  const percent = totalSize > 0 ? (cache.size_bytes / totalSize) * 100 : 0;

  return (
    <Card
      className="glass"
      styles={{ body: { padding: 16 } }}
    >
      <Space direction="vertical" style={{ width: '100%' }}>
        <div style={{ display: 'flex', justifyContent: 'space-between' }}>
          <Text strong style={{ color: '#fff' }}>{cache.name}</Text>
          <Text style={{ color: '#6366f1' }}>{formatSize(cache.size_bytes)}</Text>
        </div>
        <Progress
          percent={percent}
          showInfo={false}
          strokeColor="#6366f1"
          trailColor="rgba(99,102,241,0.2)"
          size="small"
        />
        <Text
          style={{ fontSize: 11, color: 'rgba(255,255,255,0.4)' }}
          ellipsis={{ tooltip: cache.path }}
        >
          {cache.path}
        </Text>
        <Button
          type="primary"
          danger
          size="small"
          icon={<DeleteOutlined />}
          loading={cleaning}
          disabled={!cache.exists || cache.size_bytes === 0}
          onClick={onClear}
          block
        >
          清理
        </Button>
      </Space>
    </Card>
  );
}

export default function SystemClean() {
  // 使用全局状态缓存
  const {
    caches, setCaches,
    cachesLoading: loading, setCachesLoading: setLoading,
    cachesLastFetch, isCacheValid
  } = useAppStore();

  const [cleaning, setCleaning] = useState<Set<string>>(new Set());

  useEffect(() => {
    if (!isCacheValid(cachesLastFetch)) {
      loadCaches();
    }
  }, []);

  async function loadCaches() {
    setLoading(true);
    try {
      const data = await invoke<CacheInfo[]>('scan_caches');
      setCaches(data);
    } catch (e) {
      message.error(String(e));
    }
    setLoading(false);
  }

  // 计算总大小
  const totalSize = caches.reduce((sum, c) => sum + c.size_bytes, 0);

  // 清理缓存映射
  const clearCommands: Record<string, string> = {
    'npm 缓存': 'clear_npm_cache',
    'pnpm 缓存': 'clear_pnpm_cache',
    'yarn 缓存': 'clear_yarn_cache',
    'cargo 缓存': 'clear_cargo_cache',
    'pip 缓存': 'clear_pip_cache',
    'Gradle 缓存': 'clear_gradle_cache',
    'Maven 缓存': 'clear_maven_cache',
    'Go 缓存': 'clear_go_cache',
  };

  // 确认清理
  function confirmClear(cache: CacheInfo) {
    Modal.confirm({
      title: '确认清理',
      icon: <ExclamationCircleOutlined />,
      content: (
        <div>
          <p>确定要清理 <strong>{cache.name}</strong> 吗？</p>
          <p style={{ color: 'rgba(255,255,255,0.5)', fontSize: 12 }}>
            将释放约 {formatSize(cache.size_bytes)} 空间
          </p>
        </div>
      ),
      okText: '确认清理',
      okType: 'danger',
      cancelText: '取消',
      onOk: () => handleClear(cache),
    });
  }

  // 执行清理
  async function handleClear(cache: CacheInfo) {
    const command = clearCommands[cache.name];
    if (!command) {
      message.error('不支持的缓存类型');
      return;
    }

    setCleaning(prev => new Set(prev).add(cache.name));

    try {
      const result = await invoke<string>(command);
      message.success(result);
      loadCaches();
    } catch (e) {
      message.error(String(e));
    }

    setCleaning(prev => {
      const next = new Set(prev);
      next.delete(cache.name);
      return next;
    });
  }

  return (
    <div>
      {/* 页面标题 */}
      <div style={{ marginBottom: 24 }}>
        <Title level={3} style={{ margin: 0, color: '#fff' }}>
          系统清理
        </Title>
        <Text style={{ color: 'rgba(255, 255, 255, 0.6)' }}>
          清理各包管理器的缓存，释放磁盘空间
        </Text>
      </div>

      {/* 统计卡片 */}
      <Card
        className="glass"
        style={{ marginBottom: 24 }}
        styles={{ body: { padding: 24 } }}
      >
        <Row gutter={24} align="middle">
          <Col span={8}>
            <Statistic
              title="缓存总大小"
              value={formatSize(totalSize)}
              valueStyle={{ color: '#6366f1', fontSize: 28 }}
            />
          </Col>
          <Col span={8}>
            <Statistic
              title="缓存项目"
              value={caches.filter(c => c.exists).length}
              suffix={`/ ${caches.length}`}
              valueStyle={{ color: '#fff' }}
            />
          </Col>
          <Col span={8}>
            <Button
              icon={<ReloadOutlined />}
              onClick={loadCaches}
              loading={loading}
            >
              刷新
            </Button>
          </Col>
        </Row>
      </Card>

      {/* 缓存列表 */}
      <Row gutter={[16, 16]}>
        {caches.map((cache) => (
          <Col xs={24} sm={12} lg={8} xl={6} key={cache.name}>
            <CacheCard
              cache={cache}
              totalSize={totalSize}
              cleaning={cleaning.has(cache.name)}
              onClear={() => confirmClear(cache)}
            />
          </Col>
        ))}
      </Row>
    </div>
  );
}