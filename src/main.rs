mod commands;
mod config;
mod db;
mod extensions;
mod framework;
mod macros;
mod utils;

extern crate serde_json;

use extensions::hub;

use anyhow::{Error, Result};
use chrono::{prelude::Utc, DateTime};
use serenity::{model::prelude::ApplicationId, prelude::Context as SerenityContext};
use std::sync::Mutex;
use std::time::Duration;

pub type Context<'a> = poise::Context<'a, State, Error>;
pub type PrefixContext<'a> = poise::PrefixContext<'a, State, Error>;

pub struct State {
    config: config::Config,
    hub: hub::Hub,
    start_time: DateTime<Utc>,
    connected: Mutex<bool>,
    db: Mutex<db::Database>,
}

impl State {
    pub async fn load() -> Result<Self> {
        let config = config::Config::load()?;

        Ok(Self {
            hub: hub::Hub::load(&config)?,
            start_time: Utc::now(),
            connected: Mutex::new(false),
            db: Mutex::new(db::Database::load(&config.data_path.dynamic)?),
            config,
        })
    }

    pub async fn set_connected(&self) -> Result<()> {
        let mut conn = self.connected.lock().unwrap();
        *conn = true;

        Ok(())
    }
}

async fn listener(
    ctx: &SerenityContext,
    event: &poise::Event<'_>,
    framework: &poise::Framework<State, Error>,
    state: &State,
) -> Result<()> {
    match event {
        poise::Event::Ready { .. } => {
            if *state.connected.lock().unwrap() {
                println!("Bot reconnected!");
                return Ok(());
            }

            state.set_connected().await?;
            println!("Bot connected!");

            state
                .hub
                .stdout
                .send_message(&ctx.http, |m| {
                    m.content(format!("DevBot v{} started.", env!("CARGO_PKG_VERSION")))
                })
                .await?;

            framework::register_commands(ctx, framework, state).await?;

            Ok(())
        }
        _ => Ok(()),
    }
}

async fn on_error(error: Error, ctx: poise::ErrorContext<'_, State, Error>) {
    match ctx {
        poise::ErrorContext::Setup => panic!("Failed to start bot: {:?}", error),
        poise::ErrorContext::Command(ctx) => match extensions::error::handle_error(error).await {
            Ok(msg) => match poise::send_reply(ctx.ctx(), |m| m.content(msg.to_string())).await {
                Ok(_) => (),
                Err(e) => println!("Failed to send error message {}.\n\nTraceback:\n{}", msg, e),
            },
            Err(e) => {
                println!("Error in command `{}`: {:#?}", ctx.command().name(), e);

                match poise::send_reply(ctx.ctx(), |m| {
                    m.content("Something went wrong. Please notifty MrDogeBro.".to_string())
                })
                .await
                {
                    Ok(_) => (),
                    Err(e) => {
                        println!("Failed to send unknown error message.\n\nTraceback:\n{}", e)
                    }
                };
            }
        },
        _ => println!("Other error: {:?}", error),
    }
}

fn init_framework() -> Result<poise::FrameworkOptions<State, Error>> {
    let mut options = poise::FrameworkOptions {
        listener: |ctx, event, framework, state| Box::pin(listener(ctx, event, framework, state)),
        prefix_options: poise::PrefixFrameworkOptions {
            edit_tracker: Some(poise::EditTracker::for_timespan(Duration::from_secs(3600))),
            ..Default::default()
        },
        on_error: |error, ctx| Box::pin(on_error(error, ctx)),
        ..Default::default()
    };

    options = commands::command_list(options)?;
    Ok(options)
}

#[tokio::main]
async fn main() -> Result<()> {
    let env = config::Env::load()?;

    let framework = poise::Framework::new(
        ApplicationId(env.application_id),
        serenity::client::ClientBuilder::new(env.token),
        |_, _, _| Box::pin(State::load()),
        init_framework()?,
    )
    .await?;

    framework.start().await?;

    Ok(())
}
