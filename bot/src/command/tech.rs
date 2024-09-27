use poise::serenity_prelude::*;
use strum::IntoEnumIterator;

use crate::{Context, Error};
use common::tech::Technology;

/// Test enum
#[poise::command(slash_command, prefix_command)]
pub async fn tech(
    ctx: Context<'_>,
    // #[autocomplete = "callback()"]
    #[autocomplete = "autocomplete_technology"] tech: String,
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
