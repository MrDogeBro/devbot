use anyhow::Result;
use dotenv::dotenv;
use std::env::var;
use std::time::Duration;
use std::{fs, path::Path};

#[derive(Clone)]
pub struct Config {
    pub env: Env,
    pub data_path: DataPath,
}

#[derive(Clone)]
pub struct Env {
    pub token: String,
    pub owner_id: String,
    pub application_id: u64,
    pub db_host: String,
    pub db_port: u16,
    pub db_name: String,
    pub db_user: String,
    pub db_pass: String,
    pub prefix: String,
    pub default_embed_color: serenity::utils::Colour,
    pub default_interaction_timeout: Duration,
    pub hub_server_id: u64,
    pub hub_stdout_id: u64,
}

#[derive(Clone)]
pub struct DataPath {
    pub dynamic: String,
    pub staticd: String,
}

impl Config {
    pub async fn load() -> Result<Self> {
        let base_data_path = "data";
        let data_path = DataPath {
            dynamic: format!("{}/dynamic", base_data_path),
            staticd: format!("{}/static", base_data_path),
        };

        if !Path::new(&data_path.dynamic).exists() {
            fs::create_dir_all(&data_path.dynamic)?;
        }

        Ok(Self {
            env: Env::load().await?,
            data_path,
        })
    }
}

impl Env {
    pub async fn load() -> Result<Self> {
        dotenv().ok();

        let default_embed_color: Vec<u8> = var("DEFAULT_EMBED_COLOR")?
            .replace("(", "")
            .replace(")", "")
            .replace(" ", "")
            .split(",")
            .collect::<Vec<&str>>()
            .iter()
            .map(|c| -> u8 { c.parse::<u8>().unwrap() })
            .collect();

        Ok(Self {
            token: var("TOKEN")?,
            owner_id: var("OWNER_ID")?,
            application_id: var("APPLICATION_ID")?.parse()?,
            db_host: var("DB_HOST")?,
            db_port: var("DB_PORT")?.parse()?,
            db_name: var("DB_NAME")?,
            db_user: var("DB_USER")?,
            db_pass: var("DB_PASS")?,
            prefix: var("PREFIX")?,
            default_embed_color: serenity::utils::Colour::from_rgb(
                *default_embed_color.get(0).unwrap(),
                *default_embed_color.get(1).unwrap(),
                *default_embed_color.get(2).unwrap(),
            ),
            default_interaction_timeout: Duration::from_secs(
                var("DEFAULT_INTERACTION_TIMEOUT")?.parse()?,
            ),
            hub_server_id: var("HUB_SERVER_ID")?.parse()?,
            hub_stdout_id: var("HUB_STDOUT_ID")?.parse()?,
        })
    }
}
