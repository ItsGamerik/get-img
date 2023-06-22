use std::path::{Path, PathBuf};

use reqwest::Client;
use tokio::{
    fs::{self, File},
    io::{self, AsyncBufReadExt, AsyncWriteExt},
};

use serenity::{
    builder::CreateApplicationCommand,
    futures::TryFutureExt,
    model::{
        prelude::{
            interaction::{
                application_command::{ApplicationCommandInteraction, CommandDataOptionValue},
                InteractionResponseType,
            },
            AttachmentType,
        },
        Permissions,
    },
    prelude::Context,
};

/// function that gets executed when the command is run
pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    let command_option = interaction
        .data
        .options
        .get(0)
        .expect("expected option")
        .resolved
        .as_ref()
        .expect("expected user object");

    let path = Path::new("./download/output.txt");
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::DeferredChannelMessageWithSource)
                .interaction_response_data(|data| data.content("downloading attachments..."))
        })
        .await
        .unwrap();
    if let Ok(meta) = fs::metadata(path).await {
        if meta.is_file() {
            read_file().await;
            if let CommandDataOptionValue::Boolean(true) = command_option {
                interaction
                    .create_followup_message(&ctx.http, |response| {
                        response
                            .content("downloaded attachments!")
                            .add_file(AttachmentType::Path(path))
                    })
                    .await
                    .unwrap();
                fs::remove_file("./download/output.txt")
                    .unwrap_or_else(|_| ())
                    .await;
            } else {
                interaction
                    .create_followup_message(&ctx.http, |response| {
                        response.content("downloaded attachments!")
                    })
                    .await
                    .unwrap();
                fs::remove_file("./download/output.txt")
                    .unwrap_or_else(|_| ())
                    .await;
            }
            println!("done downloading files from output.txt file.");
        } else {
            interaction
                .create_followup_message(&ctx.http, |response| {
                    response.content("An error occured, contact the dev to check logs.")
                })
                .await
                .unwrap();
        }
    } else {
        interaction
            .create_followup_message(&ctx.http, |response| {
                response.content("Not yet indexed. Try using `/index` first.")
            })
            .await
            .unwrap();
    }
}

/// read the urls from the file "output.txt" for them to be downloaded
async fn read_file() {
    let file = File::open("./download/output.txt").await.unwrap();
    let mut lines = io::BufReader::new(file).lines();
    let search_string = "cdn.discordapp.com";
    while let Some(line) = lines.next_line().await.unwrap() {
        if line.contains(search_string) {
            println!("download: {}", line);
            download_file(line).await;
        }
    }
}

/// use Reqwest crate to download the url from cdn.discord.com or whatever with the file name and extension of the original file.
async fn download_file(url: String) {
    let client = Client::new();
    let response = client.get(&url).send().await.unwrap();
    let content_type = response
        .headers()
        .get("content-type")
        .unwrap()
        .to_str()
        .unwrap();
    let extension = content_type.split('/').nth(1).unwrap_or("bin");
    let file_name = PathBuf::from(&url)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned()
        + "."
        + extension;
    let mut file_path = PathBuf::from("./download/").join(&file_name);

    let mut index = 0;
    while file_path.exists() {
        index += 1;
        let new_file_name = format!(
            "{}-{}.{}",
            &file_name[..file_name.len() - extension.len() - 1],
            index,
            extension
        );
        file_path.set_file_name(new_file_name);
    }

    let mut file: File = fs::File::create(&file_path).await.unwrap();
    let response_file = response.bytes().await.unwrap().to_vec();
    file.write_all(&response_file).await.unwrap();
}

/// function that registers the command with the discord api
pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("download")
        .description("download the links saved to the output file")
        .create_option(|option| {
            option
                .name("upload_result")
                .description("attach the file containing the links")
                .kind(serenity::model::prelude::command::CommandOptionType::Boolean)
                .required(true)
        })
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
