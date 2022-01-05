pub mod trigger;
pub mod data;
pub mod db_config;

use db_config::*;
use sea_orm::{DbErr, Database};
use serde_json;
use tokio::*;

#[tokio::main]
async fn main() -> Result<(), DbErr>{
    let db = Database::connect("sqlite://base.db").await.unwrap();

    println!("{:?}\n", db);
    let data: AlertData<&str>= AlertData::new("test");
    println!("data: {:?}", data);
    Ok(())
    // db.insert(b"name", serde_json::to_string(&data).unwrap().as_str()).unwrap();
    // println!("{:?}", db.get(b"name"));
    // assert_eq!(&db.get(b"name").unwrap().unwrap(), b"jacky");
    // println!("Hello, world!");
}
