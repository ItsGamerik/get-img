use std::collections::HashMap;
use tokio::task::spawn;

use serenity::builder::CreateApplicationCommand;
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::ChannelId;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::prelude::Context;

pub async fn run(_ctx: &Context, command: &ApplicationCommandInteraction) {
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

    // if let CommandDataOptionValue::Boolean(true) = option_bool {
    //     if let CommandDataOptionValue::Channel(_channel) = option_channel {
    //         channel_toggle_keys.insert(channel_id, true);
    //     }
    // } else if let CommandDataOptionValue::Boolean(false) = option_bool {
    //     if let CommandDataOptionValue::Channel(_channel) = option_channel {
    //         channel_toggle_keys.insert(channel_id, false);
    //     }
    // }
    if let (CommandDataOptionValue::Boolean(value), CommandDataOptionValue::Channel(_channel)) =
        (option_bool, option_channel)
    {
        channel_toggle_keys.insert(channel_id, *value);
    }
    // thanks to the rust discord :D

    // watch channel

    if let Some(toggle) = channel_toggle_keys.get(&channel_id) {
        if toggle.to_owned() == true {
            spawn(channel_watcher(channel_id));
        } else {
            return;
        }
    }
}

async fn channel_watcher(channel_id: ChannelId) {
    
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
