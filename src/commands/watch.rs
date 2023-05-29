use std::collections::HashMap;

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::{ChannelId, Message};
use serenity::prelude::Context;
use tokio::task;

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
        if *toggle == true {
            let ctx = ctx.clone();
            let channel_id = channel_id.clone();

            task::spawn(async move {
                background_task(&ctx, &channel_id, channel_toggle_keys).await;
            });
        }
    }
}

async fn background_task(ctx: &Context, channel_id: &ChannelId, togglemap: HashMap<ChannelId, bool>) {
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
                }
            }

            last_message_id = Some(latest_message_id);
        }

        // Wait for some time before checking for new messages again
        tokio::time::sleep(std::time::Duration::from_secs(10)).await;
    }
}

async fn channel_watcher(message: Vec<Message>, ctx: &Context) {
    let latest_message = message.get(0).unwrap();
    let latest_id = latest_message.id;

    // Retrieve the last message sent in the channel
    let channel_id = latest_message.channel_id;
    let messages = channel_id
        .messages(&ctx.http, |retriever| retriever.before(latest_id).limit(1))
        .await
        .expect("could not retrieve messages");
    let last_message = messages.get(0);

    if let Some(last_message) = last_message {
        // Compare the content of the new message with the last message
        if latest_message.id != last_message.id {
            // Messages are the same, perform your action here
            dbg!(&latest_message.content);
        }
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
