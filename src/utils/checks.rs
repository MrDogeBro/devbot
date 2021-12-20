use crate::extensions::error;

use anyhow::{Error, Result};
use serenity::{
    model::{guild::Member, permissions::Permissions},
    prelude::Context,
};

pub async fn check_permission(perm: Permissions, member: Member, ctx: &Context) -> Result<()> {
    if member.permissions(ctx)?.contains(perm) {
        return Ok(());
    }

    Err(Error::msg(perm.to_string().to_lowercase()).context(error::PERMS_ERROR))
}
