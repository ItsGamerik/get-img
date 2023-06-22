use rand::Rng;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::{
        application_command::ApplicationCommandInteraction, InteractionResponseType,
    },
    prelude::Context,
};

/// function that gets executed when the command is run
pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    let greetings = [
        "Hallo",
        "Hi",
        "Salut",
        "¿Qué tal?",
        "привіт",
        "Nǐ hǎo",
        "Salve",
        "Konnichiwa",
        "Oi",
        "Anyoung",
        "Halløj",
        "Hujambo",
        "Hoi",
        "Yassou",
        "Dzień dobry",
        "Selamat siang",
        "Namaste",
        "Selam",
        "Shalom",
        "Tjena",
        "Hei",
        "As-salamu alaykum",
    ];

    let rng = rand::thread_rng().gen_range(0..greetings.len() - 1); // BE VERY CAREFUL TO USE CORRECT ARRAY LEN

    let hello = greetings[rng].to_string();
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| data.content(hello))
        })
        .await
        .unwrap();
}

/// function that registers the command with the discord api
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("hello").description("hallo sagen jtz")
}
