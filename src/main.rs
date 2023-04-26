mod commands;

use std::env;

use serenity::model::prelude::{GuildId, Message, Ready};
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
            let content = match command.data.name.as_str() {
                "index" => commands::index::run(&command.data.options, &ctx).await,
                "info" => commands::info::run(&command.data.options, &ctx).await,
                _ => String::from("test"),
                // api ref for discord interactions
                // https://discord.com/developers/docs/interactions/application-commands
                // https://discord.com/developers/docs/reference
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(model::application::interaction::InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|message| message.content(content))
                })
                .await
            {
                println!("Cannot respond to slash command: {}", why);
            }
        }
    }

    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content.contains("<@1096476929915359323>") {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "legacy command system, use slash commands")
                .await
            {
                println!("Error: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);

        // register global command, takes long to propagate through discord/api
        // let _index_command =
        //     serenity::model::application::command::Command::create_global_application_command(
        //         &ctx.http,
        //         |command: &mut serenity::builder::CreateApplicationCommand| {
        //             commands::index::register(command)
        //         },
        //     )
        //     .await;

        // let _commands =
        //     serenity::model::application::command::Command::create_global_application_command(
        //         &ctx.http,
        //         |command: &mut serenity::builder::CreateApplicationCommand| commands::info::register(command),
        //     )
        //     .await;

        // register guild-specific command, does not take as long to update

        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("guild id expected")
                .parse()
                .expect("guild id has to be a valid integer"),
            // 927882809006235658
        );

        let commands = GuildId::set_application_commands(&guild_id, &ctx.http, |commands| {
            commands
                .create_application_command(|command| commands::index::register(command))
                .create_application_command(|command| commands::info::register(command))
        })
        .await;
        dbg!(commands);
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
