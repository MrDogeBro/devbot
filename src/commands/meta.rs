use super::{get_category_description, Context};
use anyhow::Result;
use chrono::prelude::Utc;
use serenity::collector::component_interaction_collector::CollectComponentInteraction;
use serenity::model::prelude::InteractionResponseType;
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

    let prefix = match ctx {
        poise::Context::Prefix(_) => ctx.data().config.env.prefix.to_owned(),
        _ => "/".to_string(),
    };

    let reply = poise::send_reply(ctx, |m| {
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
    .await?
    .message()
    .await?;

    let reply_channel_id = reply.channel_id;
    let reply_id = reply.id;

    loop {
        let mov_uuid_categories = uuid_categories.clone();
        let mci = CollectComponentInteraction::new(ctx.discord())
            .author_id(ctx.author().id)
            .channel_id(ctx.channel_id())
            .timeout(ctx.data().config.env.default_interaction_timeout)
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
                            "`{}{}` {}\n",
                            prefix,
                            command.name,
                            command.options.inline_help.unwrap_or("")
                        )
                        .as_str();
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
            let mut msg = ctx
                .discord()
                .http
                .get_message(*reply_channel_id.as_u64(), *reply_id.as_u64())
                .await?;

            let msg_clone = msg.clone();
            let curr_embed = msg_clone.embeds.get(0);

            msg.edit(ctx.discord(), |m| {
                m.components(|c| c);

                if let Some(curr) = curr_embed {
                    m.embed(|embed| {
                        embed.title(curr.title.as_ref().unwrap());
                        embed.description(curr.description.as_ref().unwrap());
                        embed.color(ctx.data().config.env.default_embed_color);
                        embed.footer(|f| {
                            f.text(format!(
                                "Interaction timed out at {} UTC",
                                Utc::now().format("%Y-%m-%d %H:%M:%S")
                            ));
                            f
                        });

                        for field in curr.fields.clone() {
                            embed.field(field.name, field.value, field.inline);
                        }

                        embed
                    });
                }

                m
            })
            .await?;

            break;
        }
    }

    Ok(())
}
