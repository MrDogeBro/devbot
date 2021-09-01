use anyhow::Result;
use dotenv::dotenv;
use std::env::var;

#[derive(Clone)]
pub struct Config {
    pub env: Env,
}

#[derive(Clone)]
pub struct Env {
    pub token: String,
    pub owner_id: String,
    pub application_id: u64,
    pub prefix: String,
    pub default_embed_color: u64,
    pub test_server_id: u64,
}

impl Config {
    pub fn load() -> Result<Self> {
        Ok(Self { env: Env::load()? })
    }
}

impl Env {
    pub fn load() -> Result<Self> {
        dotenv().ok();

        Ok(Self {
            token: var("TOKEN")?,
            owner_id: var("OWNER_ID")?,
            application_id: var("APPLICATION_ID")?.parse()?,
            prefix: var("PREFIX")?,
            default_embed_color: var("DEFAULT_EMBED_COLOR")?.parse()?,
            test_server_id: var("TEST_SERVER_ID")?.parse()?,
        })
    }
}
