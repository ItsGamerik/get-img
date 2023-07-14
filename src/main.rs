mod commands;
mod helper_functions;

use std::env;

use serenity::model::prelude::{Activity, Ready};
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
                "index" => commands::index::run(&ctx, &command).await,
                "hello" => commands::hello::run(&ctx, &command).await,
                "download" => commands::download::run(&ctx, &command).await,
                "watch" => commands::watch::run(&ctx, &command).await,
                _ => (),
                // api ref for discord interactions
                // https://discord.com/developers/docs/interactions/application-commands
                // https://discord.com/developers/docs/reference
            };
        }
    }
    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // set status of bot
        let activity = Activity::watching("v1.1");
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
        init_commands(&ctx).await;
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;

    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}

/// register commands function
async fn init_commands(ctx: &Context) {
    // hello command
    if let Err(e) =
        model::prelude::command::Command::create_global_application_command(&ctx.http, |command| {
            commands::hello::register(command)
        })
        .await
    {
        eprintln!(
            "an error occured while registering \"hello\" command: {}",
            e
        )
    } else {
        println!("registered hello command!");
    }

    // index command
    if let Err(e) =
        model::prelude::command::Command::create_global_application_command(&ctx.http, |command| {
            commands::index::register(command)
        })
        .await
    {
        eprintln!(
            "an error occured while registering \"index\" command: {}",
            e
        )
    } else {
        println!("registered index command!");
    }

    // download command
    if let Err(e) =
        model::prelude::command::Command::create_global_application_command(&ctx.http, |command| {
            commands::download::register(command)
        })
        .await
    {
        eprintln!(
            "an error occured while registering \"download\" command: {}",
            e
        )
    } else {
        println!("registered download command!");
    }

    // watch command
    if let Err(e) =
        model::prelude::command::Command::create_global_application_command(&ctx.http, |command| {
            commands::watch::register(command)
        })
        .await
    {
        eprintln!(
            "an error occured while registering \"watch\" command: {}",
            e
        )
    } else {
        println!("registered watch command!");
    }
}
