use anyhow::Result;
use serenity::model::id::GuildId;

pub async fn log(_guild_id: GuildId, _msg: String) -> Result<()> {
    unimplemented!()
}
