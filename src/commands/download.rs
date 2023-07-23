use std::path::{Path, PathBuf};

use crate::helper_functions::{followup_status_message, status_message};
use crate::helper_functions::DiscordMessage;

use reqwest::Client;
use tokio::{
    fs::{self, File},
    io::{self, AsyncBufReadExt, AsyncWriteExt},
};

use serenity::{
    builder::CreateApplicationCommand,
    model::{
        prelude::interaction::application_command::{
                ApplicationCommandInteraction, CommandDataOptionValue,
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

    status_message(ctx, "downloading attachments...", interaction).await;

    if let Ok(meta) = fs::metadata(path).await {
        if meta.is_file() {
            read_file().await;
            
            // if dl_to_disk is true
            if let CommandDataOptionValue::Boolean(true) = command_option {
                followup_status_message(ctx, "downloaded attachments!", interaction).await;

                // fs::remove_file("./download/output.txt")
                //     .unwrap_or_else(|_| ())
                //     .await;
            } else {
                // if it is false
                followup_status_message(ctx, "downloaded attachments!", interaction).await;

                // fs::remove_file("./download/output.txt")
                //     .unwrap_or_else(|_| ())
                //     .await;
            }
            println!("done downloading files from output.txt file.");
        } else {
            followup_status_message(ctx, "an error has occured, check logs!", interaction).await;
        }
    } else {
        followup_status_message(
            ctx,
            "Not indexed yet. Try using `/index` first.",
            interaction,
        )
        .await;
    }
}

/// read the urls from the file "output.txt" for them to be downloaded
async fn read_file() {
    let file = match File::open("./download/output.txt").await {
        Ok(file) => file,
        Err(e) => {
            eprintln!(
                "an error occured whilst trying to read \"output.txt\": {}",
                e
            );
            return;
        }
    };
    let mut lines = io::BufReader::new(file).lines();
    while let Some(line) = lines.next_line().await.unwrap() {
        let json: DiscordMessage = serde_json::from_str(&line).unwrap();
        for link in json.attachments {
            // this iterates over every link ONCE
            download_file(link).await;
        }
    }
}

/// use Reqwest crate to download the url from cdn.discord.com or whatever with the file name and extension of the original file.
/// minimum permission level: ADMINISTRATOR
async fn download_file(url: String) {
    let client = Client::new();
    let response = client.get(&url).send().await.unwrap();
    let file_name = PathBuf::from(&url)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_owned(); // filename + extension name
    let mut file_path = PathBuf::from("./download/").join(&file_name);

    // increment the index by 1 everytime the filename already exists, and add it to the beginning of the file name
    let mut index = 0;
    while file_path.exists() {
        index += 1;
        let new_file_name = format!("{}.{}", index, file_name,);
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
                .name("download_to_disk")
                .description("download attachments to disk")
                .kind(serenity::model::prelude::command::CommandOptionType::Boolean)
                .required(true)
        })
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
