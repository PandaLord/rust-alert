pub mod trigger;
pub mod alert;


use serde_json;
use tokio::*;
use chrono::prelude::*;
use rusqlite::{params, Connection, Result};
use alert::Alert;

fn open_my_db() -> Result<Connection> {
    let path = "./my_db.db3";
    let db = Connection::open(&path)?;
    println!("{}", db.is_autocommit());
    Ok(db)
}

fn main() -> Result<()> {
    let conn: Connection = open_my_db()?;
    conn.execute(
        "CREATE TABLE alert (
            id  INTEGER PRIMARY KEY,
            message  TEXT NOT NULL,
            created_on DATETIME
        )",
        []
    )?;

    let test: Alert = Alert::new(0, String::from("test"));

    conn.execute("
        INSERT INTO alert (message, created_on) 
        VALUES (?1, ?2),
    ", 
    params![test.message, test.created_on])?;

    let mut stmt = conn.prepare("SELECT id, message, created_on FROM alert")?;

    let alert_iter = stmt.query_map([], |row| {
        Ok(Alert {
            id: row.get(0)?,
            message: row.get(1)?,
            created_on: row.get(2)?,
        })
    })?;

    for alert in alert_iter {
        println!("alert: {:?}", alert.unwrap());
    }

    Ok(())

}