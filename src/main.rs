pub mod trigger;
pub mod alert;


use serde_json;
use tokio::*;
use chrono::prelude::*;
use rusqlite::{params, Connection, Result, Error, Statement};
use alert::Alert;
use dioxus::prelude::*;
use fallible_iterator::FallibleIterator;

fn open_my_db() -> Result<Connection> {
    let path = "./my_db.db3";
    let db = Connection::open(&path)?;
    println!("open database flag: {}", db.is_autocommit());
    Ok(db)
}

fn insert_alert(conn: &Connection, alert: Alert) -> Result<()> {
    conn.execute("
        INSERT INTO alert (message, created_on) 
        VALUES (?1, ?2)
    ", 
    params![alert.message, alert.created_on])?;

    Ok(())
}
fn delete_alert(conn: &Connection, alert_id: i32) -> Result<()> {
    conn.execute("DELETE FROM alert WHERE id = ?1", params![alert_id])?;

    Ok(())
}

fn query_alert(conn: &Connection) -> Result<Vec<String>> {
    let mut stmt = conn.prepare("SELECT * FROM alert")?;
    let result :Result<Vec<String>> = stmt.query([])?.map(|r| r.get(1)).collect();
    
    result
}

fn main () {
    dioxus::desktop::launch(App)
}
fn App(cx:Scope) -> Element {
    cx.render(
        rsx!(
            div {"Hello World"}
        )
    )
}
// fn main() -> Result<()> {
//     let conn: Connection = open_my_db()?;

//     let table = match conn.execute(
//         "CREATE TABLE alert (
//             id  INTEGER PRIMARY KEY,
//             message  TEXT NOT NULL,
//             created_on DATETIME
//         )",
//         []
//     ) {
//         Ok(_) => println!("table creation succeed"),
//         Err(e) => println!("table is existed")
//     };

//     let test: Alert = Alert::new(String::from("test"));

//     insert_alert(&conn, test);
//     let query = query_alert(&conn).unwrap();

//     println!("{:?}", query);
//     // let mut stmt = conn.prepare("SELECT id, message, created_on FROM alert")?;

//     // let alert_iter = stmt.query_map([], |row| {
//     //     Ok(Alert {
//     //         id: row.get(0)?,
//     //         message: row.get(1)?,
//     //         created_on: row.get(2)?,
//     //     })
//     // })?;
//     // for alert in alert_iter {
//     //     delete_alert(&conn, alert.unwrap().id);
//     // }
//     Ok(())

// }