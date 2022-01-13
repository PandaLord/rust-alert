pub mod trigger;
pub mod alert;


use serde_json;
use tokio::*;
use chrono::prelude::*;
use rusqlite::{params, Connection, Result, Error, Statement};
use fallible_iterator::FallibleIterator;

use iced::{
    button, scrollable, slider, text_input, Button, Checkbox, Color, Column,
    Container, Element, HorizontalAlignment, Image, Length, Radio, Row,
    Sandbox, Scrollable, Settings, Slider, Space, Text, TextInput,
};

// fn open_my_db() -> Result<Connection> {
//     let path = "./my_db.db3";
//     let db = Connection::open(&path)?;
//     println!("open database flag: {}", db.is_autocommit());
//     Ok(db)
// }

// fn insert_alert(conn: &Connection, alert: Alert) -> Result<()> {
//     conn.execute("
//         INSERT INTO alert (message, created_on) 
//         VALUES (?1, ?2)
//     ", 
//     params![alert.message, alert.created_on])?;

//     Ok(())
// }
// fn delete_alert(conn: &Connection, alert_id: i32) -> Result<()> {
//     conn.execute("DELETE FROM alert WHERE id = ?1", params![alert_id])?;

//     Ok(())
// }

// fn query_alert(conn: &Connection) -> Result<Vec<String>> {
//     let mut stmt = conn.prepare("SELECT * FROM alert")?;
//     let result :Result<Vec<String>> = stmt.query([])?.map(|r| r.get(1)).collect();
    
//     result
// }

// more a State for the whole application
pub struct Alert {

    alert_message: String,
    input_message: String,
    alert_state: text_input::State,
    submit_button: button::State,

}

#[derive(Debug, Clone)]
pub enum Message {
    InputChanged(String),
    Submitted,
    Loaded,
}
// pub fn main() -> iced::Result {
//     dioxus::desktop::launch(app)
// }

impl Sandbox for Alert {
    type Message = Message;

    fn new() -> Alert {
        Alert {
            alert_message: String::new(),
            input_message: String::new(),
            submit_button: button::State::new(),
            alert_state: text_input::State::new(),
        }
    }

    fn title(&self) -> String {
        String::from("App")
    }
    fn view(&mut self) -> Element<Message>{
        let content = Column::new()
        .push(
            TextInput::new(
                &mut self.alert_state, 
                "please input the alert",
                &self.input_message,
                Message::InputChanged,
            )
        )
        .push(
            Text::new(&self.alert_message)
        )
        .push(
            Button::new(&mut self.submit_button, Text::new("Submit")).on_press(Message::Submitted)
        );
        Container::new(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x()
        .center_y()
        .into()
    }
    fn update(&mut self, message:Message) {
        match message {
            Message::InputChanged(value) => {
                self.input_message = value
            }
            Message::Submitted => {
                self.alert_message = self.input_message.to_owned()
            }
            _ => {
                self.alert_message = "NULL".to_owned()
            }
        }
    }
}
fn main() -> iced::Result {
    Alert::run(Settings::default())
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