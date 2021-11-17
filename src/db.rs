use anyhow::Result;
use rusqlite::Connection;

pub struct Database {
    pub conn: Connection,
}

impl Database {
    pub fn load(dynamic_data_path: &String) -> Result<Self> {
        let conn = Connection::open(format!("{}/db.db", dynamic_data_path))?;

        conn.execute(
            "CREATE TABLE IF NOT EXISTS support (
                id text NOT NULL PRIMARY KEY,
                owner_id integer NOT NULL,
                thread_id text NOT NULL,
                created_at text NOT NULL,
                language text DEFAULT 'Unknown',
                title text DEFAULT 'Unknown',
                status text DEFAULT 'open'
            )",
            [],
        )?;

        Ok(Self { conn })
    }
}
