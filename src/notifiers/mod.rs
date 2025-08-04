pub mod feishu;
pub mod teams;
pub mod wechat;

use anyhow::Result;
use serde_json::Value;

pub trait Notifier: Send + Sync {
    fn send_text(&self, text: &str) -> Result<Value>;
    fn send_card(
        &self,
        title: &str,
        content: &str,
        color: &str,
        actions: Vec<Action>,
    ) -> Result<Value>;
}

#[derive(Debug, Clone)]
pub struct Action {
    pub text: String,
    pub url: String,
}

pub fn send_request(webhook: &str, data: Value) -> Result<Value> {
    let client = reqwest::blocking::Client::new();
    let response = client
        .post(webhook)
        .json(&data)
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .send()?;

    if response.status().is_success() {
        Ok(response.json()?)
    } else {
        Err(anyhow::anyhow!("Request failed: {}", response.status()))
    }
}
