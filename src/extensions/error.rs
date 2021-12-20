use anyhow::{Error, Result};
use phf::{phf_map, Map};

pub const PERMS_ERROR: &str = "PERMS_ERROR";

pub static ERRORS: Map<&'static str, &'static str> = phf_map! {
    "PERMS_ERROR" => "You do not have the {} permission required to use this command!",
};

pub async fn handle_error(err: Error) -> Result<String> {
    let err_msg = ERRORS.get(err.to_string().as_str());

    if let Some(err_msg) = err_msg {
        if let Some(err_ctx) = err.chain().skip(1).next() {
            return Ok(err_msg.replace("{}", err_ctx.to_string().as_str()));
        }
    }

    Err(err)
}
