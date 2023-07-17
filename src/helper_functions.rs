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
// TODO: make this
// pub async fn universal_parser(message: Message, only_attachments: bool) {
//     let message_author = message.author;
//     let message_timestamp = message.timestamp;
//     let message_content = message.content;
//     let message_attachments: Vec<&Attachment> = message.attachments.iter().collect();

//     if let Err(e) = fs::create_dir_all("./download/") {
//         eprintln!("error creating download file: {}", e);
//     }

//     let mut file = match OpenOptions::new()
//         .write(true)
//         .create(true)
//         .append(true)
//         .open("./download/output.txt")
//     {
//         Ok(file) => file,
//         Err(e) => {
//             eprintln!("could not open file \"output.txt\": {}", e);
//             return;
//         }
//     };

//     if only_attachments == true {
//         for i in message_attachments {

//         }
//     } else {
//         let formatted_message: String = format!("{}, \"{}\", {}", message_author, message_content, message_timestamp);
//     }

//     if let Err(e) = writeln!(file, "{formatted_message}") {
//         eprintln!("error writing to file output.txt: {}", e);
//     }
// }
