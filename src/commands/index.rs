use std::fs::{self, OpenOptions};
use std::io::Write;

use serenity::model::prelude::{Attachment, Message, MessageApplication};
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
        PartialChannel,
    },
    prelude::Context,
};

pub async fn run(options: &[CommandDataOption], ctx: &Context) -> String {
    // get option
    let option = options
        .get(0)
        .expect("expected option")
        .resolved
        .as_ref()
        .expect("user object");

    // response logic
    if let CommandDataOptionValue::Channel(channel) = option {
        let response = format!(
            "der ausgewählte kanal ist: {}",
            channel.name.as_ref().unwrap()
        );
        index(ctx, channel, options).await;
        return response;
    } else {
        let response = "no channel id given".to_string();
        return response;
    }
}

async fn index(ctx: &Context, channel: &PartialChannel, opt: &[CommandDataOption]) {
    let a_message = channel
        .id
        .messages(&ctx, |retriever| retriever.limit(1))
        .await
        .expect("could not retrieve message");
    let the_message_from_a_message = a_message.last().unwrap();
    let the_message_id = the_message_from_a_message.id; // ein wenig XDDDDDD

    // rebuild "old" iterator from here?

    let mut message_id = the_message_id;
    loop {
        let messages = channel
            .id
            .messages(&ctx, |retriever| retriever.before(message_id).limit(100))
            .await
            .expect("Failed to retrieve messages");

        if messages.is_empty() {
            break;
        }

        message_id = messages.last().unwrap().id;
        let index_switch = opt
            .get(1)
            .expect("expected user option")
            .resolved
            .as_ref()
            .expect("user object");
        if let CommandDataOptionValue::Boolean(true) = index_switch {
            index_images(messages).await;
        } else if let CommandDataOptionValue::Boolean(false) = index_switch {
            index_all_messages(messages).await;
        }
    }
}

async fn index_all_messages(messages: Vec<Message>) {
    for message in messages {
        let content = message.content;
        let msg_string = format!("{} said [{}] on {}", message.author, content, message.timestamp);
        parse(msg_string).await;
    }
}

async fn index_images(messages: Vec<Message>) {
    let mut attachment_vec: Vec<Attachment> = Vec::new();
    let mut link_vec: Vec<String> = Vec::new();
    let mut image_vec: Vec<&Attachment> = Vec::new();
    for message in messages {
        // println!("{:?}", message);
        let has_attachment = message.attachments.iter().any(|a| !a.url.is_empty());
        if has_attachment {
            println!(
                "message {} by {} has an attachment! it was uploaded: {}",
                message.id, message.author, message.timestamp
            );
            for attachment in message.attachments {
                attachment_vec.push(attachment);
            }
        } else if message.content.contains("cdn.discordapp.com") {
            link_vec.push(message.content);
        } else {
            continue;
        }
    }
    for attachment in &attachment_vec {
        if attachment
            .content_type
            .as_ref()
            .map(|s| s == "image/png")
            .unwrap_or(false)
        {
            // println!("Attachment {} is a PNG image", attachment.id);
            image_vec.push(attachment);
        }
    }
    for i in attachment_vec
        .iter()
        .map(|attachment| attachment.url.clone())
    {
        link_vec.push(i);
    }
    for i in link_vec {
        parse(i).await;
    }
}

async fn parse(content: String) {
    if let Err(why) = fs::create_dir_all("./download/") {
        eprintln!("error creating file: {}", why);
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("./download/output.txt")
        .unwrap();
    if let Err(why) = writeln!(file, "{content}") {
        eprintln!("error while writing to file: {}", why);
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("index")
        .description("index images or messages in channel")
        .create_option(|option| {
            option
                .name("id") // MAN KANN NICHT EIN LEERZEICHEN BENUTZEN
                .description("channel id of target channel")
                .kind(serenity::model::prelude::command::CommandOptionType::Channel)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("only_images")
                .description("wether or not to index every message instead of simply the images")
                .kind(serenity::model::prelude::command::CommandOptionType::Boolean)
                .required(true)
        })
}
