use std::fs::OpenOptions;
use std::io::Write;
use std::{env, fs};

use serenity::async_trait;
use serenity::futures::StreamExt;

use serenity::model::prelude::{ChannelId, Message, MessageId, Ready};
use serenity::prelude::*;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    // async fn message(&self, ctx: Context, msg: Message) {
    //     if msg.content == "!ping" {
    //         // Sending a message can fail, due to a network error, an
    //         // authentication error, or lack of permissions to post in the
    //         // channel, so log to stdout when some error happens, with a
    //         // description of it.
    //         if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
    //             println!("Error sending message: {:?}", why);
    //         }
    //     }
    // }

    // 927882552046399538
    async fn message(&self, ctx: Context, msg: Message) {
        if msg.content == "<@1096476929915359323> index" {
            if let Err(why) = msg
                .channel_id
                .say(&ctx.http, "nachriten auflisten...")
                .await
            {
                println!("Error: {:?}", why)
            }
            let index = index_messages2(msg.channel_id, &ctx, msg.into()).await;
            let search_strings = ["https://cdn.discordapp.com", "https://media.discordapp.net"];
            for i in index.split_whitespace() {
                let i_trim = i.trim().replace("\"", "");
                downloader(i_trim);
            }
        } else if msg.content == "<@1096476929915359323> ping" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "pong!").await {
                println!("Error: {:?}", why)
            }
        } else if msg.content == "<@1096476929915359323>" {
            if let Err(why) = msg.channel_id.say(&ctx.http, "commands: index, ping").await {
                println!("Error: {}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

async fn _index_messages(channel_id: ChannelId, ctx: &Context) -> String {
    let mut messages = channel_id.messages_iter(&ctx).boxed();

    let mut s = String::new();

    while let Some(message_result) = messages.next().await {
        match message_result {
            Ok(message) => {
                let line = format!("{} said \"{}\" ", message.author.name, message.content);
                s.push_str(&line);
            }
            Err(error) => eprintln!("Uh oh! Error: {}", error),
        }
    }
    s
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
                println!("message {} by {} has an attachment!", message.id, message.author);
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
        if attachment.content_type.as_ref().map(|s| s == "image/png").unwrap_or(false) {
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
















fn downloader(url: String) {
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
