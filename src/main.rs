use std::env;

use log::{error, info, warn, LevelFilter};
use serenity::all::{GuildId, Interaction};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use simple_logger::SimpleLogger;

mod commands;
mod helper_functions;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx: Context, msg: Message) {
        // TODO
        panic!("message watcher placeholder")
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        info!("bot \"{}\" has started", ready.user.name);

        let guild_id = GuildId::new(env::var("GUILD_ID").unwrap().parse().unwrap());

        if let Err(e) = guild_id
            .set_commands(
                &ctx.http,
                vec![
                    commands::help::register(),
                    commands::index::register(),
                    commands::indexall::register(),
                ],
            )
            .await
        {
            error!("error creating one or more commands: {e}")
        };
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            info!("recieved commands interaction");

            let _ = match command.data.name.as_str() {
                "help" => {
                    commands::help::run(ctx, &command).await;
                    Some(())
                }
                "index" => {
                    commands::index::run(ctx, &command, &command.data.options()).await;
                    Some(())
                }
                "indexall" => {
                    commands::indexall::run(ctx, command).await;
                    Some(())
                }
                _ => Some(warn!(
                    "command not implemented: {}",
                    format!("{}", command.data.name)
                )),
            };

            // if let Some(content) = content {
            //     let data = CreateInteractionResponseMessage::new().content(content);
            //     let builder = CreateInteractionResponse::Message(data);
            //     if let Err(why) = command.create_response(&ctx.http, builder).await {
            //         println!("Cannot respond to slash command: {why}");
            //     }
            // }
        }
    }
}

#[tokio::main]
async fn main() {
    SimpleLogger::new()
        .with_colors(true)
        .env()
        .with_level(LevelFilter::Off)
        .with_module_level("get_img", LevelFilter::Info)
        .init()
        .unwrap();
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

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .unwrap();

    if let Err(e) = client.start().await {
        error!("an error occured starting client: {e}");
        return;
    }
}
