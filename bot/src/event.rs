use poise::serenity_prelude::{self as serenity, futures::StreamExt, Http, MembersIter};

use crate::{sync, Data, Error};

pub async fn event_handler(
    ctx: &serenity::Context,
    event: &serenity::FullEvent,
    _framework: poise::FrameworkContext<'_, Data, Error>,
    data: &Data,
) -> Result<(), Error> {
    if let Err(error) = match event {
        serenity::FullEvent::Ready { data_about_bot, .. } => sync::sync(ctx, data_about_bot).await,
        serenity::FullEvent::Message { new_message } => Ok(()),
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
