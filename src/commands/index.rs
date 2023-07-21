use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::helper_functions::{edit_status_message, status_message, universal_parser};

use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::{Attachment, Message};
use serenity::model::Permissions;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        interaction::application_command::{CommandDataOption, CommandDataOptionValue},
        PartialChannel,
    },
    prelude::Context,
};

/// function that gets executed when the command is run
pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    let option = interaction
        .data
        .options
        .get(0)
        .expect("expected user option")
        .resolved
        .as_ref()
        .expect("expected user objexct");

    if let CommandDataOptionValue::Channel(channel) = option {
        let response_string = format!(
            "added messages from channel **{}** to index",
            channel.name.as_ref().unwrap()
        );
        status_message(ctx, "indexing channel...", interaction).await;

        index(ctx, channel, &interaction.data.options).await;

        edit_status_message(ctx, &response_string, interaction).await;
    } else {
        status_message(ctx, "an error occured", interaction).await;
    }
}

/// used to index the selected channel depending on wether or not "only_images" is true or false
async fn index(ctx: &Context, channel: &PartialChannel, opt: &[CommandDataOption]) {
    let message_vector = channel
        .id
        .messages(&ctx, |retriever| retriever.limit(1))
        .await
        .expect("could not retrieve message");
    let single_message: &Message = message_vector.last().unwrap();
    let mut single_message_id = single_message.id;
    loop {
        let messages = channel
            .id
            .messages(&ctx, |retriever| {
                retriever.before(single_message_id).limit(100)
            })
            .await
            .expect("Failed to retrieve messages");

        if messages.is_empty() {
            break;
        }

        // try to iterate "upwards" in the channel
        single_message_id = messages.last().unwrap().id;
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

/// dont index attachments
pub async fn index_all_messages(messages: Vec<Message>) {
    if let Err(why) = fs::create_dir_all("./download/") {
        eprintln!("error creating file: {}", why);
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("./download/output.txt")
        .unwrap();
    let header_string = "userid,message,attachments,timestamp".to_string();
    if let Err(e) = writeln!(file, "{header_string}") {
        eprintln!("error writing to file: {}", e);
    };
    for message in messages {
        universal_parser(message).await;
        // let content = message.content;
        // let msg_string = format!("{}, \"{}\",{},", message.author, content, message.timestamp);
        // parse(msg_string).await;
    }
}

/// ONLY index attachments
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

/// parse the messages/attachments to the file "output.txt"
pub async fn parse(content: String) {
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
    };
}

/// function that registers the command with the discord api
/// minimum permission level: ADMINISTRATOR
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
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
