mod ai;
mod commands;

use std::env;
use std::sync::Arc;

use serenity::http::{Http, Typing};
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
                "hello" => commands::hello::run().await,
                "render" => commands::asciirender::run(&command).await,
                _ => String::from("no command"),
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
            let channel_id = msg.channel_id;
            let channel = channel_id.to_channel(&ctx).await.expect("channel expected");
            if msg.is_private() {
                println!("message attempt in DMs");
                if let Err(e) = msg
                    .reply(&ctx.http, "keine nachrichten in den DMs pls")
                    .await
                {
                    println!("error: {}", e)
                }
            } else if channel.is_nsfw() == false {
                if let Err(e) = msg
                    .reply(&ctx.http, "nicht in diesem kanal, nsfw kanal erforderlich")
                    .await
                {
                    println!("error: {}", e)
                }
            } else {
                let token = env::var("DISCORD_TOKEN").expect("Expected a token in the environment");
                let http = Http::new(&token);
                let typing = Typing::start(Arc::new(http), msg.channel_id.into())
                    .expect("could not start typing");
                let response = ai::ai_chat::message_responder(&msg).await;
                typing.stop().expect("could not stop typing");

                if let Err(e) = msg.reply(&ctx.http, response).await {
                    println!("error: {}", e)
                }
            }
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
                .create_application_command(|command| commands::index::register(command))
                .create_application_command(|command| commands::info::register(command))
                .create_application_command(|command| commands::hello::register(command))
        })
        .await;
        println!("guild commands created: {:#?}", commands);

        // global command

        let global_hello =
            serenity::model::application::command::Command::create_global_application_command(
                &ctx.http,
                |command| commands::hello::register(command)
                
            )
            .await;
        let global_render = serenity::model::application::command::Command::create_global_application_command(
            &ctx.http,
            |command| commands::asciirender::register(command),
        ).await;
        println!("registered global command: {:#?}", global_hello);
        println!("registered global command: {:#?}", global_render);
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
