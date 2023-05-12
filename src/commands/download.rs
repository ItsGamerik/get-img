use std::path::PathBuf;

use reqwest::Client;
use tokio::{
    fs::{self, File},
    io::{self, AsyncBufReadExt, AsyncWriteExt},
};

use serenity::{builder::CreateApplicationCommand, model::{Permissions, prelude::interaction::{application_command::ApplicationCommandInteraction, InteractionResponseType}}, futures::TryFutureExt, prelude::Context};

pub async fn run(ctx: &Context, interaction: &ApplicationCommandInteraction) {
    interaction.create_interaction_response(&ctx.http, |response| {
        response.kind(InteractionResponseType::DeferredChannelMessageWithSource).interaction_response_data(|data| {
            data.content("downloading attachments...")
        })
    }).await.unwrap();
    read_file().await;
    interaction.create_followup_message(&ctx.http, |response| {
        response.content("downloaded attachments!")
    }).await.unwrap();
}

// read the urls from the file "output.txt"
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
    fs::remove_file("./download/output.txt").unwrap_or_else(|_| ()).await;
}

// use Reqwest crate to download the url from cdn.discord.com or whatever with the file name and extension of the original file.
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

pub fn register(command: &mut CreateApplicationCommand) -> &mut CreateApplicationCommand {
    command
        .name("download")
        .description("download the links saved to the output file")
        .default_member_permissions(Permissions::ADMINISTRATOR)
}
