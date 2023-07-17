// this file contains some helper functions

use serenity::{
    model::{
        prelude::{
            application_command::ApplicationCommandInteraction, Activity, InteractionResponseType,
        },
        user::OnlineStatus,
    },
    prelude::Context,
};

/// used to create interaction responses.
pub async fn status_message(ctx: &Context, msg: &str, interaction: &ApplicationCommandInteraction) {
    start_action(ctx).await;
    interaction
        .create_interaction_response(&ctx.http, |response| {
            response
                .kind(InteractionResponseType::ChannelMessageWithSource)
                .interaction_response_data(|data| data.content(msg.to_string()))
        })
        .await
        .unwrap();
}

/// used to edit an already existing status message
pub async fn edit_status_message(
    ctx: &Context,
    response_string: &str,
    interaction: &ApplicationCommandInteraction,
) {
    stop_action(ctx).await;
    interaction
        .edit_original_interaction_response(&ctx.http, |response| response.content(response_string))
        .await
        .unwrap();
}

/// create followup message
pub async fn followup_status_message(
    ctx: &Context,
    response_string: &str,
    interaction: &ApplicationCommandInteraction,
) {
    stop_action(ctx).await;
    interaction
        .create_followup_message(&ctx.http, |response| response.content(response_string))
        .await
        .unwrap();
}

async fn start_action(ctx: &Context) {
    // the status update seems to be very slow, so it probably wont actually show unless a task is started that takes a long time
    let status = OnlineStatus::DoNotDisturb;
    ctx.set_presence(Some(Activity::watching("currently working...")), status)
        .await;
}

async fn stop_action(ctx: &Context) {
    let status = OnlineStatus::Online;
    ctx.set_presence(Some(Activity::watching("v1.2 - ready for work")), status)
        .await;
}
