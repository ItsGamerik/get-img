use serenity::{
    builder::CreateApplicationCommand,
    model::{prelude::{interaction::application_command::{CommandDataOption, CommandDataOptionValue}}},
};

pub fn run(options: &[CommandDataOption]) -> String {
    let option = options.get(0).expect("option").resolved.as_ref().expect("user object");

    if let CommandDataOptionValue::Channel(channel) = option {
        format!("der ausgewählte kanal ist: {}", channel.name.as_ref().unwrap())
    } else {
        "please gönn channel id".to_string()
    }
}

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("index")
        .description("index slash command")
        .create_option(|option| {
            option
                .name("channel id")
                .description("channel id of target channel")
                .kind(serenity::model::prelude::command::CommandOptionType::Channel)
                .required(true)
        })
}
