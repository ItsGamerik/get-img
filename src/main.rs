use log::{error, info, LevelFilter, warn};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use simple_logger::SimpleLogger;
use std::env;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        warn!("not implemented!")
    }
    async fn ready(&self, _: Context, ready: Ready) {
        info!("bot \"{}\" has started", ready.user.name)
    }
}


#[tokio::main]
async fn main() {
    SimpleLogger::new().with_colors(true).with_level(LevelFilter::Off).with_module_level("get_img", LevelFilter::Info).init().unwrap();
    let token: String = match env::var("DISCORD_TOKEN") {
        Ok(token) => {
            info!("token found in env!");
            token
        }
        Err(e) => {
            error!("no discord token found in env: {e}");
            return;
        }
    };

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents).event_handler(Handler).await.unwrap();

    if let Err(e) = client.start().await {
        error!("an error occured starting client: {e}");
        return;
    }
}
