use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context, utils::Colour,
};

/// function that gets executed when the command is run
pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    interaction
        .create_interaction_response(&ctx.http, |repsonse| {
            repsonse.interaction_response_data(|data| {
                data.embed(|embed| {
                    embed
                        .title("help")
                        .description("list of available commands")
                        .field("test", "test", true)
                        .url("https://github.com/ItsGamerik/get-img#commands")
                        .color(Colour::DARK_GREEN)
                })
            })
        })
        .await
        .unwrap();
}

/// function that registers the command with the discord api
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("help").description("help command")
}
