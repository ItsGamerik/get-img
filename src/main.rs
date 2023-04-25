mod commands;

use std::fs::OpenOptions;
use std::io::Write;
use std::{env, fs};

use serenity::{async_trait, model};
// use serenity::futures::StreamExt;
use serenity::model::prelude::{ChannelId, Message, MessageId, Ready};

use serenity::prelude::*;

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
                "index" => commands::index::run(&command.data.options, &ctx).await, // command handling?
                _ => String::from("test"),
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

    // 927882552046399538
    async fn message(&self, ctx: Context, msg: Message) {
        // if msg.content == "<@1096476929915359323> index" {
        //     match msg.author {
        //         User {
        //             id: UserId(292662037300117514),
        //             ..
        //         } => {
        //             if let Err(why) = msg.channel_id.say(&ctx.http, "yes okay").await {
        //                 println!("Error: {:?}", why)
        //             }
        //             let index = index_messages2(msg.channel_id, &ctx, msg.into()).await;
        //             for i in index.split_whitespace() {
        //                 let i_trim = i.trim().replace("\"", "");
        //                 parser(i_trim.replace(",", ""));
        //             }
        //         }
        //         _ => {
        //             if let Err(why) = msg.channel_id.say(&ctx, "nö").await {
        //                 println!("error: {}", why);
        //             }
        //         }
        //     }
        // } else if msg.content == "<@1096476929915359323> ping" {
        //     if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
        //         println!("Error: {:?}", why)
        //     }
        //     println!("user: {:?}", msg.author);
        // } else if msg.content == ("<@1096476929915359323> fm") {
        //     println!("command recieved");
        //     firstmessage(&ctx, msg.channel_id, msg.id).await; }
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

        // register global command
        let _index_command =
            serenity::model::application::command::Command::create_global_application_command(
                &ctx.http,
                |command: &mut serenity::builder::CreateApplicationCommand| {
                    // full mod path required idk?
                    commands::index::register(command)
                },
            )
            .await;

        // register command for autofill?
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

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::builder(&token, intents)
        .event_handler(Handler)
        .await
        .expect("Err creating client");
    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}
