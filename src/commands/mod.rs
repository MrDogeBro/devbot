mod meta;

pub use super::*;

use anyhow::{Error, Result};

pub fn command_list(
    mut options: poise::FrameworkOptions<State, Error>,
) -> Result<poise::FrameworkOptions<State, Error>> {
    options.command(meta::info(), |f| f.category("Meta"));
    options.command(meta::help(), |f| f.category("Meta"));
    options.command(meta::source(), |f| f.category("Meta"));

    Ok(options)
}

pub fn get_category_description(category: &str) -> &str {
    match category {
        "Meta" => "Information about the bot",
        _ => "???",
    }
}
