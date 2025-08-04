use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Config {
    pub channels: ChannelConfig,
    pub notifications: HashMap<String, Vec<String>>,
    pub quiet_hours: QuietHours,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChannelConfig {
    pub teams: Option<TeamConfig>,
    pub feishu: Option<FeishuConfig>,
    pub wechat: Option<WechatConfig>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TeamConfig {
    pub enabled: bool,
    pub webhook: String,
    #[serde(default)]
    pub default_channel: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FeishuConfig {
    pub enabled: bool,
    pub webhook: String,
    #[serde(default)]
    pub at_all_on_critical: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WechatConfig {
    pub enabled: bool,
    pub service: WechatServiceType,
    pub key: String, // Server酱的key或PushPlus的token
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum WechatServiceType {
    ServerChan,
    PushPlus,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct QuietHours {
    pub enabled: bool,
    pub start: String,
    pub end: String,
}

impl Default for Config {
    fn default() -> Self {
        let mut notifications = HashMap::new();
        notifications.insert(
            "build_success".to_string(),
            vec!["teams".to_string(), "feishu".to_string()],
        );
        notifications.insert(
            "build_failure".to_string(),
            vec![
                "teams".to_string(),
                "feishu".to_string(),
                "wechat".to_string(),
            ],
        );
        notifications.insert(
            "security_alert".to_string(),
            vec![
                "teams".to_string(),
                "feishu".to_string(),
                "wechat".to_string(),
            ],
        );
        notifications.insert("daily_report".to_string(), vec!["feishu".to_string()]);

        Config {
            channels: ChannelConfig {
                teams: None,
                feishu: None,
                wechat: None,
            },
            notifications,
            quiet_hours: QuietHours {
                enabled: true,
                start: "22:00".to_string(),
                end: "08:00".to_string(),
            },
        }
    }
}

impl Config {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_path()?;

        if config_path.exists() {
            let content = fs::read_to_string(&config_path)?;
            Ok(serde_json::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }

    pub fn save(&self) -> Result<()> {
        let config_path = Self::config_path()?;

        // 确保目录存在
        if let Some(parent) = config_path.parent() {
            fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(self)?;
        fs::write(&config_path, content)?;
        Ok(())
    }

    fn config_path() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| anyhow::anyhow!("Cannot find home directory"))?;
        Ok(home.join(".claude").join("notifiers").join("config.json"))
    }
}
