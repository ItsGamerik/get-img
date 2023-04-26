use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{
        command::{CommandOptionType},
        interaction::application_command::CommandDataOption,
    },
    prelude::Context,
};

pub async fn run(options: &[CommandDataOption], ctx: &Context) -> String {
    // let option = options
    //     .get(0)
    //     .expect("user option expected")
    //     .resolved
    //     .as_ref()
    //     .expect("reference");

    // combine channel id and message id to retrieve message

    "XD".to_string()
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("info")
        .description("get message information")
        .create_option(|option| {
            option
                .name("id") // MAN KANN NICHT EIN LEERZEICHEN BENUTZEN
                .description("a message id")
                .kind(CommandOptionType::Number)
                .required(true)
        })
}
