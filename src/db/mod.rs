use std::cmp::Reverse;
use std::fmt::Debug;
use rusqlite::{params, Connection, Result, Error, Statement};
use crate::alert::Alert;

use fallible_iterator::FallibleIterator;

#[derive(Debug)]
pub struct Database {
    inner: Connection,
}
// fn delete_alert(conn: &Connection, alert_id: i32) -> Result<()> {
//     conn.execute("DELETE FROM alert WHERE id = ?1", params![alert_id])?;

//     Ok(())
// }

impl Default for Database {
    fn default() -> Self {
        // let path = dirs::runtime_dir().expect("Cannot open runtime dir");
        
        // let path = path.join("my_db.db3");
        let path = "./my_db.db3";

        println!("Opening Database {:?}", path);

        Database {
            inner: Connection::open(path).unwrap(),
        }
    }
}
impl Database {

    pub fn insert(&self, alert: Alert) -> Result<usize> {
        println!("{:?}", alert.created_on.format("%Y-%m-%d %H:%M:%S").to_string());

        let id = self.inner.execute("
            INSERT INTO alert (message, created_on) 
            VALUES (?1, ?2)
        ", 
        params![alert.message, alert.created_on.format("%Y-%m-%d %H:%M:%S").to_string()]).unwrap();

        Ok(id)
    }

    pub fn query(&self) -> Result<Vec<Alert>> {
        let mut stmt = self.inner.prepare("SELECT id, message,created_on FROM alert")?;

        let result:Result<Vec<Alert>> = stmt.query([])?.map(
            |r| Ok(Alert { id: r.get(0)?, message: r.get(1)?, created_on: r.get(2)?, })).collect();

        result
    }
}