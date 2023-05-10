use std::path::PathBuf;

use reqwest::Client;
use tokio::{
    fs::{self, File},
    io::{self, AsyncBufReadExt, AsyncWriteExt},
};

use serenity::{builder::CreateApplicationCommand, model::Permissions};

pub async fn run() -> String {
    read_file().await;
    "test".to_string()
}

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
