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
    poise::send_reply(ctx, |m| {
        m.content("this is a test".to_string());
        m.components(|c| {
            c.create_action_row(|ar| {
                ar.create_select_menu(|sm| {
                    sm.placeholder("testing");
                    sm.min_values(1);
                    sm.max_values(1);
                    sm.custom_id("testing");

                    sm.options(|o| {
                        o.create_option(|o| {
                            o.label("test");
                            o.value("0");
                            o.description("this is a test");
                            o
                        })
                    })
                })
            })
        })
    })
    .await?;

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
