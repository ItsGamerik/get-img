use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

use log::{error, info};
use serde::{Deserialize, Serialize};
use serenity::all::{
    ChannelId, CommandInteraction, CommandOptionType, Context, CreateCommand, CreateCommandOption,
    PartialChannel, Permissions, ResolvedOption, ResolvedValue, UserId,
};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};

use crate::helper_functions::{edit_status_message, status_message};

#[derive(Debug, Serialize, Deserialize)]
pub struct WatcherEntry {
    pub id: ChannelId,
    pub creator: UserId,
}

pub async fn run(ctx: Context, interaction: &CommandInteraction, options: &[ResolvedOption<'_>]) {
    let option_channel: &&PartialChannel;
    let option_bool: &bool;

    if let Some(ResolvedOption {
        value: ResolvedValue::Channel(channel),
        ..
    }) = options.get(0)
    {
        option_channel = channel
    } else {
        error!("could not parse watcher command options");
        return;
    }

    if let Some(ResolvedOption {
        value: ResolvedValue::Boolean(bool),
        ..
    }) = options.get(1)
    {
        option_bool = bool
    } else {
        error!("could not parse watcher command options");
        return;
    }

    let mut watch_track_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open("./.watchers")
    {
        Ok(file) => file,
        Err(e) => {
            error!("could not open watcherfile: {e}");
            return;
        }
    };

    status_message(&ctx, "creating/updating channel watcher...", interaction).await;

    if let &true = option_bool {
        let watcher_entry = WatcherEntry {
            id: option_channel.id,
            creator: interaction.user.id,
        };

        let json = serde_json::to_string(&watcher_entry).unwrap();

        if let Err(e) = writeln!(watch_track_file, "{json}") {
            error!("an error occurred writing to file: {e}");
        }
    } else {
        let file = File::open("./.watchers").await.unwrap();
        let mut lines = BufReader::new(file).lines();
        while let Some(line) = lines.next_line().await.unwrap() {
            let json: WatcherEntry = serde_json::from_str(&line).unwrap();
            if json.id == option_channel.id {
                if let Err(e) = delete_line_from_file("./.watchers", option_channel.id).await {
                    error!("an error occurred writing to file: {e}")
                }
            }
        }
    }

    let update_msg = format!(
        "created/updated watcher for channel **{}**!",
        option_channel.name.as_ref().unwrap()
    );

    edit_status_message(&ctx, &update_msg, interaction).await;
}

async fn delete_line_from_file(
    file_path: &str,
    watch_channel_id: ChannelId,
) -> Result<(), Box<(dyn Error)>> {
    let file = File::open(file_path).await?;
    let mut lines = BufReader::new(file).lines();

    let mut new_content = String::new();

    while let Some(line) = lines.next_line().await? {
        let json: WatcherEntry = serde_json::from_str(&line)?;

        if json.id == watch_channel_id {
            info!("Found channel watcher! (ID: {})", json.id);
        } else {
            new_content.push_str(&line);
            new_content.push('\n');
        }
    }

    let mut file = File::create(file_path).await?;

    file.write_all(new_content.as_bytes()).await?;
    Ok(())
}

pub fn register() -> CreateCommand {
    CreateCommand::new("watch")
        .name("watch")
        .description("watch a channel")
        .set_options(vec![
            CreateCommandOption::new(CommandOptionType::Channel, "id", "a channel id")
                .required(true),
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "toggle",
                "toggle watcher on and off",
            )
            .required(true),
        ])
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
