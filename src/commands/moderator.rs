use super::Context;
use anyhow::Result;
use serenity::model::guild::Member;

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
    if let Some(reason) = reason {
        member
            .kick_with_reason(&ctx.discord().http, &reason)
            .await?;
    } else {
        member.kick(&ctx.discord().http).await?;
    }

    poise::send_reply(ctx, |m| {
        m.content(format!("Successfully kicked {}.", member))
    })
    .await?;

    Ok(())
}