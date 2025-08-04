use super::{send_request, Action, Notifier};
use anyhow::Result;
use serde_json::{json, Value};

pub struct TeamsNotifier {
    webhook: String,
}

impl TeamsNotifier {
    pub fn new(webhook: String) -> Self {
        Self { webhook }
    }
}

impl Notifier for TeamsNotifier {
    fn send_text(&self, text: &str) -> Result<Value> {
        let data = json!({
            "text": text
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
        let mut card = json!({
            "@type": "MessageCard",
            "@context": "http://schema.org/extensions",
            "themeColor": color,
            "summary": title,
            "sections": [{
                "activityTitle": title,
                "text": content,
                "markdown": true
            }]
        });

        if !actions.is_empty() {
            let mut potential_actions = Vec::new();
            for action in actions {
                potential_actions.push(json!({
                    "@type": "OpenUri",
                    "name": action.text,
                    "targets": [{
                        "os": "default",
                        "uri": action.url
                    }]
                }));
            }
            card["potentialAction"] = json!(potential_actions);
        }

        send_request(&self.webhook, card)
    }
}
