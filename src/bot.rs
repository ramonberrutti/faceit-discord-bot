use serenity::{async_trait, model::channel::Message, prelude::*};
use std::sync::{Arc};
use crate::api;

pub struct Bot<'a> {
    api: Arc<api::Api<'a>>,
}

impl <'a> Bot<'a> {
    pub fn new(api: api::Api<'a>) -> Self {
        Self {
            api: Arc::new(api),
        }
    }
}

#[async_trait]
impl <'a>EventHandler for Bot<'a> {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut parts = msg.content.splitn(2, char::is_whitespace);
        if parts.next() == Some("!faceit") {
            let nickname = parts.next().unwrap_or_default();
            if nickname.is_empty() {
                let _ = msg.channel_id.say(ctx.http, "you need to provide a nickname. !faceit <nickname>").await;
                return;
            }

            let details = self.api.get_player_details(nickname).await;
            if details.is_none() {
                let _ = msg.channel_id.say(&ctx.http, format!("{} not found!", nickname)).await;
                return;
            }

            // We can unwrap safe because we change before for none.
            let details = details.unwrap();

            let stats = self.api.get_player_stats(&details.player_id).await;
            if stats.is_none() {
                let _ = msg.channel_id.say(&ctx.http, format!("Error getting stats from {}", nickname)).await;
                return;
            }
            
            let stats = stats.unwrap();

            let _ = msg.channel_id.send_message(&ctx.http, |m| {
                m.content(&format!("User {}", details.nickname));
                m.embed(|e| {
                    e.title(&details.nickname);
                    e.description(serde_json::to_string_pretty(&stats.lifetime).unwrap_or_default());
                    e
                });
                m
            }).await;
        }
    }
}