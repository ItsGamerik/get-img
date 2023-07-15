use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

use crate::helper_functions::status_message;

/// function that gets executed when the command is run
pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {

}

/// function that registers the command with the discord api
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("indexall").description("index all messages in a server")
}
