use std::error::Error;
use std::fs::OpenOptions;
use std::io::Write;

use crate::config::config_functions::CONFIG;
use log::{error, info, warn};
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
    pub autodl: bool,
}

pub async fn run(ctx: Context, interaction: &CommandInteraction, options: &[ResolvedOption<'_>]) {
    let option_channel: &&PartialChannel;
    let option_toggle: &bool;
    let option_dl: &bool;

    if let Some(ResolvedOption {
        value: ResolvedValue::Channel(channel),
        ..
    }) = options.first()
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
        option_toggle = bool
    } else {
        error!("could not parse watcher command options");
        return;
    }

    if let Some(ResolvedOption {
        value: ResolvedValue::Boolean(bool),
        ..
    }) = options.get(2)
    {
        option_dl = bool
    } else {
        option_dl = &false;
        info!("defaulting to false for watcher")
    }

    let lock = CONFIG.lock().await;
    let cfg = lock.get().unwrap();
    let path = &cfg.directories.watchfile;

    let mut watch_track_file = match OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(path.to_string() + ".watchers")
    {
        Ok(file) => file,
        Err(e) => {
            error!("could not open watcherfile: {e}");
            return;
        }
    };

    status_message(&ctx, "creating/updating channel watcher...", interaction).await;

    let mut file2 = match File::open(path.to_string() + ".watchers").await {
        Ok(f) => f,
        Err(e) => {
            warn!("watcher file not found, not indexing message ({e})");
            return;
        }
    };

    let mut lines = BufReader::new(&mut file2).lines();
    while let Some(line) = lines.next_line().await.unwrap() {
        let json: WatcherEntry = match serde_json::from_str(&line) {
            Ok(entry) => entry,
            Err(_) => {
                info!("watch file does not exist yet");
                return;
            }
        };
        // note: using format! is apparently slower than using .as_str etc.
        if json.id == option_channel.id {
            delete_line_from_file(&format!("{path}.watchers"), option_channel.id)
                .await
                .unwrap();
        }
    }

    match (option_toggle, option_dl) {
        (true, true) => {
            let watcher_entry = WatcherEntry {
                id: option_channel.id,
                creator: interaction.user.id,
                autodl: true,
            };
            let json = serde_json::to_string(&watcher_entry).unwrap();
            if let Err(e) = writeln!(watch_track_file, "{json}") {
                error!("an error occurred writing to file: {e}");
            }
        }
        (true, false) => {
            let watcher_entry = WatcherEntry {
                id: option_channel.id,
                creator: interaction.user.id,
                autodl: false,
            };
            let json = serde_json::to_string(&watcher_entry).unwrap();
            if let Err(e) = writeln!(watch_track_file, "{json}") {
                error!("an error occurred writing to file: {e}");
            }
        }
        _ => {
            let mut lines = BufReader::new(file2).lines();
            while let Some(line) = lines.next_line().await.unwrap() {
                let json: WatcherEntry = serde_json::from_str(&line).unwrap();
                if json.id == option_channel.id {
                    if let Err(e) = delete_line_from_file((path.to_string() + ".watchers").as_str(), option_channel.id).await {
                        error!("an error occurred writing to file: {e}")
                    }
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
            CreateCommandOption::new(
                CommandOptionType::Boolean,
                "autodownload",
                "automatically download attachments",
            )
            .required(false),
        ])
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
