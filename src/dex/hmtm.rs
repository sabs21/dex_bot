use crate::Error;
use poise::serenity_prelude as serenity;

pub async fn hmtm_component(
    ctx: serenity::Context,
    i: &serenity::ComponentInteraction,
    _pokemon_id: u16,
) -> Result<(), Error> {
    i.create_response(
        ctx,
        serenity::CreateInteractionResponse::Message(
            serenity::CreateInteractionResponseMessage::new()
                .ephemeral(true)
                .content("hmtm not implemented yet."),
        ),
    )
    .await?;
    Ok(())
}
