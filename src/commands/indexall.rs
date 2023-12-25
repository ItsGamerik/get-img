use log::{error, info, warn};
use serenity::all::{
    ChannelId, CommandInteraction, Context, CreateCommand, GetMessages, Permissions,
};

use crate::commands::index::index_all_messages;
use crate::helper_functions::*;

pub async fn run(ctx: Context, interaction: CommandInteraction) {
    info!(
        "starting to index server {}, this may take a long time.",
        interaction.guild_id.unwrap()
    );
    status_message(
        &ctx,
        "starting to index entire server, this might take a long time",
        &interaction,
    )
    .await;

    index_server(&ctx, &interaction).await;

    info!("done indexing server.");
    edit_status_message(&ctx, "done indexing server.", &interaction).await;
}

async fn index_server(ctx: &Context, interaction: &CommandInteraction) {
    let guild_channels = match interaction.guild_id.unwrap().channels(&ctx.http).await {
        Ok(channels) => channels,
        Err(e) => {
            error!("could not find any channels to index: {e}");
            return;
        }
    };

    for gchannel in guild_channels.values() {
        if gchannel.is_text_based() {
            info!("found channel: {}", gchannel.name);
            let channel_id: ChannelId = gchannel.into();
            get_messages(ctx, channel_id).await;
        } else {
            info!(
                "skipping  \"{}\" because it is not a text channel",
                gchannel.name
            )
        }
    }
}

async fn get_messages(ctx: &Context, channel_id: ChannelId) {
    let one_message = match channel_id.messages(&ctx, GetMessages::new().limit(1)).await {
        Ok(message) => message,
        Err(e) => {
            error!("could not get messages: {e}");
            return;
        }
    };

    let last_message = match one_message.last() {
        Some(message_id) => message_id,
        None => {
            warn!("no message id could be found!");
            return;
        }
    };

    let mut one_message_id = last_message.id;

    loop {
        let messages = match channel_id
            .messages(&ctx, GetMessages::new().limit(100).before(one_message_id))
            .await
        {
            Ok(messages) => messages,
            Err(e) => {
                error!("could not retrieve messages: {e}");
                return;
            }
        };

        if messages.is_empty() {
            break;
        }

        one_message_id = messages.last().unwrap().id;

        index_all_messages(messages, ctx).await;
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("indexall")
        .name("indexall")
        .description("index messages from the ENTIRE server")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
