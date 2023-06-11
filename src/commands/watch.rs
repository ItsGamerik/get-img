use std::collections::HashMap;

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::ChannelId;
use serenity::prelude::Context;
// try and use the correct imports :)
use crate::commands::{self};
use tokio::task::{self, JoinHandle};

pub async fn run(
    ctx: &Context,
    command: &ApplicationCommandInteraction,
    watch_map: &mut HashMap<ChannelId, JoinHandle<()>>,
) {
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
    // TODO: use persistent database
    if let Some(toggle) = channel_toggle_keys.get(&channel_id) {
        if *toggle {
            // toggle is "true"
            let ctx = ctx.clone();
            let task = task::spawn(async move {
                background_task(&ctx, &channel_id).await;
            });

            println!(
                "started watching: {} with task handle {:?}",
                &channel_id, &task
            );

            watch_map.insert(channel_id, task);
            println!("added {} to the watchlist", &channel_id)
        } else if !(*toggle) {
            // toggle is "false"
            let handle = match watch_map.get(&channel_id) {
                Some(handle) => handle,
                None => {
                    eprintln!("error whilst getting handle for channel: {}", &channel_id);
                    return;
                }
            };
            handle.abort();
        }
    }

    // command
    //     .create_followup_message(&ctx.http, |response| response.content("did the thing"))
    //     .await
    //     .unwrap();
}

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
                if last_id != latest_message_id {
                    dbg!(&latest_message.content);
                    commands::index::parse(latest_message.content.to_string()).await;
                }
            }

            last_message_id = Some(latest_message_id);
        }

        // TODO: execute every time a new message is sent
        tokio::time::sleep(std::time::Duration::from_secs(3)).await;
    }
}

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
