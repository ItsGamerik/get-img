use serenity::{
    builder::CreateApplicationCommand,
    futures::StreamExt,
    model::prelude::{
        interaction::{
            application_command::{CommandDataOption, CommandDataOptionValue},
        }, PartialChannel,
    },
    prelude::Context,
};

pub async fn run(options: &[CommandDataOption], ctx: &Context) -> String {
    // get option
    let option = options
        .get(0)
        .expect("option")
        .resolved
        .as_ref()
        .expect("user object");

    // response logic
    let mut response = String::new();
    if let CommandDataOptionValue::Channel(channel) = option {
        response = format!(
            "der ausgewählte kanal ist: {}",
            channel.name.as_ref().unwrap()
        );
        message_index(&ctx, channel).await;
    } else {
        response = "no channel id given".to_string();
    }
    response
}

async fn message_index(ctx: &Context, channel: &PartialChannel) {
    loop {
        let mut messages = channel.id.messages_iter(&ctx).boxed();
        while let Some(message_result) = messages.next().await {
            match message_result {
                Ok(message) => println!(
                    "message indexed: id {}, timestamp {}",
                    message.id, message.timestamp
                ),
                Err(error) => eprintln!("error XD: {}", error),
            }
        
        }
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
