import { Typography, Card, Switch, Form, Select, Divider } from 'antd';

const { Title, Text } = Typography;

export default function Settings() {
  return (
    <div>
      {/* Header */}
      <div style={{ marginBottom: 24 }}>
        <Title level={4} style={{ margin: 0, color: '#1a1f36' }}>
          设置
        </Title>
        <Text type="secondary">
          配置应用程序的行为和外观
        </Text>
      </div>

      {/* Settings Cards */}
      <div style={{ maxWidth: 600 }}>
        <Card
          title="通用设置"
          style={{ marginBottom: 16 }}
        >
          <Form layout="horizontal" labelCol={{ span: 8 }} wrapperCol={{ span: 16 }}>
            <Form.Item label="启动时自动扫描">
              <Switch defaultChecked />
            </Form.Item>
            <Form.Item label="显示系统包">
              <Switch />
            </Form.Item>
            <Form.Item label="语言">
              <Select
                defaultValue="zh-CN"
                options={[
                  { value: 'zh-CN', label: '简体中文' },
                  { value: 'en-US', label: 'English' },
                ]}
                style={{ width: 200 }}
              />
            </Form.Item>
          </Form>
        </Card>

        <Card title="关于">
          <div style={{ display: 'flex', flexDirection: 'column', gap: 8 }}>
            <div>
              <Text type="secondary">版本：</Text>
              <Text>0.1.0</Text>
            </div>
            <div>
              <Text type="secondary">技术栈：</Text>
              <Text>Tauri + React + Rust</Text>
            </div>
            <Divider style={{ margin: '12px 0' }} />
            <Text type="secondary">
              DevTool Manager 是一个开发者工具管理应用，帮助你管理 npm、cargo、pip 等包管理器安装的工具。
            </Text>
          </div>
        </Card>
      </div>
    </div>
  );
}
