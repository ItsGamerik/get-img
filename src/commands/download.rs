use std::path::PathBuf;
use std::sync::Arc;

use crate::config::config_functions::CONFIG;
use log::{error, info};
use regex::Regex;
use serenity::all::CommandOptionType::Boolean;
use serenity::all::{
    ActivityData, CommandInteraction, Context, CreateAttachment, CreateCommand,
    CreateCommandOption, CreateInteractionResponseFollowup, OnlineStatus, Permissions,
    ResolvedOption, ResolvedValue,
};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt};
use tokio::sync::Semaphore;
use tokio::{fs, io};

use crate::helper_functions::{followup_status_message, status_message, DiscordMessage};

pub async fn run(ctx: Context, interaction: &CommandInteraction, options: &[ResolvedOption<'_>]) {
    let option_bool: &bool;
    if let Some(ResolvedOption {
        value: ResolvedValue::Boolean(bool),
        ..
    }) = options.first()
    {
        option_bool = bool;
    } else {
        error!("non-boolean recieved!");
        return;
    }

    let lock = CONFIG.lock().await;
    let cfg = lock.get().unwrap();
    let path = &cfg.directories.downloads;
    let path2 = path.clone();
    drop(lock);

    status_message(&ctx, "downloading attachments...", interaction).await;

    if let Ok(meta) = fs::metadata(path2.to_string() + "/output.txt").await {
        if meta.is_file() {
            if *option_bool {
                // case if dltodisk is true
                let output_file = File::open(path2.to_string() + "/output.txt").await.unwrap();
                let attachment = CreateAttachment::file(&output_file, "output.txt")
                    .await
                    .unwrap();
                read_file().await;

                // TODO: remove some of the unwraps
                info!("started to download attachments");
                interaction
                    .create_followup(
                        &ctx.http,
                        CreateInteractionResponseFollowup::new()
                            .content("downloaded to disk!")
                            .add_file(attachment),
                    )
                    .await
                    .unwrap();
                ctx.set_presence(
                    Some(ActivityData::watching("Ready to go :D")),
                    OnlineStatus::Online,
                );
            } else {
                // if it is false
                let output_file = File::open(path2.to_string() + "/output.txt").await.unwrap();
                let attachment = CreateAttachment::file(&output_file, "output.txt")
                    .await
                    .unwrap();
                read_file().await;
                interaction
                    .create_followup(
                        &ctx.http,
                        CreateInteractionResponseFollowup::new()
                            .content("here is the list of messages!")
                            .add_file(attachment),
                    )
                    .await
                    .unwrap();
                ctx.set_presence(
                    Some(ActivityData::watching("Ready to go :D")),
                    OnlineStatus::Online,
                );
            }
        }
    } else {
        followup_status_message(
            &ctx,
            "not indexed yet. Try using `/index` to index first.",
            interaction,
        )
        .await;
    }
}

async fn read_file() {
    let lock = CONFIG.lock().await;
    let cfg = lock.get().unwrap();
    let path = &cfg.directories.downloads;
    let file = match File::open(path.to_string() + "/output.txt").await {
        Ok(f) => f,
        Err(e) => {
            error!("error reading output.txt: {e}");
            return;
        }
    };

    let dl_count = &cfg.downloading.parallel_downloads;
    let sems = Arc::new(Semaphore::new(*dl_count));
    let mut handles = Vec::new();

    let mut lines = io::BufReader::new(file).lines();
    while let Some(line) = lines.next_line().await.unwrap() {
        let json: DiscordMessage = serde_json::from_str(&line).unwrap();
        for link in json.attachments {
            let sem_clone = Arc::clone(&sems);
            let handle = tokio::spawn(async move {
                let _ = sem_clone.acquire().await.unwrap();
                download_file(link).await;
            });
            handles.push(handle);
        }
    }
}

pub async fn download_file(url: String) {
    let lock = CONFIG.lock().await;
    let cfg = lock.get().unwrap();
    let path = &cfg.directories.downloads;
    let client = reqwest::Client::new();
    let response = match client.get(&url).send().await {
        Ok(r) => r,
        Err(e) => {
            error!("could not complete web request: {e}");
            return;
        }
    };
    let file_name = PathBuf::from(&url)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned();

    let re = Regex::new(r"(\?ex=).*?$").unwrap();

    let cleansed_file_name = re.replace(&file_name, "").to_string();

    let root_path = path.to_string() + "/attachments/";
    if fs::metadata(&root_path).await.is_err() {
        match fs::create_dir_all(&root_path).await {
            Ok(_) => info!("created attachment download dir, as it did not exist"),
            Err(e) => {
                error!("could not create download dir: {e}");
                return;
            }
        }
    }

    let mut file_path = PathBuf::from(&root_path).join(&cleansed_file_name);

    let mut index = 0;
    while file_path.exists() {
        index += 1;
        let new_file_name = format!("{}.{}", index, cleansed_file_name);
        file_path.set_file_name(new_file_name);
    }

    let mut file: File = File::create(&file_path).await.unwrap();
    let response_file = response.bytes().await.unwrap().to_vec();
    match file.write_all(&response_file).await {
        Ok(_) => info!("successfully downloaded \"{}\"", file_name),
        Err(e) => error!("error downloading file \"{file_name}\": {e}"),
    }
}

pub fn register() -> CreateCommand {
    CreateCommand::new("download")
        .name("download")
        .description("download the links saved to the output file")
        .add_option(
            CreateCommandOption::new(Boolean, "download_to_disk", "download attachments to disk")
                .required(true),
        )
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
