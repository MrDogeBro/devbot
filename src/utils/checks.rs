use crate::extensions::error;

use anyhow::{Error, Result};
use serenity::{
    model::{guild::Member, permissions::Permissions},
    prelude::Context,
};

pub async fn check_member_permission(
    perm: Permissions,
    member: Member,
    ctx: &Context,
) -> Result<()> {
    if member.permissions(ctx)?.contains(perm) {
        return Ok(());
    }

    Err(Error::msg(perm.to_string().to_lowercase()).context(error::PERMS_ERROR))
}

pub async fn check_bot_permission(perm: Permissions, bot: Member, ctx: &Context) -> Result<()> {
    if bot.permissions(ctx)?.contains(perm) {
        return Ok(());
    }

    Err(Error::msg(perm.to_string().to_lowercase()).context(error::BOT_PERMS_ERROR))
}

pub async fn check_permission(
    perm: Permissions,
    bot: Member,
    member: Member,
    ctx: &Context,
) -> Result<()> {
    check_bot_permission(perm, bot, ctx).await?;
    check_member_permission(perm, member, ctx).await?;

    Ok(())
}
