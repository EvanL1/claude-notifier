use super::{send_request, Action, Notifier};
use anyhow::Result;
use serde_json::{json, Value};

pub struct FeishuNotifier {
    webhook: String,
}

impl FeishuNotifier {
    pub fn new(webhook: String, _at_all_on_critical: bool) -> Self {
        Self { webhook }
    }
}

impl Notifier for FeishuNotifier {
    fn send_text(&self, text: &str) -> Result<Value> {
        let data = json!({
            "msg_type": "text",
            "content": {
                "text": text
            }
        });
        send_request(&self.webhook, data)
    }

    fn send_card(
        &self,
        title: &str,
        content: &str,
        color: &str,
        actions: Vec<Action>,
    ) -> Result<Value> {
        let mut elements = vec![json!({
            "tag": "markdown",
            "content": content
        })];

        if !actions.is_empty() {
            let action_elements: Vec<Value> = actions
                .into_iter()
                .map(|action| {
                    json!({
                        "tag": "button",
                        "text": {
                            "tag": "plain_text",
                            "content": action.text
                        },
                        "url": action.url,
                        "type": "default"
                    })
                })
                .collect();

            elements.push(json!({
                "tag": "action",
                "actions": action_elements
            }));
        }

        let data = json!({
            "msg_type": "interactive",
            "card": {
                "header": {
                    "title": {
                        "tag": "plain_text",
                        "content": title
                    },
                    "template": color
                },
                "elements": elements
            }
        });

        send_request(&self.webhook, data)
    }
}
