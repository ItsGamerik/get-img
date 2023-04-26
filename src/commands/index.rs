use std::fs::{self, OpenOptions};
use std::io::Write;

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
    let mut response: String = String::new();
    if let CommandDataOptionValue::Channel(channel) = option {
        response = format!(
            "der ausgewählte kanal ist: {}",
            channel.name.as_ref().unwrap()
        );
        message_index(&ctx, channel).await;
    } else {
        response = "no channel id given".to_string();
    }
    response
}

async fn message_index(ctx: &Context, channel: &PartialChannel) {
    let a_message = channel
        .id
        .messages(&ctx, |retriever| retriever.limit(1))
        .await
        .expect("could not retrieve message");
    let the_message_from_a_message = a_message.last().unwrap();
    let the_message_id = the_message_from_a_message.id; // ein wenig XDDDDDD

    // rebuild "old" iterator from here?

    let mut attachment_vec = Vec::new();
    let mut image_vec = Vec::new();
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
        for message in messages {
            // println!("{:?}", message);
            let has_attachment = message.attachments.iter().any(|a| a.url != "");
            if has_attachment == true {
                println!(
                    "message {} by {} has an attachment! it was uploaded: {}",
                    message.id, message.author, message.timestamp
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
            // println!("Attachment {} is a PNG image", attachment.id);
            image_vec.push(attachment);
        }
    }
    for i in attachment_vec
        .iter()
        .map(|attachment| attachment.url.clone())
    {
        parse(i).await;
    }
}

async fn parse(url: String) {
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

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("index")
        .description("index slash command")
        .create_option(|option| {
            option
                .name("channel id")
                .description("channel id of target channel")
                .kind(serenity::model::prelude::command::CommandOptionType::Channel)
                .required(true)
        })
}
