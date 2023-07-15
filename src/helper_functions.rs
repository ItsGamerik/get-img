// this file contains some helper functions

use serenity::{
    model::prelude::{application_command::ApplicationCommandInteraction, InteractionResponseType},
    prelude::Context,
};

/// used to create interaction responses.
pub async fn status_message(ctx: &Context, msg: &str, interaction: &ApplicationCommandInteraction) {
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
    interaction: &ApplicationCommandInteraction,
    ctx: &Context,
    response_string: &str,
) {
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
    interaction
        .create_followup_message(&ctx.http, |response| response.content(response_string))
        .await
        .unwrap();
}
