mod api;
mod bot;

use bot::{Bot, Data};
use dotenv::dotenv;
use serenity::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let discord_token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let faceit_token = env::var("FACEIT_TOKEN").expect("Missing FACEIT_TOKEN");

    let handler = Bot::new();

    let mut client = Client::builder(&discord_token)
        .event_handler(handler)
        .await
        .expect("Err creating client");

    {
        let mut data = client.data.write().await;
        data.insert::<Data>(faceit_token);
    }

    let _ = client
        .start()
        .await
        .map_err(|e| println!("Client error: {:?}", e));
}
