use crate::extensions::logging;
use crate::utils::checks;
use crate::Context;

use anyhow::Result;
use serenity::model::{guild::Member, id::UserId, permissions::Permissions, user::User};

// ========================================================================================
//                                  Kick Command
// ========================================================================================

/// Kicks a member
///
/// Kicks a member from the server, with an optional reason. ```
/// <<prefix>>kick <member> [reason]
/// ```
#[poise::command(slash_command)]
pub async fn kick(
    ctx: Context<'_>,
    #[description = "The member who will be kicked"] member: Member,
    #[description = "The reason the member is being kicked"]
    #[rest]
    reason: Option<String>,
) -> Result<()> {
    checks::check_permission(
        Permissions::KICK_MEMBERS,
        ctx.guild()
            .unwrap()
            .member(
                &ctx.discord().http,
                UserId(*ctx.framework().application_id().as_u64()),
            )
            .await?,
        ctx.guild()
            .unwrap()
            .member(&ctx.discord().http, ctx.author().id)
            .await?,
        ctx.discord(),
    )
    .await?;

    if let Some(reason) = reason {
        member
            .kick_with_reason(&ctx.discord().http, &reason)
            .await?;
    } else {
        member.kick(&ctx.discord().http).await?;
    }

    logging::log(
        ctx.guild().unwrap().id,
        format!("Successfully kicked {}.", member),
    )
    .await?;

    poise::send_reply(ctx, |m| {
        m.content(format!("Successfully kicked {}.", member))
    })
    .await?;

    Ok(())
}

// ========================================================================================
//                                  Ban Command
// ========================================================================================

/// Bans a member
///
/// Bans a member from the server, with an optional reason. ```
/// <<prefix>>ban <member> [days] [reason]
/// ```
#[poise::command(slash_command)]
pub async fn ban(
    ctx: Context<'_>,
    #[description = "The member who will be banned"] member: Member,
    #[description = "The number of days worth of messages to delete"] days: Option<u8>,
    #[description = "The reason the member is being banned"]
    #[rest]
    reason: Option<String>,
) -> Result<()> {
    checks::check_permission(
        Permissions::BAN_MEMBERS,
        ctx.guild()
            .unwrap()
            .member(
                &ctx.discord().http,
                UserId(*ctx.framework().application_id().as_u64()),
            )
            .await?,
        ctx.guild()
            .unwrap()
            .member(&ctx.discord().http, ctx.author().id)
            .await?,
        ctx.discord(),
    )
    .await?;

    let mut new_days: u8 = 7;

    if let Some(days) = days {
        if days <= 7 {
            new_days = days;
        }
    }

    if let Some(reason) = reason {
        member
            .ban_with_reason(&ctx.discord().http, new_days, &reason)
            .await?;
    } else {
        member.ban(&ctx.discord().http, new_days).await?;
    }

    logging::log(
        ctx.guild().unwrap().id,
        format!("Successfully banned {}.", member),
    )
    .await?;

    poise::send_reply(ctx, |m| {
        m.content(format!("Successfully banned {}.", member))
    })
    .await?;

    Ok(())
}

// ========================================================================================
//                                  Unban Command
// ========================================================================================

/// Unbans a member
///
/// Unbans a member from the server, with an optional reason. ```
/// <<prefix>>unban <member> [reason]
/// ```
#[poise::command(slash_command)]
pub async fn unban(
    ctx: Context<'_>,
    #[description = "The id of the user who will be unbanned"] id: UserId,
    #[description = "The reason the user is being unbanned"]
    #[rest]
    reason: Option<String>,
) -> Result<()> {
    checks::check_permission(
        Permissions::BAN_MEMBERS,
        ctx.guild()
            .unwrap()
            .member(
                &ctx.discord().http,
                UserId(*ctx.framework().application_id().as_u64()),
            )
            .await?,
        ctx.guild()
            .unwrap()
            .member(&ctx.discord().http, ctx.author().id)
            .await?,
        ctx.discord(),
    )
    .await?;

    let user: User = id.to_user(&ctx.discord().http).await?;

    if !ctx
        .guild()
        .unwrap()
        .bans(&ctx.discord().http)
        .await?
        .iter()
        .any(|b| b.user == user)
    {
        poise::send_reply(ctx, |m| {
            m.content(format!("User {} was never banned.", user))
        })
        .await?;

        return Ok(());
    }

    ctx.guild().unwrap().unban(&ctx.discord().http, id).await?;

    logging::log(
        ctx.guild().unwrap().id,
        format!("Successfully unbanned {}.", user),
    )
    .await?;

    poise::send_reply(ctx, |m| {
        m.content(format!("Successfully unbanned {}.", user))
    })
    .await?;

    Ok(())
}
