use anyhow::Result;

pub fn into_titlecase(string: &mut str) -> Result<String> {
    if let Some(c) = string.get_mut(0..1) {
        c.make_ascii_uppercase();
    }

    Ok(string.to_string())
}
