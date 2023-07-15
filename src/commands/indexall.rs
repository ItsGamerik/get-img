use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::{interaction::application_command::ApplicationCommandInteraction, ChannelId},
        Permissions,
    },
    prelude::Context,
};

use crate::helper_functions::{edit_status_message, status_message};
use crate::commands::index::index_all_messages;

/// function that gets executed when the command is run
pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    status_message(ctx, "starting to index server...", interaction).await;

    index_server(ctx, interaction).await;

    edit_status_message(interaction, ctx, "done indexing server.").await;
}

async fn index_server(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    let guild_channels = match interaction.guild_id.unwrap().channels(&ctx.http).await {
        Ok(channels) => channels,
        Err(e) => {
            eprintln!("could not find any channels to index: {}", e);
            return;
        }
    };

    for i in guild_channels.values() {
        println!("found channel: {}", i);
        let channel_id: ChannelId = i.into();
        get_messages(&ctx, channel_id).await;
    }
}

async fn get_messages(ctx: &Context, channel_id: ChannelId) {
    let one_message = match channel_id
        .messages(&ctx.http, |retriever| retriever.limit(1))
        .await
    {
        Ok(message) => message,
        Err(e) => {
            eprintln!("could not get messaage: {}", e);
            return;
        }
    };
    let last_message = match one_message.last() {
        Some(message_id) => message_id,
        None => {
            eprintln!("no message id could be found");
            return;
        }
    };
    let mut one_message_id = last_message.id;
    loop {
        let messages = match channel_id
            .messages(&ctx.http, |retriever| {
                retriever.before(one_message_id).limit(100)
            })
            .await
        {
            Ok(messages) => messages,
            Err(e) => {
                eprintln!("could not retrieve messages: {}", e);
                return;
            }
        };

        if messages.is_empty() {
            break;
        }

        one_message_id = messages.last().unwrap().id;

        index_all_messages(messages).await;


    }
}

/// function that registers the command with the discord api
/// minimum permission level: ADMINISTRATOR
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("indexall")
        .description("index messages from the entire server")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
