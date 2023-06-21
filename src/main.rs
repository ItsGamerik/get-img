mod commands;

use std::collections::HashMap;
use std::env;
use std::sync::{Arc, Mutex};

use serenity::model::prelude::{Activity, ChannelId, Ready};
use serenity::prelude::*;
use serenity::{async_trait, model};
use tokio::task::JoinHandle;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(
        &self,
        ctx: Context,
        interaction: model::application::interaction::Interaction,
    ) {
        if let model::application::interaction::Interaction::ApplicationCommand(command) =
            interaction
        {
            println!("Received command interaction");
            let _content = match command.data.name.as_str() {
                "index" => commands::index::run(&ctx, &command).await,
                "hello" => commands::hello::run(&ctx, &command).await,
                "download" => commands::download::run(&ctx, &command).await,
                "watch" => {
                    commands::watch::run(&ctx, &command).await;
                }
                _ => (),
                // api ref for discord interactions
                // https://discord.com/developers/docs/interactions/application-commands
                // https://discord.com/developers/docs/reference
            };
        }
    }
    // async fn message(&self, ctx: Context, msg: Message) {
    //     if
    // }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // set status of bot
        let activity = Activity::watching("v1.1.0");
        ctx.set_activity(activity).await;

        // register guild-specific command, does not take as long to update

        // let guild_id = GuildId(
        //     env::var("GUILD_ID")
        //         .expect("guild id expected")
        //         .parse()
        //         .expect("guild id has to be a valid integer"),
        // );

        // let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
        //     commands.create_application_command(|command| commands::hello::register(command))
        // })
        // .await;
        // println!("guild commands created: {:#?}", commands);

        // global command registering
        // TODO: handle this better altogether

        // hello command
        if let Err(e) = model::prelude::command::Command::create_global_application_command(
            &ctx.http,
            |command| commands::hello::register(command),
        )
        .await
        {
            eprintln!(
                "an error occured while registering \"hello\" command: {}",
                e
            )
        }

        // index command
        if let Err(e) = model::prelude::command::Command::create_global_application_command(
            &ctx.http,
            |command| commands::index::register(command),
        )
        .await
        {
            eprintln!(
                "an error occured while registering \"index\" command: {}",
                e
            )
        }

        // download command
        if let Err(e) = model::prelude::command::Command::create_global_application_command(
            &ctx.http,
            |command| commands::download::register(command),
        )
        .await
        {
            eprintln!(
                "an error occured while registering \"download\" command: {}",
                e
            )
        }

        // watch command
        if let Err(e) = model::prelude::command::Command::create_global_application_command(
            &ctx.http,
            |command| commands::watch::register(command),
        )
        .await
        {
            eprintln!(
                "an error occured while registering \"watch\" command: {}",
                e
            )
        }
    }
}

#[tokio::main]
async fn main() {
    let _watch_map: Arc<Mutex<HashMap<ChannelId, JoinHandle<()>>>> =
        Arc::new(Mutex::new(HashMap::new()));
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    // Set gateway intents, which decides what events the bot will be notified about
    let intents = GatewayIntents::GUILD_MESSAGES
        | GatewayIntents::DIRECT_MESSAGES
        | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
