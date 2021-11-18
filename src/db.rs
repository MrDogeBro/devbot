use anyhow::Result;
use rusqlite::Connection;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn load(dynamic_data_path: &String) -> Result<Self> {
        let conn = Connection::open(format!("{}/db.db", dynamic_data_path))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS prefs (
                id integer NOT NULL PRIMARY KEY
            )",
            [],
        )?;

        Ok(Self { conn })
    }
}
