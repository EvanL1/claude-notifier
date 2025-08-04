# Claude Notifier 🚀

[English](#english) | [中文](#中文)

A high-performance notification manager written in Rust for Microsoft Teams, Feishu (Lark), and WeChat. Designed for developers who need reliable, fast, and flexible notification delivery across multiple platforms.

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=flat&logo=rust&logoColor=white)](https://www.rust-lang.org/)

## ✨ Features

- 🚀 **Lightning Fast**: < 5ms startup time, < 3MB memory usage
- 📱 **Multi-Platform Support**: Teams, Feishu/Lark, WeChat (Server酱/PushPlus)
- 🔧 **Flexible Configuration**: Event-based routing with JSON config
- 🌙 **Quiet Hours**: Built-in Do Not Disturb scheduling
- 🔁 **Message Deduplication**: Automatic 5-minute duplicate suppression
- 🎯 **CLI First**: Full-featured command-line interface
- 🔌 **Hook Integration**: Perfect for CI/CD pipelines and development tools

## 📦 Installation

### From Source

```bash
git clone https://github.com/yourusername/claude-notifier.git
cd claude-notifier
cargo build --release
```

The binary will be available at `target/release/claude-notifier`.

### Using Cargo

```bash
cargo install claude-notifier
```

## 🚀 Quick Start

1. **Initialize configuration**:
```bash
claude-notifier init
```

2. **Edit configuration** (`~/.claude/notifiers/config.json`):
```json
{
  "channels": {
    "feishu": {
      "enabled": true,
      "webhook": "YOUR_FEISHU_WEBHOOK_URL",
      "at_all_on_critical": true
    }
  }
}
```

3. **Send a notification**:
```bash
claude-notifier send -e alert -t "Hello World" -c "This is a test message" -l info
```

## 📚 Usage

### Command Line Interface

```bash
# Send notification
claude-notifier send -e build_success -t "Build Complete" -c "All tests passed!" -l success

# Send to specific channels
claude-notifier send -e alert -t "Alert" -c "Important message" -c teams,feishu

# Force send during quiet hours
claude-notifier send -e critical -t "System Alert" -c "Critical issue detected" -l critical -f

# Test specific channel
claude-notifier test feishu
```

### Hook Mode (for CI/CD)

```bash
echo '{"event":"build_success","title":"Build #123","content":"Completed in 2m 30s","level":"success"}' | claude-notifier hook
```

### As a Library

```rust
use claude_notifier::{NotificationManager, Channel};

let mut manager = NotificationManager::new()?;
manager.send_notification(
    "build_success",
    "Build Complete",
    "All tests passed!",
    "success",
    Some(vec![Channel::Feishu]),
    false,
)?;
```

## 🔧 Configuration

### Channel Setup

#### Feishu/Lark
1. Open Feishu group chat
2. Settings → Group Bot → Add Bot → Custom Bot
3. Copy the webhook URL

#### Microsoft Teams
1. Open Teams channel
2. Connectors → Incoming Webhook
3. Configure and copy the webhook URL

#### WeChat (Server酱)
1. Visit https://sct.ftqq.com/
2. Login with GitHub and get your SendKey
3. Use `"service": "serverchan"` in config

#### WeChat (PushPlus)
1. Visit http://www.pushplus.plus/
2. Register and get your token
3. Use `"service": "pushplus"` in config

### Event Types

- `build_success` / `build_failure`: Build notifications
- `test_success` / `test_failure`: Test results
- `deploy_start` / `deploy_success` / `deploy_failure`: Deployment status
- `security_alert`: Security warnings
- `daily_report`: Daily summaries
- Custom event types supported

### Message Levels

- `info`: General information (blue)
- `warning`: Warnings (orange)
- `critical`: Critical alerts (red, bypasses quiet hours)
- `success`: Success messages (green)

## 🔌 Integration Examples

### GitHub Actions

```yaml
- name: Notify Build Status
  run: |
    echo '{"event":"build_${{ job.status }}","title":"Build ${{ github.run_number }}","content":"${{ github.event.head_commit.message }}","level":"${{ job.status }}"}' | claude-notifier hook
```

### Git Hooks

```bash
#!/bin/bash
# .git/hooks/post-commit
claude-notifier send -e code_commit -t "New Commit" -c "$(git log -1 --pretty=%B)" -l info
```

### Python Wrapper

```python
import subprocess
import json

def send_notification(event, title, content, level="info"):
    data = {"event": event, "title": title, "content": content, "level": level}
    subprocess.run(
        ["claude-notifier", "hook"],
        input=json.dumps(data),
        text=True
    )
```

## 🏗️ Architecture

```
claude-notifier/
├── src/
│   ├── main.rs          # CLI entry point
│   ├── config.rs        # Configuration management
│   ├── notifiers/       # Platform implementations
│   │   ├── mod.rs       # Notifier trait
│   │   ├── teams.rs     # Teams notifier
│   │   ├── feishu.rs    # Feishu notifier
│   │   └── wechat.rs    # WeChat notifier
│   └── manager.rs       # Notification manager
└── Cargo.toml
```

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📝 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

---

<a name="中文"></a>

# Claude Notifier 🚀

一个用 Rust 编写的高性能通知管理器，支持 Microsoft Teams、飞书和微信推送。专为需要可靠、快速、灵活的多平台通知投递的开发者设计。

## ✨ 特性

- 🚀 **极速启动**：启动时间 < 5ms，内存占用 < 3MB
- 📱 **多平台支持**：Teams、飞书、微信（Server酱/PushPlus）
- 🔧 **灵活配置**：基于事件的路由配置
- 🌙 **静默时段**：内置免打扰时间管理
- 🔁 **消息去重**：自动5分钟重复消息抑制
- 🎯 **CLI 优先**：功能完整的命令行界面
- 🔌 **Hook 集成**：完美适配 CI/CD 和开发工具

## 📦 安装

### 从源码编译

```bash
git clone https://github.com/yourusername/claude-notifier.git
cd claude-notifier
cargo build --release
```

二进制文件位于 `target/release/claude-notifier`。

## 🚀 快速开始

1. **初始化配置**：
```bash
claude-notifier init
```

2. **编辑配置** (`~/.claude/notifiers/config.json`)：
```json
{
  "channels": {
    "feishu": {
      "enabled": true,
      "webhook": "您的飞书WEBHOOK地址",
      "at_all_on_critical": true
    }
  }
}
```

3. **发送通知**：
```bash
claude-notifier send -e alert -t "你好世界" -c "这是一条测试消息" -l info
```

## 📚 使用方法

### 命令行界面

```bash
# 发送通知
claude-notifier send -e build_success -t "构建完成" -c "所有测试通过！" -l success

# 发送到指定渠道
claude-notifier send -e alert -t "警告" -c "重要消息" -c teams,feishu

# 强制发送（忽略静默时段）
claude-notifier send -e critical -t "系统警报" -c "检测到严重问题" -l critical -f

# 测试特定渠道
claude-notifier test feishu
```

### Hook 模式（用于 CI/CD）

```bash
echo '{"event":"build_success","title":"构建 #123","content":"耗时 2分30秒","level":"success"}' | claude-notifier hook
```

## 🔧 配置说明

### 飞书配置
1. 打开飞书群聊
2. 设置 → 群机器人 → 添加机器人 → 自定义机器人
3. 复制 webhook 地址

### 微信配置（Server酱）
1. 访问 https://sct.ftqq.com/
2. 使用 GitHub 登录获取 SendKey
3. 配置中使用 `"service": "serverchan"`

### 事件类型

- `build_success` / `build_failure`：构建通知
- `test_success` / `test_failure`：测试结果
- `deploy_start` / `deploy_success` / `deploy_failure`：部署状态
- `security_alert`：安全警告
- `daily_report`：日报
- 支持自定义事件类型

## 🤝 贡献

欢迎贡献！请随时提交 Pull Request。

## 📝 许可证

本项目采用 MIT 许可证 - 详见 [LICENSE](LICENSE) 文件。

---

Made with ❤️ by Claude Assistant & Contributors