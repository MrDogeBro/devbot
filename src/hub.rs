use anyhow::Result;
use serenity::model::id::ChannelId;

pub struct Hub {
    pub stdout: ChannelId,
}

impl Hub {
    pub fn load(config: &crate::config::Config) -> Result<Self> {
        Ok(Self {
            stdout: ChannelId(config.env.hub_stdout_id),
        })
    }
}
