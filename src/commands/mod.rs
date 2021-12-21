mod config;
mod meta;
mod moderator;

use crate::State;
use crate::{register_commands, register_commands_group};

use anyhow::{Error, Result};

pub fn command_list(
    mut options: poise::FrameworkOptions<State, Error>,
) -> Result<poise::FrameworkOptions<State, Error>> {
    register_commands!(options, meta, info, help, source);
    register_commands!(options, moderator, kick, ban, unban);
    // register_commands_group!(options, config, "Config", config, test);
    options.command(config::config(), |f| f.subcommand(config::test(), |f| f));

    Ok(options)
}

pub fn get_category_description(category: &str) -> &str {
    match category {
        "Meta" => "Information about the bot",
        "Moderator" => "Preform moderator actions",
        "Config" => "Configure per-server settings",
        _ => "???",
    }
}
