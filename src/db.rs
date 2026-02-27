use rusqlite::Connection;
use std::error::Error;



pub struct Database {
    conn: Connection,
}

impl Database {
    pub fn new(path: &str) -> Result<Self, Box<dyn Error>> {
        let conn = Connection::open(path)?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS repos (
                username TEXT NOT NULL,
                name TEXT NOT NULL,
                data TEXT NOT NULL,
                PRIMARY KEY (username, name)
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS unicode_chars (
                codepoint TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                block TEXT NOT NULL
            )",
            [],
        )?;
        conn.execute(
            "CREATE TABLE IF NOT EXISTS unicode_blocks (
                name TEXT PRIMARY KEY,
                range_start TEXT NOT NULL,
                range_end TEXT NOT NULL
            )",
            [],
        )?;
        
        Ok(Database { conn })
    }

    pub fn conn(&mut self) -> &mut Connection {
        &mut self.conn
    }
}
