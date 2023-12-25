use serenity::all::{
    Colour, CommandInteraction, CreateEmbed, CreateInteractionResponse,
    CreateInteractionResponseMessage,
};
use serenity::builder::CreateCommand;
use serenity::prelude::Context;

pub async fn run(ctx: Context, interaction: &CommandInteraction) {
    let embed = CreateEmbed::new().title("help").description("## list of available commands")
        .field("`/help`", "shows this message", false)
        .field("`/index`", "index every message with an attachment in a channel", false)
        .field("`/download`", "sends the index file into the discord channel, and will download attachments if specified.", false)
        .field("`/watch`", "toggles the automatic indexing for the specified channel on and off.", false)
        .field("`/indexall`", "index all messages of the server where the interaction was sent. Due to API limitations, this can take quite a long time, especially for larger servers. Progress is indicated by the bot's status.", false)
        .url("https://github.com/ItsGamerik/get-img#commands").color(Colour::DARK_GREEN);

    let builder = CreateInteractionResponse::Message(
        CreateInteractionResponseMessage::new().add_embed(embed),
    );

    interaction
        .create_response(ctx.http, builder)
        .await
        .unwrap();
}
pub fn register() -> CreateCommand {
    CreateCommand::new("help").description("help command for get-img bot")
}
