use crate::api;
use api::Api;
use serenity::{async_trait, model::channel::Message, prelude::*};

pub struct Data {}

impl TypeMapKey for Data {
    type Value = String;
}

pub struct Bot;

impl Bot {
    pub fn new() -> Self {
        Self
    }
}

#[async_trait]
impl EventHandler for Bot {
    async fn message(&self, ctx: Context, msg: Message) {
        let mut parts = msg.content.splitn(2, char::is_whitespace);
        if parts.next() == Some("!faceit") {
            let nickname = parts.next().unwrap_or_default();
            if nickname.is_empty() {
                let _ = msg
                    .channel_id
                    .say(
                        ctx.http,
                        "you need to provide a nickname. !faceit <nickname>",
                    )
                    .await;
                return;
            }

            let data = ctx.data.read().await;
            let token = data.get::<Data>().unwrap();

            let api = Api::new(token);

            let details = api.get_player_details(nickname).await;
            if details.is_none() {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, format!("Nickname {} not found!", nickname))
                    .await;
                return;
            }

            // We can unwrap safe because we change before for none.
            let details = details.unwrap();

            let stats = api.get_player_stats(&details.player_id).await;
            if stats.is_none() {
                let _ = msg
                    .channel_id
                    .say(&ctx.http, format!("Error getting stats for {}", nickname))
                    .await;
                return;
            }

            let stats = stats.unwrap();

            let steam_profile =
                format!("https://steamcommunity.com/profiles/{}", details.steam_id64);
            let country_flag =
                country_emoji::flag(&details.country.to_uppercase()).unwrap_or("--".to_string());

            let _ = msg
                .channel_id
                .send_message(&ctx.http, |m| {
                    m.content(&format!("User {}", details.nickname));
                    m.embed(|e| {
                        e.title(&details.nickname);
                        e.description(&format!(
                            "[Faceit Profile]({}) -- [Steam Profile]({})",
                            details.faceit_url.replace("{lang}", "en"),
                            steam_profile
                        ));
                        e.thumbnail(&details.avatar);
                        e.field("Country:", &country_flag, true);
                        e.field("Wins:", &stats.lifetime.wins, true);
                        e.field("WinRate:", &stats.lifetime.win_rate, true);
                        e.field("K/D:", &stats.lifetime.k_d_ratio, true);
                        e.field(
                            "Longest WinStreak:",
                            &stats.lifetime.longest_win_streak,
                            true,
                        );
                        e.color(0xe6851e);
                        e
                    });
                    m
                })
                .await;
        }
    }
}
