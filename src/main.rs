use std::{env, fs};

use log::{error, info, LevelFilter};
use serenity::all::{GuildId, Interaction};
use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;
use simple_logger::SimpleLogger;
use tokio::fs::File;
use tokio::io::AsyncBufReadExt;

use crate::commands::download::download_file;
use crate::commands::watch::WatcherEntry;
use crate::config::config_functions::CONFIG;
use crate::helper_functions::universal_message_writer;

mod commands;
mod config;
mod helper_functions;

struct Handler;
#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        let lock = CONFIG.lock().await;

        let cfg = lock.get().unwrap();

        let path = &cfg.directories.watchfile;

        let watcher_file = match File::open(path.to_string() + ".watchers").await {
            Ok(f) => f,
            Err(e) => {
                error!("watchfile not found: {e}");
                return;
            }
        };

        drop(lock);

        let mut lines = tokio::io::BufReader::new(watcher_file).lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            let json: WatcherEntry = match serde_json::from_str(&line) {
                Ok(entry) => entry,
                Err(e) => {
                    error!("did u mess with the watchfile >:( ({e})");
                    return;
                }
            };
            if msg.channel_id == json.id {
                info!("Channel watcher found a new message: {}", msg.id);
                universal_message_writer(msg.clone()).await;
                if json.autodl {
                    for attachment in msg.attachments.clone() {
                        download_file(attachment.url).await;
                    }
                } else {
                    continue;
                }
            }
        }
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
                    commands::watch::register(),
                    commands::download::register(),
                ],
            )
            .await
        {
            error!("error creating one or more commands: {e}")
        };
    }
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = interaction {
            info!("recieved command interaction");

            match command.data.name.as_str() {
                "watch" => {
                    commands::watch::run(ctx, &command, &command.data.options()).await;
                    Some(())
                }
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
                "download" => {
                    commands::download::run(ctx, &command, &command.data.options()).await;
                    Some(())
                }
                _ => Some(error!(
                    "command not implemented: {}",
                    format!("{}", command.data.name)
                )),
            };
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
    if let Err(e) = config::config_functions::read_config().await {
        error!("error reading config file: {e}");
        return;
    } else {
        info!("successfully read config file!")
    };

    let lock = CONFIG.lock().await;
    let cfg = lock.get().unwrap();
    let path = &cfg.directories.watchfile;

    match fs::OpenOptions::new()
        .write(true)
        .create(true)
        .open(path.to_string() + ".watchers")
    {
        Ok(_) => (),
        Err(e) => {
            error!("could not create watchfile: {e}");
        }
    }

    drop(lock);

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
