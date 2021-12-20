use anyhow::{Error, Result};
use phf::{phf_map, Map};

pub const PERMS_ERROR: &str = "PERMS_ERROR";
pub const BOT_PERMS_ERROR: &str = "BOT_PERMS_ERROR";

pub static ERRORS: Map<&'static str, &'static str> = phf_map! {
    "PERMS_ERROR" => "You do not have the {} permission required to use this command!",
    "BOT_PERMS_ERROR" => "The bot does not have the {} permission required to execute this command!",
};

pub static BUILTIN_ERRORS: Map<&'static str, &'static str> = phf_map! {
    "Failed to parse argument: Member not found or unknown format" => "Invalid member argument provided! Make sure they are a member of the current guild.",
};

pub async fn handle_error(err: Error) -> Result<String> {
    let err_msg = ERRORS.get(err.to_string().as_str());

    if let Some(err_msg) = err_msg {
        if let Some(err_ctx) = err.chain().skip(1).next() {
            return Ok(err_msg.replace("{}", err_ctx.to_string().as_str()));
        }
    }

    for e in BUILTIN_ERRORS.keys() {
        if err.to_string().starts_with(e) {
            if let Some(builtin_err_msg) = BUILTIN_ERRORS.get(e) {
                return Ok(builtin_err_msg.to_string());
            }
        }
    }

    Err(err)
}
