use std::env;

use rand::seq::index;
use serenity::{
    builder::CreateApplicationCommand,
    model::{prelude::{interaction::application_command::ApplicationCommandInteraction, GuildId}, Permissions, guild},
    prelude::Context, futures::TryFutureExt,
};

use crate::helper_functions::{status_message, edit_status_message};

/// function that gets executed when the command is run
pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {

    status_message(ctx, "starting to index server...", interaction).await;

    index_server(ctx, interaction).await;

    edit_status_message(interaction, ctx, "done indexing server.").await;

}

async fn index_server(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    let guild_id: u64 = interaction.guild_id.unwrap().into();

    let channels = serenity::model::prelude::GuildId(guild_id).channels(&ctx.http).await.unwrap();
    dbg!(channels);


}


/// function that registers the command with the discord api
/// minimum permission level: ADMINISTRATOR
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
    .name("indexall")
    .description("index messages from the entire server")
    .default_member_permissions(Permissions::ADMINISTRATOR)
}
