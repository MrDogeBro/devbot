mod meta;
mod moderator;

use crate::register_commands;
use crate::State;

use anyhow::{Error, Result};

pub fn command_list(
    mut options: poise::FrameworkOptions<State, Error>,
) -> Result<poise::FrameworkOptions<State, Error>> {
    register_commands!(options, meta, "Meta", info, help, source);
    register_commands!(options, moderator, "Moderator", kick, ban, unban);

    Ok(options)
}

pub fn get_category_description(category: &str) -> &str {
    match category {
        "Meta" => "Information about the bot",
        "Moderator" => "Preform moderator actions",
        _ => "???",
    }
}
