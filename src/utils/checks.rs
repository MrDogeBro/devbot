use crate::extensions::error;

use anyhow::{Error, Result};
use serenity::model::{guild::Member, permissions::Permissions};

pub async fn check_permission(perm: Permissions, member: Member) -> Result<()> {
    if !member.permissions.unwrap().contains(perm) {
        return Err(Error::msg(perm.to_string().to_lowercase()).context(error::PERMS_ERROR));
    }

    Ok(())
}
