use serenity::{
    builder::CreateApplicationCommand,
    model::{prelude::interaction::application_command::ApplicationCommandInteraction, Permissions},
    prelude::Context,
};

use crate::helper_functions::status_message;

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

}

/// function that registers the command with the discord api
/// minimum permission level: ADMINISTRATOR
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
    .name("indexall")
    .description("index messages from the entire server")
    .create_option(|option| {
        option
            .name("only_images")
            .description("wether or not to index every message instead of simply the images")
            .kind(serenity::model::prelude::command::CommandOptionType::Boolean)
            .required(true)
    })
    .default_member_permissions(Permissions::ADMINISTRATOR)
}
