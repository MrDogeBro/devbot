use super::Context;
use anyhow::Result;

/// Shows information about the bot
#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<()> {
    poise::send_reply(ctx, |m| m.embed(|e| e.description("testing"))).await?;

    Ok(())
}

/// Shows all of the bots commands
#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<()> {
    poise::send_reply(ctx, |m| m.embed(|e| e.description("help"))).await?;

    let mut categories: Vec<(Option<&str>, Vec<&poise::PrefixCommand<_, _>>)> = Vec::new();
    for cmd_meta in &ctx.framework().options().prefix_options.commands {
        if let Some((_, commands)) = categories
            .iter_mut()
            .find(|(key, _)| *key == cmd_meta.category)
        {
            commands.push(&cmd_meta.command);
        } else {
            categories.push((cmd_meta.category, vec![&cmd_meta.command]));
        }
    }

    for (category_name, commands) in categories {
        println!("{}", category_name.unwrap_or("Other"));
    }

    Ok(())
}
