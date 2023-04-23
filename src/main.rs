mod commands;

use std::fs::OpenOptions;
use std::io::Write;

use std::{env, fs};

use serenity::async_trait;
use serenity::futures::StreamExt;
use serenity::model::prelude::{ChannelId, Message, MessageId, Ready, UserId, Interaction, InteractionResponseType, GuildId};
use serenity::model::{timestamp, Timestamp, guild};
use serenity::model::user::User;
use serenity::prelude::*;


struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::ApplicationCommand(command) = interaction {
            println!("Received command interaction: {:#?}", command);

            let content = match command.data.name.as_str() {
                "index" => "test".to_string(), // command handling?
                _ => "not implemented :(".to_string(),
            };

            if let Err(why) = command
                .create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
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
        if msg.content == "<@1096476929915359323> index" {
            match msg.author {
                User {
                    id: UserId(292662037300117514),
                    ..
                } => {
                    if let Err(why) = msg.channel_id.say(&ctx.http, "yes okay").await {
                        println!("Error: {:?}", why)
                    }
                    let index = index_messages2(msg.channel_id, &ctx, msg.into()).await;
                    for i in index.split_whitespace() {
                        let i_trim = i.trim().replace("\"", "");
                        parser(i_trim.replace(",", ""));
                    }
                }
                _ => {
                    if let Err(why) = msg.channel_id.say(&ctx, "nö").await {
                        println!("error: {}", why);
                    }
                }
            }
        } else if msg.content == "<@1096476929915359323> ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
                println!("Error: {:?}", why)
            }
            println!("user: {:?}", msg.author);
        } else if msg.content == ("<@1096476929915359323> fm") {
            println!("command recieved");
            firstmessage(&ctx, msg.channel_id, msg.id).await;
        } else if msg.content.contains("<@1096476929915359323>") {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "commands: index, ping, fm")
                .await
            {
                println!("Error: {}", why);
            }
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
        
        let guild_id = GuildId(
            env::var("GUILD_ID")
                .expect("Expected GUILD_ID in environment")
                .parse()
                .expect("GUILD_ID must be an integer"),
        );
        // register global command
        let _index_command = serenity::model::application::command::Command::create_global_application_command(&ctx.http, |command| { // full mod path required idk?
            commands::index::register(command)
        }).await;

        // register command for autofill?
    }
}

async fn index_messages2(channel_id: ChannelId, ctx: &Context, msg_id: MessageId) -> String {
    // start program
    // -> get current message -> move "up" 100 messages
    // -> get 100th message -> move "up" 100 messages
    // repeat until at last message
    // discord api limits 100/time

    //
    // get messages
    //

    let mut attachment_vec = Vec::new();
    let mut image_vec = Vec::new();
    let mut message_id = msg_id;
    loop {
        let messages = channel_id
            .messages(&ctx, |retriever| retriever.before(message_id).limit(100))
            .await
            .expect("Failed to retrieve messages");

        if messages.is_empty() {
            break;
        }

        message_id = messages.last().unwrap().id;
        for message in messages {
            // println!("{:?}", message);
            let has_attachment = message.attachments.iter().any(|a| a.url != "");
            if has_attachment == true {
                println!(
                    "message {} by {} has an attachment!",
                    message.id, message.author
                );
                for attachment in message.attachments {
                    attachment_vec.push(attachment);
                }
            } else {
                continue;
            }
        }
    }

    //
    // filter for images
    //

    for attachment in &attachment_vec {
        if attachment
            .content_type
            .as_ref()
            .map(|s| s == "image/png")
            .unwrap_or(false)
        {
            println!("Attachment {} is a PNG image", attachment.id);
            image_vec.push(attachment);
        }
    }
    let url_string = attachment_vec
        .iter()
        .map(|attachment| attachment.url.clone())
        .collect::<Vec<String>>()
        .join(", ");
    println!("url: {}", &url_string);
    url_string
}

async fn firstmessage(ctx: &Context, channel_id: ChannelId, msg_id: MessageId) {
    let current_message = channel_id.message(&ctx.http, msg_id).await.expect("error getting current message");
    let current_message_time = current_message.timestamp;
    let mut time_keeper = current_message_time;
    loop {
        let messages = channel_id
            .messages(&ctx, |retriever| retriever.before(msg_id).limit(100))
            .await
            .expect("Failed to retrieve messages");

            // 1099428229024075887 -> 1.
            // 1099428266886058116 -> 2.
            // 1099428296644628571 -> 3.

        for message in messages.iter().rev() {
            let date = message.timestamp;
            // dbg!(date);
            if time_keeper > date {
                time_keeper = date;
            } else {
                // dbg!(date);
            }
        }
    }
}

fn parser(url: String) {
    if let Err(why) = fs::create_dir_all("./download/") {
        eprintln!("error creating file: {}", why);
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("./download/output.txt")
        .unwrap();
    if let Err(why) = writeln!(file, "{url}") {
        eprintln!("error while writing to file: {}", why);
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
