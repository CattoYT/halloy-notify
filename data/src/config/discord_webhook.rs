use std::{sync::Arc, thread::sleep, time::Duration};

use serde::Deserialize;

// frankly, this is all very shit so dont bother

#[derive(Debug, Clone, Deserialize)]
#[serde(default)]
pub struct DiscordWebhook {
    #[serde(rename = "webhook")]
    pub webhook: Option<String>,
    pub user_id: Option<String>,
    pub urgency_level: i8,
    pub spam_for_interview: bool,
}

#[derive(Debug, Clone, PartialEq)]
pub enum MessageType {
    Highlight(String),      //include highlighter
    Interview(String, i32), //include username of person being interviewed maybe
    SelfInterview,
    Test,
    Netsplit,
}

impl Default for DiscordWebhook {
    fn default() -> Self {
        Self {
            webhook: None,
            user_id: None,
            urgency_level: 1,
            spam_for_interview: true,
        }
    }
}

pub enum WebhookError {
    EmptyWebhookUrl,
    Idk,
    EmptyUid,
}

impl DiscordWebhook {
    pub fn send_message(
        &self,
        message_contents: String,
        message_type: MessageType,
    ) -> Result<(), WebhookError> {
        let should_spam = self.spam_for_interview;
        let urgency_level = self.urgency_level;

        let url = match &self.webhook {
            Some(url) => url.clone(),
            None => return Err(WebhookError::EmptyWebhookUrl),
        };

        let userid = match &self.user_id {
            Some(uid) => {
                log::debug!("Missing discord uid!");
                uid
            }
            None => {
                log::debug!("Missing discord uid!");
                return Err(WebhookError::EmptyUid);
            }
        };

        let (message, urgency): (String, i8) = match &message_type {
            MessageType::Highlight(user) => {
                (format!("<@{userid}> You were highlighted by {user}"), 2)
            }
            MessageType::Interview(user, q_length) => (
                format!(
                    "<@{userid}> {user} is being interviewed! Queue length: {q_length}"
                ),
                1,
            ),
            MessageType::SelfInterview => {
                (format!("<@{userid}> YOU ARE BEING INTERVIEWED!"), 3)
            }
            MessageType::Netsplit => {
                (format!("<@{userid}> NETSPLIT DETECTED"), 3)
            }
            MessageType::Test => (format!("<@{userid}> Test message"), 1),
        };

        tokio::spawn(async move {
            let client = reqwest::Client::new();
            if should_spam && message_type == MessageType::SelfInterview {
                for _ in 0..5 {
                    let _ = client
                        .post(&url)
                        .json(&serde_json::json!({
                            "content": &message,
                            "embeds": [
                                {
                                    "title": "Contents of alerted message:",
                                    "description": message_contents,
                                    "color": null
                                }
                            ],
                            "attachments": []
                        }))
                        .send()
                        .await;
                    sleep(Duration::from_millis(300));
                }
            } else {
                if urgency_level <= urgency {
                    let _ = client
                        .post(&url)
                        .json(&serde_json::json!({
                            "content": &message,
                            "embeds": [
                                {
                                    "title": "Contents of alerted message:",
                                    "description": message_contents,
                                    "color": null
                                }
                            ],
                            "attachments": []
                        }))
                        .send()
                        .await;
                } else {
                    log::debug!(
                        "Skipped lower urgency event of {:?}",
                        message_type
                    )
                }
            }
        });
        Ok(())
    }
}
