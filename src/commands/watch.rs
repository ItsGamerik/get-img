use std::collections::HashMap;

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;
// try and use the correct imports :)
use crate::commands::{self};
use tokio::task::{self};

static mut BACKGROUND_TASK: Option<task::JoinHandle<()>> = None;

/// function that gets executed when the command is run
pub async fn run(ctx: &Context, command: &ApplicationCommandInteraction) {
    // get the command options etcetc
    let option_channel = command
        .data
        .options
        .get(0)
        .expect("expected user option")
        .resolved
        .as_ref()
        .expect("expected user object");
    let option_bool = command
        .data
        .options
        .get(1)
        .expect("expected user input thingy")
        .resolved
        .as_ref()
        .expect("expected user object");

    // toggle watch on and off

    let mut channel_toggle_keys: HashMap<serenity::model::prelude::ChannelId, bool> =
        HashMap::new();
    let channel_id = command.channel_id;

    if let (CommandDataOptionValue::Boolean(value), CommandDataOptionValue::Channel(_channel)) =
        (option_bool, option_channel)
    {
        channel_toggle_keys.insert(channel_id, *value);
    }
    // thanks to the rust discord :D

    // watch channel
    if let Some(toggle) = channel_toggle_keys.get(&channel_id) {
        if *toggle {
            let channel_id_format = format!("{}{}{}", "<", &channel_id, ">");
            // toggle is "true"
            unsafe {
                if BACKGROUND_TASK.is_some() {
                    command
                        .create_interaction_response(&ctx.http, |message| {
                            message.interaction_response_data(|content| {
                                content.content(
                                    "could not create watcher: stop watching other channel first",
                                )
                            })
                        })
                        .await
                        .unwrap();
                    return;
                }
            }
            let ctx = ctx.clone();
            command
                .create_interaction_response(&ctx.http, |response| {
                    response.interaction_response_data(|messsage| {
                        messsage.content(format!(
                            "creating channel watcher for {}",
                            channel_id_format
                        ))
                    })
                })
                .await
                .unwrap();
            let task = task::spawn(async move {
                background_task(&ctx, &channel_id).await;
            });
            // wtf
            unsafe {
                BACKGROUND_TASK = Some(task);
            }

            println!("started watching: {}", &channel_id);
        } else if !(*toggle) {
            // toggle is "false"
            unsafe {
                if let Some(task) = &BACKGROUND_TASK {
                    task.abort();
                    BACKGROUND_TASK = None;
                }
            }
            command
                .create_interaction_response(&ctx.http, |response| {
                    response.interaction_response_data(|message| {
                        message.content("stopped watching channel")
                    })
                })
                .await
                .unwrap();
        }
    }
}

/// background task for keeping track of selected channel.
/// will run i a separate thread
async fn background_task(ctx: &Context, channel_id: &ChannelId) {
    let mut last_message_id: Option<u64> = None;
    loop {
        let messages = channel_id
            .messages(&ctx.http, |retriever| retriever.limit(1))
            .await
            .expect("could not retrieve messages");

        if let Some(latest_message) = messages.first() {
            let latest_message_id = latest_message.id.0;

            if let Some(last_id) = last_message_id {
                if last_id != latest_message_id && !latest_message.author.bot {
                    if latest_message.attachments.iter().any(|a| !a.url.is_empty()) {
                        let mut attachment_vec = Vec::new();
                        for attachment in &latest_message.attachments {
                            attachment_vec.push(attachment);
                        }
                        for i in attachment_vec
                            .iter()
                            .map(|attachment| attachment.url.clone())
                        {
                            commands::index::parse(i).await;
                        }
                    } else {
                        commands::index::parse(latest_message.content.to_string()).await;
                    }
                }
            }

            last_message_id = Some(latest_message_id);
        }

        // TODO: execute every time a new message is sent
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

/// function that registers the command with the discord api
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("watch")
        .description("watch a channel")
        .create_option(|option| {
            option
                .name("id")
                .description("a channel id to watch")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("toggle")
                .description("toggle watching on and off")
                .kind(CommandOptionType::Boolean)
                .required(true)
        })
}
