mod config;
mod notifiers;

use anyhow::Result;
use chrono::Local;
use clap::{Parser, Subcommand, ValueEnum};
use notifiers::Notifier;
use serde_json::json;
use std::collections::HashMap;
use std::io::{self, Read};
use std::sync::Arc;

#[derive(Parser)]
#[command(name = "claude-notifier")]
#[command(about = "High-performance notification manager for Teams, Feishu, and WeChat")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Send a notification
    Send {
        /// Event type (e.g., build_success, build_failure, security_alert)
        #[arg(short, long)]
        event: String,

        /// Notification title
        #[arg(short = 't', long)]
        title: String,

        /// Notification content
        #[arg(short, long)]
        content: String,

        /// Notification level (info, warning, critical, success)
        #[arg(short = 'l', long, default_value = "info")]
        level: String,

        /// Specific channels to send to (overrides config)
        #[arg(short = 'c', long, value_delimiter = ',')]
        channels: Option<Vec<Channel>>,

        /// Force send even during quiet hours
        #[arg(short = 'f', long)]
        force: bool,
    },

    /// Process notification from stdin (for hook integration)
    Hook,

    /// Initialize configuration
    Init,

    /// Test notification to specific channel
    Test {
        /// Channel to test
        #[arg(value_enum)]
        channel: Channel,
    },
}

#[derive(Debug, Clone, ValueEnum)]
enum Channel {
    Teams,
    Feishu,
    Wechat,
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Channel::Teams => write!(f, "teams"),
            Channel::Feishu => write!(f, "feishu"),
            Channel::Wechat => write!(f, "wechat"),
        }
    }
}

struct NotificationManager {
    config: config::Config,
    notifiers: HashMap<String, Arc<dyn Notifier>>,
    message_cache: HashMap<String, i64>,
}

impl NotificationManager {
    fn new() -> Result<Self> {
        let config = config::Config::load()?;
        let mut notifiers = HashMap::new();

        // 初始化Teams
        if let Some(teams_config) = &config.channels.teams {
            if teams_config.enabled && !teams_config.webhook.is_empty() {
                notifiers.insert(
                    "teams".to_string(),
                    Arc::new(notifiers::teams::TeamsNotifier::new(
                        teams_config.webhook.clone(),
                    )) as Arc<dyn Notifier>,
                );
            }
        }

        // 初始化飞书
        if let Some(feishu_config) = &config.channels.feishu {
            if feishu_config.enabled && !feishu_config.webhook.is_empty() {
                notifiers.insert(
                    "feishu".to_string(),
                    Arc::new(notifiers::feishu::FeishuNotifier::new(
                        feishu_config.webhook.clone(),
                        feishu_config.at_all_on_critical,
                    )) as Arc<dyn Notifier>,
                );
            }
        }

        // 初始化微信
        if let Some(wechat_config) = &config.channels.wechat {
            if wechat_config.enabled && !wechat_config.key.is_empty() {
                let notifier = match wechat_config.service {
                    config::WechatServiceType::ServerChan => {
                        notifiers::wechat::WechatNotifier::new_serverchan(wechat_config.key.clone())
                    }
                    config::WechatServiceType::PushPlus => {
                        notifiers::wechat::WechatNotifier::new_pushplus(wechat_config.key.clone())
                    }
                };
                notifiers.insert(
                    "wechat".to_string(),
                    Arc::new(notifier) as Arc<dyn Notifier>,
                );
            }
        }

        Ok(Self {
            config,
            notifiers,
            message_cache: HashMap::new(),
        })
    }

    fn is_quiet_hours(&self) -> bool {
        if !self.config.quiet_hours.enabled {
            return false;
        }

        let now = Local::now().format("%H:%M").to_string();
        let start = &self.config.quiet_hours.start;
        let end = &self.config.quiet_hours.end;

        if start < end {
            now >= *start && now <= *end
        } else {
            now >= *start || now <= *end
        }
    }

