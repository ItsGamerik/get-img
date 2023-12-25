use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use log::error;
use serde::{Deserialize, Serialize};
use serenity::all::{ActivityData, Attachment, CommandInteraction, Context, CreateInteractionResponseFollowup, Message, OnlineStatus};

#[derive(Debug, Serialize, Deserialize)]
pub struct DiscordMessage {
    pub timestamp: String,
    pub author: String,
    pub content: String,
    pub attachments: Vec<String>,
}

pub async fn start_action(ctx: &Context, interaction: &CommandInteraction) {

    ctx.set_presence(Some(ActivityData::watching("working...")), OnlineStatus::DoNotDisturb);

    if let Err(e) = interaction.defer(ctx.http.clone()).await {
        error!("could not defer interaction: {e}");
    };
}

pub async fn end_action(ctx: &Context, interaction: &CommandInteraction) {
    ctx.set_presence(Some(ActivityData::watching("TODO")), OnlineStatus::Online);

    // wtf is this shit
    if let Err(e) = interaction.create_followup(ctx.http.clone(), CreateInteractionResponseFollowup::content(CreateInteractionResponseFollowup::new(), "test")).await {
        error!("could not ... interaction: {e}");
    }
}

pub async fn universal_message_writer(message: Message) {

    let message_attachments: Vec<Attachment> = message.attachments;

    if let Err(e) = fs::create_dir_all("./download/") {
        error!("error creating download file: {e}")
    }

    let mut file = match OpenOptions::new().write(true).create(true).append(true).open("./download/output.txt") {
        Ok(file) => file,
        Err(e) => {
            error!("could not open file: {e}");
            return;
        }
    };

    let mut attachment_link_vec = Vec::new();

    for attachment in message_attachments {
        attachment_link_vec.push(attachment.url)
    }

    let json_object = DiscordMessage {
        author: format!("{}", message.author),
        content: message.content,
        attachments: attachment_link_vec,
        timestamp: format!("{}", message.timestamp)
    };

    let serialized = serde_json::to_string(&json_object).unwrap();

    if let Err(e) = writeln!(file, "{serialized}") {
        error!("error writing to file: {e}")
    }
}