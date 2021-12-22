use anyhow::Result;
use serenity::{builder::CreateApplicationCommands, prelude::Context as SerenityContext};

pub async fn register_commands(
    ctx: &SerenityContext,
    framework: &poise::Framework<crate::State, anyhow::Error>,
    state: &crate::State,
) -> Result<()> {
    let mut commands_builder = CreateApplicationCommands::default();
    let commands = &framework.options().application_options.commands;

    for cmd in commands {
        commands_builder.create_application_command(|f| cmd.create(f));
    }

    let commands_builder = serde_json::Value::Array(commands_builder.0);

    if cfg!(debug_assertions) {
        // register only for test guild in develop
        let commands = ctx
            .http
            .get_guild_application_commands(state.config.env.hub_server_id)
            .await?;

        for cmd in commands {
            ctx.http
                .delete_guild_application_command(state.config.env.hub_server_id, cmd.id.0)
                .await?;
        }

        println!("Commands unregistered (develop)");

        ctx.http
            .create_guild_application_commands(state.config.env.hub_server_id, &commands_builder)
            .await?;

        println!("Commands registered (develop)");

        return Ok(());
    }

    // register globally in prod
    let commands = ctx.http.get_global_application_commands().await?;

    for cmd in commands {
        ctx.http.delete_global_application_command(cmd.id.0).await?;
    }

    println!("Commands unregistered");

    ctx.http
        .create_global_application_commands(&commands_builder)
        .await?;

    println!("Commands registered");

    Ok(())
}
