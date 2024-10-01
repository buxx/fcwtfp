use common::session::tech::Technology;
use poise::serenity_prelude::*;
use strum::IntoEnumIterator;

use crate::{Context, Error};

#[poise::command(slash_command, prefix_command)]
pub async fn technology(
    ctx: Context<'_>,
    #[autocomplete = "autocomplete_technology"] tech: Technology,
) -> Result<(), Error> {
    let response = format!("Ya! {}", tech);
    ctx.say(response).await?;
    Ok(())
}

async fn autocomplete_technology(_: Context<'_>, partial: &str) -> Vec<AutocompleteChoice> {
    Technology::iter()
        .filter(|technology| {
            technology
                .to_string()
                .to_ascii_lowercase()
                .contains(&partial.to_ascii_lowercase())
        })
        .map(|technology| AutocompleteChoice::new(technology.to_string(), technology.to_string()))
        .collect()
}
