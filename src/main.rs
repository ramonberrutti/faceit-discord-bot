mod api;
mod bot;

use api::Api;
use bot::Bot;
use dotenv::dotenv;
use serenity::Client;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let discord_token = env::var("DISCORD_TOKEN").expect("Missing DISCORD_TOKEN");
    let faceit_token = env::var("FACEIT_TOKEN").expect("Missing FACEIT_TOKEN");

    // We leak memory, but the event_handler is 'static.
    let api = Api::new(Box::leak(faceit_token.into_boxed_str()));
    let handler = Bot::new(api);

    let _ = Client::builder(&discord_token)
        .event_handler(handler)
        .await
        .expect("Err creating client")
        .start()
        .await
        .map_err(|e| println!("Client error: {:?}", e));
}
