pub mod trigger;
pub mod data;
pub mod alert;
pub mod entities;

use sea_orm::{DbErr, Database};
use entities::*;
use serde_json;
use tokio::*;
use chrono::prelude::*;
use sea_orm::{entity::*, error::*, DbConn};

#[tokio::main]
async fn main() -> Result<(), DbErr>{
    let db = Database::connect("sqlite://base.db").await.unwrap();

    println!("{:?}\n", db);

    let alert_data = alert::ActiveModel {
        alert_message: Set("Test".to_owned()),
        created_on: Set("2021".to_owned()),
        ..Default::default()
    };
    let res = Alert::insert(alert_data).exec(&db).await?;
    // let data: AlertData<&str>= AlertData::new("test");
    println!("data: {:?}", res);
    Ok(())
    // db.insert(b"name", serde_json::to_string(&data).unwrap().as_str()).unwrap();
    // println!("{:?}", db.get(b"name"));
    // assert_eq!(&db.get(b"name").unwrap().unwrap(), b"jacky");
    // println!("Hello, world!");
}