    fn should_send(&mut self, message_key: &str) -> bool {
        let now = Local::now().timestamp();

        if let Some(&last_sent) = self.message_cache.get(message_key) {
            if now - last_sent < 300 {
                // 5分钟去重
                return false;
            }
        }

        self.message_cache.insert(message_key.to_string(), now);

        // 清理过期缓存
        self.message_cache.retain(|_, &mut v| now - v < 600);

        true
    }

    fn send_notification(
        &mut self,
        event_type: &str,
        title: &str,
        content: &str,
        level: &str,
        override_channels: Option<Vec<Channel>>,
        force: bool,
    ) -> Result<HashMap<String, serde_json::Value>> {
        // 检查静默时段
        if !force && self.is_quiet_hours() && level != "critical" {
            return Ok(HashMap::from([(
                "status".to_string(),
                json!("quiet_hours"),
            )]));
        }

        // 消息去重
        // 使用chars()处理Unicode字符边界
        let content_preview: String = content.chars().take(50).collect();
        let message_key = format!("{}:{}:{}", event_type, title, content_preview);
        if !self.should_send(&message_key) {
            return Ok(HashMap::from([("status".to_string(), json!("duplicate"))]));
        }

        // 确定发送渠道
        let channels = if let Some(override_channels) = override_channels {
            override_channels.iter().map(|c| c.to_string()).collect()
        } else {
            self.config
                .notifications
                .get(event_type)
                .cloned()
                .unwrap_or_default()
        };

        // 颜色映射
        let color = match level {
            "info" => "0078D4",
            "warning" => "FFA500",
            "critical" => "DC3545",
            "success" => "28A745",
            _ => "0078D4",
        };

        let mut results = HashMap::new();

        // 发送到各个渠道
        for channel in channels {
            if let Some(notifier) = self.notifiers.get(&channel) {
                // 对于critical级别的飞书消息，添加@all
                let final_content = if level == "critical" && channel == "feishu" {
                    format!("{}\n<at user_id='all'></at>", content)
                } else {
                    content.to_string()
                };

                let result = notifier.send_card(title, &final_content, color, vec![]);

                results.insert(
                    channel.clone(),
                    match result {
                        Ok(val) => json!({"success": true, "response": val}),
                        Err(e) => json!({"success": false, "error": e.to_string()}),
                    },
                );
            }
        }

        Ok(results)
    }
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    match cli.command {
        Commands::Send {
            event,
            title,
            content,
            level,
            channels,
            force,
        } => {
            let mut manager = NotificationManager::new()?;
            let results =
                manager.send_notification(&event, &title, &content, &level, channels, force)?;
            println!("{}", serde_json::to_string_pretty(&results)?);
        }

        Commands::Hook => {
            // 从stdin读取JSON
            let mut input = String::new();
            io::stdin().read_to_string(&mut input)?;

            let data: serde_json::Value = serde_json::from_str(&input)?;

            let event = data["event"].as_str().unwrap_or("notification");
            let title = data["title"].as_str().unwrap_or("Notification");
            let content = data["content"].as_str().unwrap_or("");
            let level = data["level"].as_str().unwrap_or("info");

            let mut manager = NotificationManager::new()?;
            let results = manager.send_notification(event, title, content, level, None, false)?;
            println!("{}", serde_json::to_string(&results)?);
        }

        Commands::Init => {
            let config = config::Config::default();
            config.save()?;
            println!("Configuration initialized at: ~/.claude/notifiers/config.json");
            println!("Please edit the configuration file to add your webhook URLs.");
        }

        Commands::Test { channel } => {
            let mut manager = NotificationManager::new()?;
            let results = manager.send_notification(
                "test",
                "Test Notification",
                &format!("This is a test message from Claude Notifier to {}", channel),
                "info",
                Some(vec![channel]),
                true,
            )?;
            println!("{}", serde_json::to_string_pretty(&results)?);
        }
    }

    Ok(())
}
