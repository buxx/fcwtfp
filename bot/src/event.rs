use poise::serenity_prelude::{self as serenity};

use crate::{sync, Data, Error};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    _data: &Data,
) -> Result<(), Error> {
    if let Err(error) = match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => {
            //
            sync::sync_all(ctx, data_about_bot).await
        }
        serenity::FullEvent::GuildMemberAddition { new_member } => {
            //
            sync::sync_new_member(ctx, new_member).await
        }
        _ => Ok(()),
    } {
        eprintln!(
            "Error during process of event {}: {}",
            event.snake_case_name(),
            error
        );
    }
    Ok(())
}
