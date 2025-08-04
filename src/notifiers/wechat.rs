use super::{send_request, Action, Notifier};
use anyhow::Result;
use serde_json::{json, Value};

/// 微信推送通知器 - 支持Server酱和PushPlus
pub enum WechatService {
    ServerChan { key: String },
    PushPlus { token: String },
}

pub struct WechatNotifier {
    service: WechatService,
}

impl WechatNotifier {
    pub fn new_serverchan(key: String) -> Self {
        Self {
            service: WechatService::ServerChan { key },
        }
    }

    pub fn new_pushplus(token: String) -> Self {
        Self {
            service: WechatService::PushPlus { token },
        }
    }
}

impl Notifier for WechatNotifier {
    fn send_text(&self, text: &str) -> Result<Value> {
        match &self.service {
            WechatService::ServerChan { key } => {
                let url = format!("https://sctapi.ftqq.com/{}.send", key);
                let data = json!({
                    "title": "通知",
                    "desp": text
                });
                send_request(&url, data)
            }
            WechatService::PushPlus { token } => {
                let url = "http://www.pushplus.plus/send";
                let data = json!({
                    "token": token,
                    "title": "通知",
                    "content": text,
                    "template": "txt"
                });
                send_request(url, data)
            }
        }
    }

    fn send_card(
        &self,
        title: &str,
        content: &str,
        _color: &str,
        actions: Vec<Action>,
    ) -> Result<Value> {
        let mut formatted_content = content.to_string();

        // 添加操作链接
        if !actions.is_empty() {
            formatted_content.push_str("\n\n---\n");
            for action in actions {
                formatted_content.push_str(&format!("[{}]({})\n", action.text, action.url));
            }
        }

        match &self.service {
            WechatService::ServerChan { key } => {
                let url = format!("https://sctapi.ftqq.com/{}.send", key);
                let data = json!({
                    "title": title,
                    "desp": formatted_content
                });
                send_request(&url, data)
            }
            WechatService::PushPlus { token } => {
                let url = "http://www.pushplus.plus/send";
                let data = json!({
                    "token": token,
                    "title": title,
                    "content": formatted_content,
                    "template": "markdown"
                });
                send_request(url, data)
            }
        }
    }
}
