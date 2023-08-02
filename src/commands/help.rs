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
                        .description("## list of available commands")
                        .field("`/help`", "shows this message", false)
                        .field("`/index`", "index every message with an attachment in a channel.", false)
                        .field("`/download`", "sends the index file into the discord channel, and will download attachments if specified.", false)
                        .field("`/watch`", "toggles the automatic indexing for a single channel on and off, can only be ON for one channel at a time.", false)
                        .field("`/indexall`", "index all messages of the server where the interaction was sent. Due to API limitations, this can take quite a long time, especially for larger servers. Progress is indicated by the bot's status.", false)
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
