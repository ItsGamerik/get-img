use serenity::builder::CreateApplicationCommand;

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command.name("index").description("index slash command")
} // basic command desc