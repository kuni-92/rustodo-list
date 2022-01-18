use rusqlite::{Connection, Result, params};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ToDo {
    pub id: i32,
    pub content: String,
    pub finished: bool,
}

impl ToDo {
    pub fn new(content: String) -> Self {
        ToDo{id: 0, content, finished: false}
    }

    pub fn insert_db(content: &String, finished: bool) -> Result<()> {
        let conn = connect_to_db()?;
        conn.execute(
            "INSERT INTO todolist(content, finished) VALUES (?1, ?2)",
            params![content, finished as i32]
        )?;
        Ok(())
    }

    pub fn update_db(todo: &ToDo) -> Result<()> {
        let conn = connect_to_db()?;
        conn.execute(
            "UPDATE todolist set content = ?1, finished = ?2 WHERE id = ?3",
            params![todo.content, todo.finished as i32, todo.id]
        )?;
        Ok(())
    }
}

fn connect_to_db() -> Result<Connection> {
    Connection::open("./tododb.sqlite3")
}
