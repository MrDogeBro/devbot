use super::{get_category_description, Context};
use anyhow::Result;
use serenity::collector::component_interaction_collector::CollectComponentInteraction;
use serenity::model::prelude::InteractionResponseType;
use std::time::Duration;
use uuid::Uuid;

// ========================================================================================
//                                  Info Command
// ========================================================================================

/// Shows information about the bot
///
/// Shows information about the bot, its code, etc.
#[poise::command(slash_command)]
pub async fn info(ctx: Context<'_>) -> Result<()> {
    poise::send_reply(ctx, |m| m.embed(|e| e.description("testing"))).await?;

    Ok(())
}

// for guild in ctx.discord().cache.guilds() {
// println!("{}", guild);
// }

// ========================================================================================
//                                  Help Command
// ========================================================================================

/// Shows all of the bots commands
///
/// Shows the commands the bot has. To get detailed information about a specific command,
/// use the following syntax. ```
/// /help <command>
/// ```
#[poise::command(slash_command)]
pub async fn help(ctx: Context<'_>) -> Result<()> {
    let uuid_categories = Uuid::new_v4();
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

    poise::send_reply(ctx, |m| {
        m.embed(|embed| {
            embed.title("Help");
            embed.description("Get started by selecting a category from the select menu below");
            embed.color(ctx.data().config.env.default_embed_color);
            embed
        });
        m.components(|c| {
            c.create_action_row(|ar| {
                ar.create_select_menu(|sm| {
                    sm.placeholder("Select a category");
                    sm.min_values(1);
                    sm.max_values(1);
                    sm.custom_id(&uuid_categories);

                    sm.options(|o| {
                        for (category_name, _) in &categories {
                            let category = category_name.unwrap_or("Other");

                            o.create_option(|o| {
                                o.label(category);
                                o.description(get_category_description(category));
                                o.value(category.to_lowercase().replace(" ", "-"));
                                o
                            });
                        }
                        o
                    });
                    sm
                });
                ar
            });
            c
        });
        m
    })
    .await?;

    loop {
        let mov_uuid_categories = uuid_categories.clone();
        let mci = CollectComponentInteraction::new(ctx.discord())
            .author_id(ctx.author().id)
            .channel_id(ctx.channel_id())
            .timeout(Duration::from_secs(120))
            .filter(move |mci| mci.data.custom_id == mov_uuid_categories.to_string())
            .await;

        if let Some(mci) = mci {
            let mut msg = mci.message.clone().regular().unwrap();

            if mci.data.values.is_empty() {
                mci.create_interaction_response(ctx.discord(), |ir| {
                    ir.kind(InteractionResponseType::DeferredUpdateMessage)
                })
                .await?;
            }

            for (category_name, commands) in &categories {
                let category = category_name.unwrap_or("Other");

                if category.to_lowercase().replace(" ", "-") == mci.data.values[0].as_str() {
                    let mut cmds: String = "".to_string();

                    for command in commands {
                        if command.options.hide_in_help {
                            continue;
                        }

                        cmds += format!(
                            "`{}` {}\n",
                            command.name,
                            command.options.inline_help.unwrap_or("")
                        )
                        .as_str();

                        // println!("{}", ctx.prefix);
                    }

                    msg.edit(ctx.discord(), |m| {
                        m.embed(|embed| {
                            embed.title(category);
                            embed.description(get_category_description(category));
                            embed.color(ctx.data().config.env.default_embed_color);

                            embed.field("Commands", cmds, false);
                            embed
                        })
                    })
                    .await?;

                    break;
                }
            }

            mci.create_interaction_response(ctx.discord(), |ir| {
                ir.kind(InteractionResponseType::DeferredUpdateMessage)
            })
            .await?;
        } else {
            println!("Collector returned None, returning.");
            break;
        }
    }

    Ok(())
}
