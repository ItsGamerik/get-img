use std::fs::{self, OpenOptions};
use std::io::Write;

use crate::helper_functions::{edit_status_message, status_message, universal_parser};

use serenity::model::prelude::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::Message;
use serenity::model::Permissions;
use serenity::{
    builder::CreateApplicationCommand,
    model::prelude::{interaction::application_command::CommandDataOptionValue, PartialChannel},
    prelude::Context,
};

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

    if let CommandDataOptionValue::Channel(channel) = option {
        let response_string = format!(
            "added messages from channel **{}** to index",
            channel.name.as_ref().unwrap()
        );
        status_message(ctx, "indexing channel...", interaction).await;

        index(ctx, channel).await;

        edit_status_message(ctx, &response_string, interaction).await;
    } else {
        status_message(ctx, "an error occured", interaction).await;
    }
}

/// used to index the selected channel depending on wether or not "only_images" is true or false
async fn index(ctx: &Context, channel: &PartialChannel) {
    let message_vector = channel
        .id
        .messages(&ctx, |retriever| retriever.limit(1))
        .await
        .expect("could not retrieve message");
    let single_message: &Message = message_vector.last().unwrap();
    let mut single_message_id = single_message.id;
    loop {
        let messages = channel
            .id
            .messages(&ctx, |retriever| {
                retriever.before(single_message_id).limit(100)
            })
            .await
            .expect("Failed to retrieve messages");

        if messages.is_empty() {
            break;
        }

        // try to iterate "upwards" in the channel
        single_message_id = messages.last().unwrap().id;
        index_all_messages(messages).await;
    }
}

/// dont index attachments
pub async fn index_all_messages(messages: Vec<Message>) {
    if let Err(why) = fs::create_dir_all("./download/") {
        eprintln!("error creating file: {}", why);
    }
    for message in messages {
        universal_parser(message).await;
    }
}

/// DO NOT USE
pub async fn parse(content: String) {
    if let Err(why) = fs::create_dir_all("./download/") {
        eprintln!("error creating file: {}", why);
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("./download/output.txt")
        .unwrap();
    if let Err(why) = writeln!(file, "{content}") {
        eprintln!("error while writing to file: {}", why);
    };
}

/// function that registers the command with the discord api
/// minimum permission level: ADMINISTRATOR
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("index")
        .description("index images or messages in channel")
        .create_option(|option| {
            option
                .name("id") // MAN KANN NICHT EIN LEERZEICHEN BENUTZEN
                .description("channel id of target channel")
                .kind(serenity::model::prelude::command::CommandOptionType::Channel)
                .required(true)
        })
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
