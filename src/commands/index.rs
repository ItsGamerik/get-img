use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::helper_functions::{edit_status_message, status_message, universal_parser};

use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::Message;
use serenity::model::Permissions;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{interaction::application_command::CommandDataOptionValue, PartialChannel},
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

        index(ctx, channel).await;

        edit_status_message(ctx, &response_string, interaction).await;
    } else {
        status_message(ctx, "an error occured, check logs", interaction).await;
    }
}

/// used to index the selected channel
async fn index(ctx: &Context, channel: &PartialChannel) {
    let message_vector = channel
        .id
        .messages(&ctx, |retriever| retriever.limit(1))
        .await
        .expect("could not retrieve message");
    let single_message: &Message = message_vector.last().unwrap();
    let mut single_message_id = single_message.id;
    loop {
        let messages = match channel
            .id
            .messages(&ctx, |retriever| {
                retriever.before(single_message_id).limit(100)
            })
            .await
        {
            Ok(messages) => messages,
            Err(e) => {
                eprintln!("an error occured while retrieving messages: {}", e);
                return;
            }
        };

        if messages.is_empty() {
            break;
        }

        // try to iterate "upwards" in the channel
        single_message_id = messages.last().unwrap().id;
        index_all_messages(messages, ctx).await;
    }
}

/// index all messages and parse them
pub async fn index_all_messages(messages: Vec<Message>, ctx: &Context) {
    if let Err(why) = fs::create_dir_all("./download/") {
        eprintln!("error creating file: {}", why);
    }

    // check if it is a thread
    for message in messages {
        if let Some(thread) = &message.thread {
            let thread_last_message_id = match thread.last_message_id {
                Some(messageid) => {
                    println!("found message ({}) in thread: {}", messageid, thread.id);
                    messageid
                }
                None => {
                    eprintln!("no message could be found for thread {}", thread.id);
                    return;
                }
            };

            let thread_to_message = match thread.message(&ctx.http, thread_last_message_id).await {
                Ok(message) => message,
                Err(e) => {
                    eprintln!("an error occured trying to get message: {}", e);
                    return;
                }
            };

            // you can treat threads kinda like channels
            let messages_in_thread = thread_to_message
                .channel(&ctx.http)
                .await
                .unwrap()
                .id()
                .messages(&ctx.http, |builder| {
                    builder.limit(1).before(thread_to_message.id)
                })
                .await
                .unwrap();

            let single_message: &Message = messages_in_thread.last().unwrap();
            let mut single_message_id = single_message.id;
            loop {
                let messages = match thread_to_message
                    .channel(&ctx.http)
                    .await
                    .unwrap()
                    .id()
                    .messages(&ctx, |retriever| {
                        retriever.before(single_message_id).limit(100)
                    })
                    .await
                {
                    Ok(messages) => messages,
                    Err(e) => {
                        eprintln!("an error occured while retrieving messages: {}", e);
                        return;
                    }
                };

                if messages.is_empty() {
                    break;
                }

                single_message_id = messages.last().unwrap().id;
                for message in messages {
                    universal_parser(message).await;
                }
            }
        }
        universal_parser(message).await;
    }
}

/// DO NOT USE
#[deprecated(
    since = "1.1.6",
    note = "use the \"universal_parser()\" function instead"
)]
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
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
