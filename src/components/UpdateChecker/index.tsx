import { useState, useEffect } from 'react';
import { Modal, Button, Typography, Space, Tag } from 'antd';
import {
  CloudDownloadOutlined,
  RocketOutlined
} from '@ant-design/icons';
import { invoke } from '@tauri-apps/api/core';

const { Text, Paragraph } = Typography;

interface UpdateInfo {
  available: boolean;
  current_version: string;
  latest_version: string | null;
  release_notes: string | null;
}

export default function UpdateChecker() {
  const [visible, setVisible] = useState(false);
  const [updateInfo, setUpdateInfo] = useState<UpdateInfo | null>(null);
  const [downloading, setDownloading] = useState(false);

  // 启动时自动检查更新
  useEffect(() => {
    checkUpdate();
  }, []);

  async function checkUpdate() {
    try {
      const info = await invoke<UpdateInfo>('check_for_updates');
      setUpdateInfo(info);
      if (info.available) {
        setVisible(true);
      }
    } catch (e) {
      console.error('检查更新失败:', e);
    }
  }

  async function handleUpdate() {
    setDownloading(true);
    try {
      await invoke('download_and_install_update');
    } catch (e) {
      console.error('更新失败:', e);
      setDownloading(false);
    }
  }

  function handleSkip() {
    setVisible(false);
  }

  if (!updateInfo?.available) {
    return null;
  }

  return (
    <Modal
      title={
        <Space>
          <RocketOutlined style={{ color: '#6366f1' }} />
          <span>发现新版本</span>
        </Space>
      }
      open={visible}
      onCancel={handleSkip}
      footer={null}
      centered
      width={480}
    >
      <div style={{ padding: '16px 0' }}>
        <div style={{
          display: 'flex',
          justifyContent: 'center',
          alignItems: 'center',
          gap: 16,
          marginBottom: 24
        }}>
          <Tag color="default" style={{ fontSize: 14, padding: '4px 12px' }}>
            v{updateInfo.current_version}
          </Tag>
          <span style={{ color: 'rgba(255,255,255,0.5)' }}>→</span>
          <Tag color="purple" style={{ fontSize: 14, padding: '4px 12px' }}>
            v{updateInfo.latest_version}
          </Tag>
        </div>

        {updateInfo.release_notes && (
          <div style={{
            background: 'rgba(99, 102, 241, 0.1)',
            borderRadius: 8,
            padding: 16,
            marginBottom: 24,
            maxHeight: 200,
            overflow: 'auto'
          }}>
            <Text strong style={{ color: '#a5b4fc', marginBottom: 8, display: 'block' }}>
              更新内容
            </Text>
            <Paragraph
              style={{
                color: 'rgba(255,255,255,0.8)',
                margin: 0,
                whiteSpace: 'pre-wrap'
              }}
            >
              {updateInfo.release_notes}
            </Paragraph>
          </div>
        )}

        <div style={{ display: 'flex', gap: 12, justifyContent: 'flex-end' }}>
          <Button onClick={handleSkip} disabled={downloading}>
            稍后提醒
          </Button>
          <Button
            type="primary"
            icon={<CloudDownloadOutlined />}
            onClick={handleUpdate}
            loading={downloading}
          >
            {downloading ? '更新中...' : '立即更新'}
          </Button>
        </div>
      </div>
    </Modal>
  );
}
