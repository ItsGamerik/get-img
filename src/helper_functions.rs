// this file contains some helper functions

use std::fs::{self, OpenOptions};
use std::io::Write;

use serenity::{
    model::{
        prelude::{
            application_command::ApplicationCommandInteraction, Activity, Attachment,
            InteractionResponseType, Message,
        },
        user::OnlineStatus,
    },
    prelude::Context,
};

/// used to create interaction responses.
pub async fn status_message(ctx: &Context, msg: &str, interaction: &ApplicationCommandInteraction) {
    start_action(ctx).await;
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| data.content(msg.to_string()))
        })
        .await
        .unwrap();
}

/// used to edit an already existing status message
pub async fn edit_status_message(
    ctx: &Context,
    response_string: &str,
    interaction: &ApplicationCommandInteraction,
) {
    stop_action(ctx).await;
    interaction
        .edit_original_interaction_response(&ctx.http, |response| response.content(response_string))
        .await
        .unwrap();
}

/// create followup message
pub async fn followup_status_message(
    ctx: &Context,
    response_string: &str,
    interaction: &ApplicationCommandInteraction,
) {
    stop_action(ctx).await;
    interaction
        .create_followup_message(&ctx.http, |response| response.content(response_string))
        .await
        .unwrap();
}

async fn start_action(ctx: &Context) {
    // the status update seems to be very slow, so it probably wont actually show unless a task is started that takes a long time
    let status = OnlineStatus::DoNotDisturb;
    ctx.set_presence(Some(Activity::watching("currently working...")), status)
        .await;
}

async fn stop_action(ctx: &Context) {
    let status = OnlineStatus::Online;
    ctx.set_presence(Some(Activity::watching("v1.2 - ready for work")), status)
        .await;
}

// used to parse messages into the output.
// does this one message at a time.
// TODO: make indexing allways automatically index attachments and text content!
pub async fn universal_parser(message: Message) {
    let message_author: serenity::model::user::User = message.author;
    let message_timestamp: serenity::model::Timestamp = message.timestamp;
    let message_content: String = message.content;
    let message_attachments: Vec<Attachment> = message.attachments;

    if let Err(e) = fs::create_dir_all("./download/") {
        eprintln!("error creating download file: {}", e);
    }

    let mut file = match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("./download/output.txt")
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("could not open file \"output.txt\": {}", e);
            return;
        }
    };

    let mut attachment_link_vec = Vec::new();

    for attachment in message_attachments {
        attachment_link_vec.push(attachment.url);
    }
    let quoted_attachments: Vec<String> = attachment_link_vec.iter().map(|s| format!("\'{}\'", s)).collect();

    let parse_message = format!(
        "{}, \"{}\", \"{}\", {}",
        message_author,
        message_content,
        quoted_attachments.join(" "),
        message_timestamp
    );

    println!("{}", parse_message);
    // if let Err(e) = writeln!(file, "{parse_message}") {
    //     eprintln!("error writing to file output.txt: {}", e);
    // }
}
