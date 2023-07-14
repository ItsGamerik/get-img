use rand::Rng;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::interaction::application_command::ApplicationCommandInteraction,
    prelude::Context,
};

use crate::helper_functions::status_message;

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

    let hello = greetings[rng];

    status_message(ctx, hello, interaction).await;
}

/// function that registers the command with the discord api
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("hello").description("hallo sagen jtz")
}
