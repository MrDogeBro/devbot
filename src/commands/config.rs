use crate::Context;

use anyhow::Result;

// ========================================================================================
//                                  Config Command
// ========================================================================================

/// Allows you to configure server settings
///
/// Allows you to configure all settings that are on a per-server basis. ```
/// <<prefix>>config <setting> [options-per-setting]
/// ```
#[poise::command(slash_command, category = "Config")]
pub async fn config(ctx: Context<'_>) -> Result<()> {
    poise::send_reply(ctx, |m| m.content("Config Command".to_string())).await?;

    Ok(())
}

// ========================================================================================
//                                  Logging Subcommand
// ========================================================================================

/// Enables or disables logging
///
/// Enables or disables logging of actions through DevBot for the given server. ```
/// <<prefix>>config logging [enabled]
/// ```
#[poise::command(slash_command, category = "Config")]
pub async fn logging(
    ctx: Context<'_>,
    #[description = "The new state of logging in the guild"] enabled: Option<bool>,
) -> Result<()> {
    if let Some(enabled) = enabled {
        poise::send_reply(ctx, |m| m.content("State updated".to_string())).await?;

        return Ok(());
    }

    poise::send_reply(ctx, |m| m.content("State".to_string())).await?;

    Ok(())
}
