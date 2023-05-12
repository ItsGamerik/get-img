mod commands;

use std::env;

use serenity::model::prelude::{GuildId, Ready};
use serenity::prelude::*;
use serenity::{async_trait, model};

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
                "index" => commands::index::run(&command.data.options, &ctx, &command).await,
                "hello" => commands::hello::run(&ctx, &command).await,
                "download" => commands::download::run(&ctx, &command).await,
                _ => (),
                // api ref for discord interactions
                // https://discord.com/developers/docs/interactions/application-commands
                // https://discord.com/developers/docs/reference
            };
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // register guild-specific command, does not take as long to update

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("guild id expected")
                .parse()
                .expect("guild id has to be a valid integer"),
            // 927882809006235658 (testserver id)
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                // .create_application_command(|command| commands::index::register(command))
                .create_application_command(|command| commands::hello::register(command))
                // .create_application_command(|command| commands::download::register(command))
        })
        .await;
        println!("guild commands created: {:#?}", commands);

        // global command registering

        let global_hello =
            serenity::model::application::command::Command::create_global_application_command(
                &ctx.http,
                |command| commands::hello::register(command),
            )
            .await;
        let global_index =
            serenity::model::application::command::Command::create_global_application_command(
                &ctx.http,
                |command| commands::index::register(command),
            )
            .await;
        let global_download = serenity::model::application::command::Command::create_global_application_command(
            &ctx.http,
            |command| commands::download::register(command),
        )
        .await;
        println!("registered global command: {:#?}", global_download);
        println!("registered global command: {:#?}", global_hello);
        println!("registered global command: {:#?}", global_index);
    }
}

#[tokio::main]
async fn main() {
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
