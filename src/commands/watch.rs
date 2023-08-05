use std::fs::OpenOptions;
use std::io::Write;

use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use serde::{Deserialize, Serialize};
use serenity::model::application::interaction::application_command::ApplicationCommandInteraction;
use serenity::model::prelude::command::CommandOptionType;
use serenity::model::prelude::interaction::application_command::CommandDataOptionValue;
use serenity::model::prelude::{ChannelId, PartialChannel, UserId};
use serenity::prelude::Context;
use serenity::{builder::CreateApplicationCommand, model::Permissions};

use crate::helper_functions::{status_message, edit_status_message};

#[derive(Debug, Serialize, Deserialize)]
pub struct WatcherEntry {
    pub id: ChannelId,
    pub creator: UserId,
}

/// function that gets executed when the command is run
pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    // get the command options etcetc
    let option_channel = interaction
        .data
        .options
        .get(0)
        .expect("expected user option")
        .resolved
        .as_ref()
        .expect("expected user object");
    let option_bool = interaction
        .data
        .options
        .get(1)
        .expect("expected user input thingy")
        .resolved
        .as_ref()
        .expect("expected user object");

    // create channel
    let mut watch_track_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("./watchers")
    {
        Ok(file) => file,
        Err(e) => {
            eprintln!("could not open file \"watchers\": {}", e);
            return;
        }
    };

    status_message(ctx, "creating channel watcher...", interaction).await;

    let watch_channel_id = match option_channel {
        CommandDataOptionValue::Channel(channel) => Some(channel),
        _ => None,
    };

    if let CommandDataOptionValue::Boolean(true) = option_bool {
        let watcher_entry = WatcherEntry {
            id: watch_channel_id.unwrap().id,
            creator: interaction.user.id,
        };

        let j = serde_json::to_string(&watcher_entry).unwrap();

        if let Err(e) = writeln!(watch_track_file, "{j}") {
            eprintln!("an error occured writing to file: {}", e);
        }
    } else {
        let file = File::open("./watchers").await.unwrap();
        let mut lines = tokio::io::BufReader::new(file).lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            let json: WatcherEntry = serde_json::from_str(&line).unwrap();
            if json.id == watch_channel_id.unwrap().id {
                if let Err(e) = delete_line_from_file("./watchers", watch_channel_id).await {
                    eprintln!("error {e}")
                }
            }
        }
    }

    let success_string = format!("successfully created watcher for <#{}>", watch_channel_id.unwrap().id.0); 
    edit_status_message(ctx, &success_string, interaction).await;
    
}

async fn delete_line_from_file(
    file_path: &str,
    watch_channel_id: Option<&PartialChannel>,
) -> Result<(), Box<dyn std::error::Error>> {
    // Read the file
    let file = File::open(file_path).await?;
    let mut lines = BufReader::new(file).lines();

    // Prepare a new buffer to store the updated content
    let mut new_content = String::new();

    // Process each line
    while let Some(line) = lines.next_line().await? {
        let json: WatcherEntry = serde_json::from_str(&line)?;

        if watch_channel_id.is_some() && json.id == watch_channel_id.unwrap().id {
            println!("Found channel watcher! (ID: {})", json.id);
            // Skip the line by not adding it to the new_content
        } else {
            // Keep the line and add it to the new_content
            new_content.push_str(&line);
            new_content.push('\n'); // Add the newline character that was removed by `lines.next_line()`
        }
    }

    // Reopen the file for writing
    let mut file = File::create(file_path).await?;

    // Write the new content back to the file
    file.write_all(new_content.as_bytes()).await?;

    Ok(())
}
/// function that registers the command with the discord api
/// minimum permission level: ADMINISTRATOR
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("watch")
        .description("watch a channel")
        .create_option(|option| {
            option
                .name("id")
                .description("a channel id to watch")
                .kind(CommandOptionType::Channel)
                .required(true)
        })
        .create_option(|option| {
            option
                .name("toggle")
                .description("toggle watching on and off")
                .kind(CommandOptionType::Boolean)
                .required(true)
        })
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
