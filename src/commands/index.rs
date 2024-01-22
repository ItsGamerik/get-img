use std::fs;

use crate::config::config_functions::CONFIG;
use log::{error, info, warn};
use serenity::all::{
    CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    GetMessages, Message, PartialChannel, Permissions, ResolvedOption, ResolvedValue,
};

use crate::helper_functions::*;

pub async fn run(ctx: Context, interaction: &CommandInteraction, options: &[ResolvedOption<'_>]) {
    // this part already does parsing & evaluation of args, no need for CommandDataOptionValue
    // but this does feel wierd
    let target_confirmed_message: String;
    let target_channel: &&PartialChannel;

    if let Some(ResolvedOption {
        value: ResolvedValue::Channel(channel),
        ..
    }) = options.first()
    {
        info!(
            "trying to parse channel: {}",
            channel.name.as_ref().unwrap()
        );
        target_channel = channel;
        target_confirmed_message = format!(
            "added messages from channel **{}** to index",
            target_channel.name.as_ref().unwrap()
        );
    } else {
        error!("non-channel recieved!");
        return;
    }

    status_message(&ctx, "indexing channel...", interaction).await;

    index(&ctx, target_channel).await;

    edit_status_message(&ctx, &target_confirmed_message, interaction).await;
}

async fn index(ctx: &Context, channel: &&PartialChannel) {
    // TODO: add check to see if there is a thread that is NOT a message
    let message_vector = match channel.id.messages(&ctx, GetMessages::new().limit(1)).await {
        Ok(m) => m,
        Err(e) => {
            error!("an error occurred retrieving messages: {e}");
            return;
        }
    };
    let single_message = message_vector.last().unwrap();

    let mut single_message_id = single_message.id;

    loop {
        let messages = match channel
            .id
            .messages(
                &ctx,
                GetMessages::new().limit(100).before(single_message_id),
            )
            .await
        {
            Ok(messages) => messages,
            Err(e) => {
                error!("an error occurred while retrieving messages: {e}");
                return;
            }
        };
        if messages.is_empty() {
            break;
        }

        // iterate "upwards"
        single_message_id = messages.last().unwrap().id;
        index_all_messages(messages, ctx).await;
    }
}

pub async fn index_all_messages(messages: Vec<Message>, ctx: &Context) {
    let lock = CONFIG.lock().await;
    let cfg = lock.get().unwrap();
    let path = &cfg.directories.downloads;
    if let Err(e) = fs::create_dir_all(path) {
        error!("error creating directory: {e}")
    }

    for message in messages {
        if let Some(thread) = &message.thread {
            let last_thread_message_id = match thread.last_message_id {
                Some(message_id) => {
                    info!(
                        "found message ({}) in thread \"{}\" ({})",
                        message.id, thread.name, thread.id
                    );
                    message_id
                }
                None => {
                    warn!(
                        "thread {} ({}) seems to be empty. skipping.",
                        thread.name, thread.id
                    );
                    return;
                }
            };

            // i cba typing "message" anymore
            let thread_to_message = match thread.message(&ctx, last_thread_message_id).await {
                Ok(msg) => msg,
                Err(e) => {
                    error!("an error occurred getting message: {e}");
                    return;
                }
            };

            let msgs_in_thread = thread_to_message
                .channel(&ctx.http)
                .await
                .unwrap()
                .id()
                .messages(
                    &ctx.http,
                    GetMessages::new().limit(1).before(thread_to_message.id),
                )
                .await
                .unwrap();

            let single_message: &Message = msgs_in_thread.last().unwrap();

            let mut single_message_id = single_message.id;

            loop {
                let messages = match thread_to_message
                    .channel(&ctx.http)
                    .await
                    .unwrap()
                    .id()
                    .messages(
                        &ctx,
                        GetMessages::new().before(single_message_id).limit(100),
                    )
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
                    universal_message_writer(message).await;
                }
            }
        }
        universal_message_writer(message).await;
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("index")
        .name("index")
        .description("index images or messages in channel")
        .add_option(
            CreateCommandOption::new(
                CommandOptionType::Channel,
                "id",
                "channel id of target channel",
            )
            .required(true),
        )
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
