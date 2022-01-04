pub mod trigger;
pub mod data;


use sled;
fn main() {
    let db : sled::Db  = sled::open("my_db").unwrap();

    db.insert(b"name", b"jacky");
    assert_eq!(&db.get(b"name").unwrap().unwrap(), b"jacky");
    println!("Hello, world!");
}
